#![allow(dead_code)]

// mostly skidded from the egui Slider type

use eframe::{
    egui,
    emath,
    epaint,
};
use emath::{
    Numeric,
    NumExt,
    Rect,
    Pos2,
    remap_clamp,
    vec2,
    pos2,
    lerp,
    remap,
};
use egui::{
    Color32,
    Widget,
    Response,
    TextStyle,
    Ui,
    Sense,
    Key,
    DragValue,
};
use std::ops::RangeInclusive;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<([f64; 2], usize)>) -> [f64; 2]>;


use std::f64::INFINITY;
const INF_RANGE_MAGNITUDE: f64 = 10.0;


#[derive(Clone)]
struct SliderSpec {
    logarithmic: bool,

    /// For logarithmic sliders, the smallest positive value we are interested in.
    /// 1 for integer sliders, maybe 1e-6 for others.
    smallest_positive: f64,

    /// For logarithmic sliders, the largest positive value we are interested in
    /// before the slider switches to `INFINITY`, if that is the higher end.
    /// Default: INFINITY.
    largest_finite: f64,
}

/// Specifies the orientation of a [`Slider`].
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

pub struct DoubleSlider<'a> {
    get_set_value: GetSetValue<'a>,
    range: RangeInclusive<f64>,
    spec: SliderSpec,
    clamp_to_range: bool,
    smart_aim: bool,
    show_value: bool,
    orientation: SliderOrientation,
    prefix: String,
    suffix: String,
    text: String,
    text_color: Option<Color32>,
    /// Sets the minimal step of the widget value
    step: Option<f64>,
    min_decimals: usize,
    max_decimals: Option<usize>,
}

impl <'a> DoubleSlider<'a> {
    pub fn new<Num: Numeric>(value: &'a mut [Num; 2], range: RangeInclusive<Num>) -> Self {
        let range_f64 = range.start().to_f64()..=range.end().to_f64();
        let slf = Self::from_get_set(range_f64, move |v: Option<([f64; 2], usize)>| {
            if let Some(v) = v {
                value[v.1] = Num::from_f64(v.0[v.1]);
            }
            [value[0].to_f64(), value[1].to_f64()]
        });

        if Num::INTEGRAL {
            slf.integer()
        } else {
            slf
        }
    }

    pub fn from_get_set(
        range: RangeInclusive<f64>,
        get_set_value: impl 'a + FnMut(Option<([f64; 2], usize)>) -> [f64; 2],
    ) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            range,
            spec: SliderSpec {
                logarithmic: false,
                smallest_positive: 1e-6,
                largest_finite: f64::INFINITY,
            },
            clamp_to_range: true,
            smart_aim: true,
            show_value: true,
            orientation: SliderOrientation::Horizontal,
            prefix: Default::default(),
            suffix: Default::default(),
            text: Default::default(),
            text_color: None,
            step: None,
            min_decimals: 0,
            max_decimals: None,
        }
    }

    pub fn integer(self) -> Self {
        self.fixed_decimals(0).smallest_positive(1.0)
    }

    pub fn fixed_decimals(mut self, num_decimals: usize) -> Self {
        self.min_decimals = num_decimals;
        self.max_decimals = Some(num_decimals);
        self
    }

    pub fn smallest_positive(mut self, smallest_positive: f64) -> Self {
        self.spec.smallest_positive = smallest_positive;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = SliderOrientation::Vertical;
        self
    }

    pub fn get_value(&mut self) -> [f64; 2] {
        (self.get_set_value)(None)
    }

    fn allocate_slider_space(&self, ui: &mut Ui, thickness: f32) -> Response {
        let desired_size = match self.orientation {
            SliderOrientation::Horizontal => vec2(ui.spacing().slider_width, thickness),
            SliderOrientation::Vertical => vec2(thickness, ui.spacing().slider_width),
        };
        ui.allocate_response(desired_size, Sense::click_and_drag())
    }

    fn handle_radius(&self, rect: &Rect) -> f32 {
        let limit = match self.orientation {
            SliderOrientation::Horizontal => rect.height(),
            SliderOrientation::Vertical => rect.width(),
        };
        limit / 2.5
    }

    fn position_range(&self, rect: &Rect) -> RangeInclusive<f32> {
        let handle_radius = self.handle_radius(rect);
        match self.orientation {
            SliderOrientation::Horizontal => {
                (rect.left() + handle_radius)..=(rect.right() - handle_radius)
            }
            SliderOrientation::Vertical => {
                (rect.bottom() - handle_radius)..=(rect.top() + handle_radius)
            }
        }
    }

    fn pointer_position(&self, pointer_position_2d: Pos2) -> f32 {
        match self.orientation {
            SliderOrientation::Horizontal => pointer_position_2d.x,
            SliderOrientation::Vertical => pointer_position_2d.y,
        }
    }

    fn value_from_position(&self, position: f64, position_range: RangeInclusive<f64>) -> f64 {
        let normalized = remap_clamp(position, position_range, 0.0..=1.0) as f64;
        value_from_normalized(normalized, self.range(), &self.spec)
    }

    fn range(&self) -> RangeInclusive<f64> {
        self.range.clone()
    }

    pub fn step_by(mut self, step: f64) -> Self {
        self.step = if step != 0.0 { Some(step) } else { None };
        self
    }

    /*
    pub fn min_decimals(mut self, min_decimals: usize) -> Self {
        self.min_decimals = min_decimals;
        self
    }
    */

    pub fn max_decimals(mut self, max_decimals: usize) -> Self {
        self.max_decimals = Some(max_decimals);
        self
    }

    fn set_value(&mut self, mut value: [f64; 2], i: usize) {
        if self.clamp_to_range {
            let start = *self.range.start();
            let end = *self.range.end();
            value[i] = value[i].clamp(start.min(end), start.max(end));
        }
        if let Some(max_decimals) = self.max_decimals {
            value[i] = emath::round_to_decimals(value[i], max_decimals);
        }
        if let Some(step) = self.step {
            value[i] = (value[i] / step).round() * step;
        }

        // make sure `value[0]` is always smaller than `value[1]`
        if value[0] > value[1] {
            return;
        }

        set(&mut self.get_set_value, value, i);
    }

    fn rail_rect(&self, rect: &Rect, radius: f32) -> Rect {
        match self.orientation {
            SliderOrientation::Horizontal => Rect::from_min_max(
                pos2(rect.left(), rect.center().y - radius),
                pos2(rect.right(), rect.center().y + radius),
            ),
            SliderOrientation::Vertical => Rect::from_min_max(
                pos2(rect.center().x - radius, rect.top()),
                pos2(rect.center().x + radius, rect.bottom()),
            ),
        }
    }

    fn slider_ui(&mut self, ui: &mut Ui, response: &Response) {
        let rect = &response.rect;
        let position_range = {
            let og = self.position_range(rect);
            *og.start() as f64..=*og.end() as f64
        };

        if let Some(pointer_position_2d) = response.interact_pointer_pos() {
            let position = self.pointer_position(pointer_position_2d) as f64;
            let new_value = if self.smart_aim {
                let aim_radius = ui.input(|i| i.aim_radius()) as f64;
                emath::smart_aim::best_in_range_f64(
                    self.value_from_position(position - aim_radius, position_range.clone()),
                    self.value_from_position(position + aim_radius, position_range.clone()),
                )
            } else {
                self.value_from_position(position, position_range.clone())
            };
            // get the handle closest to `position` and set the value of that handle
            // ily copilot
            let mut closest_handle_distance = f64::INFINITY;
            let mut closest_handle_index = 0;
            for i in 0..2 {
                let handle_position = self.handle_position(i, position_range.clone());
                let click_position = self.pointer_position(pointer_position_2d) as f64;
                let handle_distance = (handle_position - click_position).abs();
                if handle_distance < closest_handle_distance {
                    closest_handle_distance = handle_distance;
                    closest_handle_index = i;
                }
            }
            self.set_value([new_value; 2], closest_handle_index);
        }


        if response.has_focus() {
            let (dec_key, inc_key) = match self.orientation {
                SliderOrientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
                // Note that this is for moving the slider position,
                // so up = decrement y coordinate:
                SliderOrientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
            };

            let decrement = ui.input(|i| i.num_presses(dec_key));
            let increment = ui.input(|i| i.num_presses(inc_key));
            let kb_step = increment as f64 - decrement as f64;

            if kb_step != 0.0 {
                let prev_value = self.get_value();
                let prev_position = self.position_from_value(prev_value, position_range.clone());
                for i in 0..2 {
                    let new_position = prev_position[i] + kb_step;
                    let new_value = match self.step {
                        Some(step) => prev_value[i] + (kb_step as f64 * step),
                        None if self.smart_aim => {
                            let aim_radius = ui.input(|i| i.aim_radius()) as f64;
                            emath::smart_aim::best_in_range_f64(
                                self.value_from_position(
                                    new_position - aim_radius,
                                    position_range.clone(),
                                ),
                                self.value_from_position(
                                    new_position + aim_radius,
                                    position_range.clone(),
                                ),
                            )
                        },
                        _ => self.value_from_position(new_position, position_range.clone()),
                    };
                    self.set_value([new_value; 2], i);
                }
            }
        }

        // Paint it:
        if ui.is_rect_visible(response.rect) {
            let value = self.get_value();

            let rail_radius = ui.painter().round_to_pixel(self.rail_radius_limit(rect));
            let rail_rect = self.rail_rect(rect, rail_radius);

            let visuals = ui.style().interact(response);
            ui.painter().add(epaint::RectShape {
                rect: rail_rect,
                rounding: ui.visuals().widgets.inactive.rounding,
                fill: ui.visuals().widgets.inactive.bg_fill,
                // fill: visuals.bg_fill,
                // fill: ui.visuals().extreme_bg_color,
                stroke: Default::default(),
                // stroke: visuals.bg_stroke,
                // stroke: ui.visuals().widgets.inactive.bg_stroke,
            });

            let position_1d_array = self.position_from_value(value, position_range);

            for i in 0..2 {
                let center = self.marker_center(position_1d_array[i] as f32, &rail_rect);
                ui.painter().add(epaint::CircleShape {
                    center,
                    radius: self.handle_radius(rect) + visuals.expansion,
                    fill: visuals.bg_fill,
                    stroke: visuals.fg_stroke,
                });
            }
        }
    }

    // ily copilot
    fn handle_position(&mut self, i: usize, position_range: RangeInclusive<f64>) -> f64 {
        let value = self.get_value();
        self.position_from_value(value, position_range)[i]
    }

    fn rail_radius_limit(&self, rect: &Rect) -> f32 {
        match self.orientation {
            SliderOrientation::Horizontal => (rect.height() / 4.0).at_least(2.0),
            SliderOrientation::Vertical => (rect.width() / 4.0).at_least(2.0),
        }
    }

    fn marker_center(&self, position_1d: f32, rail_rect: &Rect) -> Pos2 {
        match self.orientation {
            SliderOrientation::Horizontal => pos2(position_1d, rail_rect.center().y),
            SliderOrientation::Vertical => pos2(rail_rect.center().x, position_1d),
        }
    }

    fn value_ui(&mut self, ui: &mut Ui, position_range: RangeInclusive<f64>) -> [Response; 2] {
        // If [`DragValue`] is controlled from the keyboard and `step` is defined, set speed to `step`
        let change = {
            // Hold one lock rather than 4 (see https://github.com/emilk/egui/pull/1380).

            ui.input(|i| i.num_presses(Key::ArrowUp) as i32) + ui.input(|i| i.num_presses(Key::ArrowRight)) as i32
                - ui.input(|i| i.num_presses(Key::ArrowDown)) as i32
                - ui.input(|i| i.num_presses(Key::ArrowLeft)) as i32
        };
        let speed = match self.step {
            Some(step) if change != 0 => [step; 2],
            _ => self.current_gradient(&position_range),
        };
        let mut value = self.get_value();
        [
            {
                let response = ui.add({
                    DragValue::new(&mut value[0])
                        .speed(speed[0])
                        .clamp_range(self.clamp_range())
                        .min_decimals(self.min_decimals)
                        .max_decimals_opt(self.max_decimals)
                        .suffix(self.suffix.clone())
                        .prefix(self.prefix.clone())
                });
                if value[0] != self.get_value()[0] {
                    self.set_value(value, 0);
                }
                response
            },
            {
                let response = ui.add({
                    DragValue::new(&mut value[1])
                        .speed(speed[1])
                        .clamp_range(self.clamp_range())
                        .min_decimals(self.min_decimals)
                        .max_decimals_opt(self.max_decimals)
                        .suffix(self.suffix.clone())
                        .prefix(self.prefix.clone())
                });
                if value[1] != self.get_value()[1] {
                    self.set_value(value, 1);
                }
                response
            }
        ]
    }

    fn clamp_range(&self) -> RangeInclusive<f64> {
        if self.clamp_to_range {
            self.range()
        } else {
            f64::NEG_INFINITY..=f64::INFINITY
        }
    }

    fn current_gradient(&mut self, position_range: &RangeInclusive<f64>) -> [f64; 2] {
        // TODO: handle clamping
        let value = self.get_value();
        let value_from_pos =
            |position: f64| self.value_from_position(position, position_range.clone());
        let pos_from_value = |value: [f64; 2]| self.position_from_value(value, position_range.clone());
        [
            {
                let left_value = value_from_pos(pos_from_value(value)[0] - 0.5);
                let right_value = value_from_pos(pos_from_value(value)[0] + 0.5);
                right_value - left_value
            },
            {
                let left_value = value_from_pos(pos_from_value(value)[1] - 0.5);
                let right_value = value_from_pos(pos_from_value(value)[1] + 0.5);
                right_value - left_value
            }
        ]
    }

    pub fn text_color(mut self, text_color: Color32) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn logarithmic(mut self, logarithmic: bool) -> Self {
        self.spec.logarithmic = logarithmic;
        self
    }

    fn position_from_value(&self, value: [f64; 2], position_range: RangeInclusive<f64>) -> [f64; 2] {
        [{
            let normalized = normalized_from_value(value[0], self.range(), &self.spec);
            lerp(position_range.clone(), normalized)
        },
        {
            let normalized = normalized_from_value(value[1], self.range(), &self.spec);
            lerp(position_range, normalized)
        }]
    }

    fn add_contents(&mut self, ui: &mut Ui) -> Response {
        let thickness = ui
            .text_style_height(&TextStyle::Body)
            .at_least(ui.spacing().interact_size.y);
        let mut response = self.allocate_slider_space(ui, thickness);
        self.slider_ui(ui, &response);

        if self.show_value {
            let position_range = {
                let og = self.position_range(&response.rect);
                *og.start() as f64..=*og.end() as f64
            };
            let value_response = self.value_ui(ui, position_range);
            for i in 0..value_response.len() {
                if value_response[i].gained_focus()
                || value_response[i].has_focus()
                || value_response[i].lost_focus()
                {
                    // Use the [`DragValue`] id as the id of the whole widget,
                    // so that the focus events work as expected.
                    response = value_response[i].union(response);
                } else {
                    // Use the slider id as the id for the whole widget
                    response = response.union(value_response[i].clone());
                }
            }
        }

        if !self.text.is_empty() {
            let text_color = self.text_color.unwrap_or_else(|| ui.visuals().text_color());
            let text = egui::RichText::new(&self.text).color(text_color);
            ui.add(egui::widgets::Label::new(text).wrap(false));
        }

        response
    }
}

impl <'a> Widget for DoubleSlider<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> Response {
        let old_value: [f64; 2] = self.get_value();

        let inner_response = match self.orientation {
            SliderOrientation::Horizontal => ui.horizontal(|ui| self.add_contents(ui)),
            SliderOrientation::Vertical => ui.vertical(|ui| self.add_contents(ui)),
        };

        let mut response = inner_response.inner | inner_response.response;
        let value = self.get_value();
        response.changed = value != old_value;
        response
    }
}

fn value_from_normalized(normalized: f64, range: RangeInclusive<f64>, spec: &SliderSpec) -> f64 {
    let (min, max) = (*range.start(), *range.end());

    if min.is_nan() || max.is_nan() {
        f64::NAN
    } else if min == max {
        min
    } else if min > max {
        value_from_normalized(1.0 - normalized, max..=min, spec)
    } else if normalized <= 0.0 {
        min
    } else if normalized >= 1.0 {
        max
    } else if spec.logarithmic {
        if max <= 0.0 {
            // non-positive range
            -value_from_normalized(normalized, -min..=-max, spec)
        } else if 0.0 <= min {
            let (min_log, max_log) = range_log10(min, max, spec);
            let log = lerp(min_log..=max_log, normalized);
            10.0_f64.powf(log)
        } else {
            assert!(min < 0.0 && 0.0 < max);
            let zero_cutoff = logaritmic_zero_cutoff(min, max);
            if normalized < zero_cutoff {
                // negative
                value_from_normalized(
                    remap(normalized, 0.0..=zero_cutoff, 0.0..=1.0),
                    min..=0.0,
                    spec,
                )
            } else {
                // positive
                value_from_normalized(
                    remap(normalized, zero_cutoff..=1.0, 0.0..=1.0),
                    0.0..=max,
                    spec,
                )
            }
        }
    } else {
        assert!(
            min.is_finite() && max.is_finite(),
            "You should use a logarithmic range"
        );
        lerp(range, normalized.clamp(0.0, 1.0))
    }
}

fn logaritmic_zero_cutoff(min: f64, max: f64) -> f64 {
    assert!(min < 0.0 && 0.0 < max);

    let min_magnitude = if min == -INFINITY {
        INF_RANGE_MAGNITUDE
    } else {
        min.abs().log10().abs()
    };
    let max_magnitude = if max == INFINITY {
        INF_RANGE_MAGNITUDE
    } else {
        max.log10().abs()
    };

    let cutoff = min_magnitude / (min_magnitude + max_magnitude);
    assert!((0.0..=1.0).contains(&cutoff));
    cutoff
}

fn range_log10(min: f64, max: f64, spec: &SliderSpec) -> (f64, f64) {
    assert!(spec.logarithmic);
    assert!(min <= max);

    if min == 0.0 && max == INFINITY {
        (spec.smallest_positive.log10(), INF_RANGE_MAGNITUDE)
    } else if min == 0.0 {
        if spec.smallest_positive < max {
            (spec.smallest_positive.log10(), max.log10())
        } else {
            (max.log10() - INF_RANGE_MAGNITUDE, max.log10())
        }
    } else if max == INFINITY {
        if min < spec.largest_finite {
            (min.log10(), spec.largest_finite.log10())
        } else {
            (min.log10(), min.log10() + INF_RANGE_MAGNITUDE)
        }
    } else {
        (min.log10(), max.log10())
    }
}

fn set(get_set_value: &mut GetSetValue<'_>, value: [f64; 2], i: usize) {
    (get_set_value)(Some((value, i)));
}

fn normalized_from_value(value: f64, range: RangeInclusive<f64>, spec: &SliderSpec) -> f64 {
    let (min, max) = (*range.start(), *range.end());

    if min.is_nan() || max.is_nan() {
        f64::NAN
    } else if min == max {
        0.5 // empty range, show center of slider
    } else if min > max {
        1.0 - normalized_from_value(value, max..=min, spec)
    } else if value <= min {
        0.0
    } else if value >= max {
        1.0
    } else if spec.logarithmic {
        if max <= 0.0 {
            // non-positive range
            normalized_from_value(-value, -min..=-max, spec)
        } else if 0.0 <= min {
            let (min_log, max_log) = range_log10(min, max, spec);
            let value_log = value.log10();
            remap_clamp(value_log, min_log..=max_log, 0.0..=1.0)
        } else {
            assert!(min < 0.0 && 0.0 < max);
            let zero_cutoff = logaritmic_zero_cutoff(min, max);
            if value < 0.0 {
                // negative
                remap(
                    normalized_from_value(value, min..=0.0, spec),
                    0.0..=1.0,
                    0.0..=zero_cutoff,
                )
            } else {
                // positive side
                remap(
                    normalized_from_value(value, 0.0..=max, spec),
                    0.0..=1.0,
                    zero_cutoff..=1.0,
                )
            }
        }
    } else {
        assert!(
            min.is_finite() && max.is_finite(),
            "You should use a logarithmic range"
        );
        remap_clamp(value, range, 0.0..=1.0)
    }
}
use eframe::{egui::{self, TextEdit, TextStyle}, epaint::Color32};

fn keybind_ui(ui: &mut egui::Ui, key: &mut String) -> egui::Response {
    let castable = {
        let prefix_removed = key.trim_start_matches("0x");
        u32::from_str_radix(prefix_removed, 16).is_ok()
    };
    let text_widget = TextEdit::singleline(key).font(TextStyle::Monospace);
    // if castable to u32
    let text_widget = if !castable {
        text_widget.text_color(Color32::RED)
    } else {
        text_widget
    };
    ui.add(text_widget)
}

pub fn keybind_widget(key: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| keybind_ui(ui, key)
}
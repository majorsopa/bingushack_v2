use eframe::egui::{self, TextEdit, TextStyle};

fn keybind_ui(ui: &mut egui::Ui, key: &mut String) -> egui::Response {
    ui.add(
        TextEdit::singleline(key).font(TextStyle::Monospace)
    )
}

pub fn keybind_widget(key: &mut String) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| keybind_ui(ui, key)
}
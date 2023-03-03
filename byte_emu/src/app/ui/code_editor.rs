use crate::app::ByteEmuApp;

impl ByteEmuApp {
    pub fn show_code_editor(&mut self, ctx: &egui::Context) {
        egui::Window::new("code editor")
            .default_size(egui::vec2(666.0, 588.0))
            .default_pos(egui::pos2(693.0, 216.0))
            .show(ctx, |ui| {
                self.ui_code_editor(ui);
            });
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.state.text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        });
    }
}

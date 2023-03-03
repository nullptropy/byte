use crate::app::ByteEmuApp;

const ABOUT_TEXT: &str = "\
Byte is a fantasy console that runs on the 6502 microprocessor\
and features a compact 64x64 screen and an 8-key gamepad keyboard.\
Its primary purpose is to provide a user-friendly platform for learning\
6502 assembly language programming, with the goal of lowering the barrier\
to entry for aspiring developers.";

impl ByteEmuApp {
    pub fn show_about(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_about_open;
        egui::Window::new("About")
            .open(&mut open)
            .default_width(460.0)
            .show(ctx, |ui| {
                self.ui_about(ui);
            });
        self.state.is_about_open = open;
    }

    fn ui_about(&mut self, ui: &mut egui::Ui) {
        ui.heading("Byte");
        ui.add_space(12.0);
        ui.label(ABOUT_TEXT);
        ui.add_space(12.0);
        ui.hyperlink_to("gh/brkp/byte", "https://github.com/brkp/byte");
    }
}

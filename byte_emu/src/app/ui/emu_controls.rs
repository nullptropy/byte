use crate::app::ByteEmuApp;

impl ByteEmuApp {
    pub fn show_emu_controls(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_memory_monitor_open;
        egui::Window::new("Emulator Controls")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("TODO");
            });
        self.state.is_memory_monitor_open= open;
    }
}

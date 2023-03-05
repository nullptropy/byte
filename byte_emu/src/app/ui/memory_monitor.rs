use crate::app::ByteEmuApp;

impl ByteEmuApp {
    pub fn show_memory_monitor(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_memory_monitor_open;
        egui::Window::new("Memory Monitor")
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("TODO");
            });
        self.state.is_memory_monitor_open= open;
    }
}

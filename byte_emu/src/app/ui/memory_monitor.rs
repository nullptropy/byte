use crate::app::ByteEmuApp;

impl ByteEmuApp {
    pub fn show_memory_monitor(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_memory_monitor_open;
        egui::Window::new("Memory Monitor")
            .open(&mut open)
            .show(ctx, |ui| {
                self.ui_memory_monitor(ui);
            });
        self.state.is_memory_monitor_open = open;
    }

    fn ui_memory_monitor(&mut self, ui: &mut egui::Ui) {
        let start_str = &mut self.state.memory_window_range_str.0;
        let end_str = &mut self.state.memory_window_range_str.1;

        ui.horizontal(|ui| {
            ui.label("Start: ");
            ui.text_edit_singleline(start_str);
        });
        ui.horizontal(|ui| {
            ui.label("  End: ");
            ui.text_edit_singleline(end_str);
        });

        if let (Ok(start), Ok(end)) = (
            u16::from_str_radix(start_str.trim_start_matches("0x"), 16),
            u16::from_str_radix(end_str.trim_start_matches("0x"), 16),
        ) {
            self.state.memory_window_range = (start, end);
        }

        if let Some(mem_slice) = self
            .emu
            .cpu
            .bus
            .get_memory_region(self.state.memory_window_range)
        {
            self.state.memory_window_text_area.clear();
            mem_slice.chunks(16).for_each(|chunk| {
                self.state
                    .memory_window_text_area
                    .push_str(format!("{:02X?}\n", chunk).as_str());
            });

            egui::ScrollArea::both().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.state.memory_window_text_area)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_width(f32::INFINITY),
                );
            });
        }
    }
}

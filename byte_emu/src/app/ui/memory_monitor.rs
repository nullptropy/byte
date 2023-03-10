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
        let addr_str = &mut self.state.memory_window_range_str.0;
        let size_str = &mut self.state.memory_window_range_str.1;

        ui.horizontal(|ui| {
            ui.label("addr: ");
            ui.text_edit_singleline(addr_str);
        });
        ui.horizontal(|ui| {
            ui.label("size: ");
            ui.text_edit_singleline(size_str);
        });

        if let (Ok(start), Ok(end)) = (
            u16::from_str_radix(addr_str.trim_start_matches("0x"), 16),
            u16::from_str_radix(size_str.trim_start_matches("0x"), 16),
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
            let mut counter = self.state.memory_window_range.0;

            mem_slice.chunks(16).for_each(|chunk| {
                let ascii = format!(
                    "{: <16}",
                    chunk
                        .iter()
                        .map(|b| match b {
                            0 => '.',
                            _ => *b as char,
                        })
                        .collect::<String>()
                );
                self.state
                    .memory_window_text_area
                    .push_str(format!("{counter:04X}: {chunk:02X?} |{ascii}|\n").as_str());
                counter += chunk.len() as u16;
            });
        }

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

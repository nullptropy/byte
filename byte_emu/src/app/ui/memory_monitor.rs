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

        ui.scope(|ui| {
            ui.style_mut().override_font_id = Some(egui::FontId::monospace(14.0));

            ui.horizontal(|ui| {
                ui.label("addr:");
                ui.text_edit_singleline(addr_str);
            });
            ui.horizontal(|ui| {
                ui.label("size:");
                ui.text_edit_singleline(size_str);
            });

            ui.add_space(14.0);
        });

        if let (Ok(start), Ok(end)) = (
            u16::from_str_radix(addr_str.trim_start_matches("0x"), 16),
            u16::from_str_radix(size_str.trim_start_matches("0x"), 16),
        ) {
            self.state.memory_window_range = (start, end);
        }

        let mut counter = self.state.memory_window_range.0;
        let mem_slice = self.emu.get_memory_region(self.state.memory_window_range);
        self.state.memory_window_text_area.clear();

        mem_slice.chunks(16).for_each(|chunk| {
            let ascii = format!(
                "{: <16}",
                chunk
                    .iter()
                    .map(|b| match b {
                        0x20..=0x7e => *b as char,
                        _ => '.',
                    })
                    .collect::<String>()
            );
            let bytes = format!("{chunk:02x?}").replace(',', "");
            let output = format!(
                "{counter:04x}: {: <47} |{ascii}|\n",
                &bytes[1..bytes.len() - 1]
            );
            self.state.memory_window_text_area.push_str(output.as_str());

            counter += chunk.len() as u16;
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.state.memory_window_text_area)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        });
    }
}

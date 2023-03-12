use crate::app::ByteEmuApp;

impl ByteEmuApp {
    pub fn show_memory_monitor(&mut self, ctx: &egui::Context) {
        let mut open = self.state.is_memory_monitor_open;
        egui::Window::new("Memory Monitor")
            .open(&mut open)
            .default_width(610.0)
            .show(ctx, |ui| {
                self.ui_memory_monitor(ui);
            });
        self.state.is_memory_monitor_open = open;
    }

    fn ui_memory_monitor(&mut self, ui: &mut egui::Ui) {
        let addr_str = &mut self.state.memory_window_range_str.0;
        let size_str = &mut self.state.memory_window_range_str.1;

        ui.style_mut().override_font_id = Some(egui::FontId::monospace(14.0));
        ui.horizontal(|ui| {
            ui.label("addr:");
            ui.text_edit_singleline(addr_str);
        });
        ui.horizontal(|ui| {
            ui.label("size:");
            ui.text_edit_singleline(size_str);
        });
        ui.add_space(10.0);

        if let (Ok(start), Ok(size)) = (
            u16::from_str_radix(addr_str.trim_start_matches("0x"), 16),
            u16::from_str_radix(size_str.trim_start_matches("0x"), 16),
        ) {
            self.state.memory_window_range = (start, size.saturating_sub(1));
        }

        let mut counter = self.state.memory_window_range.0;
        let mem_slice = self.emu.get_memory_region(self.state.memory_window_range);
        self.state.memory_window_text_area.clear();

        egui::ScrollArea::both().show(ui, |ui| {
            ui.vertical(|ui| {
                mem_slice.chunks(16).for_each(|chunk| {
                    let ascii = format!(
                        "{: <16}",
                        chunk
                            .iter()
                            .map(|b| match b {
                                // printable range
                                0x20..=0x7e => *b as char,
                                _ => '.',
                            })
                            .collect::<String>()
                    );
                    let mut bytes = format!("{chunk:02x?}").replace(['[', ']', ','], "");
                    if bytes.len() > 24 {
                        bytes.insert(24, ' ');
                    }

                    ui.add(
                        egui::Label::new(format!("{counter:04x}: {: <48}  |{ascii}|", bytes))
                            .wrap(false),
                    );

                    counter += chunk.len() as u16;
                });
            });
        });
    }
}

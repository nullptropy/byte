use crate::app::ByteEmuApp;
use egui::{Color32, ColorImage};

impl ByteEmuApp {
    pub fn show_byte_console(&mut self, ctx: &egui::Context) {
        egui::Window::new("byte console")
            .default_pos(egui::pos2(170.0, 125.0))
            .show(ctx, |ui| {
                self.ui_byte_console(ui);
            });
    }

    fn ui_byte_console(&mut self, ui: &mut egui::Ui) {
        let pixels = self.framebuffer();
        self.texture.set(pixels, egui::TextureOptions::NEAREST);
        ui.image(self.texture.id(), egui::vec2(320.0, 320.0));
    }

    fn framebuffer(&mut self) -> ColorImage {
        let pixels = self
            .emu
            .framebuffer()
            .iter()
            .map(|c| {
                let [r, g, b, a] = c.to_be_bytes();
                Color32::from_rgba_unmultiplied(r, g, b, a)
            })
            .collect::<Vec<Color32>>();

        ColorImage {
            size: [64, 64],
            pixels,
        }
    }
}

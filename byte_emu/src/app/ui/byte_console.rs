use crate::app::ByteEmuApp;
use egui::{Color32, ColorImage};

impl ByteEmuApp {
    pub fn show_byte_console(&mut self, ctx: &egui::Context) {
        let framebuffer = self.framebuffer();
        self.texture.set(framebuffer, egui::TextureOptions::NEAREST);

        egui::Window::new("byte console")
            .resizable(false)
            .default_pos(egui::pos2(170.0, 125.0))
            .show(ctx, |ui| {
                self.ui_byte_console(ui);
            });
    }

    fn ui_byte_console(&mut self, ui: &mut egui::Ui) {
        const M: f32 = 14.0;  // margin
        const A: f32 = 36.0;  // height
        const B: f32 = 20.0;  // width
        const S: f32 = 320.0; // screen size
        const K: f32 = S - 2.0 * M - 4.0 * A - B - B / 4.0; // uhh

        #[rustfmt::skip]
        ui.vertical(|ui| {
            ui.add_space(M);
            ui.horizontal(|ui| {
                ui.add_space(M);
                ui.image(self.texture.id(), egui::vec2(S, S));
                ui.add_space(M);
            });
            ui.add_space(M * 3.0);

            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                ui.horizontal(|ui| {
                    ui.add_space(A + M * 2.0);         btn(ui, [B, A]);
                });
                ui.horizontal(|ui| {
                    ui.add_space(M * 2.0);             btn(ui, [A, B]);
                    ui.add_space(B);                   btn(ui, [A, B]);
                    ui.add_space(K);                   btn(ui, [A, B]);
                    ui.add_space(B / 4.0);             btn(ui, [A, B]);
                });
                ui.horizontal(|ui| {
                    ui.add_space(A + M * 2.0);         btn(ui, [B, A]);
                    ui.add_space((K - B / 4.0) / 2.0); btn(ui, [A, B]);
                    ui.add_space(B / 4.0);             btn(ui, [A, B]);
                });
            });

            ui.add_space(M * 2.0);
        });
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

fn btn(ui: &mut egui::Ui, size: impl Into<egui::Vec2>) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size.into(), egui::Sense::click());
    let visuals = ui.style().interact(&response);

    if ui.is_rect_visible(rect) {
        ui.painter()
            .rect(rect, 1.0, visuals.bg_fill, visuals.bg_stroke);
    }

    response
}

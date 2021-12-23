use crate::{app::ByteEmuApp, emu::core::ByteInputState};
use egui::{load::SizedTexture, Color32, ColorImage};

const M: f32 = 14.0; // margin
const A: f32 = 36.0; // height
const B: f32 = 20.0; // width
const S: f32 = 320.0; // screen size
const K: f32 = S - 2.0 * M - 4.0 * A - B - B / 4.0; // uhh

impl ByteEmuApp {
    pub fn show_byte_console(&mut self, ctx: &egui::Context, input_state: &mut ByteInputState) {
        let framebuffer = self.framebuffer();
        self.texture.set(framebuffer, egui::TextureOptions::NEAREST);

        egui::Window::new("byte console")
            .resizable(false)
            .default_pos(egui::pos2(170.0, 125.0))
            .show(ctx, |ui| {
                // hacky way to ensure that we update the
                // input_state only when this window is focused
                let current_layer = ui.layer_id();
                let top_middle_layer = ctx.memory(|mem| {
                    mem.layer_ids()
                        .filter(|l| l.order == egui::layers::Order::Middle)
                        .last()
                });
                if Some(current_layer) == top_middle_layer {
                    input_state.insert(ctx.input(|i| i.keys_down.clone()).into());
                }

                self.ui_byte_console(ui, input_state);
            });
    }

    // TODO: this kinda sucks, find some other way to draw the gamepad ui
    fn ui_byte_console(&mut self, ui: &mut egui::Ui, input_state: &mut ByteInputState) {
        #[rustfmt::skip]
        ui.vertical(|ui| {
            ui.add_space(M);
            ui.horizontal(|ui| {
                ui.add_space(M);
                ui.image(SizedTexture::new(self.texture.id(), egui::vec2(S, S)));
                ui.add_space(M);
            });
            ui.add_space(M * 3.0);

            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                ui.horizontal(|ui| {
                    ui.add_space(A + M * 2.0);         btn(ui, [B, A], input_state, ByteInputState::UP);
                });
                ui.horizontal(|ui| {
                    ui.add_space(M * 2.0);             btn(ui, [A, B], input_state, ByteInputState::LEFT);
                    ui.add_space(B);                   btn(ui, [A, B], input_state, ByteInputState::RIGHT);
                    ui.add_space(K);                   btn(ui, [A, B], input_state, ByteInputState::A);
                    ui.add_space(B / 4.0);             btn(ui, [A, B], input_state, ByteInputState::B);
                });
                ui.horizontal(|ui| {
                    ui.add_space(A + M * 2.0);         btn(ui, [B, A], input_state, ByteInputState::DOWN);
                    ui.add_space((K - B / 4.0) / 2.0); btn(ui, [A, B], input_state, ByteInputState::SELECT);
                    ui.add_space(B / 4.0);             btn(ui, [A, B], input_state, ByteInputState::START);
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

fn btn(
    ui: &mut egui::Ui,
    size: impl Into<egui::Vec2>,
    input_state: &mut ByteInputState,
    state: ByteInputState,
) {
    let (rect, response) = ui.allocate_exact_size(size.into(), egui::Sense::click_and_drag());
    let visuals = if input_state.contains(state) {
        &ui.style().visuals.widgets.active
    } else {
        ui.style().interact(&response)
    };

    if ui.is_rect_visible(rect) {
        ui.painter()
            .rect(rect, 1.0, visuals.bg_fill, visuals.bg_stroke);
    }

    if response.is_pointer_button_down_on() {
        input_state.insert(state);
    }
}

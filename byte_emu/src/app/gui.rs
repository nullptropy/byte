use super::file_diag::FileProcesser;
use crate::emu::ByteEmu;

use egui::{Color32, ColorImage, Context, Visuals};

const DEFAULT_BINARY: &[u8; 1 << 16] = include_bytes!("../../assets/static.bin");
const DEFAULT_SOURCE: &str = include_str!("../../assets/static.s");

#[derive(Debug)]
pub enum FileProcesserMessage {
    BinaryFile((String, Vec<u8>)),
    SourceFile((String, Vec<u8>)),
}

// `State` that we would like to persist (serialize).
#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    text: String,
}

pub struct ByteEmuApp {
    emu: ByteEmu,
    file_processer: FileProcesser<FileProcesserMessage>,
    frame_history: super::frame_history::FrameHistory,
    state: State,
    texture: Option<egui::TextureHandle>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: DEFAULT_SOURCE.to_string(),
        }
    }
}

impl Default for ByteEmuApp {
    fn default() -> Self {
        let mut emu = ByteEmu::default();
        let mut state = State::default();

        emu.load_program(DEFAULT_BINARY, 0x0000);
        state.text = DEFAULT_SOURCE.to_string();

        Self {
            emu,
            file_processer: FileProcesser::new(),
            frame_history: Default::default(),
            state,
            texture: None,
        }
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.show_ui(ctx, frame);
        self.process_files();
        self.step_emulator(ctx.input(|i| i.keys_down.iter().next().copied()))
    }
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, program: Option<(Vec<u8>, u16)>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        let mut app = Self::default();

        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                app.state = state;
            }
        }
        if let Some((program, start)) = program {
            app.emu.load_program(&program, start);
        }

        app
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

    fn step_emulator(&mut self, key_pressed: Option<egui::Key>) {
        self.emu.step(key_pressed);
    }

    fn process_files(&mut self) {
        self.file_processer
            .consume_messages()
            .iter()
            .for_each(|m| match m {
                FileProcesserMessage::BinaryFile((_, data)) => {
                    // load the program
                    // and then issue a RST interrupt
                    self.emu.load_program(data, 0x0000);
                }
                FileProcesserMessage::SourceFile((_, data)) => {
                    self.state.text = String::from_utf8_lossy(data).to_string()
                }
            });
    }

    fn show_ui(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let top = |name: &str| egui::TopBottomPanel::top(name.to_string());
        let win = |name: &str| egui::Window::new(name);

        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        top("top").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });
        win("screen")
            .default_pos(egui::pos2(170.0, 125.0))
            .show(ctx, |ui| {
                self.show_pixel_buffer(ui);
            });
        win("code editor")
            .default_size(egui::vec2(666.0, 588.0))
            .default_pos(egui::pos2(693.0, 216.0))
            .show(ctx, |ui| {
                self.show_code_editor(ui);
            });
        win("performance")
            .default_pos(egui::pos2(58.0, 700.0))
            .show(ctx, |ui| {
                self.frame_history.ui(ui);
            });

        ctx.request_repaint();
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                use FileProcesserMessage::*;

                if ui.button("Load binary program").clicked() {
                    self.file_processer
                        .read(|name, data| BinaryFile((name, data)));
                    ui.close_menu();
                }
                if ui.button("Load source file").clicked() {
                    self.file_processer
                        .read(|name, data| SourceFile((name, data)));
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Reset GUI state").clicked() {
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
                if ui.button("Reset everything").clicked() {
                    self.state = State::default();
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
            });
        });
    }

    fn show_code_editor(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.state.text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        });
    }

    fn show_pixel_buffer(&mut self, ui: &mut egui::Ui) {
        let pixels = self.framebuffer();
        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "framebuffer",
                ColorImage::new([32, 32], Color32::BLACK),
                Default::default(),
            )
        });

        texture.set(pixels, egui::TextureOptions::NEAREST);
        ui.image(texture.id(), egui::vec2(320.0, 320.0));
    }
}

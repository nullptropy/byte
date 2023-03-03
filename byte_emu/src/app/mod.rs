mod file_processor;
mod ui;

use crate::{emu::ByteEmu, DEFAULT_BINARY, DEFAULT_SOURCE};
use file_processor::FileProcesser;

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
    frame_history: ui::frame_history::FrameHistory,
    state: State,
    texture: egui::TextureHandle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: DEFAULT_SOURCE.to_string(),
        }
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        self.show_menu_bar(ctx);
        self.show_byte_console(ctx);
        self.show_code_editor(ctx);
        self.show_frame_history(ctx);

        self.process_files();
        self.step_emulator(ctx.input(|i| i.keys_down.iter().next().copied()))
    }
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, program: Option<(Vec<u8>, u16)>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut app = Self {
            emu: ByteEmu::default(),
            file_processer: FileProcesser::new(),
            frame_history: Default::default(),
            state: State::default(),
            texture: cc.egui_ctx.load_texture(
                "framebuffer",
                egui::ColorImage::new([64, 64], egui::Color32::BLACK),
                Default::default(),
            ),
        };

        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                app.state = state;
            }
        }

        match program {
            Some((program, start)) => app.emu.load_program(&program, start),
            None => app.emu.load_program(DEFAULT_BINARY, 0x0000),
        }

        app
    }

    fn step_emulator(&mut self, key_pressed: Option<egui::Key>) {
        self.emu
            .step(key_pressed.and_then(|key| key.try_into().ok()));
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
}

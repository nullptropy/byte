mod file_processor;
mod ui;

use self::ui::code_editor::Theme as CodeEditorTheme;
use crate::{
    emu::core::{ByteEmu, ByteInputState},
    DEFAULT_BINARY, DEFAULT_SOURCE,
};
use file_processor::FileProcesser;

#[derive(Debug)]
pub enum FileProcesserMessage {
    BinaryFile((String, Vec<u8>)),
    SourceFile((String, Vec<u8>)),
}

// `State` that we would like to persist (serialize).
#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    // TODO: this is getting out of hand
    text: String,
    memory_window_range: (u16, u16),
    memory_window_range_str: (String, String),
    memory_window_text_area: String,
    code_editor_theme: CodeEditorTheme,

    is_about_open: bool,
    is_code_editor_open: bool,
    is_emu_controls_open: bool,
    is_memory_monitor_open: bool,

    file_system: vfs::MemoryFS,
}

pub struct ByteEmuApp {
    emu: ByteEmu,
    file_processer: FileProcesser<FileProcesserMessage>,
    state: State,
    texture: egui::TextureHandle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: DEFAULT_SOURCE.to_string(),
            memory_window_range: (0, 0x100),
            memory_window_range_str: ("0x0000".into(), "0x100".into()),
            memory_window_text_area: String::new(),
            code_editor_theme: CodeEditorTheme::Default,

            is_about_open: true,
            is_code_editor_open: true,
            is_emu_controls_open: false,
            is_memory_monitor_open: false,

            file_system: vfs::MemoryFS::new(),
        }
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut input_state = ByteInputState::empty();

        self.show_menu_bar(ctx);
        self.show_code_editor(ctx);
        self.show_emu_controls(ctx);
        self.show_memory_monitor(ctx);
        self.show_about(ctx);
        self.show_byte_console(ctx, &mut input_state);

        self.process_files();
        self.emu.step(input_state);

        // TODO: this might cause some problems when
        // `State` (specifically `file_system`) gets too big
        if let Some(storage) = frame.storage_mut() {
            self.save(storage);
            storage.flush();
        }

        ctx.request_repaint();
    }
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, program: Option<(Vec<u8>, u16)>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut app = Self {
            emu: ByteEmu::default(),
            file_processer: FileProcesser::new(),
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

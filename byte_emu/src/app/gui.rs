use super::file_diag::FileProcesser;
use egui::{Color32, ColorImage, Context, Visuals};

#[derive(Debug)]
pub enum FileProcesserMessage {
    BinaryFile((String, Vec<u8>)),
    SourceFile((String, Vec<u8>)),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ByteEmuApp {
    #[serde(skip)]
    pub emu: crate::emu::ByteEmu,
    #[serde(skip)]
    file_processer: FileProcesser<FileProcesserMessage>,
    #[serde(skip)]
    frame_history: super::frame_history::FrameHistory,
    text: String,
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
}

impl Default for ByteEmuApp {
    fn default() -> Self {
        Self {
            emu: Default::default(),
            file_processer: FileProcesser::new(),
            frame_history: Default::default(),
            text: String::new(),
            texture: None,
        }
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.show_ui(ctx, frame);
        self.process_files();
        self.step_emulator(ctx.input(|i| i.keys_down.iter().next().copied()))
    }
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        app.init_app();
        app
    }

    fn init_app(&mut self) {
        self.emu
            .load_program(include_bytes!("../../assets/static.bin"), 0x0000);

        if self.text.is_empty() {
            self.text = include_str!("../../assets/static.s").to_string();
        }
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
            size: [32, 32],
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
                    self.text = String::from_utf8_lossy(data).to_string()
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
        win("screen").show(ctx, |ui| {
            self.show_pixel_buffer(ui);
        });
        win("code editor").show(ctx, |ui| {
            self.show_code_editor(ui);
        });
        win("performance").show(ctx, |ui| {
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
            });
        });
    }

    fn show_code_editor(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text)
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

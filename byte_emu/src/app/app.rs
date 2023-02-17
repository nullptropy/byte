use super::file_diag;
use egui::{Color32, ColorImage, Context, Rect};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ByteEmuApp {
    #[serde(skip)]
    emu: crate::emu::ByteEmu,
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
    #[serde(skip)]
    frame_history: super::frame_history::FrameHistory,
    #[serde(skip)]
    file_processer: file_diag::FileProcesser,
}

impl ByteEmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self {
                emu: Default::default(),
                texture: None,
                frame_history: Default::default(),
                file_processer: Default::default(),
            }
        };

        app.init_app();
        app
    }

    fn init_app(&mut self) {
        self.emu.load_program(
            &[
                0xa5, 0xfe, 0x8d, 0x00, 0x02, 0x8d, 0x01, 0x02, 0x8d, 0x02, 0x02, 0x8d, 0x03, 0x02,
                0x8d, 0x04, 0x02, 0x4c, 0x00, 0x80,
            ],
            0x8000,
        );
        self.emu.cpu.reg.pc = 0x8000;
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
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        egui::SidePanel::left("left").show(ctx, |ui| {
            self.file_processer.ui(ui);
            self.frame_history.ui(ui);
        });

        self.file_processer
            .consume_messages()
            .iter()
            .for_each(|m| tracing::debug!("{m:?}"));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("this is a label");

            let pixels = self.framebuffer();
            let texture: &mut egui::TextureHandle = self.texture.get_or_insert_with(|| {
                ctx.load_texture(
                    "framebuffer",
                    ColorImage::new([320, 320], Color32::BLACK),
                    Default::default(),
                )
            });

            texture.set(pixels, egui::TextureOptions::NEAREST);
            ui.painter().image(
                texture.id(),
                Rect::from_min_size(ui.cursor().min, egui::vec2(320.0, 320.0)),
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        });

        self.emu
            .step(ctx.input(|i| i.keys_down.iter().nth(0).copied()));
    }
}

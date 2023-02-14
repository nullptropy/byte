#[allow(dead_code, unused_imports, unused_variables)]
use byte_core::*;

use eframe::{CreationContext, Frame, Storage};
use egui::{Color32, ColorImage, Context};

const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;
const COLOR_PALETTE: [u32; 16] = [
    0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF, 0xEEEE77FF,
    0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF, 0x0088FFFF, 0xBBBBBBFF,
];

struct RAM {
    pub data: Vec<u8>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ByteEmuApp {
    #[serde(skip)]
    cpu: cpu::CPU,
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }
}

impl bus::Peripheral for RAM {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}

impl ByteEmuApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self { cpu: cpu::CPU::default(), texture: None }
    }

    pub fn laod_program(&mut self) {
        self.cpu.bus.attach(0x0000, 0xffff, RAM::new(0x10000)).unwrap();
        self.cpu.load(
            &[
                0xa9, 0x00, 0x8d, 0x00, 0x02, 0xa9, 0x01, 0x8d, 0x01, 0x02, 0xa9, 0x02, 0x8d, 0x02,
                0x02, 0xa9, 0x03, 0x8d, 0x03, 0x02, 0xa9, 0x04, 0x8d, 0x04, 0x02, 0x4c, 0x00, 0x80,
            ],
            0x8000,
        );
        self.cpu.reg.pc = 0x8000;
    }

    pub fn framebuffer(&mut self) -> ColorImage {
        let pixels = (0x200..0x600)
            .map(|i| {
                let c = COLOR_PALETTE[(self.cpu.bus.read(i) & 0x0f) as usize].to_be_bytes();
                Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3])
            })
            .collect::<Vec<Color32>>();

        ColorImage {
            size: [32, 32],
            pixels,
        }
    }

    pub fn run_cpu(&mut self) {
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            self.cpu.step();
        }
    }
}

impl eframe::App for ByteEmuApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let None = self.texture {
            self.laod_program();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let pixels = self.framebuffer();
            let texture: &mut egui::TextureHandle = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "framebuffer",
                    ColorImage::new([320, 320], Color32::BLACK),
                    Default::default(),
                )
            });

            texture.set(pixels, Default::default());
            ui.image(texture as &_, texture.size_vec2());
        });

        self.run_cpu();
    }
}

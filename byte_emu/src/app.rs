#[allow(dead_code, unused_imports, unused_variables)]

use byte_core::*;

use eframe::{CreationContext, Frame, Storage};
use egui::{pos2, vec2, Color32, ColorImage, Context, Rect, TextureHandle, TextureOptions};

const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;
const COLOR_PALETTE: [u32; 16] = [
    0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF, 0xEEEE77FF,
    0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF, 0x0088FFFF, 0xBBBBBBFF,
];

fn random_seed() -> u64 {
    std::hash::Hasher::finish(&std::hash::BuildHasher::build_hasher(
        &std::collections::hash_map::RandomState::new(),
    ))
}

fn random_numbers(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;

    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

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
    #[serde(skip)]
    frame_history: crate::frame_history::FrameHistory,
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

        Self {
            cpu: cpu::CPU::default(),
            texture: None,
            frame_history: Default::default(),
        }
    }

    pub fn load_program(&mut self) {
        let ram = RAM::new(0x10000);
        self.cpu.bus.attach(0x0000, 0xffff, ram).unwrap();

        self.cpu.reg.pc = 0x8000;
        self.cpu.load(
            &[
                0xa5, 0xfe, 0x8d, 0x00, 0x02, 0x8d, 0x01, 0x02, 0x8d, 0x02, 0x02, 0x8d, 0x03, 0x02,
                0x8d, 0x04, 0x02, 0x4c, 0x00, 0x80,
            ],
            0x8000,
        );
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

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        if let None = self.texture {
            self.load_program();
        }

        // egui::TopBottomPanel::bottom("nice").show(ctx, |ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.frame_history.ui(ui);

            let pixels = self.framebuffer();
            let texture: &mut TextureHandle = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "framebuffer",
                    ColorImage::new([320, 320], Color32::BLACK),
                    Default::default(),
                )
            });

            texture.set(pixels, TextureOptions::NEAREST);
            ui.painter().image(
                texture.id(),
                Rect::from_min_size(pos2(0.0, 200.0), vec2(320.0, 320.0)),
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        });

        let mut rand = random_numbers(random_seed() as u32);
        self.cpu.bus.write(0xfe, rand.next().unwrap() as u8);
        self.run_cpu();

        ctx.request_repaint();
    }
}

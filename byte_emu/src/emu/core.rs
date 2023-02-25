use super::rand;
use byte_core::*;

const COLOR_PALETTE: [u32; 16] = [
    0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF, 0xEEEE77FF,
    0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF, 0x0088FFFF, 0xBBBBBBFF,
];
const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;

const REG_VIDEO: u16 = 0xfd;
const REG_RANDOM: u16 = 0xfe;
const REG_INPUT: u16 = 0xff;
const FRAMEBUFFER_SIZE: usize = 64 * 64;

pub struct ByteEmu {
    cpu: cpu::CPU,
    rand: Box<dyn Iterator<Item = u32>>,
}

impl Default for ByteEmu {
    fn default() -> Self {
        let mut cpu = cpu::CPU::default();
        cpu.bus
            .attach(0x0000, 0xffff, super::ram::Ram::default())
            .unwrap();

        Self {
            cpu,
            rand: Box::new(rand::random_numbers(rand::random_seed() as u32)),
        }
    }
}

impl ByteEmu {
    pub fn load_program(&mut self, program: &[u8], start: u16) {
        // TODO: this sucks
        self.cpu.load(program, start);
        self.cpu.interrupt(cpu::Interrupt::RST);
    }

    pub fn framebuffer(&self) -> [u32; FRAMEBUFFER_SIZE] {
        let mut frame = [0u32; FRAMEBUFFER_SIZE];
        let video_ptr = (self.cpu.bus.read(REG_VIDEO) as u16 & 0xf) << 0xc;

        frame.iter_mut().enumerate().for_each(|(i, p)| {
            let color = self.cpu.bus.read(video_ptr + i as u16) & 0xf;
            *p = COLOR_PALETTE[color as usize];
        });
        frame
    }

    pub fn step(&mut self, key_pressed: Option<egui::Key>) {
        // TODO: filter out the non-relevant keys
        self.cpu.bus.write(
            REG_INPUT,
            if let Some(key) = key_pressed {
                key as u8
            } else {
                0x00
            },
        );

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if let Some(n) = self.rand.next() {
                self.cpu.bus.write(REG_RANDOM, n as u8);
            }
            self.cpu.step();
        }

        self.cpu.interrupt(cpu::Interrupt::IRQ);
    }
}

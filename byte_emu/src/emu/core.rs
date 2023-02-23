use super::rand;
use byte_core::*;

const COLOR_PALETTE: [u32; 16] = [
    0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF, 0xEEEE77FF,
    0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF, 0x0088FFFF, 0xBBBBBBFF,
];
const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;

pub struct ByteEmu {
    pub cpu: cpu::CPU,
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

    pub fn framebuffer(&self) -> [u32; 32 * 32] {
        let mut frame = [0u32; 32 * 32];
        frame.iter_mut().enumerate().for_each(|(i, p)| {
            let color = self.cpu.bus.read(0x200 + i as u16) & 0xf;
            *p = COLOR_PALETTE[color as usize];
        });
        frame
    }

    pub fn step(&mut self, key_pressed: Option<egui::Key>) {
        self.cpu.bus.write(
            0xff,
            if let Some(key) = key_pressed {
                key as u8
            } else {
                0x00
            },
        );

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if let Some(n) = self.rand.next() {
                self.cpu.bus.write(0xfe, n as u8);
            }
            self.cpu.step();
        }
    }
}

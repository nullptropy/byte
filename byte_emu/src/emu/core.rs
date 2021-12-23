use super::rand;
use std::collections::HashSet;

use bitflags::bitflags;
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

bitflags! {
    pub struct ByteInputState: u8 {
        const RIGHT  = 0b00000001;
        const LEFT   = 0b00000010;
        const DOWN   = 0b00000100;
        const UP     = 0b00001000;
        const START  = 0b00010000;
        const SELECT = 0b00100000;
        const B      = 0b01000000;
        const A      = 0b10000000;
    }
}

impl From<HashSet<egui::Key>> for ByteInputState {
    fn from(val: HashSet<egui::Key>) -> ByteInputState {
        use egui::Key::*;
        let mut state = ByteInputState::empty();

        #[rustfmt::skip]
        val.iter().for_each(|key| match key {
            A          => state.insert(ByteInputState::SELECT),
            S          => state.insert(ByteInputState::START),
            D          => state.insert(ByteInputState::A),
            F          => state.insert(ByteInputState::B),
            ArrowUp    => state.insert(ByteInputState::UP),
            ArrowDown  => state.insert(ByteInputState::DOWN),
            ArrowLeft  => state.insert(ByteInputState::LEFT),
            ArrowRight => state.insert(ByteInputState::RIGHT),
            _          => ()
        });

        state
    }
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

    pub fn step(&mut self, input_state: ByteInputState) {
        self.cpu.bus.write(REG_INPUT, input_state.bits());

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if let Some(n) = self.rand.next() {
                self.cpu.bus.write(REG_RANDOM, n as u8);
            }
            if let Err(err) = self.cpu.step() {
                log::error!("{err}");
            };
        }

        self.cpu.interrupt(cpu::Interrupt::IRQ);
    }

    pub fn get_memory_region(&self, range: (u16, u16)) -> &[u8] {
        self.cpu.bus.get_memory_region(range)
    }
}

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

#[derive(Debug)]
pub enum ByteKey {
    Up = 0x01,
    Down,
    Left,
    Right,
    Select,
    Start,
    A,
    B,
}

impl TryInto<ByteKey> for egui::Key {
    type Error = ();

    fn try_into(self) -> Result<ByteKey, Self::Error> {
        use egui::Key;
        use ByteKey::*;

        match self {
            Key::ArrowUp => Ok(Up),
            Key::ArrowDown => Ok(Down),
            Key::ArrowLeft => Ok(Left),
            Key::ArrowRight => Ok(Right),
            Key::A => Ok(Select),
            Key::S => Ok(Start),
            Key::D => Ok(A),
            Key::F => Ok(B),
            _ => Err(()),
        }
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

    pub fn step(&mut self, key_pressed: Option<ByteKey>) {
        self.cpu
            .bus
            .write(REG_INPUT, key_pressed.map(|key| key as u8).unwrap_or(0x00));

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if let Some(n) = self.rand.next() {
                self.cpu.bus.write(REG_RANDOM, n as u8);
            }
            if let Err(err) = self.cpu.step() {
                tracing::error!("{err}");
            };
        }

        self.cpu.interrupt(cpu::Interrupt::IRQ);
    }
}

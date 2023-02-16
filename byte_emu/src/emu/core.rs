use byte_core::*;

const COLOR_PALETTE: [u32; 16] = [
    0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF, 0xEEEE77FF,
    0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF, 0x0088FFFF, 0xBBBBBBFF,
];
const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;

pub struct ByteEmu {
    pub cpu: cpu::CPU,
}

impl Default for ByteEmu {
    fn default() -> Self {
        let mut cpu = cpu::CPU::default();
        cpu.bus
            .attach(0x0000, 0xffff, super::ram::Ram::default())
            .unwrap();

        Self { cpu }
    }
}

impl ByteEmu {
    pub fn load_program(&mut self, program: &[u8], start: u16) {
        self.cpu.load(program, start);
    }

    pub fn framebuffer(&self) -> [u32; 32 * 32] {
        let mut frame = [0u32; 32 * 32];
        frame.iter_mut().enumerate().for_each(|(i, p)| {
            let color = self.cpu.bus.read(0x200 + i as u16) & 0xf;
            // print!("{color}:{i} ");
            *p = COLOR_PALETTE[color as usize];
        });
        // print!("\n");
        frame
    }

    pub fn step(&mut self, key_pressed: Option<egui::Key>) {
        let mut byte = [0u8];
        if let Err(_why) = getrandom::getrandom(&mut byte) {
            // TODO: log the error somehow
        };

        self.cpu.bus.write(0xfe, byte[0]);
        if let Some(key) = key_pressed {
            self.cpu.bus.write(0xff, key as u8);
        }

        for _ in 0..INSTRUCTIONS_PER_FRAME {
            self.cpu.step();
        }
    }
}

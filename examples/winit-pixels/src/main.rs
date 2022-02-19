use mos6502::prelude::*;
use minifb::{
    Key, Window, WindowOptions};

static PALETTE: [u32; 8] = [
    0x00000000, 0x00ffffff, 0x00ff0000, 0x0000ff00,
    0x000000ff, 0x00ffff00, 0x0000ffff, 0x00ff00ff,
];
static VRAM_START: u16 = 0xe000;

struct RAM {
    data: Vec<u8>,
}

struct Console {
    cpu: CPU,
    win: Window,
    buf: [u32; 64 * 64],
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
}

impl Peripheral for RAM {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}

impl Console {
    fn new() -> Self {
        let mut cpu = CPU::new();
        cpu.bus.attach(0x000, 0xffff, RAM::new(0x10000)).unwrap();

        Self {
            cpu,
            buf: [0; 64 * 64],
            win: Window::new(
                "mos6502", 64, 64,
                WindowOptions {
                    scale: minifb::Scale::X8,
                    ..WindowOptions::default()
                }
            ).unwrap(),
        }
    }

    fn load_rom(&mut self, data: &[u8], start: u16) {
        self.cpu.load(data, start);
    }

    fn main_loop(&mut self) {
        self.cpu.interrupt(Interrupt::RST);
        self.win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while self.win.is_open() && !self.win.is_key_down(Key::Escape) {
            self.cpu.cycle = 0;

            while self.cpu.cycle < 16600 {
                self.cpu.step();
            }

            self.buf.iter_mut().enumerate().for_each(|(i, b)| {
                *b = PALETTE[self.cpu.bus.read(VRAM_START + i as u16) as usize];
            });

            self.win
                .update_with_buffer(&self.buf, 64, 64)
                .unwrap();
        }
    }
}

fn main() {
    let mut console = Console::new();

    console.load_rom(
        &std::fs::read(
            std::env::args().nth(1).unwrap()).unwrap(), 0x0000);
    console.main_loop();
}

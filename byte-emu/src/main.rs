#![allow(dead_code, unused_variables, unused_imports)]

// TODO: replace minifb with something that works on the web
use minifb::{Scale, Window, WindowOptions};
use mos6502::prelude::*;

use std::fs::File;
use std::io::Read;

const W: usize = 32;
const H: usize = 32;
const S: usize = 10; // scale, TODO: make this into something configurable

const COLOR_PALETTE: [u32; 16] = [
    0x00000000, 0x00FFFFFF, 0x00880000, 0x00AAFFEE, 0x00CC44CC, 0x0000CC55, 0x000000AA, 0x00EEEE77,
    0x00664400, 0x00FF7777, 0x00333333, 0x00777777, 0x00AAFF66, 0x000088FF, 0x000088FF, 0x00BBBBBB,
];

struct RAM {
    pub data: Vec<u8>,
}

struct State {
    cpu: CPU,
    win: minifb::Window,
    buf: Vec<u32>,
    size: (usize, usize),
    rand: Box<dyn Iterator<Item = u32>>,
}

fn draw_rect(buf: &mut Vec<u32>, x: usize, y: usize, s: usize, ss: usize, color: u32) {
    for dy in y..(y + s) {
        let start = dy * ss + x;
        buf[start..start + s].fill(color);
    }
}

pub fn random_seed() -> u64 {
    std::hash::Hasher::finish(&std::hash::BuildHasher::build_hasher(
        &std::collections::hash_map::RandomState::new(),
    ))
}

pub fn random_numbers(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;

    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
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

impl State {
    fn new(width: usize, height: usize, cpu: CPU) -> Self {
        let buf = vec![0; width * height];
        let win = Window::new(
            "byte emu",
            width,
            height,
            WindowOptions {
                resize: false,
                ..WindowOptions::default()
            },
        )
        // TODO: return a Result<Self> instead of panicking
        .expect("failed to create a window");

        Self {
            cpu,
            win,
            buf,
            size: (width, height),
            rand: Box::new(random_numbers(random_seed() as u32)),
        }
    }

    // this function will give the last pressed
    // key to the emulator at the address $ff
    fn handle_events(&mut self) {
        if let Some(key) = self.win.get_keys().iter().nth(0) {
            println!("holding key: {:?}", key);
            self.cpu.bus.write(0xff, *key as u8);
        }
    }

    // this function will provide
    // a randomly generated number to the emulator
    fn update(&mut self) {
        if let Some(rnd) = self.rand.next() {
            self.cpu.bus.write(0xfe, rnd as u8);
        }
    }

    fn draw(&mut self) {
        let scale = self.size.0 / 32;

        // clear the screen
        self.buf.fill(0x000000);

        // for each u8 in this buffer, paint a single rectangle of size SxS on the screen
        (0x200..0x600).for_each(|i| {
            let (x, y) = ((i - 0x200) % 32, (i - 0x200) / 32);
            let color = COLOR_PALETTE[(self.cpu.bus.read(i as u16) & 0x0f) as usize];

            if color != COLOR_PALETTE[0] {
                draw_rect(
                    &mut self.buf,
                    x * scale,
                    y * scale,
                    scale,
                    self.size.0,
                    color,
                );
                return;
            }
        });
    }

    fn run(&mut self) {
        while self.win.is_open() {
            self.handle_events();
            self.update();
            self.draw();

            self.win
                .update_with_buffer(&self.buf, self.size.0, self.size.1)
                .expect("failed to update the window");

            // execute 1000 instructions per frame
            for _ in 0..1 {
                self.cpu.step();
            }
        }
    }
}

fn main() {
    // TODO: load programs
    let mut cpu = CPU::new();
    cpu.bus
        .attach(0x0000, 0xffff, RAM::new(0x10000))
        .expect("failed to attach the ram to the bus");

    let program = match std::env::args().nth(1) {
        Some(path) => {
            let mut program = Vec::new();
            let mut file = std::fs::File::open(path).expect("couldn't open the program file");

            file.read_to_end(&mut program)
                .expect("failed to read the program file to the end");
            program
        }
        None => {
            eprintln!("Usage: ./byte-emu program.bin");
            return;
        }
    };

    // cpu.reg.pc = 0x8000;
    // cpu.load(
    //     &[
    //         0xa5, 0xff, // LDA $ff
    //         0x8d, 0x00, 0x02, // STA $200
    //         0x8d, 0x01, 0x02, // STA $200
    //         0x8d, 0x02, 0x02, // STA $200
    //         0x4c, 0x00, 0x80, // JMP $8000
    //     ],
    //     0x8000,
    // );

    cpu.load(&program, 0x0000);
    cpu.interrupt(Interrupt::RST);

    State::new(W * S, H * S, cpu)
        // TODO: make this more configurable
        .run();
}

// This simple package was written as an example to demonstrate
// the intended way of using the 'mos6502' crate.
//
// [0x0000..0x0100): zero page
// [0x0100..0x0200]: stack
// [0x0600..0x0a00]: vram buffer
//
// 0xff: key input
// 0xfe: random number

use std::io::prelude::*;
use mos6502::prelude::*;

use winit::{
    dpi::LogicalSize,
    event::{self, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{PowerPreference, RequestAdapterOptions};

const VRAM_ADDR: u16 = 0x0600;
const VRAM_SIZE: u32 = 32;

const PALETTE: [u32; 8] = [
    0x000000ff, 0xffffffff, 0xff0000ff, 0x00ff00ff,
    0x0000ffff, 0x00ffffff, 0xffff00ff, 0xff00ffff,
];

struct RAM {
    data: Vec<u8>,
}

struct Platform {
    cpu: CPU,
    cpf: u64,
}

impl RAM {
    fn new(size: usize) -> Self {
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

impl Platform {
    fn new(mhz: f64) -> Self {
        let mut cpu = CPU::new();
        let cpf = std::cmp::max(1, (mhz * 1000000.0 / 60.0) as u64); // cycles per frame

        cpu.bus.attach(
            0x0000, 0xffff,
            RAM::new(0x10000)).unwrap();

        Self { cpu, cpf }
    }

    fn load_rom(&mut self, data: &[u8], start: u16) {
        self.cpu.load(data, start);
    }

    fn create_window(&self, event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new((VRAM_SIZE * 10) as f64, (VRAM_SIZE * 10) as f64);
        WindowBuilder::new()
            .with_title("6502")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    fn create_pixels(&self, window: &winit::window::Window) -> Result<Pixels, pixels::Error> {
        let winsize = window.inner_size();
        let texture = SurfaceTexture::new(winsize.width, winsize.height, &window);

        PixelsBuilder::new(VRAM_SIZE, VRAM_SIZE, texture)
            .request_adapter_options(RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .enable_vsync(true)
            .build()
    }

    fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .enumerate()
            .for_each(|(i, pixel)| {
                pixel.copy_from_slice(&u32::to_be_bytes(
                    PALETTE[self.cpu.bus.read(VRAM_ADDR + i as u16) as usize],
                ))
            });
    }

    fn main_loop(mut self) -> Result<(), pixels::Error> {
        self.cpu.interrupt(Interrupt::RST);

        let mut input = WinitInputHelper::new();
        let event_loop = EventLoop::new();
        let window = self.create_window(&event_loop);
        let mut pixels = self.create_pixels(&window)?;

        let mut counter = 0;
        let mut rbuffer = [0; 1];
        let mut urandom = std::fs::File::open("/dev/urandom").unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            if { counter += 1; counter } == self.cpf {
                window.request_redraw();
            }

            if input.update(&event) {
                if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                    return *control_flow = ControlFlow::Exit;
                }
            }

            match event {
                Event::RedrawRequested { .. } => {
                    counter = 0;
                    self.draw(pixels.get_frame());

                    if pixels
                        .render()
                        .map_err(|e| eprintln!("rendering failed: {}", e))
                        .is_err()
                    {
                        return *control_flow = ControlFlow::Exit;
                    }
                },
                Event::WindowEvent {
                    event:
                        event::WindowEvent::KeyboardInput {
                            input:
                                event::KeyboardInput {
                                    state: event::ElementState::Pressed,
                                    scancode,
                                    ..
                                },
                            ..
                        },
                    ..
                } => self.cpu.bus.write(0x00ff, scancode as u8),
                _ => {}
            }

            urandom.read(&mut rbuffer).unwrap();
            self.cpu.bus.write(0x00fe, rbuffer[0]);
            self.cpu.step();
        });
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (rom, mhz) = match args.len() {
        2 => (&args[1], 1.0),
        3 => (&args[1], args[2].parse().unwrap()),
        _ => return println!("./usage: {} <rom image> [mhz: 1]", args[0])
    };

    let mut platform = Platform::new(mhz);

    platform.load_rom(
        &std::fs::read(rom).unwrap(), 0x0000);
    platform.main_loop().unwrap();
}

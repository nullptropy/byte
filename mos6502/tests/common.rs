#![allow(dead_code)]

pub use mos6502::*;

pub struct MockRAM {
    pub data: Vec<u8>
}

impl MockRAM {
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
}

impl bus::Peripheral for MockRAM {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}

pub fn init_cpu() -> cpu::CPU {
    let mut cpu = cpu::CPU::new();
    cpu.bus
        .attach(0x0000, 0xffff, MockRAM::new(0x10000))
        .unwrap();
    cpu
}

pub fn execute_nsteps(config: fn(&mut cpu::CPU), program: &[u8], addr: u16, n: usize) -> cpu::CPU {
    let mut cpu = init_cpu();
    config(&mut cpu);

    cpu.load(program, addr);
    cpu.reg.pc = addr;

    for _ in 0..n { cpu.step(); }

    cpu
}

# mos6502

This crate emulates the famous [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) CPU. This implementation currently can pass [Klaus' test suite for 6502](https://github.com/Klaus2m5/6502_65C02_functional_tests).

I intend to use this crate in anothor project of mine which will be a custom fantasy console with a wide set of tools to develop/debug/write software for.

# Example Usage
```rust
use mos6502::*;

struct RAM {
    pub data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
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

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.bus
        .attach(0x0000, 0xffff, RAM::new(0x10000))
        .unwrap();

    cpu.reg.pc = 0x8000;
    cpu.load(&[
        0xa9, 0xc0, // lda #$c0
        0xaa,       // tax
        0xe8,       // inx
        0x00,       // brk
    ], 0x8000);
    for _ in 0..4 { cpu.step(); }

    println!("{:#x?}", cpu.reg);
}
```

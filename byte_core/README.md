# byte_core

The `byte_core` crate emulates the [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) CPU and is designed to match its features. This implementation is passes [Klaus's test suite for 6502](https://github.com/Klaus2m5/6502_65C02_functional_tests) and can be used as an independent component, separate from the `byte` project.

# Example Usage
```rust
use byte_core::*;

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
    let mut cpu = cpu::CPU::default();
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

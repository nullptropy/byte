# 6502

```rust
use mos6502::{
    cpu::CPU,
    bus::Peripheral};

pub struct RAM {
    data: Vec<u8>
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

fn main() {
    let mut cpu = CPU::new();

    cpu.bus.attach(0x0000, 0x01ff, RAM::new(0x0200))
        .unwrap(); // first two pages of memory are required
                   // for the cpu to function properly
                   // [0x000, 0x0ff]: zero page,
                   // [0x100, 0x1ff]: where stack lies
    cpu.bus.attach(0x0200, 0xffff, RAM::new(0xfe00)).unwrap();

    cpu.bus.write(0x1000, 0x05);
    cpu.bus.write_u16(0xfffc, 0x8000);

    // load some arbitrary program into ram
    // starting from the address 0x8000
    [
        0xad, 0x00, 0x10,  // LDA $1000
        0xaa,              // TAX
        0xa9, 0x00,        // LDA #$00
        0xca,              // DEX
        0xd0, 0xfd,        // BNE $0003
        0x00,              // BRK
    ]
        .iter()
        .enumerate()
        .for_each(|(i, b)| cpu.bus.write((0x8000 + i) as u16, *b as u8));

    cpu.reg.pc = cpu.bus.read_u16(0xfffc);
    cpu.run_until_brk();
}
```
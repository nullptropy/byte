use std::ops::ControlFlow;
use mos6502::prelude::*;

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
                   // [0x0000, 0x0100): zero page,
                   // [0x0100, 0x0200): where stack lies
    cpu.bus.attach(0x0200, 0xffff, RAM::new(0xfe00)).unwrap();

    // cpu.bus.write(0x1000, 0x02);
    // cpu.bus.write_u16(0xfffe, 0x8000);
    cpu.bus.write_u16(0xfffc, 0x8000);

    // load some arbitrary program into ram starting from the address 0x8000
    // cpu.load(&[
    //     0xad, 0x00, 0x10,  // LDA $1000
    //     0xaa,              // TAX
    //     0xa9, 0x00,        // LDA #$00
    //     0xca,              // DEX
    //     0xd0, 0xfd,        // BNE $0003
    //     0x00,              // BRK
    // ], 0x8000);

    cpu.bus.write_u16(0x0000, 0x8006);
    cpu.load(&[
        0x20, 0x00, 0x00,  // JSR $00000
        0x00,              // BRK
        0xea,              // NOP
        0xea,              // NOP
        0xa9, 0xff,        // LDA #$0xff : $8004
        0x60               // RTS
    ], 0x8000);

    // cpu.set_irq(false); // level triggered
    cpu.interrupt(Interrupt::RST);

    cpu.run_with_callback(|cpu| {
        if cpu.reg.p.contains(Flags::BREAK) {
            return ControlFlow::Break(());
        }

        let code = cpu.bus.read(cpu.reg.pc);
        let opcode = OPCODE_MAP.get(&code)
            .unwrap_or_else(|| panic!("unrecognized opcode: {:x}", code));

        println!("[{:x?}:{:08}][{:?}]", cpu.reg, cpu.cycle, opcode);

        ControlFlow::Continue(())
    });
}
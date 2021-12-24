use core::{fmt, panic};
use bitflags::bitflags;

use crate::bus::Bus;
use crate::opcode::*;

bitflags! {
/// 6502 status flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
    #[derive(Default)]
    pub struct Flags: u8 {
        const NEGATIVE     = 0b10000000;
        const OVERFLOW     = 0b01000000;
        const UNUSUED      = 0b00100000;
        const BREAK        = 0b00010000;
        const DECIMAL      = 0b00001000;
        const INTERRUPT    = 0b00000100;
        const ZERO         = 0b00000010;
        const CARRY        = 0b00000001;
    }
}

#[derive(Clone, Copy)]
pub struct Registers {
    sp: u16,
    pc: u16,

    x: u8, y: u8,
    a: u8, p: Flags,
}

#[allow(non_snake_case)]
pub struct CPU {
    pub reg: Registers,
    pub bus: Bus,
    pub cycle: u32
}

impl Registers {
    fn new() -> Self {
        Self {
            sp: 0, pc: 0,

            x: 0, y: 0,
            a: 0, p: Flags::default()
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{:04x}:{:04x}:{:08b}:{:02x}:{:02x}:{:02x}",
            self.pc, self.sp, self.p, self.a, self.x, self.y
        )
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: Registers::new(),
            bus: Bus::new(),
            cycle: 0,
        }
    }

    pub fn reset(&mut self) {
        self.reg = Registers::new();
        self.reg.pc = self.bus.read_u16(0xfffc);
    }

    pub fn run(&mut self) {
        self.reset();

        loop {
            let code = self.bus.read(self.reg.pc);
            let opcode = OPCODE_MAP.get(&code)
                .expect("unrecognized opcode");

            self.reg.pc += 1;
            let pc_state = self.reg.pc;

            match code {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode),

                0xaa => self.tax(&opcode),
                0xe8 => self.inx(&opcode),
                0xca => self.dex(&opcode),
                0x00 => break,

                _ => ()
            }

            if pc_state == self.reg.pc {
                self.reg.pc += (opcode.size - 1) as u16;
            }

            println!("[{:x?}][{:?}]", self.reg, opcode);
        }
    }

    fn tax(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.a;
        self.update_flags(self.reg.x);
    }

    fn inx(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_flags(self.reg.x);
    }

    fn dex(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_flags(self.reg.x);
    }

    fn lda(&mut self, opcode: &Opcode) {
        self.reg.a = self.bus.read(self.get_operand_address(opcode.mode));
        self.update_flags(self.reg.a);
    }

    fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.reg.pc,

            AddressingMode::ZeroPage  => self.bus.read(self.reg.pc) as u16,
            AddressingMode::ZeroPageX => self.bus.read(self.reg.pc).wrapping_add(self.reg.x) as u16,
            AddressingMode::ZeroPageY => self.bus.read(self.reg.pc).wrapping_add(self.reg.y) as u16,

            AddressingMode::Absolute  => self.bus.read_u16(self.reg.pc),
            AddressingMode::AbsoluteX => self.bus.read_u16(self.reg.pc).wrapping_add(self.reg.x as u16),
            AddressingMode::AbsoluteY => self.bus.read_u16(self.reg.pc).wrapping_add(self.reg.y as u16),

            AddressingMode::Indirect  => self.bus.read_u16(self.bus.read_u16(self.reg.pc)),
            AddressingMode::IndirectX => self.bus.read_u16(
                self.bus.read(self.reg.pc).wrapping_add(self.reg.x) as u16),
            AddressingMode::IndirectY => self.bus.read_u16(
                self.bus.read(self.reg.pc) as u16).wrapping_add(self.reg.y as u16),

            _ => panic!("shouldn't be called with {:?}", mode),
        }
    }

    fn update_flags(&mut self, value: u8) {
        match value {
            0 => self.reg.p.insert(Flags::ZERO),
            _ => self.reg.p.remove(Flags::ZERO)
        }

        self.update_negative_flag(value);
    }

    fn update_negative_flag(&mut self, value: u8) {
        match value & 0b10000000 {
            0 => self.reg.p.remove(Flags::NEGATIVE),
            _ => self.reg.p.insert(Flags::NEGATIVE),
        }
    }
}
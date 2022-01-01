use core::fmt;
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
    pub sp: u16,
    pub pc: u16,

    pub x: u8, pub y: u8,
    pub a: u8, pub p: Flags,
}

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

    pub fn load(&mut self, program: &[u8], addr: u16) {
        self.bus.write_u16(0xfffc, addr);

        program
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.bus.write(addr + i as u16, *b));
    }

    pub fn load_and_run(&mut self, program: &[u8], addr: u16) {
        self.load(program, addr);
        self.run();
    }

    pub fn run(&mut self) {
        self.reg.pc = self.bus.read_u16(0xfffc);

        loop {
            let code = self.bus.read(self.reg.pc);
            let opcode = OPCODE_MAP.get(&code)
                .expect("unrecognized opcode");

            self.reg.pc += 1;
            let pc_state = self.reg.pc;

            match code {
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&opcode),
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode),

                0xe8 => self.inx(&opcode), 0xca => self.dex(&opcode),
                0xaa => self.tax(&opcode), 0x8a => self.txa(&opcode),
                0xa8 => self.tay(&opcode), 0x98 => self.tya(&opcode),

                0x00 => break,

                _ => ()
            }

            if pc_state == self.reg.pc {
                self.reg.pc += (opcode.size - 1) as u16;
            }

            println!("[{:x?}][{:?}]", self.reg, opcode);
        }
    }

    fn and(&mut self, opcode: &Opcode) {
        self.reg.a &= self.bus.read(self.get_operand_address(opcode.mode));
        self.update_flags(self.reg.a);
    }

    fn lda(&mut self, opcode: &Opcode) {
        self.reg.a = self.bus.read(self.get_operand_address(opcode.mode));
        self.update_flags(self.reg.a);
    }

    fn inx(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_flags(self.reg.x);
    }

    fn dex(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_flags(self.reg.x);
    }

    fn tax(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.a;
        self.update_flags(self.reg.x);
    }

    fn txa(&mut self, opcode: &Opcode) {
        self.reg.a = self.reg.x;
        self.update_flags(self.reg.a);
    }

    fn tay(&mut self, opcode: &Opcode) {
        self.reg.y = self.reg.a;
        self.update_flags(self.reg.y);
    }

    fn tya(&mut self, opcode: &Opcode) {
        self.reg.a = self.reg.y;
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
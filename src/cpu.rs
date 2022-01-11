use core::fmt;
use bitflags::bitflags;

use crate::bus::Bus;
use crate::opcode::{self, *};

pub const STACK_BASE: u16 = 0x0100;

pub const NMI_VECTOR: u16 = 0xfffa;
pub const RST_VECTOR: u16 = 0xfffc;
pub const IRQ_VECTOR: u16 = 0xfffe;

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
        const UNUSED       = 0b00100000;
        const BREAK        = 0b00010000;
        const DECIMAL      = 0b00001000;
        const INTERRUPT    = 0b00000100;
        const ZERO         = 0b00000010;
        const CARRY        = 0b00000001;
    }
}

#[derive(Clone, Copy)]
pub struct Registers {
    pub sp: u8,
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
            pc: 0, sp: 0xff,

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

    pub fn run_until_brk(&mut self) {
        loop {
            if self.reg.p.contains(Flags::BREAK) {
                break;
            }

            self.step();
        }
    }

    pub fn step(&mut self) {
        let code = self.bus.read(self.reg.pc);
        let opcode = OPCODE_MAP.get(&code)
            .unwrap_or_else(|| panic!("unrecognized opcode: {:x}", code));

        self.reg.pc += 1;
        let pc_state = self.reg.pc;

        match code {
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(opcode),
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(opcode),
            0x0a | 0x06 | 0x16 | 0x0e | 0x1e                      => self.asl(opcode),

            0xe8 => self.inx(opcode), 0xca => self.dex(opcode),
            0xaa => self.tax(opcode), 0x8a => self.txa(opcode),
            0xa8 => self.tay(opcode), 0x98 => self.tya(opcode),

            0x90 => self.branch(opcode, !self.reg.p.contains(Flags::CARRY)),
            0xb0 => self.branch(opcode,  self.reg.p.contains(Flags::CARRY)),
            0xf0 => self.branch(opcode,  self.reg.p.contains(Flags::ZERO)),
            0xd0 => self.branch(opcode, !self.reg.p.contains(Flags::ZERO)),
            0x10 => self.branch(opcode, !self.reg.p.contains(Flags::NEGATIVE)),
            0x30 => self.branch(opcode,  self.reg.p.contains(Flags::NEGATIVE)),
            0x70 => self.branch(opcode,  self.reg.p.contains(Flags::OVERFLOW)),
            0x50 => self.branch(opcode, !self.reg.p.contains(Flags::OVERFLOW)),

            0xea => self.nop(opcode),
            0x00 => self.brk(opcode),

            _ => ()
        }

        if pc_state == self.reg.pc {
            self.reg.pc += (opcode.size - 1) as u16;
        }
        self.cycle += opcode.tick as u32;

        println!("[{:x?}:{:08}][{:?}]", self.reg, self.cycle, opcode);
    }

    fn stack_push(&mut self, byte: u8) {
        self.bus.write(STACK_BASE.wrapping_add(self.reg.sp as u16), byte);
        self.reg.sp = self.reg.sp.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        self.stack_push((data >> 0x8) as u8);
        self.stack_push((data & 0xff) as u8);
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.reg.p.insert(Flags::CARRY);
        } else {
            self.reg.p.remove(Flags::CARRY);
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

    fn on_tick_modifier(&mut self, lo: u8, hi: u8, byte: u8, modifier: TickModifier) -> Operand {
        match (lo.overflowing_add(byte).1, modifier) {
            (true, TickModifier::PageCrossed) => self.cycle += 1,
            (true, TickModifier::Branch)      => self.cycle += 1,

            _ => ()
        }

        Operand::Address(((hi as u16) << 8 | lo as u16).wrapping_add(byte as u16))
    }

    fn branch(&mut self, opcode: &Opcode, condition: bool) {
        if !condition {
            return;
        }

        self.cycle += 1;

        if let Operand::Address(addr) = self.get_operand(opcode) {
            let page = self.reg.pc >> 8;

            self.reg.pc = self.reg.pc
                .wrapping_add(1)
                .wrapping_add(i8::from_le_bytes(self.bus.read(addr).to_le_bytes()) as u16);

            if page != self.reg.pc >> 8 {
                self.cycle += 1;
            }
        }
    }

    fn get_operand(&mut self, opcode: &Opcode) -> Operand {
        match opcode.mode {
            AddressingMode::Relative    => Operand::Address(self.reg.pc),
            AddressingMode::Immediate   => Operand::Address(self.reg.pc),
            AddressingMode::Accumulator => Operand::Accumulator,

            AddressingMode::ZeroPage  => Operand::Address(self.bus.read(self.reg.pc) as u16),
            AddressingMode::ZeroPageX => {
                Operand::Address(self.bus.read(self.reg.pc).wrapping_add(self.reg.x) as u16)
            }
            AddressingMode::ZeroPageY => {
                Operand::Address(self.bus.read(self.reg.pc).wrapping_add(self.reg.y) as u16)
            }

            AddressingMode::Absolute  => Operand::Address(self.bus.read_u16(self.reg.pc)),
            AddressingMode::AbsoluteX => {
                if let Some(modifier) = opcode.tick_modifier {
                    let lo = self.bus.read(self.reg.pc);
                    let hi = self.bus.read(self.reg.pc + 1);

                    return self.on_tick_modifier(lo, hi, self.reg.x, modifier);
                }

                Operand::Address(
                    self.bus
                        .read_u16(self.reg.pc)
                        .wrapping_add(self.reg.x as u16),
                )
            }
            AddressingMode::AbsoluteY => {
                if let Some(modifier) = opcode.tick_modifier {
                    let lo = self.bus.read(self.reg.pc);
                    let hi = self.bus.read(self.reg.pc + 1);

                    return self.on_tick_modifier(lo, hi, self.reg.y, modifier);
                }

                Operand::Address(
                    self.bus
                        .read_u16(self.reg.pc)
                        .wrapping_add(self.reg.y as u16),
                )
            }

            AddressingMode::Indirect  => {
                Operand::Address(self.bus.read_u16(self.bus.read_u16(self.reg.pc)))
            }
            AddressingMode::IndirectX => Operand::Address(
                self.bus
                    .read_u16(self.bus.read(self.reg.pc).wrapping_add(self.reg.x) as u16),
            ),
            AddressingMode::IndirectY => {
                if let Some(modifier) = opcode.tick_modifier {
                    let lo = self.bus.read(self.bus.read(self.reg.pc) as u16);
                    let hi = self.bus.read(self.bus.read(self.reg.pc) as u16 + 1);

                    return self.on_tick_modifier(lo, hi, self.reg.y, modifier);
                }

                Operand::Address(
                    self.bus
                        .read_u16(self.bus.read(self.reg.pc) as u16)
                        .wrapping_add(self.reg.y as u16),
                )
            }

            _ => panic!("Eh"),
        }
    }

    fn nop(&self, opcode: &Opcode) {}

    fn and(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a &= self.bus.read(addr);
            self.update_flags(self.reg.a);
        }
    }

    fn asl(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.set_carry_flag(self.reg.a >> 7 != 0);
                self.reg.a = self.reg.a.wrapping_shl(1);
                self.update_flags(self.reg.a);
            },
            Operand::Address(addr) => {
                self.set_carry_flag(self.bus.read(addr) >> 7 != 0);
                self.bus.write(addr, self.bus.read(addr).wrapping_shl(1));
                self.update_flags(self.bus.read(addr));
            }
        }
    }

    fn brk(&mut self, opcode: &Opcode) {
        self.reg.p.insert(Flags::BREAK);

        self.stack_push(((self.reg.pc + 2) >> 0x8) as u8);
        self.stack_push(((self.reg.pc + 2) & 0xff) as u8);

        self.stack_push(self.reg.p.bits());

        self.reg.pc = self.bus.read_u16(0xfffe);
    }

    fn lda(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a = self.bus.read(addr);
            self.update_flags(self.reg.a);
        }
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
}
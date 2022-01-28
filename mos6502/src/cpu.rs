use bitflags::bitflags;

use core::fmt;
use std::cmp::Ordering;
use std::ops::{ControlFlow, BitAnd};

use crate::bus::Bus;
use crate::opcode::{self, *};

pub const STACK_BASE: u16 = 0x0100;

pub const NMI_VECTOR: u16 = 0xfffa;
pub const RST_VECTOR: u16 = 0xfffc;
pub const IRQ_VECTOR: u16 = 0xfffe;

#[derive(Debug, PartialEq, Eq)]
pub enum Interrupt {
    IRQ,
    NMI,
    BRK,
    RST,
}

bitflags! {
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
    pub cycle: u32,

    pub irq: bool,
    pub nmi: bool, pub nmi_edge: bool,
}

impl Registers {
    fn new() -> Self {
        Self {
            pc: 0, sp: 0,

            x: 0, y: 0,
            a: 0, p: Flags::default()
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{:04x}:{:02x}:{:08b}:{:02x}:{:02x}:{:02x}",
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

            irq: true,
            nmi: true, nmi_edge: false,
        }
    }

    pub fn reset(&mut self) {
        self.reg = Registers::new();
        self.reg.pc = self.bus.read_u16(0xfffc);
    }

    pub fn load(&mut self, program: &[u8], addr: u16) {
        program
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.bus.write(addr + i as u16, *b));
    }

    pub fn run_with_callback(&mut self, callback: fn(&mut CPU) -> ControlFlow<()>) {
        loop {
            if callback(self).is_break() {
                break;
            }

            self.step();
        }
    }

    pub fn set_irq(&mut self, irq: bool) {
        self.irq = irq;
    }

    /// NMI is only executed once on a negative transition
    /// from HIGH to LOW
    pub fn set_nmi(&mut self, nmi: bool) {
        self.nmi_edge = !nmi;
        self.nmi = nmi;
    }

    pub fn interrupt(&mut self, interrupt: Interrupt) {
        let (pc, flag, vector): (u16, Option<Flags>, u16) = match interrupt {
            Interrupt::BRK => (self.reg.pc + 1, Some(Flags::BREAK), IRQ_VECTOR),
            Interrupt::IRQ => (self.reg.pc, Some(Flags::INTERRUPT), IRQ_VECTOR),
            Interrupt::NMI => (self.reg.pc, None, NMI_VECTOR),
            Interrupt::RST => (0, None, RST_VECTOR),
        };

        if interrupt != Interrupt::RST {
            self.stack_push_u16(pc);
            self.stack_push(self.reg.p.bits());

            if let Some(flag) = flag {
                self.reg.p.insert(flag);
            }
        }

        self.reg.pc = self.bus.read_u16(vector);
        self.cycle += 7;
    }

    pub fn step(&mut self) {
        if !self.irq && !self.reg.p.contains(Flags::INTERRUPT) {
            return self.interrupt(Interrupt::IRQ);
        } else if self.nmi_edge {
            self.nmi_edge = false;
            return self.interrupt(Interrupt::NMI);
        }

        let code = self.bus.read(self.reg.pc);
        let opcode = OPCODE_MAP.get(&code)
            .unwrap_or_else(|| panic!("unrecognized opcode: {:x}", code));

        self.reg.pc  = self.reg.pc.wrapping_add(1);
        let pc_state = self.reg.pc;

        match code {
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(opcode),
            0x0a | 0x06 | 0x16 | 0x0e | 0x1e                      => self.asl(opcode),
            0x24 | 0x2c                                           => self.bit(opcode),
            0xc6 | 0xd6 | 0xce | 0xde                             => self.dec(opcode),
            0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => self.eor(opcode),
            0xe6 | 0xf6 | 0xee | 0xfe                             => self.inc(opcode),
            0x4c | 0x6c                                           => self.jmp(opcode),
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(opcode),
            0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe                      => self.ldx(opcode),
            0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc                      => self.ldy(opcode),
            0x4a | 0x46 | 0x56 | 0x4e | 0x5e                      => self.lsr(opcode),
            0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => self.ora(opcode),
            0x2a | 0x26 | 0x36 | 0x2e | 0x3e                      => self.rol(opcode),

            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.compare(opcode, self.reg.a),
            0xe0 | 0xe4 | 0xec                                    => self.compare(opcode, self.reg.x),
            0xc0 | 0xc4 | 0xcc                                    => self.compare(opcode, self.reg.y),

            0xe8 => self.inx(opcode), 0xca => self.dex(opcode),
            0xc8 => self.iny(opcode), 0x88 => self.dey(opcode),
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

            0x18 => self.set_flag(Flags::CARRY, false),
            0xd8 => self.set_flag(Flags::DECIMAL, false),
            0x58 => self.set_flag(Flags::INTERRUPT, false),
            0xb8 => self.set_flag(Flags::OVERFLOW, false),

            0x48 => self.pha(opcode),
            0x08 => self.php(opcode),
            0x68 => self.pla(opcode),
            0x28 => self.plp(opcode),

            0x20 => self.jsr(opcode),
            0x40 => self.rti(opcode),
            0x60 => self.rts(opcode),
            0x00 => self.interrupt(Interrupt::BRK),
            0xea => {},

            _ => ()
        }

        if pc_state == self.reg.pc {
            self.reg.pc += (opcode.size - 1) as u16;
        }
        if opcode.code != 0x00 {
            self.cycle += opcode.tick as u32;
        }
    }

    pub fn stack_push(&mut self, byte: u8) {
        self.bus.write(STACK_BASE.wrapping_add(self.reg.sp as u16), byte);
        self.reg.sp = self.reg.sp.wrapping_sub(1);
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        self.stack_push((data >> 0x8) as u8);
        self.stack_push((data & 0xff) as u8);
    }

    pub fn stack_pull(&mut self) -> u8 {
        self.reg.sp = self.reg.sp.wrapping_add(1);
        self.bus.read(STACK_BASE.wrapping_add(self.reg.sp as u16))
    }

    pub fn stack_pull_u16(&mut self) -> u16 {
        let lo = self.stack_pull() as u16;
        let hi = self.stack_pull() as u16;

        (hi << 8) | lo
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        self.reg.p.set(flag, value);
    }

    fn update_nz_flags(&mut self, value: u8) {
        self.set_flag(Flags::ZERO, value == 0);
        self.set_flag(Flags::NEGATIVE, (value as i8) < 0);
    }

    fn on_tick_modifier(&mut self, lo: u8, hi: u8, byte: u8, modifier: TickModifier) -> Operand {
        match (lo.overflowing_add(byte).1, modifier) {
            (true, TickModifier::PageCrossed) => self.cycle += 1,
            (true, TickModifier::Branch)      => self.cycle += 1,

            _ => ()
        }

        Operand::Address(((hi as u16) << 8 | lo as u16).wrapping_add(byte as u16))
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

    fn and(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a &= self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    #[inline]
    fn _asl(&mut self, value: u8) -> u8 {
        self.set_flag(Flags::CARRY, value >> 7 == 1);
        let result = value.wrapping_shl(1);
        self.update_nz_flags(result);
        result
    }

    fn asl(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._asl(self.reg.a);
            },
            Operand::Address(addr) => {
                let byte = self._asl(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
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


    fn bit(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let value = self.reg.a & self.bus.read(addr);

            self.update_nz_flags(value);
            self.reg.p.set(Flags::NEGATIVE, value & 0x80 > 0);
            self.reg.p.set(Flags::OVERFLOW, value & 0x40 > 0);
        }
    }

    fn compare(&mut self, opcode: &Opcode, reg: u8) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let operand = self.bus.read(addr);

            self.set_flag(Flags::CARRY, reg <= operand);
            self.update_nz_flags(reg.wrapping_sub(operand));
        }
    }

    fn dec(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let value = self.bus.read(addr).wrapping_sub(1);
            self.bus.write(addr, value);
            self.update_nz_flags(value);
        }
    }

    fn dex(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_nz_flags(self.reg.x);
    }

    fn dey(&mut self, opcode: &Opcode) {
        self.reg.y = self.reg.y.wrapping_sub(1);
        self.update_nz_flags(self.reg.y);
    }

    fn eor(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a ^= self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    fn inc(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let value = self.bus.read(addr).wrapping_add(1);
            self.bus.write(addr, value);
            self.update_nz_flags(value);
        }
    }

    fn inx(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_nz_flags(self.reg.x);
    }

    fn iny(&mut self, opcode: &Opcode) {
        self.reg.y = self.reg.y.wrapping_add(1);
        self.update_nz_flags(self.reg.y);
    }

    fn jmp(&mut self, opcode: &Opcode) {
        let operand = self.bus.read_u16(self.reg.pc);

        if opcode.code == 0x6c {
            if operand & 0xff != 0xff {
                return { self.reg.pc = self.bus.read_u16(operand); };
            }

            // 6502 indirect jump bug

            let lo = self.bus.read(operand) as u16;
            let hi = self.bus.read(operand & 0xff00) as u16;

            return { self.reg.pc = (hi << 8) | lo; };
        }

        self.reg.pc = operand;
    }

    // TODO: combine load instructions into one `load` function

    fn lda(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a = self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    fn ldx(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.x = self.bus.read(addr);
            self.update_nz_flags(self.reg.x);
        }
    }

    fn ldy(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.y = self.bus.read(addr);
            self.update_nz_flags(self.reg.y);
        }
    }

    #[inline]
    fn _lsr(&mut self, value: u8) -> u8 {
        self.set_flag(Flags::CARRY, value & 0x1 != 0);
        let result = value.wrapping_shr(1);
        self.set_flag(Flags::ZERO, value == 0);
        result
    }

    fn lsr(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator   => {
                self.reg.a = self._lsr(self.reg.a);
            },
            Operand::Address(addr) => {
                let byte = self._lsr(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
    }

    fn ora(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a |= self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    #[inline]
    fn _rol(&mut self, value: u8) -> u8 {
        let carry = self.reg.p.contains(Flags::CARRY);
        let result = (value.rotate_left(1) & 0xfe) | carry as u8;

        self.set_flag(Flags::CARRY, value >> 7 != 0);
        self.update_nz_flags(result);

        result
    }

    fn rol(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._rol(self.reg.a);
            },
            Operand::Address(addr) => {
                let byte = self._rol(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
    }

    fn pha(&mut self, opcode: &Opcode) {
        self.stack_push(self.reg.a);
    }

    fn php(&mut self, opcode: &Opcode) {
        self.set_flag(Flags::BREAK, true);
        self.set_flag(Flags::UNUSED, true);
        self.stack_push(self.reg.p.bits());
    }

    fn pla(&mut self, opcode: &Opcode) {
        self.reg.a = self.stack_pull();
    }

    fn plp(&mut self, opcode: &Opcode) {
        self.reg.p.bits = self.stack_pull();
        self.set_flag(Flags::BREAK, false);
        self.set_flag(Flags::UNUSED, false);
    }

    fn tax(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.a;
        self.update_nz_flags(self.reg.x);
    }

    fn txa(&mut self, opcode: &Opcode) {
        self.reg.a = self.reg.x;
        self.update_nz_flags(self.reg.a);
    }

    fn tay(&mut self, opcode: &Opcode) {
        self.reg.y = self.reg.a;
        self.update_nz_flags(self.reg.y);
    }

    fn tya(&mut self, opcode: &Opcode) {
        self.reg.a = self.reg.y;
        self.update_nz_flags(self.reg.a);
    }

    fn jsr(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.stack_push_u16(self.reg.pc.wrapping_add(2));
            self.reg.pc = self.bus.read_u16(addr);
        }
    }

    fn rti(&mut self, opcode: &Opcode) {
        self.reg.p.bits = self.stack_pull();
        self.reg.pc = self.stack_pull_u16();

        self.reg.p.remove(Flags::BREAK);
    }

    fn rts(&mut self, opcode: &Opcode) {
        self.reg.pc = self.stack_pull_u16();
    }
}
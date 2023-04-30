use crate::bus::Bus;
use crate::Error;

use bitflags::bitflags;
use byte_common::opcode::*;

pub const STACK_BASE: u16 = 0x0100;
pub const NMI_VECTOR: u16 = 0xfffa;
pub const RST_VECTOR: u16 = 0xfffc;
pub const IRQ_VECTOR: u16 = 0xfffe;

#[derive(Debug)]
pub enum Operand {
    Accumulator,
    Address(u16),
}

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

#[derive(Default, Debug, Clone, Copy)]
pub struct Registers {
    pub sp: u8,
    pub pc: u16,

    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub p: Flags,
}

#[derive(Default)]
pub struct CPU {
    pub bus: Bus,
    pub cycle: u64,
    pub reg: Registers,
}

impl CPU {
    pub fn load(&mut self, program: &[u8], start: u16) {
        program
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.bus.write(start + i as u16, *b));
    }

    pub fn interrupt(&mut self, interrupt: Interrupt) {
        let (pc, vector) = match interrupt {
            Interrupt::BRK => (self.reg.pc + 1, IRQ_VECTOR),
            Interrupt::IRQ => (self.reg.pc, IRQ_VECTOR),
            Interrupt::NMI => (self.reg.pc, NMI_VECTOR),
            Interrupt::RST => (0, RST_VECTOR),
        };

        if interrupt != Interrupt::RST {
            let mut p = self.reg.p;
            p.set(Flags::UNUSED, true);
            p.set(Flags::BREAK, interrupt == Interrupt::BRK);

            self.stack_push_u16(pc);
            self.stack_push(p.bits());
            self.set_flag(Flags::INTERRUPT, true);
        }

        self.reg.pc = self.bus.read_u16(vector);
        self.cycle += 7;
    }

    // attrs on expressions is still experimental
    // move this to the line where we match on `opcode.code`
    #[rustfmt::skip]
    pub fn step(&mut self) -> Result<(), Error> {
        let opcode = self.bus.read(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        let pc_copy = self.reg.pc;

        let opcode = OPCODE_MAP
            .get(opcode as usize)
            .and_then(|opcode| opcode.as_ref())
            .ok_or(Error::UnrecognizedOpcode(opcode))?;

        match opcode.code {
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(opcode),
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(opcode),
            0x0a | 0x06 | 0x16 | 0x0e | 0x1e                      => self.asl(opcode),
            0x24 | 0x2c                                           => self.bit(opcode),
            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.cmp(opcode, self.reg.a),
            0xe0 | 0xe4 | 0xec                                    => self.cmp(opcode, self.reg.x),
            0xc0 | 0xc4 | 0xcc                                    => self.cmp(opcode, self.reg.y),
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
            0x6a | 0x66 | 0x76 | 0x6e | 0x7e                      => self.ror(opcode),
            0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => self.sbc(opcode),
            0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91        => self.str(opcode, self.reg.a),
            0x86 | 0x96 | 0x8e                                    => self.str(opcode, self.reg.x),
            0x84 | 0x94 | 0x8c                                    => self.str(opcode, self.reg.y),

            0x90 => self.branch(opcode, !self.reg.p.contains(Flags::CARRY)),
            0xb0 => self.branch(opcode,  self.reg.p.contains(Flags::CARRY)),
            0xf0 => self.branch(opcode,  self.reg.p.contains(Flags::ZERO)),
            0xd0 => self.branch(opcode, !self.reg.p.contains(Flags::ZERO)),
            0x10 => self.branch(opcode, !self.reg.p.contains(Flags::NEGATIVE)),
            0x30 => self.branch(opcode,  self.reg.p.contains(Flags::NEGATIVE)),
            0x70 => self.branch(opcode,  self.reg.p.contains(Flags::OVERFLOW)),
            0x50 => self.branch(opcode, !self.reg.p.contains(Flags::OVERFLOW)),

            0xca => self.dex(opcode),
            0x88 => self.dey(opcode),

            0xe8 => self.inx(opcode),
            0xc8 => self.iny(opcode),

            0x18 => self.set_flag(Flags::CARRY, false),
            0xd8 => self.set_flag(Flags::DECIMAL, false),
            0x58 => self.set_flag(Flags::INTERRUPT, false),
            0xb8 => self.set_flag(Flags::OVERFLOW, false),

            0x48 => self.pha(opcode),
            0x08 => self.php(opcode),
            0x68 => self.pla(opcode),
            0x28 => self.plp(opcode),

            0x40 => self.rti(opcode),
            0x60 => self.rts(opcode),

            0x38 => self.set_flag(Flags::CARRY, true),
            0xf8 => self.set_flag(Flags::DECIMAL, true),
            0x78 => self.set_flag(Flags::INTERRUPT, true),

            0xaa => self.tax(opcode),
            0x8a => self.txa(opcode),
            0xa8 => self.tay(opcode),
            0x98 => self.tya(opcode),
            0xba => self.tsx(opcode),
            0x9a => self.txs(opcode),

            0x00 => {
                self.interrupt(Interrupt::BRK);
                return Ok(());
            }
            0x20 => self.jsr(opcode),
            0xea => {},
            _    => {}
        }

        if pc_copy == self.reg.pc {
            self.reg.pc = self.reg.pc.wrapping_add((opcode.size - 1) as u16);
        }

        self.cycle += opcode.tick as u64;
        Ok(())
    }

    pub fn stack_push(&mut self, byte: u8) {
        self.bus
            .write(STACK_BASE.wrapping_add(self.reg.sp as u16), byte);
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
        self.set_flag(Flags::NEGATIVE, value & 0x80 > 0);
    }

    // this function doesn't actually get called on `TickModifier::Branch`, instead
    // the `branch` function keeps track of branching to a different page
    fn on_tick_modifier(&mut self, lo: u8, hi: u8, byte: u8, _modifier: TickModifier) -> Operand {
        let addr = ((hi as u16) << 8 | (lo as u16)).wrapping_add(byte as u16);

        if hi != (addr >> 8) as u8 {
            self.cycle += 1
        }

        Operand::Address(addr)
    }

    fn get_operand(&mut self, opcode: &Opcode) -> Operand {
        match opcode.mode {
            AddressingMode::Relative => Operand::Address(self.reg.pc),
            AddressingMode::Immediate => Operand::Address(self.reg.pc),
            AddressingMode::Accumulator => Operand::Accumulator,

            AddressingMode::ZeroPage => Operand::Address(self.bus.read(self.reg.pc) as u16),
            AddressingMode::ZeroPageX => {
                Operand::Address(self.bus.read(self.reg.pc).wrapping_add(self.reg.x) as u16)
            }
            AddressingMode::ZeroPageY => {
                Operand::Address(self.bus.read(self.reg.pc).wrapping_add(self.reg.y) as u16)
            }

            AddressingMode::Absolute => Operand::Address(self.bus.read_u16(self.reg.pc)),
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

            AddressingMode::Indirect => {
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

            _ => unreachable!(),
        }
    }
}

// Opcode implementations

impl CPU {
    #[inline]
    fn _asl(&mut self, value: u8) -> u8 {
        self.set_flag(Flags::CARRY, value >> 7 == 1);
        let result = value.wrapping_shl(1);
        self.update_nz_flags(result);
        result
    }

    #[inline]
    fn _lsr(&mut self, value: u8) -> u8 {
        self.set_flag(Flags::CARRY, value & 0x1 != 0);
        let result = value.wrapping_shr(1);
        self.set_flag(Flags::ZERO, value == 0);
        self.set_flag(Flags::NEGATIVE, false);
        result
    }

    #[inline]
    fn _rol(&mut self, value: u8) -> u8 {
        let result = value.rotate_left(1) & 0xfe | self.reg.p.contains(Flags::CARRY) as u8;

        self.set_flag(Flags::CARRY, value & 0x80 > 0);
        self.update_nz_flags(result);

        result
    }

    #[rustfmt::skip]
    #[inline]
    fn _ror(&mut self, value: u8) -> u8 {
        let result = value.rotate_right(1)
                   & 0x7f
                   | ((self.reg.p.contains(Flags::CARRY) as u8) << 7);

        self.set_flag(Flags::CARRY, value & 0x1 > 0);
        self.update_nz_flags(result);

        result
    }

    #[rustfmt::skip]
    fn adc(&mut self, opcode: &Opcode) {
        let m = self.reg.a as u16;
        let c = self.reg.p.contains(Flags::CARRY) as u16;

        if let Operand::Address(addr) = self.get_operand(opcode) {
            let n = self.bus.read(addr) as u16;

            if self.reg.p.contains(Flags::DECIMAL) {
                let mut l = (m & 0x0f) + (n & 0x0f) + c;
                let mut h = (m & 0xf0) + (n & 0xf0);

                if l > 0x09 {
                    l = (l + 0x06) & 0x0f; h += 0x10;
                };
                self.set_flag(Flags::OVERFLOW, !(m ^ n) & (m ^ h) & 0x80 != 0);
                if h > 0x90 {
                    h += 0x60;
                };
                self.set_flag(Flags::CARRY, h >> 8 > 0);

                self.reg.a = (h | l) as u8
            } else {
                let s = m + n + c;

                self.set_flag(Flags::CARRY, s > 0xff);
                self.set_flag(Flags::OVERFLOW, !(m ^ n) & (m ^ s) & 0x80 != 0);

                self.reg.a = s as u8;
            }

            self.update_nz_flags(self.reg.a);
        }
    }

    fn and(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a &= self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    fn asl(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._asl(self.reg.a);
            }
            Operand::Address(addr) => {
                let byte = self._asl(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
    }

    fn bit(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let operand = self.bus.read(addr);
            let result = self.reg.a & operand;

            self.update_nz_flags(result);
            self.set_flag(Flags::NEGATIVE, operand & 0x80 > 0);
            self.set_flag(Flags::OVERFLOW, operand & 0x40 > 0);
        }
    }

    fn branch(&mut self, opcode: &Opcode, condition: bool) {
        if !condition {
            return;
        }

        self.cycle += 1;

        if let Operand::Address(addr) = self.get_operand(opcode) {
            let page = self.reg.pc >> 8;

            self.reg.pc = self
                .reg
                .pc
                .wrapping_add(1)
                .wrapping_add(i8::from_le_bytes(self.bus.read(addr).to_le_bytes()) as u16);

            if page != self.reg.pc >> 8 {
                self.cycle += 1;
            }
        }
    }

    fn cmp(&mut self, opcode: &Opcode, reg: u8) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let operand = self.bus.read(addr);

            self.set_flag(Flags::ZERO, reg == operand);
            self.set_flag(Flags::CARRY, reg >= operand);
            self.set_flag(Flags::NEGATIVE, reg.wrapping_sub(operand) & 0x80 > 0);
        }
    }

    fn dec(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let value = self.bus.read(addr).wrapping_sub(1);
            self.bus.write(addr, value);
            self.update_nz_flags(value);
        }
    }

    fn dex(&mut self, _opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_nz_flags(self.reg.x);
    }

    fn dey(&mut self, _opcode: &Opcode) {
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

    fn inx(&mut self, _opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_nz_flags(self.reg.x);
    }

    fn iny(&mut self, _opcode: &Opcode) {
        self.reg.y = self.reg.y.wrapping_add(1);
        self.update_nz_flags(self.reg.y);
    }

    fn jmp(&mut self, opcode: &Opcode) {
        let operand = self.bus.read_u16(self.reg.pc);

        if opcode.code == 0x6c {
            if operand & 0xff != 0xff {
                return self.reg.pc = self.bus.read_u16(operand);
            }

            // 6502 indirect jump bug
            let lo = self.bus.read(operand) as u16;
            let hi = self.bus.read(operand & 0xff00) as u16;

            return self.reg.pc = (hi << 8) | lo;
        }

        self.reg.pc = operand;
    }

    fn jsr(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.stack_push_u16(self.reg.pc.wrapping_add(1));
            self.reg.pc = addr;
        }
    }

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

    fn lsr(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._lsr(self.reg.a);
            }
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

    fn pha(&mut self, _opcode: &Opcode) {
        self.stack_push(self.reg.a);
    }

    fn php(&mut self, _opcode: &Opcode) {
        self.stack_push(self.reg.p.bits() | 0x30);
    }

    fn pla(&mut self, _opcode: &Opcode) {
        self.reg.a = self.stack_pull();
        self.update_nz_flags(self.reg.a);
    }

    fn plp(&mut self, _opcode: &Opcode) {
        self.reg.p.bits = self.stack_pull() | 0x30;
    }

    fn rol(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._rol(self.reg.a);
            }
            Operand::Address(addr) => {
                let byte = self._rol(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
    }

    fn ror(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.reg.a = self._ror(self.reg.a);
            }
            Operand::Address(addr) => {
                let byte = self._ror(self.bus.read(addr));
                self.bus.write(addr, byte);
            }
        }
    }

    fn rti(&mut self, _opcode: &Opcode) {
        self.reg.p.bits = self.stack_pull() | 0x30;
        self.reg.pc = self.stack_pull_u16();
    }

    fn rts(&mut self, _opcode: &Opcode) {
        self.reg.pc = self.stack_pull_u16() + 1;
    }

    #[rustfmt::skip]
    fn sbc(&mut self, opcode: &Opcode) {
        let m = self.reg.a;
        let c = self.reg.p.contains(Flags::CARRY) as u8;

        if let Operand::Address(addr) = self.get_operand(opcode) {
            let n = self.bus.read(addr);
            let mut s =  m as u16
                      + !n as u16
                      +  c as u16;

            self.update_nz_flags(s as u8);
            self.set_flag(Flags::CARRY, s > 0xff);
            self.set_flag(Flags::OVERFLOW, (m ^ n) & (m ^ s as u8) & 0x80 > 0);

            if self.reg.p.contains(Flags::DECIMAL) {
                let mut l = (m & 0x0f) as i16 - (n & 0x0f) as i16 + (c as i16) - 1;
                let mut h = (m & 0xf0) as i16 - (n & 0xf0) as i16;

                if l < 0x00 { l = (l - 0x06) & 0x0f; h -= 0x10; }
                if h < 0x00 { h = (h - 0x60) & 0xf0; }

                s = (h | l) as u16;
            }

            self.reg.a = s as u8;
            self.update_nz_flags(self.reg.a);
        }
    }

    fn str(&mut self, opcode: &Opcode, reg: u8) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.bus.write(addr, reg);
        }
    }

    fn tax(&mut self, _opcode: &Opcode) {
        self.reg.x = self.reg.a;
        self.update_nz_flags(self.reg.x);
    }

    fn tay(&mut self, _opcode: &Opcode) {
        self.reg.y = self.reg.a;
        self.update_nz_flags(self.reg.y);
    }

    fn tsx(&mut self, _opcode: &Opcode) {
        self.reg.x = self.reg.sp;
        self.update_nz_flags(self.reg.x);
    }

    fn txa(&mut self, _opcode: &Opcode) {
        self.reg.a = self.reg.x;
        self.update_nz_flags(self.reg.a);
    }

    fn txs(&mut self, _opcode: &Opcode) {
        self.reg.sp = self.reg.x;
    }

    fn tya(&mut self, _opcode: &Opcode) {
        self.reg.a = self.reg.y;
        self.update_nz_flags(self.reg.a);
    }
}

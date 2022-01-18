use bitflags::bitflags;

use core::fmt;
use std::cmp::Ordering;
use std::ops::ControlFlow;

use crate::bus::Bus;
use crate::opcode::*;

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
            pc: 0, sp: 0xff,

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
            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.cmp(opcode, self.reg.a),
            0xe0 | 0xe4 | 0xec                                    => self.cmp(opcode, self.reg.x),
            0xc0 | 0xc4 | 0xcc                                    => self.cmp(opcode, self.reg.y),
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(opcode),

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

            0x18 => self.set_flag(Flags::CARRY, false),
            0xd8 => self.set_flag(Flags::DECIMAL, false),
            0x58 => self.set_flag(Flags::INTERRUPT, false),
            0xb8 => self.set_flag(Flags::OVERFLOW, false),

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

    fn asl(&mut self, opcode: &Opcode) {
        match self.get_operand(opcode) {
            Operand::Accumulator => {
                self.set_flag(Flags::CARRY, self.reg.a >> 7 == 1);
                self.reg.a = self.reg.a.wrapping_shl(1);
                self.update_nz_flags(self.reg.a);
            },
            Operand::Address(addr) => {
                let mut data = self.bus.read(addr);
                self.set_flag(Flags::CARRY, data >> 7 == 1);

                data = data.wrapping_shl(1);
                self.bus.write(addr, data);
                self.update_nz_flags(data);
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

    fn cmp(&mut self, opcode: &Opcode, reg: u8) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            let operand = self.bus.read(addr);
            eprintln!("{:x},{:x}, {}", reg, operand, (reg.wrapping_sub(operand)) as i8);

            self.set_flag(Flags::CARRY, reg <= operand);
            self.update_nz_flags(reg.wrapping_sub(operand));
        }
    }

    fn lda(&mut self, opcode: &Opcode) {
        if let Operand::Address(addr) = self.get_operand(opcode) {
            self.reg.a = self.bus.read(addr);
            self.update_nz_flags(self.reg.a);
        }
    }

    fn inx(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_add(1);
        self.update_nz_flags(self.reg.x);
    }

    fn dex(&mut self, opcode: &Opcode) {
        self.reg.x = self.reg.x.wrapping_sub(1);
        self.update_nz_flags(self.reg.x);
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
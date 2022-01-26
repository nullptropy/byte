use core::fmt;
use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Implied, Immediate,
    Relative, Accumulator,
    ZeroPage, ZeroPageX, ZeroPageY,
    Absolute, AbsoluteX, AbsoluteY,
    Indirect, IndirectX, IndirectY,
}

pub struct Opcode {
    pub code: u8,
    pub size: u8,
    pub tick: u8,
    pub name: &'static str,
    pub mode: AddressingMode,
    pub tick_modifier: Option<TickModifier>,
}

pub enum Operand {
    Accumulator,
    Address(u16),
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}:{:02x}:{}:{}:{:?}:{:?}",
            self.name, self.code,
            self.size, self.tick,
            self.mode, self.tick_modifier
        )
    }
}

impl Opcode {
    pub fn new(
        code: u8,
        size: u8,
        tick: u8,
        name: &'static str,
        mode: AddressingMode,
        tick_modifier: Option<TickModifier>,
    ) -> Self {
        Self {
            code, size, tick,
            name, mode, tick_modifier,
        }
    }
}

lazy_static! {
    pub static ref OPCODE_MAP: HashMap<u8, Opcode> = HashMap::from([
        (0x00, Opcode::new(0x00, 1, 7, "BRK", AddressingMode::Implied, None)),

        (0x29, Opcode::new(0x29, 2, 2, "AND", AddressingMode::Immediate, None)),
        (0x25, Opcode::new(0x25, 2, 3, "AND", AddressingMode::ZeroPage, None)),
        (0x35, Opcode::new(0x35, 2, 4, "AND", AddressingMode::ZeroPageX, None)),
        (0x2d, Opcode::new(0x2d, 3, 4, "AND", AddressingMode::Absolute, None)),
        (0x3d, Opcode::new(0x3d, 3, 4, "AND", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0x39, Opcode::new(0x39, 3, 4, "AND", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0x21, Opcode::new(0x21, 2, 6, "AND", AddressingMode::IndirectX, None)),
        (0x31, Opcode::new(0x31, 2, 5, "AND", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),

        (0x0a, Opcode::new(0x0a, 1, 2, "ASL", AddressingMode::Accumulator, None)),
        (0x06, Opcode::new(0x06, 2, 5, "ASL", AddressingMode::ZeroPage, None)),
        (0x16, Opcode::new(0x16, 2, 6, "ASL", AddressingMode::ZeroPageX, None)),
        (0x0e, Opcode::new(0x0e, 3, 6, "ASL", AddressingMode::Absolute, None)),
        (0x1e, Opcode::new(0x1e, 3, 7, "ASL", AddressingMode::AbsoluteX, None)),

        (0x90, Opcode::new(0x90, 2, 2, "BCC", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0xb0, Opcode::new(0xb0, 2, 2, "BCS", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0xf0, Opcode::new(0xf0, 2, 2, "BEQ", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0x30, Opcode::new(0x30, 2, 2, "BMI", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0xd0, Opcode::new(0xd0, 2, 2, "BNE", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0x10, Opcode::new(0x10, 2, 2, "BPL", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0x50, Opcode::new(0x50, 2, 2, "BVC", AddressingMode::Relative, Some(TickModifier::Branch))),
        (0x70, Opcode::new(0x70, 2, 2, "BVS", AddressingMode::Relative, Some(TickModifier::Branch))),

        (0x24, Opcode::new(0x24, 2, 3, "BIT", AddressingMode::ZeroPage, None)),
        (0x2c, Opcode::new(0x2c, 3, 4, "BIT", AddressingMode::Absolute, None)),

        (0x00, Opcode::new(0x00, 1, 7, "BRK", AddressingMode::Implied, None)),

        (0x18, Opcode::new(0x18, 1, 2, "CLC", AddressingMode::Implied, None)),
        (0xd8, Opcode::new(0xd8, 1, 2, "CLD", AddressingMode::Implied, None)),
        (0x58, Opcode::new(0x58, 1, 2, "CLI", AddressingMode::Implied, None)),
        (0xb8, Opcode::new(0xb8, 1, 2, "CLV", AddressingMode::Implied, None)),

        (0xc9, Opcode::new(0xc9, 2, 2, "CMP", AddressingMode::Immediate, None)),
        (0xc5, Opcode::new(0xc5, 2, 3, "CMP", AddressingMode::ZeroPage, None)),
        (0xd5, Opcode::new(0xd5, 2, 4, "CMP", AddressingMode::ZeroPageX, None)),
        (0xcd, Opcode::new(0xcd, 3, 4, "CMP", AddressingMode::Absolute, None)),
        (0xdd, Opcode::new(0xdd, 3, 4, "CMP", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0xd9, Opcode::new(0xd9, 3, 4, "CMP", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0xc1, Opcode::new(0xc1, 2, 6, "CMP", AddressingMode::IndirectX, None)),
        (0xd1, Opcode::new(0xd1, 2, 5, "CMP", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),

        (0xe0, Opcode::new(0xe0, 2, 2, "CPX", AddressingMode::Immediate, None)),
        (0xe4, Opcode::new(0xe4, 2, 3, "CPX", AddressingMode::ZeroPage, None)),
        (0xec, Opcode::new(0xec, 3, 4, "CPX", AddressingMode::Absolute, None)),

        (0xc0, Opcode::new(0xc0, 2, 2, "CPY", AddressingMode::Immediate, None)),
        (0xc4, Opcode::new(0xc4, 2, 3, "CPY", AddressingMode::ZeroPage, None)),
        (0xcc, Opcode::new(0xcc, 3, 4, "CPY", AddressingMode::Absolute, None)),

        (0xc6, Opcode::new(0xc6, 2, 5, "DEC", AddressingMode::ZeroPage, None)),
        (0xd6, Opcode::new(0xd6, 2, 6, "DEC", AddressingMode::ZeroPageX, None)),
        (0xce, Opcode::new(0xce, 3, 6, "DEC", AddressingMode::Absolute, None)),
        (0xde, Opcode::new(0xde, 3, 7, "DEC", AddressingMode::AbsoluteX, None)),

        (0xca, Opcode::new(0xca, 1, 2, "DEX", AddressingMode::Implied, None)),
        (0x88, Opcode::new(0x88, 1, 2, "DEY", AddressingMode::Implied, None)),

        (0x49, Opcode::new(0x49, 2, 2, "EOR", AddressingMode::Immediate, None)),
        (0x45, Opcode::new(0x45, 2, 3, "EOR", AddressingMode::ZeroPage, None)),
        (0x55, Opcode::new(0x55, 2, 4, "EOR", AddressingMode::ZeroPageX, None)),
        (0x4d, Opcode::new(0x4d, 3, 4, "EOR", AddressingMode::Absolute, None)),
        (0x5d, Opcode::new(0x5d, 3, 4, "EOR", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0x59, Opcode::new(0x59, 3, 4, "EOR", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0x41, Opcode::new(0x41, 2, 6, "EOR", AddressingMode::IndirectX, None)),
        (0x51, Opcode::new(0x51, 2, 5, "EOR", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),

        (0xe6, Opcode::new(0xe6, 2, 5, "INC", AddressingMode::ZeroPage, None)),
        (0xf6, Opcode::new(0xf6, 2, 6, "INC", AddressingMode::ZeroPageX, None)),
        (0xee, Opcode::new(0xee, 3, 6, "INC", AddressingMode::Absolute, None)),
        (0xfe, Opcode::new(0xfe, 3, 7, "INC", AddressingMode::AbsoluteX, None)),

        (0xe8, Opcode::new(0xe8, 1, 2, "INX", AddressingMode::Implied, None)),
        (0xc8, Opcode::new(0xc8, 1, 2, "INY", AddressingMode::Implied, None)),

        (0x4c, Opcode::new(0x4c, 3, 3, "JMP", AddressingMode::Absolute, None)),
        (0x6c, Opcode::new(0x6c, 3, 5, "JMP", AddressingMode::Indirect, None)),

        (0x20, Opcode::new(0x20, 3, 6, "JSR", AddressingMode::Absolute, None)),

        (0xa9, Opcode::new(0xa9, 2, 2, "LDA", AddressingMode::Immediate, None)),
        (0xa5, Opcode::new(0xa5, 2, 3, "LDA", AddressingMode::ZeroPage, None)),
        (0xb5, Opcode::new(0xb5, 2, 4, "LDA", AddressingMode::ZeroPageX, None)),
        (0xad, Opcode::new(0xad, 3, 4, "LDA", AddressingMode::Absolute, None)),
        (0xbd, Opcode::new(0xbd, 3, 4, "LDA", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0xb9, Opcode::new(0xb9, 3, 4, "LDA", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0xa1, Opcode::new(0xa1, 2, 6, "LDA", AddressingMode::IndirectX, None)),
        (0xb1, Opcode::new(0xb1, 2, 5, "LDA", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),

        (0xaa, Opcode::new(0xaa, 1, 2, "TAX", AddressingMode::Implied, None)),
        (0x8a, Opcode::new(0x8a, 1, 2, "TXA", AddressingMode::Implied, None)),
        (0xa8, Opcode::new(0xa8, 1, 2, "TAY", AddressingMode::Implied, None)),
        (0x98, Opcode::new(0x98, 1, 2, "TYA", AddressingMode::Implied, None)),

        (0x40, Opcode::new(0x40, 1, 6, "RTI", AddressingMode::Implied, None)),
        (0x60, Opcode::new(0x60, 1, 6, "RTS", AddressingMode::Implied, None)),

        (0xea, Opcode::new(0xea, 1, 2, "NOP", AddressingMode::Implied, None)),
    ]);
}
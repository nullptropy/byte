use core::fmt;
use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum TickModifier {
    PageCrossed,
    BranchOnSamePage,
    BranchToDifferentPage,
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

impl TickModifier {
    fn get_cycles(&self) -> i32 {
        match self {
            Self::PageCrossed | Self::BranchOnSamePage => 1,
            Self::BranchToDifferentPage => 2,
        }
    }
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
        (0xe8, Opcode::new(0xe8, 1, 2, "INX", AddressingMode::Implied, None)),
        (0xca, Opcode::new(0xca, 1, 2, "DEX", AddressingMode::Implied, None)),

        (0xaa, Opcode::new(0xaa, 1, 2, "TAX", AddressingMode::Implied, None)),
        (0x8a, Opcode::new(0x8a, 1, 2, "TXA", AddressingMode::Implied, None)),
        (0xa8, Opcode::new(0xa8, 1, 2, "TAY", AddressingMode::Implied, None)),
        (0x98, Opcode::new(0x98, 1, 2, "TYA", AddressingMode::Implied, None)),

        (0x29, Opcode::new(0x29, 2, 2, "AND", AddressingMode::Immediate, None)),
        (0x25, Opcode::new(0x25, 2, 3, "AND", AddressingMode::ZeroPage, None)),
        (0x35, Opcode::new(0x35, 2, 4, "AND", AddressingMode::ZeroPageX, None)),
        (0x2d, Opcode::new(0x2d, 3, 4, "AND", AddressingMode::Absolute, None)),
        (0x3d, Opcode::new(0x3d, 3, 4, "AND", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0x39, Opcode::new(0x39, 3, 4, "AND", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0x21, Opcode::new(0x21, 2, 6, "AND", AddressingMode::IndirectX, None)),
        (0x31, Opcode::new(0x31, 2, 5, "AND", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),

        (0xa9, Opcode::new(0xa9, 2, 2, "LDA", AddressingMode::Immediate, None)),
        (0xa5, Opcode::new(0xa5, 2, 3, "LDA", AddressingMode::ZeroPage,  None)),
        (0xb5, Opcode::new(0xb5, 2, 4, "LDA", AddressingMode::ZeroPageX, None)),
        (0xad, Opcode::new(0xad, 3, 4, "LDA", AddressingMode::Absolute,  None)),
        (0xbd, Opcode::new(0xbd, 3, 4, "LDA", AddressingMode::AbsoluteX, Some(TickModifier::PageCrossed))),
        (0xb9, Opcode::new(0xb9, 3, 4, "LDA", AddressingMode::AbsoluteY, Some(TickModifier::PageCrossed))),
        (0xa1, Opcode::new(0xa1, 2, 6, "LDA", AddressingMode::IndirectX, None)),
        (0xb1, Opcode::new(0xb1, 2, 5, "LDA", AddressingMode::IndirectY, Some(TickModifier::PageCrossed))),
    ]);
}
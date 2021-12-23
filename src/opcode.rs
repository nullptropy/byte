use core::fmt;
use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum TickModifier {
    None,
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
    pub tick_modifier: TickModifier,
}

impl TickModifier {
    fn get_cycles(&self) -> i32 {
        match self {
            Self::PageCrossed | Self::BranchOnSamePage => 1,
            Self::BranchToDifferentPage => 2,
            Self::None => 0,
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
        tick_modifier: TickModifier,
    ) -> Self {
        Self {
            code, size, tick,
            name, mode, tick_modifier,
        }
    }
}

lazy_static! {
    pub static ref OPCODE_MAP: HashMap<u8, Opcode> = HashMap::from([
        (0x00, Opcode::new(0x00, 1, 7, "BRK", AddressingMode::Implied, TickModifier::None)),
    ]);
}
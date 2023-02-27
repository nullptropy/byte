use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Opcode {
    pub code: u8,
    pub size: u8,
    pub tick: u8,
    pub name: &'static str,
    pub mode: AddressingMode,
    pub tick_modifier: Option<TickModifier>,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{:02x}:{}:{}:{:?}:{:?}",
            self.name, self.code, self.size, self.tick, self.mode, self.tick_modifier
        )
    }
}

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

include!(concat!(env!("OUT_DIR"), "/mnemonics.rs"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Opcode {
    pub code: u8,
    pub size: u8,
    pub tick: u8,
    pub mnemonic: Mnemonic,
    pub mode: AddressingMode,
    pub tick_modifier: Option<TickModifier>,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}:{:02x}:{}:{}:{:?}:{:?}",
            self.mnemonic, self.code, self.size, self.tick, self.mode, self.tick_modifier
        )
    }
}

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

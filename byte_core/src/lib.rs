pub mod bus;
pub mod cpu;
pub mod opcode;

pub mod prelude {
    pub use crate::bus::{Bus, Peripheral};
    pub use crate::cpu::{Flags, Interrupt, CPU, IRQ_VECTOR, NMI_VECTOR, RST_VECTOR};
    pub use crate::opcode::{AddressingMode, Opcode, Operand, TickModifier, OPCODE_MAP};
}

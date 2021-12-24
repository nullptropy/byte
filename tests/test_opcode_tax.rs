use mos6502::*;

#[test]
fn opcode_0x8a_implied_txa() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}
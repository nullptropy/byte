use mos6502::*;

#[test]
fn opcode_0xa9_immediate_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xa5_zeropage_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb5_zeropagex_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xad_absolute_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xbd_absolutex_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb9_absolutey_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xa1_indirectx_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb1_indirecty_lda() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}
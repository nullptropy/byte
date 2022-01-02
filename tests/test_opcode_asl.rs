mod common;

#[test]
fn opcode_0x0a_accumulator_asl() {
    let mut cpu = common::init_cpu();

    cpu.reg.a = 0b1010_1010;
    cpu.load_and_run(&[
        0x0a,  // ASL A
        0x00,  // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.a, 0b0101_0100);

    assert!( cpu.reg.p.contains(common::cpu::Flags::CARRY));
    assert!(!cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0x06_zeropage_asl() {
    let mut cpu = common::init_cpu();

    cpu.bus.write(0xaa, 0b1010_1010);
    cpu.load_and_run(&[
        0x06, 0xaa,  // ASL $aa
        0x00         // BRK
    ], 0x8000);

    assert_eq!(cpu.bus.read(0xaa), 0b0101_0100);

    assert!( cpu.reg.p.contains(common::cpu::Flags::CARRY));
    assert!(!cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0x16_zeropagex_asl() {}

#[test]
fn opcode_0x0e_absolute_asl() {}

#[test]
fn opcode_0x1e_absolutex_asl() {}
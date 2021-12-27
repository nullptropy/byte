mod common;

#[test]
fn opcode_0xaa_implied_tax() {
    let mut cpu = common::init_cpu();

    cpu.reg.a = 0x5;
    cpu.load_and_run(&[
        0xaa,   // TAX
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.x, 0x5);
    assert_eq!(cpu.reg.p.bits(), 0x00);
}
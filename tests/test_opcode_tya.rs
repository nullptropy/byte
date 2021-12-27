mod common;

#[test]
fn opcode_0x98_implied_tya() {
    let mut cpu = common::init_cpu();

    cpu.reg.y = 0x5;
    cpu.load_and_run(&[
        0x98,   // TYA
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.a, 0x5);
    assert_eq!(cpu.reg.p.bits(), 0x00);
}
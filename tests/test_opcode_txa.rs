mod common;

#[test]
fn opcode_0x8a_implied_txa() {
    let mut cpu = common::init_cpu();

    cpu.reg.x = 0x5;
    cpu.load_and_run(&[
        0x8a,   // TXA
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.a, 0x5);
    assert_eq!(cpu.reg.p.bits(), 0x00);
}
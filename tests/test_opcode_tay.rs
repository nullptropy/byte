mod common;

#[test]
fn opcode_0xa8_implied_tay() {
    let mut cpu = common::init_cpu();

    cpu.reg.a = 0x5;
    cpu.load_and_run(&[
        0xa8,   // TAY
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.y, 5);
    assert_eq!(cpu.reg.p.bits(), 0x00);
}
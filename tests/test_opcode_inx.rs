mod common;

#[test]
fn opcode_0xe8_implied_inx() {
    let mut cpu = common::init_cpu();

    cpu.load_and_run(&[
        0xe8,   // INX
        0xe8,   // INX
        0xe8,   // INX
        0xe8,   // INX
        0xe8,   // INX
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.x, 0x5);
}
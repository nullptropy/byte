mod common;

#[test]
fn opcode_0xca_implied_dex() {
    let mut cpu = common::init_cpu();

    cpu.reg.x = 0x3;
    cpu.load_and_run(&[
        0xca,   // DEX
        0xca,   // DEX
        0xca,   // DEX
        0x00,   // BRK
    ], 0x8000);

    assert_eq!(cpu.reg.x, 0x0);
    assert!(cpu.reg.p.contains(common::cpu::Flags::ZERO));
}
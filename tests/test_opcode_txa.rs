mod common;

#[test]
fn opcode_0x8a_implied_txa() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}
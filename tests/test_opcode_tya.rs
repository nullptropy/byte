mod common;

#[test]
fn opcode_0x98_implied_tya() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}
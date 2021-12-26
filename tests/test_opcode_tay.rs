mod common;

#[test]
fn opcode_0xa8_implied_tay() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}
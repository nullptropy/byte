mod common;

#[test]
fn opcode_0xca_implied_dex() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}
mod common;

// testing only one addressing mode (at least for and) suffices
// since lda's tests implement rest of the addressing modes.
// thus we rely on lda's tests.

#[test]
fn opcode_0x29_immediate_and() {
    let mut cpu = common::init_cpu();

    cpu.reg.a = 0b1111_1111;
    cpu.load_and_run(&[
        0x29, 0x80,  // AND #$80
        0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0x80);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0x25_zeropage_and() {}

#[test]
fn opcode_0x35_zeropagex_and() {}

#[test]
fn opcode_0x2d_absolute_and() {}

#[test]
fn opcode_0x3d_absolutex_and() {}

#[test]
fn opcode_0x39_absolutey_and() {}

#[test]
fn opcode_0x21_indirectx_and() {}

#[test]
fn opcode_0x31_indirecty_and() {}
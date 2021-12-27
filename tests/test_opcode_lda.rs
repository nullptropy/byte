mod common;

#[test]
fn opcode_0xa9_immediate_lda() {
    let mut cpu = common::init_cpu();

    cpu.load_and_run(&[
       0xa9, 0xff,  // LDA #$ff
       0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xa5_zeropage_lda() {
    let mut cpu = common::init_cpu();

    cpu.bus.write(0xee, 0xff);
    cpu.load_and_run(&[
        0xa5, 0xee,  // LDA $ee
        0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xb5_zeropagex_lda() {
    let mut cpu = common::init_cpu();

    cpu.reg.x = 0x3;
    cpu.bus.write(0xee, 0xff);
    cpu.load_and_run(&[
        0xb5, 0xeb,  // LDA $eb,x
        0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xad_absolute_lda() {
    let mut cpu = common::init_cpu();

    cpu.bus.write(0xfaaf, 0xff);
    cpu.load_and_run(&[
        0xad, 0xaf, 0xfa,  // LDA $0xfaaf
        0x00,              // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xbd_absolutex_lda() {
    let mut cpu = common::init_cpu();

    cpu.reg.x = 0x3;
    cpu.bus.write(0xfaaf, 0xff);
    cpu.load_and_run(&[
        0xbd, 0xac, 0xfa,  // LDA $faac,x
        0x00,              // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xb9_absolutey_lda() {
    let mut cpu = common::init_cpu();

    cpu.reg.y = 0x3;
    cpu.bus.write(0xfaaf, 0xff);
    cpu.load_and_run(&[
        0xb9, 0xac, 0xfa,  // LDA $faac,y
        0x00,              // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xa1_indirectx_lda() {
    let mut cpu = common::init_cpu();

    cpu.reg.x = 0x4;
    cpu.bus.write(0xfaaf, 0xff);
    cpu.bus.write_u16(0x09, 0xfaaf);

    cpu.load_and_run(&[
        0xa1, 0x05,  // LDA ($05,x)
        0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}

#[test]
fn opcode_0xb1_indirecty_lda() {
    let mut cpu = common::init_cpu();

    cpu.reg.y = 0x4;
    cpu.bus.write(0xff04, 0xff);
    cpu.bus.write_u16(0x10, 0xff00);

    cpu.load_and_run(&[
        0xb1, 0x10,  // LDA ($10),y
        0x00,        // BRK
    ], 0x8000);

    assert!(cpu.reg.a == 0xff);
    assert!(cpu.reg.p.contains(common::cpu::Flags::NEGATIVE));
}
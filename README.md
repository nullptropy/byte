# byte

A work-in-progress fantasy console designed to provide a user-friendly platform for those who want to learn 6502 assembly.

# Progress

- [x] core emulation of 6502, passes [Klaus' test suite](https://github.com/Klaus2m5/6502_65C02_functional_tests)
- [ ] functional emulator
- [ ] custom assembler
- [ ] custom programming language

There is a simple PoC deployed at [brkp.github.io/byte](https://brkp.github.io/byte), running [static.s](byte_emu/assets/static.s). I intend to focus the development efforts on creating the assembler before working on the emulator part of the project.

# Misc

**[WIP]**: Everything described here is intended solely as a starting point; nothing is definitive or permanent.

Byte is a fantasy console that runs on the 6502 microprocessor and features a compact 64x64 screen and an 8-key gamepad keyboard. Its primary purpose is to provide a user-friendly platform for learning 6502 assembly language programming, with the goal of lowering the barrier to entry for aspiring developers.

# Special Registers

* **0xfd**: **Video Page Pointer**
  -  This register contains a pointer to the page that will contain the framebuffer.
* **0xfe**: **RNG Source**
  - This register resets after each executed instruction and serves as a source of random numbers.
* **0xff**: **Input Register**
  - This register holds the key that is currently being pressed down.

|Key  |Mapping|Value|
|-----|-------|-----|
|Up   |Up     |0x01 |
|Down |Down   |0x02 |
|Left |Left   |0x03 |
|Right|Right  |0x04 |
|A    |Select |0x05 |
|S    |Start  |0x06 |
|D    |A      |0x07 |
|F    |B      |0x08 |

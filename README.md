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

Byte is a 6502 based fantasy console that features a 64x64 and 8 key game pad keyboard. It is designed to create a platform for those who want to learn 6502 assembly, its aim is to lower the entry barrier.

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

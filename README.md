# byte

A work-in-progress fantasy console designed to provide a user-friendly platform for those who want to learn 6502 assembly.

# Progress

- [x] core emulation of 6502, passes [Klaus' test suite](https://github.com/Klaus2m5/6502_65C02_functional_tests)
- [ ] functional emulator
  - [x] loading binary/text files
  - [x] base emulator implementation (the console with a screen and keypad)
  - [x] interactive memory monitor
  - [ ] step debugger
  - [ ] code editor
  - [ ] in memory virtual file system for the wasm target [fork: gh/heaptr/rust-vfs](https://github.com/heaptr/rust-vfs)
- [ ] custom assembler
- [ ] custom programming language

There is a simple PoC deployed at [heaptr.github.io/byte](https://heaptr.github.io/byte), running [demo.s](byte_emu/assets/demo.s).

# Special Registers

* **0xfd**: **Video Page Pointer**
  -  This register contains a pointer to the page that will contain the framebuffer.
* **0xfe**: **RNG Source**
  - This register resets after each executed instruction and serves as a source of random numbers.
* **0xff**: **Input Register**
  - This register holds the key that is currently being pressed down.

**Key mapping**:

| Key    | Mapping    | Mask |
|--------|------------|------|
| Right  | ArrowRight | 0x01 |
| Left   | ArrowLeft  | 0x02 |
| Down   | ArrowDown  | 0x04 |
| Up     | ArrowUp    | 0x08 |
| Start  | S          | 0x10 |
| Select | A          | 0x20 |
| B      | F          | 0x40 |
| A      | D          | 0x80 |

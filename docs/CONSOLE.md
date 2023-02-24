# Byte

**[WIP]**: This document is intended solely as a starting point; nothing is definitive or permanent.

Byte is a 6502 based fantasy console that features a 64x64 and 8 key game pad keyboard. It is designed to create a platform for those who want to learn 6502 assembly, its aim is to lower the entry barrier.

# Special Registers

* **0xfd**: **Video Page Pointer**
  -  This register contains a pointer to the page that will contain the framebuffer.
* **0xfe**: **RNG Source**
  - This register resets after each executed instruction and serves as a source of random numbers.
* **0xff**: **Input Register**
  - This register holds the key that is currently being pressed down.
.org $0000
.org $8000

start:
  jsr init
  jsr loop

loop:
  jsr draw
  jsr update
  jmp loop
  rts

init:
  lda #$00
  sta $10 ; low byte of the counter
  lda #$02
  sta $11 ; high byte of the counter
  rts

draw:
  ldy #$0
  lda $fe
  sta ($10), y
  rts

update:
  ; increment the counter that keeps track of
  ; the pixel to be painted
  lda $10
  clc
  adc #$01
  sta $10
  bcc ret     ; if the carry bit is clear, return
  lda $11 
  cmp #$06    ; check if the upper byte of the counter is 0x06
  bne not_equal
  jsr init    ; if the counter has reached to 0x0600, re-init the counter
  jmp ret
not_equal;
  clc
  adc #$01
  sta $11
ret:
  rts

.org $fffc
  .DW $8000
  .DW $0000

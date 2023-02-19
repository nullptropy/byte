.org $0000
.org $8000
main:
  jsr init
  jsr loop
  jmp main

init:
  lda #$00
  sta $10 ; low byte of the counter
  lda #$02
  sta $11 ; high byte of the counter
  rts
  
loop:
  jsr set_pixel
  jsr increment
  jmp loop
end_loop:
  rts

set_pixel:
  ldy #$0
  lda $fe
  sta ($10), y
  rts

increment:
  lda $10
  clc
  adc #$01
  sta $10
  bcc ret ; if the carry bit is clear, return
  lda $11
  clc
  adc #$01
  sta $11
  cmp #$06
  bne ret
  jsr init
ret:
  rts

.org $fffc
  .DW $8000
  .DW $0000

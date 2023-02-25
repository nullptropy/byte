.org $0000
.org $8000

VIDEO  EQU $fd
RANDOM EQU $fe
INPUT  EQU $ff

reset:
    ldx #$ff
    txs         ; initialize the stack pointer
    lda #$1
    sta VIDEO   ; set the video page to $1000

    jsr init
    jmp loop

init:
    lda #$00
    sta $10     ; low byte of the counter
    lda #$10
    sta $11     ; high byte of the counter
    rts

loop:
    jsr draw
    jsr update
    jmp loop

draw:
    ldy #$00
    lda RANDOM
    sta ($10), y
    rts

update:
    lda $10
    clc
    adc #$01
    sta $10
    bcc ret     ; if the carry bit is clear, return
    lda $11
    clc
    adc #$01
    sta $11
    cmp #$20
    bne ret     ; if the counter has reached to $2000, re-init the counter
    jsr init
ret:
    rts

.org $fffc
    .DW reset
    .DW $0000
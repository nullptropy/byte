; TODO: make use of macros to clear this up
; TODO: add bounds checking so that we don't corrupt the memory when moving around

.org $0000
    .DB "memory monitor"
.org $8000

VIDEO  EQU $fd
RANDOM EQU $fe
INPUT  EQU $ff
COLOR  EQU $fc

POS_L EQU $10
POS_H EQU $11

reset:
    cld         ; disable decimal mode
    ldx #$ff
    txs         ; initialize the stack pointer
    lda #$1
    sta VIDEO   ; set the video page to $1000

    jsr init
    jmp loop

init:
    lda #$00
    sta POS_L     ; low byte of the counter
    lda #POS_L
    sta POS_H     ; high byte of the counter

    lda #$01      ; initial color
    sta COLOR
    rts

loop:
    jmp loop

VBLANK_IRQ:
    jsr handle_input
    jsr update
    jsr draw
    rti

draw:
    ldy #$00
    lda COLOR
    sta (POS_L), y
    rts

update:
    rts

handle_input:
    lda INPUT
    and #%00000100 ; down
    beq input_up
    jsr move_down
    jmp input_ret
input_up:
    lda INPUT
    and #%00001000
    beq input_left
    jsr move_up
    jmp input_ret
input_left:
    lda INPUT
    and #%00000010
    beq input_right
    jsr move_left
    jmp input_ret
input_right:
    lda INPUT
    and #%00000001
    beq input_select
    jsr move_right
input_select:
    lda INPUT
    and #%00100000
    beq input_ret
    inc COLOR
input_ret:
    rts

move_left:
    jsr clear
    lda POS_L
    sec
    sbc #$01
    sta POS_L
    bcs move_left_ret:
    dec POS_H
move_left_ret:
    rts

move_right:
    jsr clear
    lda POS_L
    clc
    adc #$01
    sta POS_L
    bcc move_right_ret
    inc POS_H
move_right_ret:
    rts

move_down:
    jsr clear
    lda POS_L
    clc
    adc #$40
    sta POS_L
    bcc move_down_ret 
    inc POS_H
move_down_ret:
    rts

move_up:
    jsr clear
    lda POS_L
    sec
    sbc #$40
    sta POS_L
    bcs move_up_ret
    dec POS_H
move_up_ret:
    rts

clear:
    lda COLOR
    pha
    lda #$00
    sta COLOR
    jsr draw
    pla
    sta COLOR
    rts

.org $fffc
    .DW reset
    .DW VBLANK_IRQ

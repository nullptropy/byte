; Simple square demo with position and color control.
; Position controlled by arrow keys, color cycled by SELECT.
;
; Key Mappings:
; SELECT: 'A'
;  START: 'S'
;      A: 'D'
;      B: 'F'
;     UP: Up arrow key
;   DOWN: Down arrow key
;   LEFT: Left arrow key
;  RIGHT: Right arrow key

; TODO: Simplify input handling with macros.

VIDEO    EQU $fd
RANDOM   EQU $fe
INPUT    EQU $ff
COLOR    EQU $fc

POS_L    EQU $15
POS_H    EQU $16
PREV_KEY EQU $17
CNT_L    EQU $18
CNT_H    EQU $19

SQ_SIZE  EQU $09
CNT_ROW  EQU $20

.org $0000
    .DB "some random string"
.org $8000

reset:
    cld           ; disable decimal mode
    ldx #$ff
    txs           ; initialize the stack pointer
    lda #$1
    sta VIDEO     ; set the video page to $1000

    jsr init
    jmp loop

init:
    lda #$1c
    sta POS_L     ; low byte of the counter
    lda #$17
    sta POS_H     ; high byte of the counter

    lda #$00      ; initial color
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
    jsr clear
    jsr draw_player
    rts

update:
    rts

handle_input:
    lda INPUT
    beq handle_input_save   ; return if no key is being pressed
handle_input_select:
    and #%00100000
    beq handle_input_up     ; branch if another key is being pressed
    lda PREV_KEY
    and #%00100000          ; check if the prev key is SELECT
    bne handle_input_up
    inc COLOR
    jmp handle_input_save
handle_input_up:
    lda INPUT
    and #%00001000
    beq handle_input_down
    jsr move_up
    jmp handle_input_save
handle_input_down:
    lda INPUT
    and #%00000100
    beq handle_input_left
    jsr move_down
    jmp handle_input_save
handle_input_left:
    lda INPUT
    and #%00000010
    beq handle_input_right
    jsr move_left
    jmp handle_input_save
handle_input_right:
    lda INPUT
    and #%00000001
    beq handle_input_save
    jsr move_right
handle_input_save:
    lda INPUT
    sta PREV_KEY
handle_input_ret:
    rts

move_left:
    lda POS_L
    sec
    sbc #$01
    sta POS_L
    bcs move_left_ret:
    dec POS_H
move_left_ret:
    rts

move_right:
    lda POS_L
    clc
    adc #$01
    sta POS_L
    bcc move_right_ret
    inc POS_H
move_right_ret:
    rts

move_down:
    lda POS_L
    clc
    adc #$40
    sta POS_L
    bcc move_down_ret 
    inc POS_H
move_down_ret:
    rts

move_up:
    lda POS_L
    sec
    sbc #$40
    sta POS_L
    bcs move_up_ret
    dec POS_H
move_up_ret:
    rts

clear:
    lda #$00
    sta CNT_L
    lda #$10
    sta CNT_H
clear_loop:
    lda RANDOM
    ldy #$00
    sta (CNT_L), y
    lda CNT_L
    clc
    adc #$01
    sta CNT_L
    bcc clear_loop
    lda CNT_H
    clc
    adc #$01
    sta CNT_H
    cmp #$20
    bne clear_loop
    rts

draw_player:
    lda POS_H
    pha
    lda POS_L
    pha
    lda #SQ_SIZE
    sta CNT_ROW
    ldy #$00
draw_player_loop_outer:
    ldx #SQ_SIZE
draw_player_loop_inner:
    lda COLOR
    sta (POS_L), y
    lda POS_L
    clc
    adc #$01
    sta POS_L
    bcc draw_no_carry_inner
    lda POS_H
    clc
    adc #$01
    sta POS_H
draw_no_carry_inner:
    dex
    bne draw_player_loop_inner

    lda POS_L
    clc
    adc #($40 - SQ_SIZE)
    sta POS_L
    bcc draw_no_carry_outer
    lda POS_H
    clc
    adc #$01
    sta POS_H
draw_no_carry_outer:
    dec CNT_ROW
    bne draw_player_loop_outer
draw_ret:
    pla
    sta POS_L
    pla
    sta POS_H
    rts

.org $fffc
    .DW reset
    .DW VBLANK_IRQ

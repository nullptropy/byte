; The emulator is currently running a program that allows the user to control
; the position of a colored pixel on the screen and change its color using the SELECT key.
;
; SELECT: mapped to 'A'
;  START: mapped to 'S'
;      A: mapped to 'D'
;      B: mapped to 'F'
;     UP: mapped to the Up Arrow Key
;   DOWN: mapped to the Down Arrow Key
;   LEFT: mapped to the Left Arrow Key
;  RIGHT: mapped to the Right Arrow Key
;
; To move the pixel, use the corresponding arrow keys.

; TODO: make use of macros to clear this up

VIDEO    EQU $fd
RANDOM   EQU $fe
INPUT    EQU $ff
COLOR    EQU $fc

POS_L    EQU $15
POS_H    EQU $16
PREV_KEY EQU $17

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
    lda #$00
    sta POS_L     ; low byte of the counter
    lda #$10
    sta POS_H     ; high byte of the counter

    lda #$01      ; initial color
    sta COLOR
    rts

loop:
    jmp loop

VBLANK_IRQ:
    jsr handle_input
    ; jsr update
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

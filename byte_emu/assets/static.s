; TODO: make use of macros to clear this up
; TODO: add bounds checking so that we don't corrupt the memory when moving around

.org $0000
    .DB "memory monitor"
.org $8000

VIDEO  EQU $fd
RANDOM EQU $fe
INPUT  EQU $ff
COLOR  EQU $fc

; bitflags! {
;     pub struct ByteInputState: u8 {
;         const RIGHT  = 0b00000001;
;         const LEFT   = 0b00000010;
;         const DOWN   = 0b00000100;
;         const UP     = 0b00001000;
;         const START  = 0b00010000;
;         const SELECT = 0b00100000;
;         const B      = 0b01000000;
;         const A      = 0b10000000;
;     }
; }

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
    sta $10     ; low byte of the counter
    lda #$10
    sta $11     ; high byte of the counter
    rts

loop:
    ; jsr draw
    ; jsr update
    jmp loop

VBLANK_IRQ:
    jsr handle_input
    jsr update
    jsr draw
    rti

draw:
    ldy #$00
    lda COLOR
    sta ($10), y
    rts

update:
    lda RANDOM
    sta COLOR
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
    beq input_ret
    jsr move_right
input_ret:
    rts

move_left:
    jsr clear
    lda $10
    sec
    sbc #$01
    sta $10
    bcs move_left_ret:
    dec $11
move_left_ret:
    rts

move_right:
    jsr clear
    lda $10
    clc
    adc #$01
    sta $10
    bcc move_right_ret
    inc $11
move_right_ret:
    rts

move_down:
    jsr clear
    lda $10
    clc
    adc #$40
    sta $10
    bcc move_down_ret 
    inc $11
move_down_ret:
    rts

move_up:
    jsr clear
    lda $10
    sec
    sbc #$40
    sta $10
    bcs move_up_ret
    dec $11
move_up_ret:
    rts

clear:
    lda #$00
    sta COLOR
    jsr draw
    rts

.org $fffc
    .DW reset
    .DW VBLANK_IRQ
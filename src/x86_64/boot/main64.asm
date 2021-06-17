global long_mode_start
extern kernel_main

section .text
bits 64
long_mode_start:
    ; Load null into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; print `64-bit VOS`
    mov WORD [0xb8000], 0x1F36 ; '6'
    mov WORD [0xb8002], 0x1F34 ; '4'
    mov WORD [0xb8004], 0x1F2D ; '-'
    mov WORD [0xb8006], 0x1F62 ; 'b'
    mov WORD [0xb8008], 0x1F69 ; 'i'
    mov WORD [0xb800A], 0x1F74 ; 't'

    mov WORD [0xb800C], 0x1F20 ; Space

    mov WORD [0xb800E], 0x1F56 ; 'V' in black
    mov WORD [0xb8010], 0x1F4F ; 'O' in black
    mov WORD [0xb8012], 0x1F53 ; 'S' in black

    mov WORD [0xb8014], 0x1F20 ; Space

    call kernel_main
    hlt
global long_mode_start

extern kernel_start

section .text
bits 64

long_mode_start:
    ; zero out data segment registers
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov edx, kernel_start

    extern kmain
    call kmain

    hlt

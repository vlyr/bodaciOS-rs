extern kmain
global start

section .text
bits 32
    start:
        mov esp, stack_top
        mov edi, ebx
        mov esi, stack_top

        jmp kmain

section .bss
    align 4096

    stack_bottom:
        resb 4096 * 4

    stack_top:

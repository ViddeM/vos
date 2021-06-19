global start
extern long_mode_start

section .text
bits 32
start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]
    jmp gdt64.code_segment:long_mode_start

    ; print `32-bit VOS`
    mov WORD [0xb8000], 0x1F33 ; '3'
    mov WORD [0xb8002], 0x1F32 ; '2'
    mov WORD [0xb8004], 0x1F2D ; '-'
    mov WORD [0xb8006], 0x1F62 ; 'b'
    mov WORD [0xb8008], 0x1F69 ; 'i'
    mov WORD [0xb800A], 0x1F74 ; 't'

    mov WORD [0xb800C], 0x1F20 ; Space

    mov WORD [0xb800E], 0x1F56 ; 'V' in black
    mov WORD [0xb8010], 0x1F4F ; 'O' in black
    mov WORD [0xb8012], 0x1F53 ; 'S' in black

    mov WORD [0xb8014], 0x1F20 ; Space
    hlt

check_multiboot:
    cmp eax, 0x36D76289 ; magic value that multiboot2 writes to the EAX register
    jne .no_multiboot
    ret

.no_multiboot:
    mov al, "M" ; Error message "M" - Multiboot error
    jmp error

check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21 ; ID bit is bit 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx ; Check if the bit was 'flippable'
    je .no_cpuid
    ret

.no_cpuid:
    mov al, "C" ; Error code "C" - Cpuid error
    jmp error


check_long_mode: ; Check if we support 64-bit (long) mode
    mov eax, 0x80000000
    cpuid ; Takes the value in eax, if long_mode is supported the value of eax will be greater after this operation.
    cmp eax, 0x80000001
    jb .no_long_mode

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29 ; LM bit is bit 29
    jz .no_long_mode
    ret

.no_long_mode:
    mov al, "L" ; Error code "L" - Long mode
    jmp error


setup_page_tables:
    mov eax, page_table_l3
    or eax, 0b11 ; present, writable
    mov [page_table_l4], eax

    mov eax, page_table_l2
    or eax, 0b11 ; present, writable
    mov [page_table_l3], eax

    ; TODO: Worry about level 1 tables, currently using 'huge tables' in l2
    mov ecx, 0 ; counter

.loop:
    mov eax, 0x200000 ; 2MiB
    mul ecx
    or eax, 0b10000011 ; huge page, present, writable
    mov [page_table_l2 + ecx * 8], eax

    inc ecx ; increment counter
    cmp ecx, 512 ; check if the whole table is mapped
    jne .loop ; if not, continue

    ret

enable_paging:
	; pass page table location to cpu
	mov eax, page_table_l4
	mov cr3, eax

	; enable PAE
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; enable long mode
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	; enable paging
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret


error:
    ; print "ERR:  X" where X is the error code encountered
    mov dword [0xb8000], 0x4F524F45
    mov dword [0xb8004], 0x4F3A4F52
    mov dword [0xb8008], 0x4F204F20
    mov byte  [0xb800a],  al
    hlt


section .bss
align 4096 ; Align page-tables to 4kb
page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096

stack_bottom:
    resb 4096 * 4 ; 4kb
stack_top:

section .rodata
gdt64: ; Global Descriptor Table
    dq 0 ; zero entry
.code_segment: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53); code segments
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

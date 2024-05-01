org 0
bits 64

global _start

section .text

_start:
	mov qword rax, 0x42
	ret

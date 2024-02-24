.global _start
.align 2

.text
print:
    adrp x0, num@PAGE
    add x0, x0, num@PAGEOFF

    // Load value from the stack.
    ldr x1, [sp], #64

    // Convert number in x1 to its ASCII representation
    mov x2, #10              // Base 10
    mov x3, #19               // Initialize index

convert_loop:
    udiv x4, x1, x2          // Divide by 10, quotient in x4, remainder in x1
    mul x5, x4, x2           // Multiply quotient by 10
    sub x6, x1, x5           // Calculate remainder

    and w6, w6, #0xFF
    add x6, x6, #'0'         // Convert remainder to ASCII

    // Store ASCII character in num
    strb w6, [x0, x3]

    sub x3, x3, #1           // Decrement index
    mov x1, x4               // Update x1 with quotient

    cmp x1, #0               // Check if quotient is zero
    bne convert_loop         // If not zero, continue loop

    // Recompute the address of num
    adrp x4, num@PAGE
    add x4, x4, num@PAGEOFF

    mov x1, x4               // Load the correct address into x1


    mov x0, #1               // 1 = StdOut
    mov x2, #20               // Length of num
    mov x16, #4              // Unix write system call
    svc #0x80                 // Call kernel to output the string

	// Print newline character
    adrp x0, newline@PAGE
    add x0, x0, newline@PAGEOFF

    mov x1, x0               // Load the correct address into x1

    mov x0, #1               // 1 = StdOut
    mov x2, #1               // Length of newline
    mov x16, #4              // Unix write system call
    svc #0x80                 // Call kernel to output the newline character

	//Zero out number
	mov x1, #20
loop:
    strb wzr, [x4], #1   ; Store zero (wzr = register containing zero) and increment the address
    subs x1, x1, #1       ; Decrement the length
    bne loop            ; Repeat the loop if length is not zero

    ret

_start: 
    // push 
    ldr x0, =34
    str x0, [sp, #-64]!

    // push 
    ldr x0, =12345678912345678912
    str x0, [sp, #-64]!

    bl print

    bl print

    // exit syscall
    mov x0, #0
    mov x16, #1
    svc #0x80

.data
num: .zero 20
newline: .asciz "\n"

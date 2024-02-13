.global _start
.align 2

.text
print:
    adrp x0, num@PAGE
    add x0, x0, num@PAGEOFF

    // Load value from the stack.
    ldr   x1, [sp], #16

    // Convert number in x1 to its ASCII representation
    mov x2, #10              // Base 10
    mov x3, #7               // Initialize index

convert_loop:
    sdiv x4, x1, x2          // Divide by 10, quotient in x4, remainder in x1
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
    adrp x0, num@PAGE
    add x0, x0, num@PAGEOFF

    mov x1, x0               // Load the correct address into x1

    mov x0, #1               // 1 = StdOut
    mov x2, #8               // Length of num
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

    ret


_start: 
    // push 
    mov x0, #34
    str x0, [sp, #-16]!

    // push 
    mov x0, #35
    str x0, [sp, #-16]!

    bl print

    bl print

    // exit syscall
    mov x0, #0
    mov x16, #1
    svc #0x80

.data
num: .zero 8
newline: .asciz "\n"

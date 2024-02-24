.global _start
.align 2

.text
print:
    adrp x0, num@PAGE
    add x0, x0, num@PAGEOFF
    ldr   x1, [sp], #16
    mov x2, #10
    mov x3, #7
convert_loop:
    sdiv x4, x1, x2
    mul x5, x4, x2
    sub x6, x1, x5 
    and w6, w6, #0xFF
    add x6, x6, #'0'
    strb w6, [x0, x3]
    sub x3, x3, #1
    mov x1, x4
    cmp x1, #0
    bne convert_loop
    adrp x4, num@PAGE
    add x4, x4, num@PAGEOFF
    mov x1, x4
    mov x0, #1
    mov x2, #8
    mov x16, #4
    svc #0x80
    adrp x0, newline@PAGE
    add x0, x0, newline@PAGEOFF
    mov x1, x0
    mov x0, #1
    mov x2, #1
    mov x16, #4 
    svc #0x80
    mov x3, #0
    str x3, [x4]
    ret

_start: 
    // push 
    mov x0, #0
    str x0, [sp, #-16]!

addr_1:

    // dup 
    ldr x0, [sp], #16
    str x0, [sp, #-16]!
    str x0, [sp, #-16]!

    // push 
    mov x0, #10
    str x0, [sp, #-16]!

    // > 
    ldr x0, [sp], #16
    ldr x1, [sp], #16
    cmp x1, x0
    cset w0, LT
    str w0, [sp, #-16]!

    // do 
    ldr x0, [sp], #16
    cmp x0, #0
    beq addr_11

    // dup 
    ldr x0, [sp], #16
    str x0, [sp, #-16]!
    str x0, [sp, #-16]!

    // print 
    bl print

    // push 
    mov x0, #1
    str x0, [sp, #-16]!

    // plus 
    ldr x0, [sp], #16
    ldr x1, [sp], #16
    add x3, x0, x1
    str x3, [sp, #-16]!

    b addr_1
addr_11:

    // exit syscall
    mov x0, #0
    mov x16, #1
    svc #0x80

.data
    num: .zero 8
    newline: .asciz "\n" 

.global _start
.align 2

.text
print:
    adrp x0, num@PAGE
    add x0, x0, num@PAGEOFF

    // Store val in num.
    ldr   x1, [sp], #16
    str x1, [x0]
  
    // Load the contents of num into x1.
    adrp x1, num@PAGE
    add x1, x1, num@PAGEOFF
  
    mov	X0, #1			// 1 = StdOut
    mov	X2, #3    	    // length of our num
    mov	X16, #4		    // Unix write system call
    svc	#0x80		    // Call kernel to output the string
 	
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

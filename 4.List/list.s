            .data
bytes:      .byte   10, 20, 30, 40
shorts:     .hword  100, 200, 300, 400       
            
            .text
            .global byte_list
byte_list:
            ldr     r1,=bytes
            
            ldrb    r2,[r1]
            ldrb    r3,[r1,#1]

            add     r0,r2,r3

            ldrb    r2,[r1,#2]
            add     r0,r0,r2

            ldrb    r2,[r1,#3]
            add     r0,r0,r2

            bx      lr

            .global short_list
short_list:
            ldr     r1,=shorts
            
            ldrh    r2,[r1]
            ldrh    r3,[r1,#2]

            add     r0,r2,r3

            ldrh    r2,[r1,#4]
            add     r0,r0,r2

            ldrh    r2,[r1,#6]
            add     r0,r0,r2

            bx      lr


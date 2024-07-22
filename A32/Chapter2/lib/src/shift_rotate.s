            .text
            .global MovRegA
MovRegA:    //  int MovRegA(unsigned int a, unsigned int *b);
            push    {r4-r7} 
            mov     r7,r1   // Storing *b to r7
            
/*  This part of code perform different form of shifting and two form of 
    rotation.   */
            mov     r2,r0,asr #2
            mov     r3,r0,lsl #4
            mov     r4,r0,lsr #5
            mov     r5,r0,ror #3
            mov     r6,r0,rrx

            str     r2,[r7]
            str     r3,[r7,#4]
            str     r4,[r7,#8]
            str     r5,[r7,#12]
            str     r6,[r7,#16]

            pop     {r4-r7}
            bx      lr

            .global MovRegB
MovRegB:    //  int MovRegB(unsigned int a, unsigned int *b);
            push    {r4-r7} 
            mov     r7,r1   // Storing *b to r7
            
/*  This part of code perform different form of shifting and two form of 
    rotation.   */
            mov     r2,r0,asr #2
            mov     r3,r0,lsr #2
            mov     r4,r0,lsr #2
            mov     r5,r0,ror #2
            mov     r6,r0,rrx

            str     r2,[r7]
            str     r3,[r7,#4]
            str     r4,[r7,#8]
            str     r5,[r7,#12]
            str     r6,[r7,#16]

            pop     {r4-r7}
            bx      lr

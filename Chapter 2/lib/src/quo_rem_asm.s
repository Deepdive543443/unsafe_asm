            .text
            .global quo_rem_asm
//  void quo_rem_asm(const int *a, const int *b, int *quo, int *rem);
quo_rem_asm:
            push {r4,r5}

// Load a and b
            ldr r4,[r0]     // Store a 2 r4
            ldr r5,[r1]     // Store b 2 r5

// Calculate quotient and remainder
            sdiv r0,r4,r5   // r0 = a / b
            str r0,[r2]     // quo = r0 (sending r0's values to stack)

            mul r1,r0,r5    // r1 = r0 * b
            sub r2,r4,r1    // r2 = a - r1
            str r2,[r3]     // rem = r2 (sending r2's values to stack)

// Restore non-volatile register and return 
            pop {r4,r5}
            bx lr

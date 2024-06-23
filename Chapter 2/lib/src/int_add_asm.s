            .text
            .global int_add_asm
//  int int_add_asm(int a, int b, int c);
int_add_asm:

/*  In this example, GNU C compiler loads functions args into
    r0, r1, r2, ... , etc. (r0, r1, r2 in our case of three args)   */
            add r0,r0,r1    // add [2 load] [2 add] [2 add]
            sub r0,r0,r2    // sub [2 load] [2 subtract] [2 subtract]
            
/*  AArch32 assembly have to use r0 to return a 32 bit length data.
    In the previous two instructions, we have the result store in register 
    r0. Then, by calling "bx lr" here, we return the value of r0 by function.

    What bx(branch and exchange) does is copy the copy the content of LR 
    register into PC register, which contain the return address. Hopefully 
    the later session will cover more details about this  */        
            bx lr
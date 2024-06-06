#include <stdio.h>
#include <stdint.h>

uint8_t  byte_list();
uint16_t short_list();

int main(int argc, char **argv)
{
    printf("Byte_list %d\n", byte_list());
    printf("Short_list %d\n", short_list());
    return 0;
}
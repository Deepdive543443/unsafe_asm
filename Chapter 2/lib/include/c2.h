#ifndef C2_H
#define C2_H

#ifdef __cplusplus
#include <cstdint>
extern "C" {
#else
#include <stdint.h>
#endif

// Chapter 2
int                int_add_asm(int a, int b, int c);
int                asm_mul(int a, int b);
long long          smull_asm(int a, int b);
unsigned long long umull_asm(unsigned int a, unsigned int b);
void               quo_rem_asm(const int *a, const int *b, int *quo, int *rem);
uint8_t            byte_list();
uint16_t           short_list();
int                MovRegA(unsigned int a, unsigned int *b);
int                MovRegB(unsigned int a, unsigned int *b);
void               bit_ops_16(uint16_t a, uint16_t b, uint16_t *result);

#ifdef __cplusplus
}
#endif

#endif // C2_H
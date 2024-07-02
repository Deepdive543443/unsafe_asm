#ifndef C7_H
#define C7_H

#ifdef __cplusplus
#include <cstdint>
extern "C" {
#else
#include <stdint.h>
#endif

float    sum_f32(float *src, int length);
uint16_t sum_u16(uint16_t *src, int length);

#ifdef __cplusplus
}
#endif

#endif  // C7_H
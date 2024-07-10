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

typedef enum {
    F32_CVT_I32,
    I32_CVT_F32,
    F32_CVT_U32,
    U32_CVT_F32,
    F32_CMP,
    U32_CMP,
    NUM_OPS
} cvtEnum;

int vec_cvt(void *f32_ptr, void *s32_ptr, void *out_ptr, cvtEnum ops);

#ifdef __cplusplus
}
#endif

#endif  // C7_H
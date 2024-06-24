#ifndef C6_H
#define C6_H

#ifdef __cplusplus
extern "C" {
#endif

float  celcius_2f_asm(float celcius);
float  fahrenheit_2c_asm(float fahrenheit);
double distance_64_asm(double x1, double y1, double z1, double x2, double y2, double z2);

#ifdef __cplusplus
}
#endif

#endif  // C6_H
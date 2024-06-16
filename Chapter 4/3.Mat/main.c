#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void mat_init(int *mat, int width, int height)
{
    for (int i = 0; i < width; i++) {
        for (int j = 0; j < height; j++) {
            mat[i * width + j] = i * j;
        }
    }
}

static void mat_square(int *dst_mat, int *src_mat, int width, int height)
{
    for (int i = 0; i < width; i++) {
        for (int j = 0; j < height; j++) {
            dst_mat[i * width + j] = src_mat[i * width + j] * src_mat[i * width + j];
        }
    }
}

void mat_square_asm(int *dst_mat, int *src_mat, int width, int height);

int main(int argc, char **argv)
{
    int w, h;
    w = argc > 1 ? atoi(argv[1]) : 12;
    h = argc > 2 ? atoi(argv[2]) : 12;
    int mat1[w][h], mat2[w][h], mat3[w][h];

    mat_init((int *)mat1, w, h);
    mat_square((int *)mat2, (int *)mat1, w, h);
    mat_square_asm((int *)mat3, (int *)mat1, w, h);

    for (int i = 0; i < w; i++) {
        for (int j = 0; j < h; j++) {
            printf("C: %10d    ASM: %10d\n", mat2[i][j], mat3[i][j]);
        }
    }
    return 0;
}
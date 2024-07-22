#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void mat_init(int *mat, int width, int height)
{

    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            mat[i * width + j] = i * j;
        }
    }
}

static void mat_square(int *dst_mat, int *src_mat, int width, int height)
{
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            dst_mat[i * width + j] = src_mat[i * width + j] * src_mat[i * width + j];
        }
    }
}

void mat_square_asm(int *dst_mat, int *src_mat, int width, int height);

int main(int argc, char **argv)
{
    int w, h;
    h = argc > 1 ? atoi(argv[1]) : 12;
    w = argc > 2 ? atoi(argv[2]) : 12;
    int mat1[h][w], mat2[h][w], mat3[h][w];

    mat_init(&mat1[0][0], w, h);
    mat_square(&mat2[0][0], &mat1[0][0], h, w);
    mat_square_asm(&mat3[0][0], &mat1[0][0], h, w);

    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w; j++) {
            printf("( %3d, %3d)Init:  %6d  C: %8d  ASM: %8d\n", i, j, mat1[i][j], mat2[i][j], mat3[i][j]);
        }
    }
    return 0;
}
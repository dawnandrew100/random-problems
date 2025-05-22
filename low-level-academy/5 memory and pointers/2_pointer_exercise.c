#include <stdio.h>

void swap(int *a, int *b);

int main() {
    int i = 1;
    int j = 2;
    int x = 44;
    int y = 43;
    printf("i: %d, j: %d\n", i, j);
    swap(&i, &j);
    printf("i: %d, j: %d\n", i, j);
    printf("x: %d, y: %d\n",x, y);
    swap(&x, &y);
    printf("x: %d, y: %d\n", x, y);
}

void swap(int *a, int *b) {
    int temp;
    temp = *a;
    *a = *b;
    *b = temp;
}

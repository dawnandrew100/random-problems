#include <stdio.h>

// return_type name(list_of_arguments) {body}

int add(int x, int y) {
    int z = x+y;
    return z;
}


int main(int argc, char *argv[]) {
    int result = add(1, 2);
    printf("%d\n", result);
    return 0;
}

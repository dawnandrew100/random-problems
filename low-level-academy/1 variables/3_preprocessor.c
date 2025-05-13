#include <stdio.h>

// Example preprocessor command
#define MAX_PERSONS 1024

// Compilation steps
// 1. Preprocessor
// 2. Compilation
// 3. Assembling
// 4. Linking

int main() {
    // to run and print
    // gcc -o variables preprocessor.c -DDEBUG && variables
    #ifdef DEBUG
    printf("WE ARE IN DEBUG MODE: %s:%d\n", __FILE__, __LINE__);
    #endif

    return 0;
}

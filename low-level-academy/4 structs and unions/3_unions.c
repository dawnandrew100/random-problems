#include <stdio.h>

// Unions creates enough memory for the biggest element, instead of memory for each element
// You can access each bite in int with a char for example
// Can be important for embedded projects

union myunion_u {
    int x; // Biggest memory of 4 bytes
    char c; // one byte
    short s; // two bytes
};

int main() {
    union myunion_u u;
    // Sets larger value of union
    u.x = 0x41424344;

    // Accesses the internals of that larger value
    // Short accesses half, char accesses a quarter
    printf("%hx %hhx\n", u.s, u.c);
    return 0;
}

/*
 * BEST PRACTICE IS TO WRAP UNION IN A STRUCT TO DETERMINE WHAT MODE DATA IS BEING USED IN
 *union myunion {
    int i;
    char c;
};

struct mycontainer {
    mode_t mode;
    union myunion u;
}

int main() {
    struct mycontainer c = {
      .mode = MODE_CHAR,
      .u.c = 'C',
    };
    ...
}
 */

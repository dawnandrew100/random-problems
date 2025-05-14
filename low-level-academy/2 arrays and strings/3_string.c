#include <stdio.h>

int main() {

    // These two are the same
    // Strings are just an array of chars in C
    char mystr[] = {'h', 'e', 'l', 'l', 'o', 0};
    char *myotherstr = "hello";


    printf("%s\n", mystr);
    printf("%s\n", myotherstr);
    return 0;
}

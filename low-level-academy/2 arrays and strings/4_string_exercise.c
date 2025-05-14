#include <stdio.h>

int main(int argc, char **argv) {

    // this is a string
    char *str = "hehe";

    // create the same string
    // as a character array 
    char otherstr[] = {0x68, 0x65, 0x68, 0x65, 0} ;

    if (!strcmp(str, otherstr)) {
        printf("Yay!\n");
    } else {
        printf("Nay!\n");
    }

    return 0;
}

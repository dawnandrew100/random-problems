#include <stdio.h>

#define MAX_IDS 32

int main() {
    /*
     Instead of this:
     int id = 0;
     int id_1 = 0;
     int id_2 = 0;

     int id_3, id_4 = 0;
     */
    // Creates an array of 32 elements
    // Each ID is size int
    int idss[MAX_IDS];

    // Or the array length can be inferred
    int idsss[] = {1, 2, 3};


    // Combine above methods
    // Remember assuming that unassigned space is 0 is bad
    int ids[MAX_IDS] = {0,1,2};
    printf("%d\n", ids[0]);

    ids[3] = 0x41;
    printf("%d\n", ids[3]);

    return 0;
}

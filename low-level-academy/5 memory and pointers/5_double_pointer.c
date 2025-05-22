#include <stdio.h>
#include <stdlib.h>


// int i; value of i
// int *pi = &i; address of i
// int **ppi = &pi; address of pi
// The double pointer gives us the ability to update a pointer with a new address

typedef enum {
    STATUS_GOOD,
    STATUS_BAD,
} status_t;

status_t increase_buffer(int **data, int len) {
    int *temp = NULL;
    temp = realloc(*data, len * sizeof(int));

    if(temp == NULL) {
        return STATUS_BAD;
    }
    *data = temp;
    return STATUS_GOOD;
}

int main() {
    size_t initial_size = 5;
    size_t new_size = 10;

    // Allocate 5 member int array
    int *arr = malloc(initial_size * sizeof(int));

    // Initialise array
    size_t i = 0;
    for(i = 0; i < initial_size; i++) {
        arr[i] = i;
    }

    printf("Initial array:\n");

    for(i = 0; i < initial_size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    if(STATUS_BAD == increase_buffer(&arr, new_size)) {
        printf("Failed to reallocate buffer size\n");
        free(arr);
        arr = NULL;
        return -1;
    }

    // Initialise newly created section
    for(i = initial_size; i < new_size; i++) {
        arr[i] = i;
    }

    for(i = 0; i < new_size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    free(arr);
    arr = NULL;
    return 0;
}

// In our employee database, this will be used to grow and shrink the heap allocated employees

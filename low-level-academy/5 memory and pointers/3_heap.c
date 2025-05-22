#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// Allocating memory dynamically might be useful to not overuse stack memory
struct employee_t {
    int id;
    int income;
    bool isstaff;
};

void initialise_employee(struct employee_t *e);

int main() {
    // This value n can come from the database header
    // It can be a runtime value calculated instead of a hardcoded define
    int n = 4;
    struct employee_t *employees = malloc(sizeof(struct employee_t)*n);
    // Don't want to use memory if allocation failed
    // Dereferencing a null pointer is can be dangerous
    if (employees == NULL) {
        printf("The allocator failed\n");
        return -1;
    }

    initialise_employee(&employees[0]);

    printf("%d\n", employees[0].income);

    free(employees); // Freeing gives memory back to the operating system (prevents memory leak)
    employees = NULL; // Deletes pointer to avoid use after free
                      // Setting a pointer to NULL is not required but is useful
                      // in avoiding dangling pointers
                      // (pointer that points to memory that is no longer valid)
}

void initialise_employee(struct employee_t *e) {
    e->id = 0;
    e->income = 0;
    e->isstaff = false;
}

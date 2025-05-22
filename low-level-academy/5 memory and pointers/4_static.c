#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#define INITIAL_EMPLOYEES 4

// Global variables can be problematic in the context of seeing where the data is 
// accessed and modified

struct employee_t {
    int id;
    int income;
    bool isstaff;
};

void initialise_employee(struct employee_t *e);

int main() {
    struct employee_t *employees = malloc(sizeof(struct employee_t)*INITIAL_EMPLOYEES);
    if (employees == NULL) {
        printf("Allocator failed\n");
        return -1;
    }

    for (int i = 0; i < INITIAL_EMPLOYEES; i++) {
        initialise_employee(&employees[i]);
        printf("New employee ID is: %d\n", employees[i].id);
    }

    free(employees);
    employees = NULL;
}

void initialise_employee(struct employee_t *e) {
    // Static variable initialises variable in global scope but only this function can access it
    // The first time this function is run, it is set to 0 and incremented by 1
    // It then continues to increment (and not reset to 0) on every subsequent function call
    static int numEmployees = 0;
    numEmployees++;

    e->id = numEmployees;
    e->income = 0;
    e->isstaff = false;
}

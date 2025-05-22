#include <stdio.h>
#include <stdbool.h>

struct employee_t {
    int id;
    int income;
    bool is_staff;
};

void initialise_employee(struct employee_t *e);

int main() {
    int x = 3; // Makes room on the stack for 3 to be stored
    // A value that equals the address of x in memory
    int *pX = &x; // Sets the value of pX to the address (&) of x

    // If I want to use pX to get the value of x, you must derefernece
    // "Get the value pointed to by x"
    // To get the value of the pointer, use %p
    printf("%d\n", *pX);
    printf("%p\n", pX);

    // Why use pointers?
    // Allows you to access values within different scopes
    // INSTEAD OF DOING THIS IN THE MAIN SCOPE
    struct employee_t Ralph;
    // Set default values
    Ralph.id = 1;
    Ralph.income = 100;
    printf("Ralph id: %d, Ralph income: %d\n", Ralph.id, Ralph.income);
    initialise_employee(&Ralph);
    printf("Ralph id: %d, Ralph income: %d\n", Ralph.id, Ralph.income);
    return 0;
}

void initialise_employee(struct employee_t *e) {
    // DO THIS IN A SEPARATE FUNCTION
    e->id = 0;
    e->income = 0;
    e->is_staff = false;
}

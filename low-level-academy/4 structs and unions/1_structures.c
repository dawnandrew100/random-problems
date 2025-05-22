#include <stdio.h>
#include <stdbool.h>

// Structs are used to combine several different data types in one block
// As opposed to an array that can only store one data type

#define MAX_EMPLOYEES 1000

struct employee_t {
    int id;
    char firstname[64];
    char lastname[64];
    float income;
    bool ismanager;
};

int main() {
    // You can create a single employee like this
    struct employee_t Fred;
    Fred.income = 100.0;
    Fred.ismanager = true;

    // You can even create an array of structs (Employees in this case)
    struct employee_t employees[MAX_EMPLOYEES];

    // Initialise employee values
    int i = 0;
    for (i = 0; i < MAX_EMPLOYEES; i++) {
        employees[i].income = 0;
        employees[i].ismanager = false;
    }

    employees[11].income = 100.0;
    printf("%f\n", employees[10].income);
    printf("%f\n", employees[11].income);
    return 0;
}

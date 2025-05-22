#include <stdio.h>
#include <stdbool.h>

// Depending on system, compilers might optimise and add padding in addition to pre-defined memory allocation
#define MAX_EMPLOYEES 1000

__attribute__((__packed__)) struct employee_t {
    int id;
    char firstname[64];
    char lastname[64];
    float income;
    bool ismanager;
};

struct unmodified_employee_t {
    int id;
    char firstname[64];
    char lastname[64];
    float income;
    bool ismanager;
};

int main() {
    struct employee_t employees[MAX_EMPLOYEES];
    struct unmodified_employee_t employee_u[MAX_EMPLOYEES];

    // Difference in size may only be different across compilers/systems
    // Using gcc on a 64-bit windows gives a size of 140 for both
    printf("%d\n", sizeof(struct employee_t));
    printf("%d\n", sizeof(struct unmodified_employee_t));

    return 0;
}

#include <stdio.h>

int main() {
    int temp;
    printf("Enter a temperature:\n");
    scanf("%d", &temp);

    if (temp >= 70) {
        printf("That's hot!\n");
    } else if (temp >= 30 && temp < 70) {
        printf("That's mild!\n");
    } else {
        printf("That's cold!\n");
    }
    return 0;
}

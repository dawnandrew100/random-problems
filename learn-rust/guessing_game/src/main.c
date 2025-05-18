#include <stdio.h>
#include <time.h>
#include <stdlib.h>

#define MAX_RAND 100

int main(){
    printf("Guess the number from 1 to 100!\n");
    srand(time(NULL)); // From time.h
    int secret_number = rand() % MAX_RAND + 1; // rand() from stdlib.h

    int guess = 0;
    while(guess != secret_number){
        printf("Please input your guess.\n");

        char term;
        if(scanf("%d%c", &guess, &term) !=2 || term != '\n') {
            printf("Please enter an integer!\n");
            while (getchar() != '\n'); // Clears buffer to avoid infinite loop
            continue;
        };
        printf("You guessed: %d\n", guess);

        if (guess < secret_number) {
            printf("Too small!\n");
        } else if (guess > secret_number) {
            printf("Too big!\n");
        } else if (guess == secret_number)  {
            printf("You win!\n");
            break;
        }
    }
    return 0;
}

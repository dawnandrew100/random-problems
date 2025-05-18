from random import randint

def main():
    print("Guess the number from 1 to 100!")

    secret_number = randint(1, 100)

    while True:
        guess = input("Please input your guess.\n")

        if guess.isdigit():
            print(f"You guessed: {guess}")
            guess = int(guess)
        else:
            print(f"Invalid guess entered of type {type(guess)}. Expected only ints.")
            continue

        if guess < secret_number:
            print("Too small!")
        elif guess > secret_number:
            print("Too big!")
        elif guess == secret_number:
            print("You win!")
            break

if __name__ == "__main__":
    main()

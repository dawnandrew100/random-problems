import string
import random

symbols = "!#$%&()*+"


def main():
    print("Welcome to the PyPassword Generator!")
    nr_letters = int(input("How many letters would you like in your password?\n"))
    nr_symbols = int(input("How many symbols would you like in your password?\n"))
    nr_numbers = int(input("How many numbers would you like in your password?\n"))

    password = []
    password.extend(random_selections(nr_letters, string.ascii_letters))
    password.extend(random_selections(nr_numbers, string.digits))
    password.extend(random_selections(nr_symbols, symbols))

    random.shuffle(password)
    print("".join(password))


def random_selections(number: int, options: str) -> list[str]:
    passgen = []
    list_options = list(options)
    for _ in range(number):
        passgen.append(random.choice(list_options))
    return passgen


if __name__ == "__main__":
    main()

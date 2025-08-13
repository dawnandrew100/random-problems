# Multiplication table


def main():
    mul_end = 10
    mul_to = 10

    for mul in range(1, mul_to + 1):
        for multiplier in range(1, mul_end + 1):
            print(f"{mul * multiplier:>3}", end=" ")
        print()


if __name__ == "__main__":
    main()

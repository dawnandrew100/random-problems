# Two Dice


def main():
    for i in range(6 + 1):
        for j in range(6 + 1):
            if i + j == 5:
                print(f"{(i, j)}")


if __name__ == "__main__":
    main()

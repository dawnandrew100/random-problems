# Triple square


def main():
    print("Part One:\n")
    for i in range(1, 10 + 1):
        trip = triple(i)
        sq = square(i)
        print(f"triple({i})=={trip} square({i})=={sq}")

    print("\nPart Two:\n")
    for i in range(1, 10 + 1):
        trip = triple(i)
        sq = square(i)
        if sq > trip:
            break
        print(f"triple({i})=={trip} square({i})=={sq}")


def triple(num: int) -> int:
    return 3 * num


def square(num: int) -> int:
    return num * num


if __name__ == "__main__":
    main()

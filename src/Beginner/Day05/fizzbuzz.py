def main():
    modulo = {3: "fizz", 5: "buzz"}
    user_input = int(input("What number would you like to fizzbuzz up to? "))
    for i in range(1, user_input + 1):
        output = []
        for num in modulo:
            if i % num == 0:
                output.append(modulo[num])
        if output == []:
            output = [str(i)]
        print("".join(output))


if __name__ == "__main__":
    main()

def main():
    print("Welcome to Day 2!")
    print("First, let's calculate your BMI!")
    while True:
        height = input("What's your height in meters?\n")
        weight = input("What's your weight in kilograms?\n")

        try:
            height = float(height)
            weight = float(weight)
            break
        except (TypeError, ValueError) as e:
            print("Please enter a number for both height and weight")
            print(e)

    bmi = bmi_calc(weight, height)
    print(f"Your BMI is {bmi:.2f}")


def bmi_calc(w: float, h: float):
    return w / (h * h)


if __name__ == "__main__":
    main()

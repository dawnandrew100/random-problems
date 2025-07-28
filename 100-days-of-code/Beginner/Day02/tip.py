def main():
    print("Welcome to the tip calculator!")
    while True:
        total_bill = input("What was your total bill?\n$")
        tip_percent = input("What percentage would you like to tip?\n")
        people_num = input("How many people are going to be splitting the bill?\n")

        try:
            total_bill = float(total_bill)
            tip_percent = float(tip_percent)
            people_num = int(people_num)
            break
        except (TypeError, ValueError) as e:
            print("Please enter numbers for all values")
            print(e)

    tipped_total = calc_tip(total_bill, tip_percent)
    print(f"Your total factoring in a tip is ${tipped_total:.2f}")
    if people_num > 1:
        print(
            f"Split between {people_num} people, each person would pay ${tipped_total / people_num:.2f}"
        )


def calc_tip(total: float, tip: float):
    multiplier = 1 + (tip / 100)
    return total * multiplier


if __name__ == "__main__":
    main()

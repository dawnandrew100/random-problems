def main():
    print("Welcome to Python Pizza Deliveries!")
    choices = {"Y": True, "N": False}
    sizes = ("S", "M", "L")
    size = loop_assign("What size pizza do you want? S, M, or L: ", sizes)
    has_pepperoni = loop_assign(
        "Do you want pepperoni on your pizza? Y or N: ", choices
    )
    has_extra_cheese = loop_assign("Do you want extra cheese? Y or N: ", choices)

    size_price = {"S": 15, "M": 20, "L": 25}
    final_bill = size_price[str(size)]
    if has_pepperoni:
        final_bill += 2 if size == "S" else 3
    final_bill += 1 if has_extra_cheese else 0

    print(f"Your final bill is ${final_bill}.00")


def loop_assign(prompt: str, options: tuple[str, ...] | dict[str, bool]) -> str | bool:
    while True:
        res = input(prompt)
        res = res.upper()
        if res in options:
            break
    if isinstance(options, dict):
        res = options[res]
    return res


if __name__ == "__main__":
    main()

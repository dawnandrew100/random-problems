def main():
    available_shapes = ["triangle", "rectangle", "circle"]
    while True:
        shape = input("Choose a shape (triangle, rectangle, circle): ").lower()
        if shape == "":
            break
        if shape not in available_shapes:
            print("Unknown shape")
            continue
        area = 0
        match shape:
            case "triangle":
                area = triangle()
            case "circle":
                area = circle()
            case "rectangle":
                area = rectangle()
        print(f"The area of your {shape} is {area:.6f}")
        if not _keep_playing():
            break


def circle():
    while True:
        radius = input("Give radius of the circle: ")
        invalids = _validate_input(radius)
        if invalids:
            print(f"{radius} is not a valid number for the radius.")
            continue
        pi = 3.14159265359
        return pi * float(radius) ** 2


def triangle():
    while True:
        base = input("Give base of the triangle: ")
        height = input("Give height of the triangle: ")
        invalids = _validate_input(base, height)
        if invalids:
            print(f"You entered the following invalid numbers: {invalids}")
            continue
        return (1 / 2) * (float(base) * float(height))


def rectangle():
    while True:
        width = input("Give width of the rectangle: ")
        height = input("Give height of the rectangle: ")
        invalids = _validate_input(width, height)
        if invalids:
            print(f"You entered the following invalid numbers: {invalids}")
            continue
        return float(width) * float(height)


def _validate_input(*nums):
    invalid_inputs = []
    for num in nums:
        if not _is_valid_input(num):
            invalid_inputs.append(num)
    return invalid_inputs


def _is_valid_input(num: str) -> bool:
    return num.isdecimal() or _is_float(num)


def _is_float(num: str) -> bool:
    try:
        float(num)
        return True
    except ValueError:
        return False


def _keep_playing() -> bool:
    while True:
        proceed = input("Would you like to continue? ").lower()
        if proceed == "no" or proceed == "n":
            return False
        if proceed == "yes" or proceed == "y":
            return True


if __name__ == "__main__":
    main()

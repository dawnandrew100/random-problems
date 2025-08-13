import re


def main():
    input_str = " afd [asd] [12 ] [a34] [ -43 ]tt [+12]xxx"
    print(integers_in_brackets(input_str))


def integers_in_brackets(string: str) -> list[str]:
    # Regex Explained
    # Match literal "["
    # Match 0 or more white space characters
    # Match 0 or 1 literal "+" characters
    # Capture 0 or 1 literal "-" chars followed by 1 or more digits
    # Match 0 or more white space characters
    # Match literal "]"
    return re.findall(r"\[\s*[+]?([-]?\d+)\s*\]", string)


if __name__ == "__main__":
    main()


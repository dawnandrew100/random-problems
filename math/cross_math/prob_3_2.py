from itertools import permutations


def main():
    options = list(range(10))
    pemdas_sols = []
    no_pemdas_sols = []
    for poss in permutations(options):
        if poss[2] == 0 or poss[8] == 0:
            continue
        if is_pemdas_solution(poss):
            pemdas_sols.append(poss)
        if is_no_pemdas_solution(poss):
            no_pemdas_sols.append(poss)

    print(f"Total pemdas solutions: {len(pemdas_sols)}")
    print(f"Total linear solutions: {len(no_pemdas_sols)}")
    print("First ten solutions:")
    print("Solutions taking pemdas into account:")
    for i in range(11):
        print(pemdas_sols[i])
    print("Solutions not taking pemdas into account:")
    for i in range(11):
        print(no_pemdas_sols[i])


def is_pemdas_solution(nums: list[int]) -> bool:
    if (
        nums[0]
        + 13 * nums[1] / nums[2]
        + nums[3]
        + 12 * nums[4]
        - nums[5]
        - 11
        + nums[6] * nums[7] / nums[8]
        - 10
        == 66
    ):
        return True
    else:
        return False


def is_no_pemdas_solution(nums: list[int]) -> bool:
    if (
        (
            (
                (
                    (
                        (
                            (((((nums[0] + 13) * nums[1]) / nums[2]) + nums[3]) + 12)
                            * nums[4]
                        )
                        - nums[5]
                    )
                    - 11
                )
                + nums[6]
            )
            * nums[7]
        )
        / nums[8]
    ) - 10 == 66:
        return True
    else:
        return False


if __name__ == "__main__":
    main()

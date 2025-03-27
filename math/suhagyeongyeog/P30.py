from collections import defaultdict
from itertools import accumulate

def main():
    target_count = 59
    accumulated = cache_int_finder(100, target_count)
    final_sol = {}

    for key, value in accumulated.items():
        if value == target_count:
            final_sol.update({key: value})
            break

    for key, value in accumulated.items():
        if final_sol == {}:
            break
        if value > target_count:
            final_sol.update({key: value})
            break

    if final_sol:
        m, M = final_sol.keys()
        print(final_sol)
        print(M + m -1)
    else:
        print("No value of k has exactly {target_count} solutions.")

def cache_int_finder(k: int, ordered_pair_count: int):
    if k <= 2:
        return None
    solutions = []
    a_cache = {}
    c_cache = {}
    target = 24**(1/5)
    k_count = defaultdict(int)
    # >= 16 causes false negatives and <= 6 causes false positives
    epsilon = 1e-15 # answer precision

    for a in range(2, k):
        for b in range(2, k):
            a_cache[(a, b)] = a**(1/b)
    for c in range(2, k):
        for d in range(2, k):
            c_cache[(c, d)] = c**(1/d)

    for a_key, a_item in a_cache.items():
        for c_key, c_item in c_cache.items():
            if abs((a_item * c_item) - target) < epsilon:
                ordered_pair = list(a_key)+list(c_key)
                k_magnitude = max(ordered_pair)
                k_count[k_magnitude] += 1
                solutions.append(list(a_key)+list(c_key))

    sorted_dict = {}
    for key in sorted(k_count):
        sorted_dict[key] = k_count[key]
    acc_dict = {key:value for key, value in zip(sorted_dict.keys(), accumulate(sorted_dict.values()))}
    return acc_dict

if __name__ == "__main__":
    main()

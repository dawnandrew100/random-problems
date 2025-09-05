import math

def main():
    print(solve_quadratic(1,-3,2))
    print(solve_quadratic_no_rec(1,-3,2))
    
    print(solve_quadratic(1,2,1))
    print(solve_quadratic_no_rec(1,2,1))

def solve_quadratic(a: int, b: int, c: int, res = None) -> tuple[float, float]:
    if res == None:
        res = []
    discriminant = (b**2) - 4*a*c
    if discriminant < 0:
        raise ValueError("No real solutions")
    sqrt = math.sqrt(discriminant)
    sqrt = sqrt if not res else -sqrt
    numerator = -b + sqrt
    denominator = 2 * a
    res.append(numerator/denominator)
    if len(res) == 1:
        solve_quadratic(a, b, c, res=res)
    return tuple(res)

def solve_quadratic_no_rec(a: int, b: int, c: int) -> tuple[float, float]:
    discriminant = (b**2) - 4*a*c
    if discriminant < 0:
        raise ValueError("No real solutions")
    sqrt = math.sqrt(discriminant)
    sol1 = (-b + sqrt) / (2*a)
    sol2 = (-b - sqrt) / (2*a)
    return (sol1, sol2)

if __name__ == "__main__":
    main()
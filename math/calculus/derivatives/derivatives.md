# Derivatives

A derivative is defined as the slope of the tangent line at a point on the curve of a function.

A tangent line shows the rate of instantaneous change at a point for a function.

The equation for the slope of a line is $\frac{y_2 - y_1}{x_2 - x_1}$.
Written in function notation this is $\frac{f(x_2) - f(x_1)}{x_2 - x_1}$.

To find the general form of this equation, we can label $`x_1`$ as $`x`$ and $`x_2`$ as $`x + h`$.

This gives us the final equation for the slope of a tangent line as $\frac{f(x+h) - f(x)}{x + h - x}$,
which can be simplified to $\displaystyle f'(x) = \frac{f(x+h) - f(x)}{h}$.

Due to the fact that h of the tangent line is equal to 0 and you cannot divide by 0,
the full definition of the slope of the tangent line is $\displaystyle f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$

The alternative to the derivative function is $\displaystyle f'(a) = \lim_{x \to a} \frac{f(x) - f(a)}{x - a}$

The difference between these two functions is that $f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$ returns a derivative function
while $f'(a) = \lim_{x \to a} \frac{f(x) - f(a)}{x - a}$ evaluates the function at a point `a` to return the value of the tangent slope at point `a`.

## Rules of derivatives

In the context of the following functions, `f` is the original function `f'` (f prime) is the first derivative of the function of `f`.

### Constant Rule

$\displaystyle \frac{d}{dx}[c] = 0$

### Power Rule

$\displaystyle \frac{d}{dx}[x^n] = nx^{n-1}$

### Constant Multiple Rule

$\displaystyle \frac{d}{dx}[c \cdot x^n] = c \cdot nx^{n-1}$

### Sum Rule

$\displaystyle \frac{d}{dx}[f + g] = f' + g'$

### Difference Rule

$\displaystyle \frac{d}{dx}[f - g] = f' - g'$

### Product Rule

$\displaystyle \frac{d}{dx}[fg] = f'g + fg'$

$\displaystyle \frac{d}{dx}[fgh] = f'gh + fg'h + fgh'$

### Quotient Rule

$\displaystyle \frac{d}{dx}[\frac{f}{g}] = \frac{f'g - fg'}{g^2}$

### Chain Rule

$\displaystyle \frac{d}{dx}[f[g(x)]] = \frac{df}{dg}((g(x))) \frac{dg}{dx}(x) = f'[g(x)] \cdot g'$

**Examples**:

$\displaystyle \frac{d}{dx}[\cos(x^2)] = -\sin(x^2) \cdot 2x$

$\displaystyle \frac{d}{dx}[x^2 - 3x]^5 = 5[x^2 - 3x]^4 \cdot (2x - 3)$

$\displaystyle \frac{d}{dx}[\sec(4x)] = \sec(4x)\tan(4x) \cdot 4$

$\displaystyle \frac{d}{dx}[ln(x^2 +1)] = \frac{1}{x^2 + 1} \cdot \frac{d}{dx}(x^2 + 1) = \frac{2x}{x^2 + 1}$

$\displaystyle \frac{d}{dx} \sqrt{x^4 - 5x + 1} = (x^4 - 5x + 1)^{\frac{1}{2}} = \frac{1}{2}(x^4 - 5x + 1)^{-\frac{1}{2}} \cdot 4x^3 - 5 = \frac{4x^3 - 5}{2\sqrt{x^4 - 5x + 1}}$

### Trigonometric Derivatives

$\displaystyle \frac{d}{dx}[\sin x] = \cos x$

$\displaystyle \frac{d}{dx}[\cos x] = -\sin x$

$\displaystyle \frac{d}{dx}[\sec x] = \sec x \tan x$

$\displaystyle \frac{d}{dx}[\csc x] = - \csc x \cot x$

$\displaystyle \frac{d}{dx}[\tan x] = \sec^{2} x$

$\displaystyle \frac{d}{dx}[\cot x] = - \csc^{2} x$

### Logarithmic Derivatives

$\displaystyle \frac{d}{dx}ln[f] = \frac{f'}{f}$

$\displaystyle \frac{d}{dx}log_a[f] = \frac{f'}{f \cdot ln \cdot a}$

**Examples**:

$\displaystyle \frac{d}{dx} ln(x) = \frac{1}{x}$

$\displaystyle \frac{d}{dx} log_a(x) = \frac{1}{x \cdot ln \cdot a}$


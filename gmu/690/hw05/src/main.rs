fn main() {
    let function = |x: f64, y: f64| -9.0 * x + x.powi(2) + 11.0 * y + 4.0 * y.powi(2) - 2.0 * x * y;
    let gradient = |x: f64, y: f64| (-9.0 + 2.0 * x - 2.0 * y, 11.0 + 8.0 * y - 2.0 * x);

    let x_init = 0.0;
    let y_init = 0.0;

    let tol = 1e-6;
    let max_iter = 1000 as usize;

    let ((x1, y1), iter) = steepest_descent(function, gradient, x_init, y_init, 0.1, 1, tol);
    println!(
        "Minimum found at ({:.6}, {:.6}) after {} iterations",
        x1, y1, iter
    );

    let ((xmin, ymin), iterations) =
        steepest_descent(function, gradient, x_init, y_init, 0.1, max_iter, tol);

    println!(
        "Minimum found at ({:.6}, {:.6}) after {} iterations",
        xmin, ymin, iterations
    );
}

fn steepest_descent<F, G>(
    f: F,
    grad: G,
    x0: f64,
    y0: f64,
    step_size: f64,
    imax: usize,
    tol: f64,
) -> ((f64, f64), usize)
where
    F: Fn(f64, f64) -> f64,
    G: Fn(f64, f64) -> (f64, f64),
{
    let mut x = x0;
    let mut y = y0;
    let mut step = step_size;
    let (mut gx, mut gy) = grad(x, y);

    for i in 1..=imax {
        // linear search
        while f(x - step * gx, y - step * gy) > f(x, y) - tol * step * (gx * gx + gy * gy) {
            step *= 0.5;
        }

        // xk+1 = xk - akgk, yk+1 = yk - akgk
        x -= step * gx;
        y -= step * gy;

        // Compute gradient at xk+1, yk+1
        (gx, gy) = grad(x, y);

        let norm_g = gx.powi(2) + gy.powi(2).sqrt();
        if norm_g <= tol {
            // if ||gk+1||2 <= tol then converged
            return ((x, y), i);
        }
    }
    ((x, y), imax)
}

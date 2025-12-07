//------------------------------------------------------
// Ralston's method Second-Order Runge-Kutta (RK2)
// -----------------------------------------------------
pub fn ralston_rk2(
    yi: f64,
    xi: f64,
    xf: f64,
    step_size: f64,
    display_interval: f64,
) -> Vec<(f64, f64)> {
    let mut x = xi;
    let mut y = yi;
    let mut m = 0;
    let mut solutions = Vec::new();
    solutions.push((x, y));
    let steps = ((xf - xi) / step_size) as usize;
    println!("{m:3}   x={x:10.5}   y={y:10.5}");

    for step in 1..=steps {
        let k1 = derivs(x, y);

        let ym = y + k1 * step_size / 2.0;
        let k2 = derivs(x + step_size / 2.0, ym);

        let slope = ((1.0 / 3.0) * k1) + ((2.0 / 3.0) * k2);
        y += slope * step_size;
        x += step_size;
        solutions.push((x, y));
        if (step as f64 * step_size) % display_interval == 0_f64 {
            m += 1;
            println!("{m:3}   x={x:10.5}   y={y:10.5}");
        }
    }
    solutions
}

//-----------------------------------------------------
// Fourth-Order Runge–Kutta (RK4)
//-----------------------------------------------------
pub fn rk4(yi: f64, xi: f64, xf: f64, step_size: f64, display_interval: f64) -> Vec<(f64, f64)> {
    let mut x = xi;
    let mut y = yi;
    let mut m = 0;
    let mut solutions = Vec::new();
    solutions.push((x, y));
    let steps = ((xf - xi) / step_size) as usize;
    println!("{m:3}   x={x:10.5}   y={y:10.5}");

    for step in 1..=steps {
        let k1 = derivs(x, y);

        let ym = y + k1 * step_size / 2.0;
        let k2 = derivs(x + step_size / 2.0, ym);

        let ym = y + k2 * step_size / 2.0;
        let k3 = derivs(x + step_size / 2.0, ym);

        let ye = y + k3 * step_size;
        let k4 = derivs(x + step_size, ye);

        let slope = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

        y += slope * step_size;
        x += step_size;
        solutions.push((x, y));

        if (step as f64 * step_size) % display_interval == 0_f64 {
            m += 1;
            println!("{m:3}   x={x:10.5}   y={y:10.5}");
        }
    }
    solutions
}

fn derivs(x: f64, y: f64) -> f64 {
    // dy/dt = y sin^3 (t)
    y * x.sin().powi(3)
}

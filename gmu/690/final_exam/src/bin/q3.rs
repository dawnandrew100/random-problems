fn main() {
    let y0 = -1_f64;
    let ti = 0_f64;
    let tf = 5_f64;
    let initial_step_size = 0.2;
    let increment = 0.01;
    // i is turnt into an integer because direct subtraction of floating point numbers
    // in Rust leads to some unfortunate side-effects
    let mut i = ((1_f64 / increment) * initial_step_size) as usize;

    loop {
        let step_size = i as f64 * increment;
        println!("Step size: {step_size}\n");
        println!("Ralston's 2nd order Runge-Kutta");
        let ral_sol = ralston_rk2(y0, ti, tf, step_size);
        println!("Ralston's RK2 solution at time t={tf}: {ral_sol}\n");

        println!("4th order Runge-Kutta");
        let rk4_sol = rk4(y0, ti, tf, step_size);
        println!("RK4 solution at time t={tf}: {rk4_sol}\n");

        let absolute_diff = (ral_sol - rk4_sol).abs();
        println!("Absolute Difference: {absolute_diff}\n");

        if absolute_diff < 1.0 {
            println!("h={step_size} reduces absolute difference below 1");
            break;
        } else if i <= 1 {
            println!("Minimum step size of {increment} reached");
            break;
        }
        i -= 1;
    }
}

pub fn ralston_rk2(yi: f64, ti: f64, tf: f64, step_size: f64) -> f64 {
    let mut t = ti;
    let mut y = yi;
    let steps = ((tf - ti) / step_size) as usize;

    for _ in 1..=steps {
        let k1 = derivs(t, y);

        let xi = t + step_size * (3.0 / 4.0);
        let yi = y + k1 * step_size * (3.0 / 4.0);
        let k2 = derivs(xi, yi);

        let slope = ((1.0 / 3.0) * k1) + ((2.0 / 3.0) * k2);
        y += slope * step_size;
        t += step_size;
    }
    y
}

fn rk4(yi: f64, ti: f64, tf: f64, step_size: f64) -> f64 {
    let mut t = ti;
    let mut y = yi;
    let steps = ((tf - ti) / step_size) as usize;

    for _ in 1..=steps {
        let k1 = derivs(t, y);

        let xi = t + step_size / 2.0;
        let yi = y + k1 * step_size / 2.0;
        let k2 = derivs(xi, yi);

        let xi = t + step_size / 2.0;
        let yi = y + k2 * step_size / 2.0;
        let k3 = derivs(xi, yi);

        let xi = t + step_size;
        let yi = y + k3 * step_size;
        let k4 = derivs(xi, yi);

        let slope = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

        y += slope * step_size;
        t += step_size;
    }
    y
}

fn derivs(t: f64, y: f64) -> f64 {
    // dy/dt = y - t^2
    y - t.powi(2)
}

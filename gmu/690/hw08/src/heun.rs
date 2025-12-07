//------------------------------------------------------
// Heun's Predictor Corrector Method
// -----------------------------------------------------
pub fn heun(yi: f64, xi: f64, xf: f64, step_size: f64, display_interval: f64) -> Vec<(f64, f64)> {
    let mut x = xi;
    let mut y = yi;
    let mut m = 0;
    let mut solutions = Vec::new();
    solutions.push((x, y));
    let steps = ((xf - xi) / step_size) as usize;
    println!("{m:3}   x={x:10.5}   y={y:10.5}");

    for step in 1..=steps {
        let dy1 = derivs(x, y);
        let ye = y + dy1 * step_size;

        let dy2 = derivs(x + step_size, ye);

        let slope = (dy1 + dy2) / 2.0;

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

//------------------------------------------------------
// Heun's Predictor Corrector Method with no corrector
// -----------------------------------------------------
pub fn heun_no_corrector(
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
        let dy1 = derivs(x, y);
        let slope = y + dy1 * step_size;

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

//------------------------------------------------------
// Midpoint (Improved Polygon/Modified Euler) Method
// -----------------------------------------------------
#[allow(dead_code)]
pub fn midpoint(
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
        let dy1 = derivs(x, y);
        let ym = y + dy1 * step_size / 2.0;

        let dy_m = derivs(x + step_size / 2.0, ym);

        y += dy_m * step_size;
        x += step_size;
        solutions.push((x, y));

        if (step as f64 * step_size) % display_interval == 0_f64 {
            m += 1;
            println!("{m:3}   x={x:10.5}   y={y:10.5}");
        }
    }
    solutions
}

//------------------------------------------------------
// Heun's Iterative Predictor Corrector Method
// -----------------------------------------------------
#[allow(dead_code)]
pub fn heun_iter(
    yi: f64,
    xi: f64,
    xf: f64,
    step_size: f64,
    display_interval: f64,
) -> Vec<(f64, f64)> {
    let es = 0.01; // stopping tolerance
    let maxit = 20;
    let mut x = xi;
    let mut y = yi;
    let mut m = 0;
    let mut solutions = Vec::new();
    solutions.push((x, y));
    let steps = ((xf - xi) / step_size) as usize;
    println!("{m:3}   x={x:10.5}   y={y:10.5}");

    for step in 1..=steps {
        let dy1 = derivs(x, y);
        let mut ye = y + dy1 * step_size;

        let mut iter = 0;

        loop {
            let ye_old = ye;

            let dy2 = derivs(x + step_size, ye);

            let slope = (dy1 + dy2) / 2.0;

            ye = y + slope * step_size;

            iter += 1;

            // approximate relative error
            let ea = ((ye - ye_old).abs() / ye.abs()) * 100.0;

            if ea <= es || iter > maxit {
                break;
            }
        }
        y = ye;
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

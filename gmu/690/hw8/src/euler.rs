//------------------------------------------------------
// Improved/modular Euler method
// -----------------------------------------------------
pub fn improved_euler(
    yi: f64,               // initial y
    xi: f64,               // initial time
    xf: f64,               // final time
    step_size: f64,        // step size
    display_interval: f64, // output interval
) -> Vec<(f64, f64)> {
    let mut x = xi;
    let mut y = yi;
    let mut m = 0;
    let mut solutions = Vec::new();
    solutions.push((x, y));

    // save first output point
    let mut xpm = x;
    let mut ypm = y;

    println!("{m:3}   x={xpm:10.5}   y={ypm:10.5}");

    loop {
        let mut xend = x + display_interval;
        if xend > xf {
            xend = xf;
        }

        let temp = integrator(&mut x, &mut y, step_size, xend);
        solutions.extend(temp);

        m += 1;
        xpm = x;
        ypm = y;

        println!("{m:3}   x={xpm:10.5}   y={ypm:10.5}");

        if x >= xf {
            break;
        }
    }
    solutions
}

fn integrator(x: &mut f64, y: &mut f64, h: f64, xend: f64) -> Vec<(f64, f64)> {
    let mut temp_sol = Vec::new();
    loop {
        let mut step = h;

        // If step overshoots xend, shorten it
        if xend - *x < h {
            step = xend - *x;
        }

        euler(x, y, step);
        temp_sol.push((*x, *y));

        if *x >= xend {
            break;
        }
    }
    temp_sol
}

fn euler(x: &mut f64, y: &mut f64, h: f64) {
    let dydx = derivs(*x, *y);
    *y += dydx * h;
    *x += h;
}

fn derivs(x: f64, y: f64) -> f64 {
    // dy/dt = y sin^3 (t)
    y * x.sin().powi(3)
}

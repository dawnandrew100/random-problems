fn main() {
    let guesses = [0.0, 1.0, 2.0];

    for guess in guesses {
        let res = newton_raphson_method(guess, 100, 0.01);
        match res {
            Some(x) => println!(
                "Starting at {guess}, root found: ({x:.5}, {:.5})",
                og_func(x).abs()
            ),
            None => println!("Starting at {guess}, no root was found within the given iterations"),
        }
    }
}

fn newton_raphson_method(x0: f64, itermax: usize, es: f64) -> Option<f64> {
    let mut iter = 0;
    let mut xr = x0;
    let mut ea = 100 as f64;
    loop {
        let xr_old = xr;
        xr = xr_old - (og_func(xr) / first_dx(xr));
        let poss_root = og_func(xr);
        iter += 1;
        if xr != 0 as f64 {
            ea = ((xr - xr_old).abs() / xr) * 100.0;
        }
        if (ea < es || iter >= itermax) && poss_root.abs() < 1e-9 {
            break;
        }
    }
    if iter >= itermax {
        return None;
    }
    Some(xr)
}

fn og_func(x: f64) -> f64 {
    -1.5 + 6.0 * x - 3.9 * x.powi(2) + 0.5 * x.powi(3)
}

fn first_dx(x: f64) -> f64 {
    6.0 - 7.8 * x + 1.5 * x.powi(2)
}

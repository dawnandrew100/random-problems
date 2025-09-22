fn main() {
    let res = bisection(0.0, 1.0, 5.0, 1000, 0.6);
    println!(
        "The approximate coords for the maximum of this function are: ({res}, {:.5})",
        polynomial(res)
    );
    println!(
        "The true x value for the maximum of this function are: (0.90449, {:.5})",
        polynomial(0.90449)
    );
}

fn bisection(xl: f64, xu: f64, es: f64, imax: usize, xr: f64) -> f64 {
    let mut iter = 0;
    let mut ea = 100.0;
    let mut xu = xu;
    let mut xl = xl;
    let mut xr = xr;
    loop {
        let xr_old = xr;
        xr = (xl + xu) / 2 as f64;
        if xr != 0 as f64 {
            ea = {
                let absv = xr - xr_old;
                (absv.abs() / xr) * 100 as f64
            };
        }
        let test = first_dx(xl) * first_dx(xr);
        if test < 0 as f64 {
            xu = xr;
        } else if test > 0 as f64 {
            xl = xr;
        } else {
            ea = 0.0;
        }
        if ea < es || iter >= imax {
            break;
        }
        iter += 1;
    }
    xr
}

fn polynomial(x: f64) -> f64 {
    -2.0 * x.powi(6) - 1.6 * x.powi(4) + 12.0 * x + 1.0
}

fn first_dx(x: f64) -> f64 {
    -12.0 * x.powi(5) - 6.4 * x.powi(3) + 12.0
}

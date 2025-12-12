fn main() {
    let ti = 0_f64;
    let tf = 5_f64;
    let step_size = 0.01;
    let z0: Vec<f64> = vec![0.0, -10.0];

    println!("Solving System of ODEs using Heun's Predictor-Corrector Method");
    println!("System: dx/dt = y, dy/dt = -12x");
    println!("Initial State: x(0)={}, y(0)={}", z0[0], z0[1]);
    println!("Integration Range: t={} to t={}", ti, tf);
    println!("Step Size: {}\n", step_size);

    let final_state = heun_ode_iter(z0, ti, tf, step_size);

    let x_final = final_state[0];
    let y_final = final_state[1];

    println!("\n--- Final Results ---");
    println!("The value of x(t=5) is: {:.8}", x_final);
    println!("The value of y(t=5) is: {:.8}", y_final);
}

//------------------------------------------------------
// Heun's Iterative Predictor Corrector Method
// -----------------------------------------------------
fn heun_ode_iter(zi: Vec<f64>, ti: f64, tf: f64, step_size: f64) -> Vec<f64> {
    let es = 0.01; // stopping tolerance
    let maxit = 20;
    let mut t = ti;
    let mut z = zi;
    let steps = ((tf - ti) / step_size) as usize;

    for _ in 1..=steps {
        let dz1 = derivs(t, &z);
        let mut ze = z
            .iter()
            .zip(dz1.iter())
            .map(|(&val_z, &val_dz)| val_z + val_dz * step_size)
            .collect::<Vec<f64>>();
        let mut iter = 0;

        loop {
            let ze_old = ze.clone();

            let dz2 = derivs(t + step_size, &ze);

            let slope = dz1
                .iter()
                .zip(dz2.iter())
                .map(|(&val_dz1, &val_dz2)| (val_dz1 + val_dz2) / 2.0)
                .collect::<Vec<f64>>();

            ze = z
                .iter()
                .zip(slope.iter())
                .map(|(&val_z, &val_slope)| val_z + val_slope * step_size)
                .collect::<Vec<f64>>();

            iter += 1;

            let ea = ze
                .iter()
                .zip(ze_old.iter())
                .map(|(&val_new, &val_old)| {
                    let ea = ((val_new - val_old).abs() / val_new.abs()) * 100.0;
                    if ea.is_finite() { ea } else { 0.0 }
                })
                .fold(0.0f64, f64::max);

            if ea <= es || iter > maxit {
                break;
            }
        }
        z = ze;
        t += step_size;
    }
    z
}

fn derivs(_t: f64, z: &[f64]) -> Vec<f64> {
    let x = z[0]; // z[0] is x
    let y = z[1]; // z[1] is y
    // dx/dt = y
    let dxdt = y;
    // dy/dt = -12x
    let dydt = -12.0 * x;
    vec![dxdt, dydt]
}

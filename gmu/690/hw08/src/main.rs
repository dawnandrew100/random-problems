mod euler;
mod heun;
mod runge_kutta;

use euler::improved_euler;
use heun::{heun, heun_no_corrector};
use runge_kutta::{ralston_rk2, rk4};

fn main() {
    let y0 = 1.0;
    let ti = 0.0;
    let tf = 3.0;
    let step_size = 0.1;
    let display_interval = 0.5;

    println!("Improved Euler");
    let sol = improved_euler(y0, ti, tf, step_size, display_interval);
    println!("Improved Euler Points: {sol:?}");

    println!("heun no corrector");
    let sol = heun_no_corrector(y0, ti, tf, step_size, display_interval);
    println!("Heun No Corrector Points: {sol:?}");

    println!("heun");
    let sol = heun(y0, ti, tf, step_size, display_interval);
    println!("Heun Points: {sol:?}");

    println!("ralston rk2");
    let sol = ralston_rk2(y0, ti, tf, step_size, display_interval);
    println!("Ralston's RK2 Points: {sol:?}");

    println!("rk4");
    let sol = rk4(y0, ti, tf, step_size, display_interval);
    println!("RK4 Points: {sol:?}");
}

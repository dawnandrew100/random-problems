const G: f64 = 9.8;

fn main() {
    let (t, v_array) = numerical_solution(0.64, 23.0, 0.0, 10.0, 15.0, 1.0);
    println!("final time is {t} and the list of points is {:?}", v_array);
}

fn numerical_solution(c: f64, m: f64, ti: f64, tf: f64, vi: f64, dt: f64) -> (f64, Vec<f64>) {
    let mut t = ti;
    let mut v = vi;
    let n = (tf - ti) / dt;

    let mut v_array = Vec::new();
    for _ in 1..=n as usize {
        let dvdt = G - c * v / m;
        v = v + dvdt * dt;
        t = t + dt;
        v_array.push(t);
    }
    (t, v_array)
}

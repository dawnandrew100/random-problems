fn main() {
    let x: Vec<f64> = vec![1.6, 2.0, 2.5, 3.2, 4.0, 4.5];
    let fx: Vec<f64> = vec![2.0, 8.0, 14.0, 15.0, 8.0, 2.0];

    new_int(&x, &fx, 6, 10.0);
}

/*
 * THIS IMPLEMENTATION IS NOT COMPLETE
 * NEEDS TO BE REWORKED
 */

fn new_int(x: &Vec<f64>, y: &Vec<f64>, n: usize, xi: f64) {
    let mut fdd = vec![vec![0.0; n]; n];
    let mut yint = vec![0.0; n];
    let mut ea = vec![0.0; n];
    for i in 0..n {
        fdd[i][0] = y[i];
    }
    for j in 1..n {
        for i in 0..n - j {
            fdd[i][j] = (fdd[i + 1][j - 1] - fdd[i][j - 1]) / (x[i + j] - x[i]);
        }
    }
    let mut xterm = 1.0;
    yint[0] = fdd[0][0];
    for i in 1..n {
        xterm *= xi - x[i];
        let yint2 = yint[i - 1] + fdd[0][i] * xterm;
        ea[i - 1] = yint2 - yint[i - 1];
        yint[i] = yint2;
    }
    println!("{fdd:?}\n{yint:?}\n{ea:?}");
}

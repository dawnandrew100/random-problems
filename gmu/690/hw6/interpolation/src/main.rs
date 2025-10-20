fn main() {
    let x: Vec<f64> = vec![1.6, 2.0, 2.5, 3.2, 4.0, 4.5];
    let fx: Vec<f64> = vec![2.0, 8.0, 14.0, 15.0, 8.0, 2.0];

    let point = 2.8;
    let degree = 5;
    let (coeffs, evaled_point, ea) = new_int(&x, &fx, degree, point);
    println!("Interpolated coeffs for degree {degree} polynomial = {coeffs:?}");
    println!("Estimated error vector = {ea:?}");
    println!("Function evaluated at point {point} = {evaled_point:.5}");
}

fn new_int(x: &Vec<f64>, y: &Vec<f64>, n: usize, eval_point: f64) -> (Vec<f64>, f64, Vec<f64>) {
    let n = n + 1;
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
        xterm *= eval_point - x[i];
        let yint2 = yint[i - 1] + fdd[0][i] * xterm;
        ea[i - 1] = yint2 - yint[i - 1];
        yint[i] = yint2;
    }

    let mut expanded_poly: Vec<f64> = vec![fdd[0][0]];
    let mut degree = 1;
    let mut term = 0.0;
    for i in 1..n {
        term += fdd[0][i];
        for j in 0..degree {
            term *= eval_point - x[j];
        }
        expanded_poly.push(term);
        degree += 1;
        term = 0.0;
    }
    let evaluated_point = expanded_poly.into_iter().sum::<f64>();

    (fdd[0].clone(), evaluated_point, ea)
}

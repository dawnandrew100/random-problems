fn main() {
    let x: Vec<f64> = vec![1_f64, 2_f64, 3_f64, 6_f64, 7_f64, 9_f64];
    let fx: Vec<f64> = vec![6.2, 105.2, 764.2, 23765.2, 51192.2, 179102.0];

    let point = 7.5;
    let degree = 5;
    for order in 0..=degree {
        println!("Interpolation degree: {order}");
        let (coeffs, evaled_point, ea) = new_int(&x, &fx, order, point);
        println!("Interpolated coeffs for degree {order} polynomial = {coeffs:?}");
        println!("Estimated error vector = {ea:?}");
        println!("Function evaluated at point {point} = {evaled_point:.5}\n");
    }
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::mut_range_bound)]
fn new_int(x: &[f64], y: &[f64], degree: usize, eval_point: f64) -> (Vec<f64>, f64, Vec<f64>) {
    let degree = degree + 1;
    let mut fdd = vec![vec![0.0; degree]; degree];
    let mut yint = vec![0.0; degree];
    let mut ea = vec![0.0; degree];
    for i in 0..degree {
        fdd[i][0] = y[i];
    }
    for j in 1..degree {
        for i in 0..degree - j {
            fdd[i][j] = (fdd[i + 1][j - 1] - fdd[i][j - 1]) / (x[i + j] - x[i]);
        }
    }
    let mut xterm = 1.0;
    yint[0] = fdd[0][0];
    for i in 1..degree {
        xterm *= eval_point - x[i];
        let yint2 = yint[i - 1] + fdd[0][i] * xterm;
        ea[i - 1] = yint2 - yint[i - 1];
        yint[i] = yint2;
    }

    let mut expanded_poly: Vec<f64> = vec![fdd[0][0]];
    let mut degree = 1;
    let mut term = 0.0;
    for i in 1..degree {
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

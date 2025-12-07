fn main() {
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let y: Vec<f64> = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0, 13.0];

    let (slope, intercept, err, r2) = ls_linear_regression(&x, &y);
    println!("{slope:.4}, {intercept:.4}, {err:.4}, {r2:.4}");

    let (a, rhs) = polynomial_regression(&x, &y, 2);
    println!("{:?} {:?}", a, rhs);

    let mut coeff_matrix = a.clone();
    let mut rhs = rhs.clone();
    let size = coeff_matrix.len();
    let mut solution = vec![0.0; size];
    let tol = 1e-12;
    let mut err = 0;

    gaussian_elimination(
        &mut coeff_matrix,
        &mut rhs,
        size,
        &mut solution,
        tol,
        &mut err,
    );
    println!("{solution:?}");
}

fn ls_linear_regression(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64, f64, f64) {
    let length = x.len() as f64;
    let sumx = x.iter().sum::<f64>();
    let sumy = y.iter().sum::<f64>();
    let sumxy = x.iter().zip(y.iter()).map(|(n, m)| n * m).sum::<f64>();
    let sumx2 = x.iter().map(|n| n.powi(2)).sum::<f64>();
    let x_mean = sumx / length;
    let y_mean = sumy / length;
    let slope = (length * sumxy - sumx * sumy) / (length * sumx2 - sumx * sumx);
    let intercept = y_mean - slope * x_mean;
    let sq_total = y.iter().map(|n| (n - y_mean).powi(2)).sum::<f64>();
    let sq_residual = y
        .iter()
        .zip(x.iter())
        .map(|(n, m)| (n - slope * m - intercept).powi(2))
        .sum::<f64>();

    let std_err = (sq_residual / (length - 2.0)).sqrt();
    let r2 = (sq_total - sq_residual) / sq_total;

    (slope, intercept, std_err, r2)
}

fn polynomial_regression(x: &Vec<f64>, y: &Vec<f64>, order: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; order + 1]; order + 1];
    let mut rhs: Vec<f64> = vec![0.0; order + 1];
    for i in 0..=order {
        for j in 0..=i {
            let k = i + j;
            let poly_sum = x.iter().map(|n| n.powi(k as i32)).sum::<f64>();
            a[i][j] = poly_sum;
            a[j][i] = poly_sum;
        }
        let poly_sum = y
            .iter()
            .zip(x.iter())
            .map(|(n, m)| n * m.powi(i as i32))
            .sum::<f64>();
        rhs[i] = poly_sum;
    }
    (a, rhs)
}

fn gaussian_elimination(
    coeff_matrix: &mut Vec<Vec<f64>>,
    rhs_vector: &mut Vec<f64>,
    size: usize,
    solution: &mut Vec<f64>,
    tolerance: f64,
    error_flag: &mut i32,
) {
    *error_flag = 0;

    // Scaling vector
    let mut scale_factor = vec![0.0; size];
    for i in 0..size {
        scale_factor[i] = coeff_matrix[i][0].abs();
        for j in 1..size {
            if coeff_matrix[i][j].abs() > scale_factor[i] {
                scale_factor[i] = coeff_matrix[i][j].abs();
            }
        }
    }
    forward_elimination(
        coeff_matrix,
        &mut scale_factor,
        size,
        rhs_vector,
        tolerance,
        error_flag,
    );

    if *error_flag != -1 {
        back_substitution(coeff_matrix, size, rhs_vector, solution);
    }
}

fn forward_elimination(
    coeff_matrix: &mut Vec<Vec<f64>>,
    scale_factor: &mut Vec<f64>,
    size: usize,
    rhs_vector: &mut Vec<f64>,
    tol: f64,
    error_flag: &mut i32,
) {
    for k in 0..(size - 1) {
        partial_pivot(coeff_matrix, rhs_vector, scale_factor, size, k);
        if (coeff_matrix[k][k] / scale_factor[k]).abs() < tol {
            *error_flag = -1;
            return;
        }
        for i in (k + 1)..size {
            let factor = coeff_matrix[i][k] / coeff_matrix[k][k];
            for j in (k + 1)..size {
                coeff_matrix[i][j] -= factor * coeff_matrix[k][j];
            }
            rhs_vector[i] -= factor * rhs_vector[k]
        }
    }
    if (coeff_matrix[size - 1][size - 1] / scale_factor[size - 1]).abs() < tol {
        *error_flag = -1;
    }
}

fn partial_pivot(
    coeff_matrix: &mut Vec<Vec<f64>>,
    rhs_vector: &mut Vec<f64>,
    scale_factor: &mut Vec<f64>,
    size: usize,
    k: usize,
) {
    let mut p = k;
    let mut big = (coeff_matrix[k][k] / scale_factor[k]).abs();
    for ii in (k + 1)..size {
        let temp = (coeff_matrix[ii][k] / scale_factor[ii]).abs();
        if temp > big {
            big = temp;
            p = ii;
        }
    }
    if p != k {
        // Swap rows in A
        coeff_matrix.swap(p, k);

        // Swap entries in rhs_vector
        rhs_vector.swap(p, k);

        // Swap entries in scale_factor
        scale_factor.swap(p, k);
    }
}

fn back_substitution(
    coeff_matrix: &Vec<Vec<f64>>,
    size: usize,
    rhs_vector: &Vec<f64>,
    solution: &mut Vec<f64>,
) {
    solution[size - 1] = rhs_vector[size - 1] / coeff_matrix[size - 1][size - 1];
    for i in (0..(size - 1)).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..size {
            sum += coeff_matrix[i][j] * solution[j]
        }
        solution[i] = (rhs_vector[i] - sum) / coeff_matrix[i][i];
    }
}

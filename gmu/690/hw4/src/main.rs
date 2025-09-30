fn main() {
    let mut coeff_matrix = vec![
        vec![8.0, 2.0, -2.0],
        vec![10.0, 2.0, 4.0],
        vec![12.0, 2.0, 2.0],
    ];

    let mut rhs_vector = vec![8.0, 16.0, 16.0];
    let size = coeff_matrix.len();
    let mut solution = vec![0.0; size];
    let tol = 1e-12;
    let mut er = 0;

    gaussian_elimination(
        &mut coeff_matrix,
        &mut rhs_vector,
        size,
        &mut solution,
        tol,
        &mut er,
    );

    if er == -1 {
        println!("System is singular or near-singular.");
    } else {
        println!("Solution:");
        for (i, sol) in solution.iter().enumerate() {
            print!("x{} = {sol}", i + 1);
            if i != solution.len() - 1 {
                print!(", ")
            }
        }
        println!("");
    }
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

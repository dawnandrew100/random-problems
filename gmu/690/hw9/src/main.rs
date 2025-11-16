use spindalis::utils::arr2D::{Arr2D, Rounding};

fn main() {
    let matrix = Arr2D::from(&[
        [3.556, -1.778, 0.0],
        [-1.778, 3.556, -1.778],
        [0.0, -1.778, 3.556],
    ]);
    let (value, vector) = eigenvalue_power_method(&matrix, 1e-10);
    println!(
        "Value = {value:.4}\nVector:\n{}\n",
        vector.round_to_decimal(5)
    );

    let inverse_matrix = matrix.inverse().unwrap();
    let (value, vector) = eigenvalue_power_method(&inverse_matrix, 1e-10);
    println!(
        "Value = {value:.4}\nVector:\n{}",
        vector.round_to_decimal(5)
    );
}

fn eigenvalue_power_method(matrix: &Arr2D<f64>, es: f64) -> (f64, Arr2D<f64>) {
    let initial_eigen_vector = Arr2D::from(&[[1.0], [1.0], [1.0]]);
    let mut eigen_vector = matrix * &initial_eigen_vector;
    let mut eigen_value = eigen_vector.max().unwrap();
    eigen_vector = eigen_vector / eigen_value;
    loop {
        eigen_vector = matrix * eigen_vector;
        let next_eigen_value = eigen_vector.max().unwrap();

        let ea = ((next_eigen_value - eigen_value) / next_eigen_value).abs();

        eigen_value = next_eigen_value;
        eigen_vector = eigen_vector / eigen_value;
        if ea < es {
            break;
        }
    }
    (eigen_value, eigen_vector)
}

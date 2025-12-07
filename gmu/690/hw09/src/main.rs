use spindalis::utils::{
    Arr2DError,
    arr2D::{Arr2D, Rounding},
};

fn main() {
    // Matrix with test data from
    // Example 27.7 and 27.8 from the book
    let matrix_1 = Arr2D::from(&[
        [3.556, -1.778, 0.0],
        [-1.778, 3.556, -1.778],
        [0.0, -1.778, 3.556],
    ]);
    let matrix_2 = Arr2D::from(&[[2.0, 8.0, 10.0], [8.0, 4.0, 5.0], [10.0, 5.0, 7.0]]);

    let matrices = vec![
        (matrix_1, "Examples 27.7 and 27.8"),
        (matrix_2, "Homework questions 27.11 and 27.12"),
    ];

    for (matrix, title) in matrices {
        println!("\n{title}\n");
        let result = eigenvalue_power_method(&matrix, 1e-10);
        match result {
            Ok((value, vector)) => println!(
                "Largest Eigen Value = {value:.4}\nVector:\n{}\n",
                vector.round_to_decimal(5)
            ),
            Err(e) => eprintln!("{e:?}"),
        }

        let inverse_res = matrix.inverse();
        match inverse_res {
            Ok(inverse_matrix) => {
                let result = eigenvalue_power_method(&inverse_matrix, 1e-10);
                match result {
                    Ok((value, vector)) => println!(
                        "Smallest Eigen Value = {:.4}\nVector:\n{}",
                        1_f64 / value,
                        vector.round_to_decimal(5),
                    ),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            Err(e) => {
                eprintln!(
                    "Error calculating the inverse for the following matrix:\n{matrix}\n{e:?}"
                )
            }
        }
    }
}

fn eigenvalue_power_method<M>(matrix: M, es: f64) -> Result<(f64, Arr2D<f64>), Arr2DError>
where
    M: TryInto<Arr2D<f64>, Error = Arr2DError>,
{
    let matrix: Arr2D<f64> = matrix.try_into()?;
    let initial_eigen_vector = Arr2D::from(&[[1.0], [1.0], [1.0]]);
    let mut eigen_vector = &matrix * &initial_eigen_vector;
    let mut eigen_value = eigen_vector.max().unwrap();
    eigen_vector = eigen_vector / eigen_value;
    loop {
        eigen_vector = &matrix * eigen_vector;
        let next_eigen_value = eigen_vector.max().unwrap();

        let ea = ((next_eigen_value - eigen_value) / next_eigen_value).abs();

        eigen_value = next_eigen_value;
        eigen_vector = eigen_vector / eigen_value;
        if ea < es {
            break;
        }
    }
    Ok((eigen_value, eigen_vector))
}

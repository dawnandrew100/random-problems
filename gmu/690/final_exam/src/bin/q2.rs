use spindalis::polynomials::{eval_polynomial_extended, parse_polynomial_extended};
use std::collections::HashMap;

fn main() {
    let polynomial_str = "x^3 - 2yz";
    let parsed_poly =
        parse_polynomial_extended(polynomial_str).expect("Failed to parse polynomial");

    let ranges = [(-1_f64, 3_f64), (0_f64, 6_f64), (-4_f64, 4_f64)];
    let segments = 4;

    let (x_min, x_max) = ranges[0];
    let (y_min, y_max) = ranges[1];
    let (z_min, z_max) = ranges[2];

    let inner_integral = |y_val: f64, z_val: f64| {
        let integrand_x = |x_val: f64| {
            let vars: HashMap<&str, f64> = [("x", x_val), ("y", y_val), ("z", z_val)]
                .iter()
                .cloned()
                .collect();
            eval_polynomial_extended(&parsed_poly, &vars)
        };
        simpson13(integrand_x, x_min, x_max, segments)
    };

    let middle_integral = |z_val: f64| {
        let integrand_y = |y_val: f64| inner_integral(y_val, z_val);
        simpson13(integrand_y, y_min, y_max, segments)
    };

    let final_result = simpson13(middle_integral, z_min, z_max, segments);

    println!("--- Triple Integral Solution (Numerical) ---");
    println!("Function: {}", polynomial_str);
    println!(
        "Region: x in [{}, {}], y in [{}, {}], z in [{}, {}]",
        x_min, x_max, y_min, y_max, z_min, z_max
    );
    println!("Segments per dimension: {}", segments);
    println!("Numerical Result: {:.2}", final_result);
}

fn simpson13<F>(integrand: F, start: f64, end: f64, segments: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let segments = if !segments.is_multiple_of(2) {
        segments + 1
    } else {
        segments
    };
    let segment_width = (end - start) / segments as f64;
    let mut xi = start;
    let mut sum = integrand(xi);
    for _ in 1..segments / 2 {
        xi += 2_f64 * segment_width;
        sum += 4_f64 * integrand(xi - segment_width) + 2_f64 * integrand(xi);
    }
    xi += 2_f64 * segment_width;
    sum += 4_f64 * integrand(xi - segment_width) + integrand(xi);

    segment_width * sum / 3_f64
}

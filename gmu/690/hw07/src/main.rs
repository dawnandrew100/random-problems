use spindalis::polynomials::{eval_simple_polynomial, parse_simple_polynomial};

fn main() {
    // Original polynomial = (4x - 3)^3
    // Expanded polynomial = 64x^3 - 144x^2 + 108x - 27
    let parsed = parse_simple_polynomial!(64x ^ 3 - 144x ^ 2 + 108x - 27);
    let ans = integral(&parsed, -3.0, 5.0, 5);
    println!("Simpson's Rule 5 segments = {ans:.2}");
    let ans = integral(&parsed, -3.0, 5.0, 4);
    println!("Simpson's Rule 4 segments = {ans:.2}");
    let ans = trapezoidal_rule(&parsed, -3.0, 5.0, 5);
    println!("Trapezoidal Rule 5 segments = {ans:.2}");
    let ans = romberg(&parsed, -3.0, 5.0, 10000, 1e-5);
    println!("Romberg Integration = {ans:.2}");
}

fn integral(poly: &Vec<f64>, start: f64, end: f64, segments: usize) -> f64 {
    let segment_width = (end - start) / segments as f64;
    let mut sum = 0.0;
    if segments == 1 {
        return trapezoidal_rule(&poly, start, end, segments);
    }
    let mut remaining_segments = segments;
    if segments % 2 != 0 {
        // Last four points encapsulate last three segments
        let mut points = [0.0; 4];
        for i in 1..points.len() {
            points[i] = end - segment_width * i as f64;
        }
        points[0] = end;
        points.reverse();
        sum += simpson38(segment_width, &poly, points);
        remaining_segments -= 3;
    }
    if remaining_segments > 1 {
        sum += simpson13(segment_width, &poly, start, remaining_segments);
    }
    sum
}

fn trapezoidal_rule(poly: &Vec<f64>, start: f64, end: f64, segments: usize) -> f64 {
    let mut xi = start;
    let segment_width = (end - start) / segments as f64;
    let mut sum = eval_simple_polynomial(xi, &poly);
    for _ in 1..segments {
        xi += segment_width;
        sum += 2_f64 * eval_simple_polynomial(xi, &poly);
    }
    sum += eval_simple_polynomial(end, &poly);

    segment_width * sum / 2_f64
}

fn simpson38(segment_width: f64, poly: &Vec<f64>, points: [f64; 4]) -> f64 {
    let mut f: Vec<f64> = Vec::new();
    for point in points {
        let x = eval_simple_polynomial(point, &poly);
        f.push(x)
    }
    3_f64 * segment_width * (f[0] + 3_f64 * f[1] + 3_f64 * f[2] + f[3]) / 8_f64
}

fn simpson13(segment_width: f64, poly: &Vec<f64>, start: f64, segments: usize) -> f64 {
    let mut xi = start;
    let mut sum = eval_simple_polynomial(xi, &poly);
    for _ in 1..segments / 2 {
        xi += 2_f64 * segment_width;
        sum += 4_f64 * eval_simple_polynomial(xi - segment_width, &poly)
            + 2_f64 * eval_simple_polynomial(xi, &poly);
    }
    xi += 2_f64 * segment_width;
    sum += 4_f64 * eval_simple_polynomial(xi - segment_width, &poly)
        + eval_simple_polynomial(xi, &poly);

    segment_width * sum / 3_f64
}

fn romberg(poly: &Vec<f64>, start: f64, end: f64, maxiter: u32, tolerance: f64) -> f64 {
    let maxiter = maxiter as usize;
    let mut romberg_table: Vec<Vec<f64>> = vec![vec![0.0; 10]; 10];
    let mut iter = 0_usize;
    let mut segments = 1;
    romberg_table[1][1] = trapezoidal_rule(poly, start, end, segments);

    loop {
        iter += 1;
        segments = 2_u32.pow(iter as u32) as usize;

        romberg_table[iter + 1][1] = trapezoidal_rule(poly, start, end, segments);
        for k in 2..=iter + 1 {
            let j = 2 + iter - k;
            let p = 4_usize.pow((k as u32) - 1) as f64;

            romberg_table[j][k] =
                (p * romberg_table[j + 1][k - 1] - romberg_table[j][k - 1]) / (p - 1.0);
        }
        let approx_err = ((romberg_table[1][iter + 1] - romberg_table[2][iter]).abs()
            / romberg_table[1][iter + 1])
            .abs()
            * 100.0;
        if iter >= maxiter || approx_err <= tolerance {
            break;
        }
    }
    romberg_table[1][iter + 1]
}

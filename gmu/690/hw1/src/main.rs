use plotters::prelude::*;

const PI: f64 = 3.14159265359;

fn main() {
    let (t, v_array, rf) = droplet_num_sol(0.1, 0.0, 10.0, 3.0, 0.25);
    println!("The radius of the droplet at the final time of {t} minutes is {rf:.4}mm^2.");
    println!("the droplet volume at each time point is {:?}", v_array);
    if let Err(e) = plot_points(&v_array) {
        eprintln!("Plot failed: {}", e);
    };
}

fn droplet_num_sol(k: f64, ti: f64, tf: f64, ri: f64, dt: f64) -> (f64, Vec<(f64, f64)>, f64) {
    // k = evaporation rate, ri = initial radius
    // ti = initial time, tf = final time, dt = step size
    // a = surface area
    // Equation = dv/dt = -kA
    // volume of hemisphere = v = 2/3 * pi * r^3
    // volume in terms of r = cuberoot((v*3)/(2*pi))
    // Surface area of hemisphere= a = 2 * pi * r^2
    let mut t = ti;
    let mut r = ri;
    let mut v = (2.0 / 3.0) * PI * r * r * r;
    let mut a = 2.0 * PI * r * r;
    let n = (tf - ti) / dt;

    let mut v_array = Vec::new();
    // Add initial conditions to vector
    v_array.push((t, v));
    // Calculate remaining volumes
    for _ in 1..=n as usize {
        let dvdt = -k * a;
        v = v + dvdt * dt;
        t = t + dt;
        r = ((3.0 * v) / (2.0 * PI)).cbrt();
        a = 2.0 * PI * r * r;
        if v <= 0.0 {
            v_array.push((t, 0.0));
            break;
        };
        v_array.push((t, v));
    }
    let time_rem = ((tf - t) / dt) as usize;
    // Prevents unnecessary calculations when droplet is gone
    if time_rem > 0 {
        for _ in 1..=time_rem {
            t += dt;
            v_array.push((t, 0.0));
        }
    }

    (t, v_array, r)
}

fn plot_points(droplet_area: &Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = droplet_area
        .iter()
        .map(|p| p.0)
        .fold(f64::INFINITY, f64::min);
    let x_max = droplet_area
        .iter()
        .map(|p| p.0)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = droplet_area
        .iter()
        .map(|p| p.1)
        .fold(f64::INFINITY, f64::min);
    let y_max = droplet_area
        .iter()
        .map(|p| p.1)
        .fold(f64::NEG_INFINITY, f64::max);

    let root = BitMapBackend::new("plotters-doc-data/droplet_area_10_min.png", (1280, 960))
        .into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 40, 110);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Droplet area over time", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .y_labels(10)
        .label_style(("sans-serif", 25))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(droplet_area.clone(), &RED))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        droplet_area.clone(),
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled());
        },
    ))?;
    root.present()?;
    Ok(())
}

use plotters::prelude::*;

const PI: f64 = 3.14159265359;

fn main() {
    let initial_radius = 3.0; // mm
    let evaporation_rate = 0.1; // mm/min
    let initial_time = 0.0; // mins
    let final_time = 10.0; //mins
    let time_step_size = 0.25; // mins
    let (t, v_array, rf) = droplet_num_sol(
        evaporation_rate,
        initial_time,
        final_time,
        initial_radius,
        time_step_size,
    );
    println!("The radius of the droplet at the final time of {t} minutes is {rf}mm^2.");
    println!("the droplet volume at each time step is {:?}", v_array);
    if let Err(e) = plot_points(&v_array, final_time) {
        eprintln!("Plot failed: {}", e);
    };
}

fn droplet_num_sol(k: f64, ti: f64, tf: f64, ri: f64, dt: f64) -> (f64, Vec<(f64, f64)>, f64) {
    // a = surface area
    // Given Equation = dv/dt = -kA
    // Equation for volume of hemisphere = v = 2/3 * pi * r^3
    // volume in terms of r = cuberoot((v*3)/(2*pi))
    // Equation for surface area of hemisphere= a = 2 * pi * r^2
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
            r = 0.0;
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

fn plot_points(droplet_area: &Vec<(f64, f64)>, tf: f64) -> Result<(), Box<dyn std::error::Error>> {
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

    let file_folder = "plotters-doc-data";
    let file_name = format!("droplet_area_{tf}_min");
    let path_name = format!(
        "{}/{}/{}.png",
        env!("CARGO_MANIFEST_DIR"), // project root (folder where Cargo.toml is located)
        file_folder,
        file_name
    );
    let path_name_ref: &str = &path_name;
    let root = BitMapBackend::new(path_name_ref, (1280, 960)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 40, 50);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Droplet area over time", ("sans-serif", 40).into_font())
        .margin(10)
        // Set the size of the label region
        .x_label_area_size(60)
        .y_label_area_size(80)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .x_desc("Time (min)")
        .y_labels(10)
        .y_desc("Volume (mm3)")
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

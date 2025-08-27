use plotters::prelude::*;

const G: f64 = 9.8;

fn main() {
    let (t, v_array) = numerical_solution(0.64, 23.0, 0.0, 10.0, 15.0, 1.0);
    println!(
        "final time is {t} and the list of droplet_area is {:?}",
        v_array
    );
    let _ = plot_points(&v_array);
}

fn numerical_solution(
    c: f64,
    m: f64,
    ti: f64,
    tf: f64,
    vi: f64,
    dt: f64,
) -> (f64, Vec<(f64, f64)>) {
    let mut t = ti;
    let mut v = vi;
    let n = (tf - ti) / dt;

    let mut v_array = Vec::new();
    for _ in 1..=n as usize {
        let dvdt = G - c * v / m;
        v = v + dvdt * dt;
        t = t + dt;
        v_array.push((t, v));
    }
    (t, v_array)
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

    let root =
        BitMapBackend::new("plotters-doc-data/droplet_area.png", (1280, 960)).into_drawing_area();
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
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:.2}, {:.2}", c.0, c.1), (15, -10), ("sans-serif", 20).into_font());
        },
    ))?;
    root.present()?;
    Ok(())
}

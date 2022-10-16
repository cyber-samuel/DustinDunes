mod manifold;
use plotters::prelude::*;
use std::collections::HashSet;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/3d-plot.gif";
const X_FLOAT_BOUNDARY: f64 = 5.0;
const Y_FLOAT_BOUNDARY: f64 = 2.0;
const Z_FLOAT_BOUNDARY: f64 = 5.0;

pub fn mock_manifold_function(x: f64, z: f64) -> f64 {
    (1.0 / (x + 2.0)).cos()
        + (1.0 / (z + 2.0)).sin()
        + (f64::powf(
            std::f64::consts::E,
            -((f64::powi(x, 2)) * (f64::powi(z, 2))),
        ) / 2.0)
        + (f64::powf(
            std::f64::consts::E,
            -(f64::powi(x - 4.0, 2) * f64::powi(z - 1.0, 2)),
        ) / 4.0)
        + (f64::powi(
            f64::powf(
                std::f64::consts::E,
                -(f64::powi(x - 1.0, 2) * f64::powi(z - 2.0, 2)),
            ) / 2.0,
            2,
        ))
        + (f64::powf(std::f64::consts::E, -(f64::powi(x, 2) * f64::powi(z, 2))) / 2.0) / 8.0
        + (z / 8.0)
        - 1.5
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let drone_placements_on_manifold = get_manifold_data();

    // then do geodesic voronoi tessallation

    draw_gif(drone_placements_on_manifold);
    Ok(())
}

fn get_manifold_data() -> Vec<(u8, i64, f64)> {
    // get manifold
    let mut m = manifold::Manifold::new();
    for x in 0..50 {
        for y in 0..50 {
            m.update_point(
                x,
                y,
                mock_manifold_function(x as f64 / 10.0, y as f64 / 10.0),
            );
        }
    }

    // then get points
    let max = m.max();
    let min = m.min();
    let metric_dist: i64 = f64::sqrt(
        f64::powi(max.1 as f64 - min.1 as f64, 2) + f64::powi(max.2 as f64 - min.2 as f64, 2),
    )
    .round() as i64;
    let metric_dist_as_usize: usize = metric_dist.abs().try_into().unwrap();
    let ball_around_max = m.ball_around_x(max.1, max.2, metric_dist_as_usize);
    let ball_around_min = m.ball_around_x(min.1, min.2, metric_dist_as_usize);
    let mut final_intersection: Vec<(u8, i64, f64)> = Vec::new();
    final_intersection.push((max.1.try_into().unwrap(), max.2.try_into().unwrap(), max.0));
    final_intersection.push((min.1.try_into().unwrap(), min.2.try_into().unwrap(), min.0));

    let mut intersection_result: Vec<u8> = Vec::new();
    for (x, _y, _z) in ball_around_max.clone().into_iter() {
        intersection_result.push(x);
    }

    let mut intersection_res_min: Vec<u8> = Vec::new();
    for (x, _y, _z) in ball_around_min.into_iter() {
        intersection_res_min.push(x);
    }

    let temp_hash_set: HashSet<u8> = intersection_res_min.into_iter().collect();
    intersection_result = temp_hash_set
        .intersection(&intersection_result.into_iter().collect())
        .map(|i| *i)
        .collect::<Vec<_>>();

    for (x, y, z) in ball_around_max.into_iter() {
        if *intersection_result.get(0).unwrap() == x || *intersection_result.get(1).unwrap() == x {
            final_intersection.push((x, y, z));
        }
    }

    final_intersection
}

fn draw_gif(drone_data: Vec<(u8, i64, f64)>) {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)
        .unwrap()
        .into_drawing_area();

    for pitch in 0..30 {
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Topographical Example", ("sans-serif", 20))
            .build_cartesian_3d(
                0.0..X_FLOAT_BOUNDARY,
                0.0..Y_FLOAT_BOUNDARY,
                0.0..Z_FLOAT_BOUNDARY,
            )
            .unwrap();
        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(5)
            .draw()
            .unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (0..60).map(|f| f as f64 / 10.0),
                    (0..60).map(|f| f as f64 / 10.0),
                    |x, z| mock_manifold_function(x, z),
                )
                .style_func(&|&v| {
                    (&HSLColor(220.0 / 360.0 - 220.0 / 360.0 * v / 5.0, 1.0, 0.7)).into()
                }),
            )
            .unwrap();

        for coord in &drone_data {
            println!("Plotting Coord Data: {:?}", coord);
            chart
                .draw_series(
                    SurfaceSeries::xoz(
                        (((coord.0 as f64 / 10.0) as i32 - 1 as i32)
                            ..((coord.0 as f64 / 10.0) as i32 + 1 as i32))
                            .map(|v| v as f64),
                        (((coord.1 as f64 / 10.0) as i32 - 1 as i32)
                            ..((coord.1 as f64 / 10.0) as i32 + 1 as i32))
                            .map(|v| v as f64),
                        |x, z| mock_manifold_function(x as f64, z as f64) + 0.1,
                    )
                    .style(RED.mix(0.7)),
                )
                .unwrap();
        }

        root.present().unwrap();
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

#[test]
fn entry_point() {
    main().unwrap()
}

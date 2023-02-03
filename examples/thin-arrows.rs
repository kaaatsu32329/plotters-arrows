use std::f64::consts::TAU;
use std::path::Path;

use plotters::prelude::*;
use plotters_arrows::ThinArrow;

fn main() -> Result<(), DrawingAreaErrorKind<<BitMapBackend<'static> as DrawingBackend>::ErrorType>>
{
    let target = Path::new("example.png");
    let graph_root = BitMapBackend::new(target, (256, 256)).into_drawing_area();
    graph_root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&graph_root)
        .margin(8)
        .x_label_area_size(24)
        .y_label_area_size(24)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    let x_count = 16;
    let y_count = 16;
    let arrow_size = 0.05;
    chart.draw_series(
        (0..=x_count)
            .flat_map(|xi| (0..=y_count).map(move |yi| (xi, yi)))
            .map(|(xi, yi)| {
                let x = xi as f64 / x_count as f64;
                let y = yi as f64 / y_count as f64;
                let dx = arrow_size * f64::cos(y * TAU);
                let dy = arrow_size * f64::cos(x * TAU);
                ThinArrow::new((x, y), (x + dx, y + dy), &RED)
            }),
    )?;

    graph_root.present()?;

    Ok(())
}

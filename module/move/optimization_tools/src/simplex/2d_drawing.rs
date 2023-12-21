use plotters::
{
  backend::BitMapBackend,
  drawing::IntoDrawingArea,
  element::{ Circle, EmptyElement },
  series::{ LineSeries, PointSeries },
  style::
  {
    full_palette::{ BLACK, WHITE },
    Color, IntoFont, TextStyle,
  }, 
  chart::ChartBuilder
};

use super::{ Problem, ExtremePoint };

pub fn draw_problem
(
  problem : &Problem,
  extreme_points : Vec< ExtremePoint >,
) -> Result< (), Box< dyn std::error::Error > > 
{
    let root = BitMapBackend::new("2d.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("2d problem", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?;

    root.present()?;

  Ok( () )
}

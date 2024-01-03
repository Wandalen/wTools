use plotters::
{
  backend::BitMapBackend,
  drawing::IntoDrawingArea,
  element::{ Circle, EmptyElement },
  series::{ LineSeries, PointSeries, AreaSeries },
  style::
  {
    full_palette::{ BLACK, WHITE, RED },
    Color, IntoFont,
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
  let root = BitMapBackend::new( "./plot/2d.png", ( 640, 480 ) ).into_drawing_area();
  root.fill( &WHITE )?;
  let mut chart = ChartBuilder::on( &root )
      .caption( "2d problem", ( "sans-serif", 30 ).into_font() )
      .margin( 15 )
      .x_label_area_size( 40 )
      .y_label_area_size( 40 )
      .build_cartesian_2d( 0f32..20f32, 0f32..20f32 )?;

  chart.configure_mesh().draw()?;

  //constraints
  for constraint in &problem.constraints 
  {
    let mut series = Vec::new();
    
    let mut x = 0f32;
    let mut y = ( ( constraint.value - x as f64 * constraint.coefs[ 0 ] ) / constraint.coefs[ 1 ] ) as f32;
    series.push( ( x, y ) );
    y = 0f32;
    x = ( ( constraint.value - x as f64 * constraint.coefs[ 1 ] ) / constraint.coefs[ 0 ] ) as f32;

    series.push( ( x, y ) );

    chart.draw_series( LineSeries::new
      (
        series.iter().map( | ( x, y ) | ( *x, *y ) ),
        &BLACK,
      ) )?;

    chart.draw_series
    (
      AreaSeries::new
      (
        series.iter().map( | ( x, y ) | ( *x, *y ) ),
        0.0,
        RED.mix( 0.2 ),
      )
      .border_style( RED ),
    )?;
  }
    // extreme points
  chart.draw_series( PointSeries::of_element
  (
    extreme_points.into_iter().map( | p | ( p.point[ 0 ] as f32, p.point[ 1 ] as f32 ) ),
    2,
    &BLACK,
    &| c, s, _st | 
    {
      EmptyElement::at( ( c.0, c.1 ) ) + Circle::new
      (
        ( 0, 0 ),
        s,
        ( &BLACK ).filled(),
      )
    },
  ) )?;

  root.present()?;

  Ok( () )
}

//! Tools for graphical representation of two dimensional linear programming problem.
//! 

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
use std::{ env, path::{ PathBuf, Path }, process::Command };
use super::{ solver::ExtremePoint, linear_problem::Problem };

/// Get path of workspace or return current if fail to get path of workspace.
pub fn workspace_dir() -> PathBuf 
{
  let output = Command::new( env!( "CARGO" ) )
  .arg( "locate-project" )
  .arg( "--workspace" )
  .arg( "--message-format=plain" )
  .output()
  ;
  if let Ok( output ) = output
  {
    let path = output.stdout;
    let cargo_path = Path::new( std::str::from_utf8( &path ).unwrap().trim() );
    cargo_path.parent().unwrap().to_path_buf()
  }
  else 
  {
    std::env::current_dir().unwrap()
  }
}

/// Create plot with linear programming problem.
pub fn draw_problem
(
  problem : &Problem,
  extreme_points : Vec< ExtremePoint >,
  file_name : String,
) -> Result< (), Box< dyn std::error::Error > > 
{
  let dir_path = format!( "{}/target/plots", workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/{}.png", dir_path, file_name );
  let root = BitMapBackend::new( &path, ( 640, 480 ) ).into_drawing_area();
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

use plotters::
{
  drawing::IntoDrawingArea,
  series::LineSeries,
  style::full_palette::{ BLACK, WHITE },
  chart::ChartBuilder,
};
use iter_tools::Itertools;
use crate::plot::PLOTS;

use piston_window::{ EventLoop, PistonWindow };
mod plotters_backend;
pub use plotters_backend::draw_piston_window;

pub fn plot_dynamically
(
  window : &mut PistonWindow,
  name : &String,
) 
{
  window.set_ups( 60 );
  window.set_max_fps( 100 as u64 );

  let mut data = Vec::new();
  while let Some( _ ) = draw_piston_window( window, | b | 
  {
    let plots_opt = PLOTS.get();
  
    if let Some( plots ) = plots_opt
    {
      let plots = plots.lock().unwrap();
      
      if let Some( series ) = plots.series.get( name ) 
      {
        data = series.iter().map( | s | ( s.0, s.1 ) ).collect_vec();
      }
    }

    let root = b.into_drawing_area();
    root.fill( &WHITE )?;

    let max_x = data
    .iter()
    .map( | ( x, _ ) | *x )
    .max_by( | a, b | a.partial_cmp( b ).unwrap() )
    .unwrap_or( 10.0 )
    ;
  
    let min_x = data
    .iter()
    .map( | ( x, _ ) | *x )
    .min_by( | a, b | a.partial_cmp( b ).unwrap() )
    .unwrap_or( 0.0 )
    ;
  
    let max_y = data
    .iter()
    .map( | ( _, y ) | *y )
    .max_by( | a, b | a.partial_cmp( b ).unwrap() )
    .unwrap_or( 10.0 )
    ;
  
    let min_y = data
    .iter()
    .map( | ( _, y ) | *y )
    .min_by( | a, b | a.partial_cmp( b ).unwrap() )
    .unwrap_or( 0.0 )
    ;
  
    let x_spec = ( 0.0f32 ).min( min_x - 0.2 * min_x.abs() )..max_x + max_x.abs() * 0.2;
    let y_spec = ( 0.0f32 ).min( min_y - 0.2 * min_y.abs() )..max_y + max_y.abs() * 0.2;

    let mut cc = ChartBuilder::on( &root )
    .margin( 10 )
    .caption( name, ( "sans-serif", 30 ) )
    .x_label_area_size( 40 )
    .y_label_area_size( 50 )
    .build_cartesian_2d( x_spec.clone(), y_spec.clone() )?
    ;

    cc.configure_mesh()
    .x_desc( "Step" )
    .y_desc( "Cost" )
    .axis_desc_style( ( "sans-serif", 15 ) )
    .draw()?
    ;

    cc.draw_series( LineSeries::new
    (
      data.iter().map( | ( x, y ) | ( *x, *y ) ),
      &BLACK,
    ) )?;

    Ok( () )

  } ) {}
}


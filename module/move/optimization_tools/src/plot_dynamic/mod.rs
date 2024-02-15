//! Dynamic plotting of data series.
//! 

use plotters::
{
  drawing::IntoDrawingArea,
  series::LineSeries,
  style::full_palette::{ BLACK, WHITE },
  chart::ChartBuilder,
};
use crate::plot::PlotOptions;

use piston_window::{ EventLoop, PistonWindow };
mod plotters_backend;
pub use plotters_backend::draw_piston_window;

use std::sync::{ OnceLock, mpsc::{ Receiver, Sender } };

/// Struct that can be accessed in any place in code to add some data to draw plots.
pub static DPLOT : OnceLock< Sender< PlotOptions > > = OnceLock::new();

pub struct DynPlotter 
{
  rx : Receiver< PlotOptions >,
  window : PistonWindow,
}

pub fn init_dyn_plotter( name : String, width : u32, height : u32 ) -> DynPlotter 
{
  let ( tx,rx ) = std::sync::mpsc::channel::< PlotOptions >();
  _ = DPLOT.set( tx );

  let window = piston_window::WindowSettings::new( name, [ width, height ] )
  .samples( 1 )
  .exit_on_esc( true )
  .build()
  .unwrap()
  ;

  DynPlotter 
  {
    window,
    rx
  }
}

pub fn dyn_plot( options : PlotOptions ) 
{
  if let Some( tx ) = DPLOT.get() 
  {
    _ = tx.send( options );
  }
}

impl DynPlotter 
{
  pub fn plot_dynamically( &mut self ) 
  {

    self.window.set_ups( 100 );
    self.window.set_max_fps( 120 as u64 );

    let mut data = Vec::new();
    while let Some( _ ) = draw_piston_window( &mut self.window, | b | 
    {

      let root = b.into_drawing_area();
      root.fill( &WHITE )?;

      let max_x : f32 = data
      .iter()
      .map( | x : &( f32, f32 ) | x.0 )
      .max_by( | a : &f32, b : &f32 | a.partial_cmp( b ).unwrap() )
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
      .x_label_area_size( 40 )
      .y_label_area_size( 50 )
      .build_cartesian_2d( x_spec.clone(), y_spec.clone() )?
      ;

      for _ in 0..5 
      {
        if let Ok( msg ) = self.rx.recv() 
        { 
          data.push( ( msg.x, msg.y ) );

          cc.configure_mesh()
          .x_desc( msg.description.x_label )
          .y_desc( msg.description.y_label )
          .axis_desc_style( ( "sans-serif", 15 ) )
          .draw()?
          ;

          cc.draw_series( LineSeries::new
          (
            data.iter().map( | ( x, y ) | ( *x, *y ) ),
            &BLACK,
          ) )?;
        }
      }

      Ok( () )

    } ) {}
  }
}


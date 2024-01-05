//! Plotting of data series to png file.
//! 

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
use iter_tools::Itertools;
use std::{ sync::{ Mutex, OnceLock }, collections::HashMap };

/// Struct that can be accessed in any place in code to add some data to draw plots.
pub static PLOTS : OnceLock< Mutex< Plots > > = OnceLock::new();

/// Struct that aggregates data to plot with description about that data.
#[ derive( Debug ) ]
pub struct Plots 
{
  pub series : HashMap< String, Vec< ( f32, f32 ) > >,
  pub descriptions : HashMap< String, PlotDescription >,
}

impl Plots 
{
  /// Create new empty Plots struct.
  fn new() -> Self 
  {
    Self 
    {
      series : HashMap::new(),
      descriptions : HashMap::new(),
    }
  }

  /// Adds new series with data to plot, or extends existing series with provided data.
  fn add_data( &mut self, plot_options : PlotOptions ) 
  {
    self.series
    .entry( plot_options.name.clone() )
    .and_modify( | v | v.push( ( plot_options.x, plot_options.y ) ) )
    .or_insert( vec![ ( plot_options.x, plot_options.y ) ] )
    ;

    self.descriptions
    .entry( plot_options.name )
    .or_insert( plot_options.description )
    ;

  }
}

/// Represents new point of data to add to plot at a time.
#[ derive( Debug ) ]
pub struct PlotOptions 
{
  pub name : String,
  pub x : f32,
  pub y : f32,
  pub description : PlotDescription,
}

/// Fixed info about plot, that remains unchanged with every new added piece of data.
#[ derive( Debug ) ]
pub struct PlotDescription 
{
  pub x_label : String,
  pub y_label : String,
  pub filename : String,
  pub plot_line : bool,
  pub y_log_coords : bool,
}

/// Default values for description of plot.
impl Default for PlotDescription 
{
  fn default() -> Self 
  {
    Self 
    {
      x_label : String::new(),
      y_label : String::new(),
      filename : String::from( "plot" ),
      plot_line : true,
      y_log_coords : false,
    }
  }
}

/// Wraps adding new piece of data for plotting to static aggregator.
pub fn plot( plot_options : PlotOptions )
{
  PLOTS
  .get_or_init( | | Mutex::new( Plots::new() ) )
  .lock()
  .unwrap()
  .add_data(plot_options)
  ;
}

/// Performs drawing of plots from stored data. Must be called at the end of execution when data is fully gathered.
pub fn draw_plots() 
{
  let plots_opt = PLOTS.get();

  if let Some( plots ) = plots_opt
  {
    let mut plots = plots.lock().unwrap();
  
    if !plots.series.is_empty() 
    {
      for plot_name in plots.series.keys() 
      {
        plot_data
        (
          &plots.series[ plot_name ]
          .iter()
          .map( | s | ( s.0, s.1 ) )
          .collect_vec(),
          &plot_name,
          &plots.descriptions[ plot_name ],
        )
        .unwrap()
        ;
      }
    }
  
    plots.series.clear();
    plots.descriptions.clear();
  }

}

/// Create tagret files and directory.
pub fn dst_file_path( file_name : String ) -> Result< String, Box< dyn std::error::Error > > 
{
  use std::env;
  use std::fs;
  let current_dir = env::current_dir()?;
  let dir_path = &format!("{}/target/plots", current_dir.display());

  fs::create_dir_all( dir_path )?;
  let file_path = format!( "{dir_path}/{file_name}.png" );

  Ok( file_path )

}

/// Draw plot from given point series and plot description.
pub fn plot_data
(
  series : &Vec< ( f32, f32 ) >,
  name : &str,
  description : &PlotDescription,
) -> Result< (), Box< dyn std::error::Error > > 
{
  let dir_path = format!( "{}/target/plots", crate::simplex::drawing::workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/{}.png", dir_path, description.filename.clone() );
  let root = BitMapBackend::new( &path, ( 4000, 960 ) ).into_drawing_area();

  root.fill( &WHITE )?;
  let root = root.margin( 20, 20, 20, 20 );

  let max_x = series
  .iter()
  .map( | ( x, _ ) | *x )
  .max_by( | a, b | a.partial_cmp( b ).unwrap() )
  .unwrap()
  ;

  let min_x = series
  .iter()
  .map( | ( x, _ ) | *x )
  .min_by( | a, b | a.partial_cmp( b ).unwrap() )
  .unwrap()
  ;

  let max_y = series
  .iter()
  .map( | ( _, y ) | *y )
  .max_by( | a, b | a.partial_cmp( b ).unwrap() )
  .unwrap()
  ;

  let min_y = series
  .iter()
  .map( | ( _, y ) | *y )
  .min_by( | a, b | a.partial_cmp( b ).unwrap() )
  .unwrap()
  ;

  let x_spec = ( 0.0f32 ).min( min_x - 0.2 * min_x.abs() )..max_x + max_x.abs() * 0.2;
  let y_spec = ( 0.0f32 ).min( min_y - 0.2 * min_y.abs() )..max_y + max_y.abs() * 0.2;

  let mut chart = ChartBuilder::on( &root )
  .caption( name, ( "sans-serif", 30 ) )
  .x_label_area_size( 40 )
  .y_label_area_size( 60 )
  .build_cartesian_2d( x_spec, y_spec )?
  ;

  chart
  .configure_mesh()
  .x_label_style( TextStyle::from( ( "sans-serif", 15 ).into_font() ) )
  .axis_desc_style( TextStyle::from( ( "sans-serif", 18 ).into_font() ) )
  .y_label_style( TextStyle::from( ( "sans-serif", 15 ).into_font() ) )
  .x_label_formatter( &| x | format!( "{}", x ) )
  .x_desc( &description.x_label )
  .y_desc( &description.y_label )
  .draw()?
  ;

  chart.draw_series( PointSeries::of_element
  (
   series.iter().enumerate().map( | ( i, ( x, y ) ) | ( *x, *y, i ) ),
   1,
   &BLACK,
    &| c, s, _st | 
    {
    EmptyElement::at( ( c.0, c.1 ) )
    + Circle::new
    (
      ( 0, 0) ,
      s,
      ( &BLACK ).filled(),
    )
    },
  ))?
  ;

  if description.plot_line
  {
    chart.draw_series( LineSeries::new
      (
        series.iter().map( | ( x, y ) | ( *x, *y ) ),
        &BLACK,
      ) )?;
  }

  Ok( () )

}
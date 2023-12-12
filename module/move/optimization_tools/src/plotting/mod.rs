use plotters::{
    backend::BitMapBackend,
    chart::{ ChartBuilder, SeriesLabelPosition },
    coord::{
        combinators::IntoLogRange,
        ranged1d::{ IntoSegmentedCoord, SegmentValue },
    },
    drawing::IntoDrawingArea,
    element::{ Circle, EmptyElement, Rectangle },
    series::{ Histogram, LineSeries, PointSeries },
    style::{
        full_palette::{ BLACK, WHITE },
        AsRelative, Color, IntoFont, Palette, Palette99, RGBAColor, TextStyle, TRANSPARENT,
    },
};
use std::sync::{ Mutex, OnceLock };

pub fn dst_file_path(file_name: String) -> Result<String, Box<dyn std::error::Error>> 
{
    use std::env;
    use std::fs;

    let current_dir = env::current_dir()?;
    let dir_path = &format!( "./plots", current_dir.display() );
    fs::create_dir_all( dir_path )?;
    let file_path = format!( "{dir_path}/{file_name}.png" );

    Ok( file_path )

}

pub fn plot_data(
    series: &Vec< ( f32, f32 ) >,
    legend: &Vec< Option< String > >,
    name: &str,
) -> Result< (), Box< dyn std::error::Error > > 
{
    let file_path = dst_file_path( "test" )?;
    let root = BitMapBackend::new( &file_path, ( 1200, 960 ) ).into_drawing_area();

    root.fill( &WHITE )?;
    let mut root = root.margin( 20, 20, 20, 20 );

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
    let _x_span = max_x - min_x;
    let _y_span = max_y - min_y;

    Ok( () )

}

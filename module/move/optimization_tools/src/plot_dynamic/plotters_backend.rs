//! Contains piston_window backend for plotters crate.
//! 

use piston_window::context::Context;
use piston_window::ellipse::circle;
use piston_window::{ circle_arc, ellipse, line, rectangle, Event, Loop };
use piston_window::{ G2d, PistonWindow };

use plotters_backend::
{
  BackendColor, BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind,
};

/// Error type for plotters backend.
#[ derive( Debug ) ]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError 
{
  fn fmt( &self, fmt : &mut std::fmt::Formatter ) -> std::fmt::Result 
  {
    write!( fmt, "{:?}", self )
  }
}

impl std::error::Error for DummyBackendError {}

/// Represents plotters backend structure configuration.
pub struct PistonBackend< 'a, 'b > 
{
  size : ( u32, u32 ),
  scale : f64,
  context : Context,
  graphics : &'b mut G2d< 'a >,
}

/// Convert plotters color to array format.
fn make_piston_rgba( color : &BackendColor ) -> [ f32; 4 ] 
{
  let ( r, g, b ) = color.rgb;
  let a = color.alpha;

  [
    r as f32 / 255.0,
    g as f32 / 255.0,
    b as f32 / 255.0,
    a as f32,
  ]
}

/// Implements scaling of pair of points.
fn make_point_pair( a : BackendCoord, b : BackendCoord, scale : f64 ) -> [ f64; 4 ] 
{
  [
    a.0 as f64 * scale,
    a.1 as f64 * scale,
    b.0 as f64 * scale,
    b.1 as f64 * scale,
  ]
}

impl< 'a, 'b > PistonBackend< 'a, 'b > 
{
  /// Create new instance of PistonBackend.
  pub fn new( size : ( u32, u32 ), scale : f64, context : Context, graphics : &'b mut G2d< 'a > ) -> Self 
  {
    Self 
    {
      size,
      context,
      graphics,
      scale,
    }
  }
}

/// Implementation of plotters Backend trait for custom piston backend struct.
impl< 'a, 'b > DrawingBackend for PistonBackend< 'a, 'b > 
{
  type ErrorType = DummyBackendError;

  /// Get size of drawing area.
  fn get_size( &self ) -> ( u32, u32 ) 
  {
    self.size
  }

  /// Checks if drawing can start.
  fn ensure_prepared( &mut self ) -> Result< (), DrawingErrorKind< DummyBackendError > > 
  {
    Ok( () )
  }

  /// Checks if drawing is performed.
  fn present( &mut self ) -> Result< (), DrawingErrorKind< DummyBackendError > > 
  {
    Ok( () )
  }

  /// Draw one colored point.
  fn draw_pixel
  (
    &mut self,
    point : BackendCoord,
    color : BackendColor,
  ) -> Result< (), DrawingErrorKind< Self::ErrorType > > 
  {
    piston_window::rectangle
    (
    make_piston_rgba( &color ),
    make_point_pair( point, ( 1, 1 ), self.scale ),
    self.context.transform,
    self.graphics,
    );

    Ok( () )
  }

  /// Draw line by given coordinates and style.
  fn draw_line< S : BackendStyle >
  (
    &mut self,
    from : BackendCoord,
    to : BackendCoord,
    style : &S,
  ) -> Result<(), DrawingErrorKind< Self::ErrorType > > 
  {
    line
    (
      make_piston_rgba( &style.color() ),
      self.scale,
      make_point_pair( from, to, self.scale ),
      self.context.transform,
      self.graphics,
    );

    Ok( () )
  }

  /// Draw rectangle by given two corners and style.
  fn draw_rect< S : BackendStyle >
  (
    &mut self,
    upper_left : BackendCoord,
    bottom_right : BackendCoord,
    style : &S,
    fill : bool,
  ) -> Result< (), DrawingErrorKind< Self::ErrorType > > 
  {
    if fill 
    {
      rectangle
      (
        make_piston_rgba( &style.color() ),
        make_point_pair
        (
          upper_left,
          ( bottom_right.0 - upper_left.0, bottom_right.1 - upper_left.1 ),
          self.scale,
        ),
        self.context.transform,
        self.graphics,
      );
    } 
    else 
    {
      let color = make_piston_rgba( &style.color() );
      let [ x0, y0, x1, y1 ] = make_point_pair( upper_left, bottom_right, self.scale );
      line
      (
        color,
        self.scale,
        [ x0, y0, x0, y1 ],
        self.context.transform,
        self.graphics,
      );
      line
      (
        color,
        self.scale,
        [ x0, y1, x1, y1 ],
        self.context.transform,
        self.graphics,
      );
      line
      (
        color,
        self.scale,
        [ x1, y1, x1, y0 ],
        self.context.transform,
        self.graphics,
      );
      line
      (
        color,
        self.scale,
        [ x1, y0, x0, y0 ],
        self.context.transform,
        self.graphics,
      );
    }
    
    Ok( () )
  }

  /// Draw circle by given center coordinates, radius and style.
  fn draw_circle< S : BackendStyle >
  (
    &mut self,
    center : BackendCoord,
    radius : u32,
    style : &S,
    fill : bool,
  ) -> Result< (), DrawingErrorKind< Self::ErrorType > > 
  {
    let rect = circle( center.0 as f64, center.1 as f64, radius as f64 );
    if fill 
    {
      ellipse
      (
        make_piston_rgba( &style.color() ),
        rect,
        self.context.transform,
        self.graphics,
      );
    } 
    else 
    {
      circle_arc
      (
        make_piston_rgba( &style.color() ),
        self.scale,
        std::f64::consts::PI,
        0.0,
        rect,
        self.context.transform,
        self.graphics,
      );
      circle_arc
      (
        make_piston_rgba( &style.color() ),
        self.scale,
        0.0,
        std::f64::consts::PI,
        rect,
        self.context.transform,
        self.graphics,
      );
    }
    Ok( () )

  }
}

/// General drawing method.
pub fn draw_piston_window< F : FnOnce( PistonBackend ) -> Result< (), Box< dyn std::error::Error > > >
(
  window : &mut PistonWindow,
  draw : F,
) -> Option< Event > 
{
  if let Some( event ) = window.next() 
  {
    window.draw_2d( &event, | c, g, _ | match event 
    {
      Event::Loop( Loop::Render( arg ) ) => 
      {
        draw( PistonBackend::new
        (
          ( arg.draw_size[ 0 ], arg.draw_size[ 1 ] ),
          arg.window_size[ 0 ] / arg.draw_size[ 0 ] as f64,
          c,
          g,
        ) )
        .ok()
        ;
      }
      _ => {}
    });
    return Some( event );
  }
  None
}
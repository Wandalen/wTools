/// Internal namespace.
mod private
{
  use crate::own::*;

  /// StrokeBrush.
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrush
  {
    pub( crate ) id : Id,
    pub( crate ) color : Rgba,
    pub( crate ) width : f32,
  }

  impl Default for StrokeBrush
  {
    fn default() -> Self
    {
      let id = Id::new::< Self >();
      let color = Default::default();
      let width = 1.0;
      Self { id, color, width }
    }
  }

  impl StrokeBrush
  {

    /// Constructor.
    pub fn new() -> Self
    {
      Default::default()
    }

    /// ChangeInterface color.
    #[ inline ]
    pub fn color< Color >( mut self, val : Color ) -> Self
    where
      Color : RgbaInterface< f32 >,
    {
      self.color = val.into_rgba();
      self
    }

    /// ChangeInterface color.
    #[ inline ]
    pub fn width( mut self, val : f32 ) -> Self
    {
      self.width = val;
      self
    }

  }

  impl HasIdInterface for StrokeBrush
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.id
    }
  }

}

crate::mod_interface!
{
  exposed use StrokeBrush;

  /// ChangerInterface of brush stroke.
  layer changer;
  /// ChangeInterface of brush stroke constructor.
  layer change_new;
  /// ChangeInterface of brush stroke to change color.
  layer change_color;
  /// ChangeInterface of brush stroke to change width.
  layer change_width;

}

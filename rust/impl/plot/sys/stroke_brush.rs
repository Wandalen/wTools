/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// StrokeBrush.
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrush
  {
    pub( crate ) id : Id,
    pub( crate ) color : Rgba,
  }

  impl StrokeBrush
  {
    /// Constructor.
    pub fn new() -> Self
    {
      let id = Id::new::< Self >();
      let color = Default::default();
      Self
      {
        color,
        id,
      }
    }
    /// ChangeInterface color.
    #[ inline ]
    pub fn color< Color >( mut self, color : Color ) -> Self
    where
      Color : RgbaInterface< f32 >,
    {
      self.color = color.into_rgba();
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
}

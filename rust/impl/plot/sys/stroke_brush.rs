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

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    private::StrokeBrush,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}

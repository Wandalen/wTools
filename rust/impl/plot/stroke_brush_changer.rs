/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  // use once_cell::sync::Lazy;
  // use std::sync::Mutex;
  // use dashmap::DashMap;
  // use std::sync::Arc;

  /// Changer of brush stroke.
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrushChanger
  {
    pub( crate ) id : Id,
    pub( crate ) context_changer : ContextChanger,
  }

  impl StrokeBrushChanger
  {
    /// Constructor.
    pub( crate ) fn _new( mut context_changer : ContextChanger ) -> Self
    {
      let id = &mut context_changer.stroke;
      if id.is_none()
      {
        *id = Some( Id::new::< StrokeBrush >() );
      }
      let id = id.take().unwrap();
      Self
      {
        id,
        context_changer,
      }
    }
  }

  impl Changer for StrokeBrushChanger
  {
  }

  impl HasIdInterface for StrokeBrushChanger
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
    private::StrokeBrushChanger,
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

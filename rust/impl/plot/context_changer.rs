/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  // use once_cell::sync::Lazy;
  // use std::sync::Mutex;
  // use dashmap::DashMap;
  // use std::sync::Arc;

  /// Context.
  #[ derive( Debug, Clone ) ]
  pub struct ContextChanger
  // where
  //   Box< dyn Change > : Clone,
  {
    pub( crate ) id : Id,
    pub( crate ) stroke : Option< Id >,
    // pub( crate ) changes : Vec< Box< dyn Change > >,
  }

  // impl< T > Clone for Generate< T >
  // {
  //   fn clone( &self ) -> Self
  //   {
  //     *self
  //   }
  // }

  impl ContextChanger
  {
    /// Constructor.
    pub( crate ) fn _new( id : Id ) -> Self
    {
      // let changes = Vec::new();
      let stroke = None;
      Self
      {
        id,
        stroke,
        // changes,
      }
    }
    /// Parameters of stroke.
    pub fn stroke( self ) -> StrokeBrushChanger
    {
      let changer = StrokeBrushChanger::_new( self );
      changer
    }
  }

  impl Changer for ContextChanger
  {
  }

  impl HasIdInterface for ContextChanger
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
    private::ContextChanger,
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

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use crate::abs::*;

  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use std::sync::Arc;

  /// Context.
  #[ derive( Debug, Clone ) ]
  pub struct Context
  {
    id : Id,
    stroke : Option< StrokeBrush >,
    drawing : Option< Drawing >,
  }

  impl Context
  {
  }

  impl Make0 for Context
  {
    fn make_0() -> Self
    {
      let id = Id::new::< Self >();
      let stroke = None;
      let drawing = None;
      Self
      {
        id,
        stroke,
        drawing,
      }
    }
  }

  impl ContextInterface for Context
  {

    type Changer = ContextChanger;

    fn changer( &mut self ) -> Self::Changer
    {
      let id = self.id();
      let stroke = self.stroke.as_ref().map( | stroke | stroke.id() );
      let drawing = self.drawing.as_ref().map( | drawing | drawing.id() );
      let changes = Vec::new();
      ContextChanger
      {
        id,
        stroke,
        drawing,
        changes,
      }
    }

  }

  impl HasIdInterface for Context
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.id
    }
  }

  /// Registry of contexts.
  pub static mut REGISTRY : Lazy< Arc< Mutex< Registry< Context > > > > = Registry::< Context >::new();

  /// Get current context.
  pub fn current() -> ContextChanger
  {
    // Safety : under mutex.
    unsafe
    {
      Registry::< Context >::current( &mut REGISTRY )
    }
  }

  // /// Obtain a new context.
  // pub fn obtain() -> ContextChanger
  // {
  //   // Safety : under mutex.
  //   unsafe
  //   {
  //     Registry::< Context >::obtain( &mut REGISTRY )
  //   }
  // }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
    private::REGISTRY,
    private::current,
    // private::obtain,
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
    private::Context,
    private::current as context,
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

/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
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

  impl From_0 for Context
  {
    fn from_0() -> Self
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

}

crate::mod_interface!
{
  protected use { REGISTRY, current };
  exposed use { Context, current as context };
}

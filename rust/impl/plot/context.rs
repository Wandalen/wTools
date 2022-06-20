/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use dashmap::DashMap;

  /// Registry of contexts.
  #[ derive( Debug ) ]
  pub struct Registry
  {
    contexts : DashMap< Id, Context >,
    contexts_with_name : DashMap< String, Id >,
    current_context_name : Option< String >,
  }

  static mut REGISTRY : Lazy< Mutex< Registry > > = Lazy::new( ||
  {
    let contexts = DashMap::new();
    let contexts_with_name = DashMap::new();
    let current_context_name = None;
    Mutex::new( Registry
    {
      contexts,
      contexts_with_name,
      current_context_name,
    })
  });

  /// Context.
  #[ derive( Debug, Clone ) ]
  pub struct Context
  {
    id : Id,
    stroke : Option< StrokeBrush >,
  }

  impl Context
  {
    /// Constructor.
    fn _new() -> Self
    {
      let id = Id::new::< Self >();
      let stroke = None;
      Self
      {
        id,
        stroke,
      }
    }
    /// Parameters of stroke.
    pub fn stroke( &mut self ) -> StrokeBrush
    {
      if self.stroke.is_none()
      {
        self.stroke = Some( StrokeBrush::new() );
      }
      self.stroke.as_ref().unwrap().clone()
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

  /// Get current context or crate a new one if it does not exist.
  pub fn current() -> Context
  {
    unsafe
    {
      let registry = REGISTRY.lock().unwrap();
      let current_name : Option< String > = registry.current_context_name.clone();
      /* xxx : redo */
      if current_name.is_none()
      {
        drop( registry );
        make()
      }
      else
      {
        let id = *registry.contexts_with_name.get( &current_name.unwrap() ).unwrap().value();
        ( *registry.contexts.get( &id ).unwrap().value() ).clone()
      }
    }
  }

  /// Construct a new context.
  pub fn make() -> Context
  {
    unsafe
    {
      let registry = REGISTRY.lock().unwrap();
      let mut current_name : Option< String > = registry.current_context_name.clone();
      if current_name.is_none()
      {
        current_name = Some( "default".into() )
      }
      let current_name = current_name.unwrap();
      if registry.contexts_with_name.contains_key( &current_name )
      {
        let id = *registry.contexts_with_name.get( &current_name ).unwrap().value();
        ( *registry.contexts.get( &id ).unwrap().value() ).clone()
      }
      else
      {
        let context = Context::_new();
        let id = context.id();
        registry.contexts_with_name.insert( current_name.clone(), context.id() );
        registry.contexts.insert( id, context );
        ( *registry.contexts.get( &id ).unwrap().value() ).clone()
      }
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
    private::current,
    private::make,
    private:: Registry,
    private::Context,
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

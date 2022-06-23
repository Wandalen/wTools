/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use dashmap::DashMap;
  use std::sync::Arc;

  /// Registry of contexts.
  #[ derive( Debug ) ]
  pub struct Registry
  {
    contexts : DashMap< Id, Context >,
    contexts_with_name : DashMap< String, Id >,
    current_context_name : Option< String >,
  }

  static mut REGISTRY : Lazy< Arc< Mutex< Registry > > > = Lazy::new( ||
  {
    let contexts = DashMap::new();
    let contexts_with_name = DashMap::new();
    let current_context_name = None;
    Arc::new( Mutex::new( Registry
    {
      contexts,
      contexts_with_name,
      current_context_name,
    }))
  });

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
        obtain()
      }
      else
      {
        let id = *registry.contexts_with_name.get( &current_name.unwrap() ).unwrap().value();
        registry.contexts.get( &id ).unwrap().value().clone()
      }
    }
  }

  /// Construct a new context.
  pub fn obtain() -> Context
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
        registry.contexts.get( &id ).unwrap().value().clone()
      }
      else
      {
        let context = Context::_new();
        let id = context.id();
        registry.contexts_with_name.insert( current_name.clone(), context.id() );
        registry.contexts.insert( id, context );
        registry.contexts.get( &id ).unwrap().value().clone()
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
    private::obtain,
    private:: Registry,
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

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use crate::abs::*;
  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use dashmap::DashMap;
  use std::sync::Arc;

  /// Registry of contexts.
  #[ derive( Debug ) ]
  pub struct Registry< Context >
  where
    Context : ContextInterface,
  {
    contexts : DashMap< Id, Context >,
    contexts_with_name : DashMap< String, Id >,
    current_context_name : Option< String >,
  }

  impl< Context > Registry< Context >
  where
    Context : ContextInterface,
  {

    /// Static constructor.
    pub const fn new() -> Lazy< Arc< Mutex< Registry< Context > > > >
    {
      Lazy::new( ||
      {
        let contexts = DashMap::new();
        let contexts_with_name = DashMap::new();
        let current_context_name = None;
        Arc::new( Mutex::new( Registry::< Context >
        {
          contexts,
          contexts_with_name,
          current_context_name,
        }))
      })
    }

    /// Construct a new context.
    pub fn current( _registry : &mut Lazy< Arc< Mutex< Registry< Context > > > >  ) -> Context::Changer
    {
      let registry = _registry.lock().unwrap();
      let mut current_name : Option< String > = registry.current_context_name.clone();
      if current_name.is_none()
      {
        current_name = Some( "default".into() )
      }
      let current_name = current_name.unwrap();
      if registry.contexts_with_name.contains_key( &current_name )
      {
        let id = *registry.contexts_with_name.get( &current_name ).unwrap().value();
        registry.contexts.get_mut( &id ).unwrap().value_mut().changer()
      }
      else
      {
        let context : Context = make!();
        let id = context.id();
        registry.contexts_with_name.insert( current_name.clone(), context.id() );
        registry.contexts.insert( id, context );
        registry.contexts.get_mut( &id ).unwrap().value_mut().changer()
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
    // private::current,
    // private::obtain,
    // private:: Registry,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::
  {
    exposed::*,
    private:: Registry,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    // private::current as context,
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

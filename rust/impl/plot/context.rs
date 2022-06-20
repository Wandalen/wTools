/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;
  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use dashmap::DashMap;

  static mut CONTEXTS : Lazy< DashMap< String, Context > > = Lazy::new( ||
  {
    DashMap::new()
  });

  static mut CURRENT_CONTEXT : Lazy< Mutex< Option< String > > > = Lazy::new( ||
  {
    Mutex::new( None )
  });

  /// Reference on context.
  #[ derive( Debug, Clone, Copy ) ]
  pub struct ContextId
  {
    #[ allow( dead_code ) ]
    tp_id : i32,
    #[ allow( dead_code ) ]
    in_id : i32,
  }

  /// Context.
  #[ derive( Debug, Clone ) ]
  pub struct Context
  {
    id : ContextId,
  }

  impl Context
  {
    /// Constructor.
    fn _new() -> Self
    {
      let id = ContextId { tp_id : 1, in_id : 1 };
      Self
      {
        id
      }
    }
    /// Get id.
    pub fn id( &self ) -> ContextId
    {
      self.id
    }
  }

  /// Get current context or crate a new one if it does not exist.
  pub fn current() -> Context
  {
    unsafe
    {
      let current_name : Option< String > = CURRENT_CONTEXT.lock().unwrap().clone();
      if current_name.is_none()
      {
        make()
      }
      else
      {
        ( *CONTEXTS.get( &current_name.unwrap() ).unwrap().value() ).clone()
      }
    }
  }

  /// Construct a new context.
  pub fn make() -> Context
  {
    unsafe
    {
      let mut current_name : Option< String > = CURRENT_CONTEXT.lock().unwrap().clone();
      if current_name.is_none()
      {
        current_name = Some( "default".into() )
      }
      let current_name = current_name.unwrap();
      if CONTEXTS.contains_key( &current_name )
      {
        ( *CONTEXTS.get( &current_name ).unwrap().value() ).clone()
      }
      else
      {
        let context = Context::_new();
        CONTEXTS.insert( current_name.clone(), context );
        ( *CONTEXTS.get( &current_name ).unwrap().value() ).clone()
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

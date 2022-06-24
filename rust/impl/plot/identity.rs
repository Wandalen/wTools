/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use once_cell::sync::Lazy; /* xxx : move to wtools */
  use std::sync::Mutex;
  use core::hash::Hash;
  // use core::any::TypeId;

  static mut COUNTER : Lazy< Mutex< i64 > > = Lazy::new( ||
  {
    Mutex::new( 0 )
  });

  /// ID interface.
  pub trait IdInterface
  where
    Self :
      fmt::Debug +
      Clone +
      Copy +
      PartialEq +
      Eq +
      Hash +
    ,
  {
    // fn tp_id() -> i32;
    // fn in_id() -> i32;
  }

  /// Has id.
  pub trait HasIdInterface
  where
    Self :
      fmt::Debug +
  {
    /// Get id.
    fn id( &self ) -> Id;
  }

  /// Reference on context.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
  pub struct Id
  {
    // #[ allow( dead_code ) ]
    // tp_id : core::any::TypeId,
    #[ allow( dead_code ) ]
    in_id : i64,
  }

  impl Id
  {
    /// Construct a new id increasing counter.
    pub fn new< T >() -> Self
    where
      T : core::any::Any,
    {
      // SAFETY : mutex guard it
      let mut c = unsafe { COUNTER.lock().unwrap() };
      *c += 1;
      Self
      {
        // tp_id : TypeId::of::< T >(),
        in_id : *c,
      }
    }
  }

  impl IdInterface for Id
  // where
  //   T : core::any::Any,
  {
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
    private::Id,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    IdInterface,
    HasIdInterface,
  };
}

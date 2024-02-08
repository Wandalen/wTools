/// Internal namespace.
#[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{
  // use crate::protected::*;
  use once_cell::sync::Lazy;
  use std::sync::Mutex;
  use core::{hash::Hash, fmt};
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
  #[ derive( Clone, Copy, PartialEq, Eq, Hash ) ]
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
        in_id : *c,
      }
    }
  }

  impl IdInterface for Id
  {
  }

  impl fmt::Debug for Id
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "id::{:?}", self.in_id ) )
    }
  }

}

#[ cfg( not( feature = "no_std" ) ) ]
::meta_tools::mod_interface!
{

  exposed use Id;
  prelude use { IdInterface, HasIdInterface };

}

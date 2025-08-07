use crate::abs::{ChangerInterface, HasIdInterface};
use std::any::Any;
use std::sync::{ Arc, Mutex };

use super::identity::Id;
use super::registry::Registry;
use lazy_static::lazy_static;

/// Interface to describe system.
pub trait ContextInterface : Send + Sync
{
  /// Get id.
  fn id( &self ) -> Id;
  /// Get changer.
  fn changer( &self ) -> Box< dyn ChangerInterface >;
  /// Get root.
  fn root( &self ) -> &dyn Any;
}

impl dyn ContextInterface
{
  /// Downcast to concrete type.
  pub fn downcast_ref< T : Any >( &self ) -> Option< &T >
  {
    self.root().downcast_ref()
  }
}

lazy_static!
{
  static ref COUNTER : Mutex< i32 > = Mutex::new( 0 );
}

impl Registry< dyn ContextInterface >
{
  /// Current.
  pub fn current< Context : ContextInterface >
  (
    _registry : &mut lazy_static::Lazy< Arc< Mutex< Registry< Context > > > >
  )
  -> Context::Changer
  {
    let mut c = unsafe { COUNTER.lock().unwrap() };
    *c += 1;
    println!( "Counter : {}", c );
    todo!( "Implement" )
  }
}

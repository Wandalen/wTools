use super :: *;
use std ::any ::Any;
use std ::sync ::Mutex;
use lazy_static ::lazy_static;

/// Interface to describe identity.
pub trait HasIdInterface: Send + Sync
{
  /// Get id.
  fn id( &self ) -> Id;
  /// Get root.
  fn root( &self ) -> &dyn Any;
}

impl dyn HasIdInterface
{
  /// Downcast to concrete type.
  pub fn downcast_ref< T: Any >( &self ) -> Option< &T >
  {
  self.root().downcast_ref()
 }
}

/// Id of resource.
#[ derive( Debug, Copy, Clone, PartialEq, Eq, Hash ) ]
pub struct Id( pub i32 );

impl Id
{
  /// Generate new id.
  pub fn next() -> Self
  {
  let mut c = unsafe { COUNTER.lock().unwrap() };
  *c += 1;
  Id( *c )
 }
}

lazy_static!
{
  static ref COUNTER: Mutex< i32 > = Mutex ::new( 0 );
}

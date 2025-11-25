use crate ::abs ::identity ::Id;
use super :: *;
use std ::any ::Any;
use std ::sync :: { Arc, Mutex };
use lazy_static ::lazy_static;

use super ::context ::ContextInterface;

/// Interface to describe registry.
#[ allow( missing_docs ) ]
pub struct Registry< Context >
{
  pub root: Arc< dyn Any + Send + Sync >,
  pub current: i32,
  phantom: std ::marker ::PhantomData< Context >,
}

impl< Context > Registry< Context >
{
  /// Constructor.
  pub fn new( root: Arc< dyn Any + Send + Sync > ) -> Self
  {
  Self
  {
   root,
   current: 0,
   phantom: std ::marker ::PhantomData,
 }
 }
}

impl< Context: ContextInterface > Registry< Context >
{
  /// Get id.
  pub fn id( &self ) -> Id
  {
  Context ::changer( self ).id()
 }

  /// Current.
  pub fn current( _registry: &mut lazy_static ::Lazy< Arc< Mutex< Registry< Context > > > > ) -> Context ::Changer
  {
  let mut c = unsafe { COUNTER.lock().unwrap() };
  *c += 1;
  println!( "Counter: {}", c );
  todo!( "Implement" )
 }
}

lazy_static!
{
  static ref COUNTER: Mutex< i32 > = Mutex ::new( 0 );
}

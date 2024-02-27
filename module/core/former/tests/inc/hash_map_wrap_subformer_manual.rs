// xxx : finish
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct HashMapWrap< K, E >
where
  K : core::hash::Hash + std::cmp::Eq
{
  pub container : std::collections::HashMap< K, E >,
}

impl< K, E > Default for HashMapWrap< K, E >
where
  K : core::hash::Hash + std::cmp::Eq
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self { container : Default::default() }
  }
}

pub trait Perform< T, Context >
{
  fn call( &self, container : Option< T >, context : Context ) -> Context;
}

impl< T, Context, F > Perform< T, Context > for F
where
  F : Fn( Option< T >, Context ) -> Context,
{
  fn call( &self, container : Option< T >, context : Context ) -> Context
  {
    self( container, context )
  }
}

pub struct NoOpPerform;

impl< T, Context > Perform< T, Context >
for NoOpPerform
{
  #[ inline( always ) ]
  fn call( &self, _container : Option< T >, context : Context ) -> Context
  {
    context
  }
}

pub fn noop< T, Context >
(
  _context : Context,
  _container : core::option::Option< T >,
)
{
}

impl< K, E > HashMapWrap< K, E >
where
  K : core::hash::Hash + std::cmp::Eq
{

  pub fn new( container : std::collections::HashMap< K, E > ) -> Self
  {
    Self { container }
  }

  pub fn former() -> HashMapWrapFormer< K, E, (), impl Perform< std::collections::HashMap< K, E >, () > >
  {
    HashMapWrapFormer::< K, E, (), NoOpPerform >::new
    (
      core::option::Option::None,
      (),
      NoOpPerform,
    )
  }

}

// #[ derive( Debug, Default ) ]
pub struct HashMapWrapFormer< K, E, Context = (), Perform = NoOpPerform >
where
  K : core::hash::Hash + std::cmp::Eq
{
  container : core::option::Option< std::collections::HashMap< K, E > >,
  context : Context,
  on_perform : Perform,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Context, P >
HashMapWrapFormer< K, E, Context, P >
where
  K : core::cmp::Eq + core::hash::Hash,
  P : Perform< std::collections::HashMap< K, E >, Context >,
{

  #[ inline( always ) ]
  fn form( mut self ) -> HashMapWrap< K, E >
  {

    let container = if self.container.is_some()
    {
      self.container.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    HashMapWrap
    {
      container,
    }

  }

  /// Make a new HashMapWrapFormer. It should be called by a former generated for your structure.
  #[ inline( always ) ]
  pub fn new
  (
    container : core::option::Option< std::collections::HashMap< K, E > >,
    context : Context,
    on_perform : P,
  ) -> Self
  {
    Self
    {
      container,
      context,
      on_perform,
      _e_phantom : core::marker::PhantomData,
      _k_phantom : core::marker::PhantomData,
    }
  }

  /// Return former of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    self.on_perform.call( self.container.take(), self.context )
  }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, src : HashMapWrap< K, E > ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( src.container );
    self
  }

}

impl< K, E, Context, P >
HashMapWrapFormer< K, E, Context, P >
where
  K : core::cmp::Eq + core::hash::Hash,
  P : Perform< std::collections::HashMap< K, E >, Context >,
{

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
  #[ inline( always ) ]
  pub fn insert< K2, E2 >( mut self, k : K2, e : E2 ) -> Self
  where
    K2 : core::convert::Into< K >,
    E2 : core::convert::Into< E >,
  {
    if self.container.is_none()
    {
      self.container = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.insert( k.into(), e.into() );
    }
    self
  }

}

//

include!( "only_test/hash_map_wrap_subformer.rs" );

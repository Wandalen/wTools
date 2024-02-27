// xxx : finish
use super::*;

#[ derive( Debug ) ]
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

pub fn noop< K, E, Context >( context : &mut Context, container : core::option::Option< std::collections::HashMap< K, E > > )
where
  K : core::hash::Hash + std::cmp::Eq
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

  pub fn former< Context : Default, Perform >() -> HashMapWrapFormer< K, E, Context, Perform >
  where
    Perform : Fn( &mut Context, core::option::Option< std::collections::HashMap< K, E > > ) + Default,
  {
    HashMapWrapFormer::< K, E, Context, Perform >::new
    (
      core::option::Option::None,
      Context::default(),
      Perform::default(),
    )
  }

}

// #[ derive( Debug, Default ) ]
pub struct HashMapWrapFormer< K, E, Context, Perform >
where
  K : core::hash::Hash + std::cmp::Eq
{
  container : core::option::Option< std::collections::HashMap< K, E > >,
  context : Context,
  on_perform : Perform,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Context, Perform >
HashMapWrapFormer< K, E, Context, Perform >
where
  K : core::cmp::Eq + core::hash::Hash,
  Perform : Fn( &mut Context, core::option::Option< std::collections::HashMap< K, E > > ),
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
    on_perform : Perform,
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
    let container = self.container.take();
    ( self.on_perform )( &mut self.context, container );
    self.context
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

impl< K, E, Context, Perform >
HashMapWrapFormer< K, E, Context, Perform >
where
  K : core::cmp::Eq + core::hash::Hash,
  Perform : Fn( &mut Context, core::option::Option< std::collections::HashMap< K, E > > ),
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

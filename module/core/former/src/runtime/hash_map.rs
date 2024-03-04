use super::*;

///
/// Trait HashMapLike adopter for Container-like containers.
///

pub trait HashMapLike< K, E >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;
}

impl< K, E > HashMapLike< K, E > for std::collections::HashMap< K, E >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  fn insert( &mut self, k : K, e : E ) -> Option< E >
  {
    std::collections::HashMap::insert( self, k, e )
  }
}

///
/// Class for forming hashmap-like fields.
///

#[ derive( Debug, Default ) ]
pub struct HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
{
  container : core::option::Option< Container >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Container, Context, End >
HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> Container
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
    container
  }

  /// Create a new instance without context or on end processing. It just returns continaer on end of forming.
  #[ inline( always ) ]
  pub fn new() -> HashMapSubformer< K, E, Container, Container, impl ToSuperFormer< Container, Container > >
  {
    HashMapSubformer::begin
    (
      None,
      None,
      crate::ReturnContainer,
    )
  }

  /// Make a new HashMapSubformer. It should be called by a context generated for your structure.
  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< Context >,
    container : core::option::Option< Container >,
    on_end : End,
  ) -> Self
  {
    Self
    {
      context,
      container,
      on_end : Some( on_end ),
      _e_phantom : core::marker::PhantomData,
      _k_phantom : core::marker::PhantomData,
    }
  }

  /// Return context of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let container = self.form();
    on_end.call( container, context )
  }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, container : Container ) -> Self
  {
    self.container = Some( container );
    self
  }

}

impl< K, E, Container, Context, End >
HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
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

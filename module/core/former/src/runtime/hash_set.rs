use super::*;

///
/// Trait HashSetLike adopter for HashSet-like containers.
///

pub trait HashSetLike< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, e : E ) -> Option< E >;
}

impl< E > HashSetLike< E > for std::collections::HashSet< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  fn insert( &mut self, e : E ) -> Option< E >
  {
    std::collections::HashSet::replace( self, e )
  }
}

///
/// Class for forming hashset-like fields.
///

#[ derive( Debug, Default ) ]
pub struct HashSetSubformer< E, HashSet, Context, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  HashSet : HashSetLike< E > + core::default::Default,
  ContainerEnd : OnEnd< HashSet, Context >,
{
  container : core::option::Option< HashSet >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< ContainerEnd >,
  _e_phantom : core::marker::PhantomData< E >,
}

impl< E, HashSet, Context, ContainerEnd >
HashSetSubformer< E, HashSet, Context, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  HashSet : HashSetLike< E > + core::default::Default,
  ContainerEnd : OnEnd< HashSet, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  fn form( mut self ) -> HashSet
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

  /// Make a new HashSetSubformer. It should be called by a context generated for your structure.
  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< Context >,
    container : core::option::Option< HashSet >,
    on_end : ContainerEnd,
  ) -> Self
  {
    Self
    {
      context : context,
      container,
      on_end : Some( on_end ),
      _e_phantom : core::marker::PhantomData,
    }
  }

  /// Return context of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take().unwrap();
    let container = self.form();
    on_end.call( container, context )
  }

  // #[ inline( always ) ]
  // pub fn end( mut self ) -> Context
  // {
  //   let container = self.container.take();
  //   ( self.on_end )( &mut self.context, container );
  //   self.context
  // }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, container : HashSet ) -> Self
  {
    self.container = Some( container );
    self
  }

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
  #[ inline( always ) ]
  pub fn insert< E2 >( mut self, e : E2 ) -> Self
  where
    E2 : core::convert::Into< E >,
  {
    if self.container.is_none()
    {
      self.container = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.insert( e.into() );
    }
    self
  }

}

//
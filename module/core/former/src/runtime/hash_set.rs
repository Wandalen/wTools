
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

#[derive( Debug, Default )]
pub struct HashSetFormer< E, HashSet, Former, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  HashSet : HashSetLike< E > + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< HashSet > ),
{
  container : Option< HashSet >,
  former : Former,
  on_end : ContainerEnd,
  _e_phantom : core::marker::PhantomData< E >,
}

impl< E, HashSet, Former, ContainerEnd >
HashSetFormer< E, HashSet, Former, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  HashSet : HashSetLike< E > + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< HashSet > ),
{

  /// Make a new HashSetFormer. It should be called by a former generated for your structure.
  pub fn new( former : Former, container : core::option::Option< HashSet >, on_end : ContainerEnd ) -> Self
  {
    Self
    {
      former,
      container,
      on_end,
      _e_phantom : core::marker::PhantomData,
    }
  }

  /// Set the whole container instead of setting each element individually.
  pub fn replace( mut self, container : HashSet ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( container );
    self
  }

  /// Return former of your struct moving container there. Should be called after configuring the container.
  pub fn end( mut self ) -> Former
  {
    let container = self.container.take();
    ( self.on_end )( &mut self.former, container );
    self.former
  }

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
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

///
/// Trait HashMapLike adopter for HashMap-like containers.
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

#[derive( Debug, Default )]
pub struct HashMapFormer< K, E, HashMap, Former, ContainerEnd >
where
  K : core::cmp::Eq + core::hash::Hash,
  HashMap : HashMapLike< K, E > + core::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< HashMap > ),
{
  container : Option< HashMap >,
  former : Former,
  on_end : ContainerEnd,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, HashMap, Former, ContainerEnd >
HashMapFormer< K, E, HashMap, Former, ContainerEnd >
where
  K : core::cmp::Eq + core::hash::Hash,
  HashMap : HashMapLike< K, E > + core::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< HashMap > ),
{

  /// Make a new HashMapFormer. It should be called by a former generated for your structure.
  #[ inline( always ) ]
  pub fn new( former : Former, container : core::option::Option< HashMap >, on_end : ContainerEnd ) -> Self
  {
    Self
    {
      former,
      container,
      on_end,
      _e_phantom : core::marker::PhantomData,
      _k_phantom : core::marker::PhantomData,
    }
  }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, container : HashMap ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( container );
    self
  }

  /// Return former of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Former
  {
    let container = self.container.take();
    ( self.on_end )( &mut self.former, container );
    self.former
  }

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

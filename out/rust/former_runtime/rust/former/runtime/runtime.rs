
///
/// Trait VectorLike adopter for Vector-like containers.
///

pub trait VectorLike< E >
{
  /// Appends an element to the back of a container.
  fn push( &mut self, e : E );
}

impl< E > VectorLike< E > for std::vec::Vec< E >
{
  fn push( &mut self, e : E )
  {
    std::vec::Vec::push( self, e );
  }
}

///
/// Class for forming vector-like fields.
///

#[derive( Debug, Default )]
pub struct VectorFormer< E, Vector, Former, ContainerEnd >
where
  Vector : VectorLike< E > + std::fmt::Debug + std::cmp::PartialEq + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Vector > ),
{
  container : Option< Vector >,
  former : Former,
  on_end : ContainerEnd,
  _phantom : core::marker::PhantomData< E >,
}

impl< E, Vector, Former, ContainerEnd > VectorFormer< E, Vector, Former, ContainerEnd >
where
  Vector : VectorLike< E > + std::fmt::Debug + std::cmp::PartialEq + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Vector > ),
{

  /// Make a new VectorFormer. It should be called by a former generated for your structure.
  pub fn new( former : Former, container : core::option::Option< Vector >, on_end : ContainerEnd ) -> Self
  {
    Self
    {
      former,
      container,
      on_end,
      _phantom : core::marker::PhantomData,
    }
  }

  /// Set the whole container instead of setting each element individually.
  pub fn replace( mut self, vector : Vector ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( vector );
    self
  }

  /// Return former of your struct moving container there. Should be called after configuring the container.
  pub fn end( mut self ) -> Former
  {
    let container = self.container.take();
    ( self.on_end )( &mut self.former, container );
    self.former
  }

  /// Appends an element to the back of a container. Make a new container if it was not made so far.
  pub fn push< E2 >( mut self, e : E2 ) -> Self
  where E2 : core::convert::Into< E >,
  {
    if self.container.is_none()
    {
      self.container = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.push( e.into() );
    }
    self
  }

}

// pub type VectorFormerStdVec< Former, E > =
//   VectorFormer< E, std::vec::Vec< E >, Former, impl Fn( &mut Former, core::option::Option< std::vec::Vec< E > > ) >;

///
/// Trait HashmapLike adopter for Hashmap-like containers.
///

pub trait HashmapLike< K, E >
where
  K : std::cmp::Eq + std::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;
}

impl< K, E > HashmapLike< K, E > for std::collections::HashMap< K, E >
where
  K : std::cmp::Eq + std::hash::Hash,
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
pub struct HashmapFormer< K, E, Hashmap, Former, ContainerEnd >
where
  K : std::cmp::Eq + std::hash::Hash,
  Hashmap : HashmapLike< K, E > + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Hashmap > ),
{
  container : Option< Hashmap >,
  former : Former,
  on_end : ContainerEnd,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Hashmap, Former, ContainerEnd >
HashmapFormer< K, E, Hashmap, Former, ContainerEnd >
where
  K : std::cmp::Eq + std::hash::Hash,
  Hashmap : HashmapLike< K, E > + std::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Hashmap > ),
{

  /// Make a new HashmapFormer. It should be called by a former generated for your structure.
  pub fn new( former : Former, container : core::option::Option< Hashmap >, on_end : ContainerEnd ) -> Self
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
  pub fn replace( mut self, vector : Hashmap ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( vector );
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
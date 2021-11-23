
pub trait VectorLike< E >
{
  fn push( &mut self, e : E );
}

impl< E > VectorLike< E > for std::vec::Vec< E >
{
  fn push( &mut self, e : E )
  {
    std::vec::Vec::push( self, e );
  }
}

//

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

  pub fn replace( mut self, vector : Vector ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( vector );
    self
  }

  pub fn end( mut self ) -> Former
  {
    let container = self.container.take();
    ( self.on_end )( &mut self.former, container );
    self.former
  }

}

// pub type VectorFormerStdVec< Former, E > =
//   VectorFormer< E, std::vec::Vec< E >, Former, impl Fn( &mut Former, core::option::Option< std::vec::Vec< E > > ) >;

//

pub trait HashmapLike< K, E >
where
  K : std::cmp::Eq + std::hash::Hash,
{
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

//

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

  pub fn replace( mut self, vector : Hashmap ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( vector );
    self
  }

  pub fn end( mut self ) -> Former
  {
    let container = self.container.take();
    ( self.on_end )( &mut self.former, container );
    self.former
  }

}

//
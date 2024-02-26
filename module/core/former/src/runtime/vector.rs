
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
  Vector : VectorLike< E > + core::fmt::Debug + core::cmp::PartialEq + core::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Vector > ),
{
  container : Option< Vector >,
  former : Former,
  on_end : ContainerEnd,
  _phantom : core::marker::PhantomData< E >,
}

impl< E, Vector, Former, ContainerEnd > VectorFormer< E, Vector, Former, ContainerEnd >
where
  Vector : VectorLike< E > + core::fmt::Debug + core::cmp::PartialEq + core::default::Default,
  ContainerEnd : Fn( &mut Former, core::option::Option< Vector > ),
{

  /// Make a new VectorFormer. It should be called by a former generated for your structure.
  #[ inline( always ) ]
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
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Vector ) -> Self
  {
    debug_assert!( self.container.is_none() );
    self.container = Some( vector );
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

  /// Appends an element to the back of a container. Make a new container if it was not made so far.
  #[ inline( always ) ]
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

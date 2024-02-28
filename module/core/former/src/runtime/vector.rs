use super::*;

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

#[ derive( Debug, Default ) ]
pub struct VectorSubformer< E, Vector, Context, ContainerEnd >
where
  Vector : VectorLike< E > + core::fmt::Debug + core::cmp::PartialEq + core::default::Default,
  ContainerEnd : OnEnd< Vector, Context >,
{
  // container : Option< Vector >,
  // context : Context,
  // on_end : ContainerEnd,
  container : core::option::Option< Vector >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< ContainerEnd >,
  _phantom : core::marker::PhantomData< E >,
}

impl< E, Vector, Context, ContainerEnd > VectorSubformer< E, Vector, Context, ContainerEnd >
where
  Vector : VectorLike< E > + core::fmt::Debug + core::cmp::PartialEq + core::default::Default,
  ContainerEnd : OnEnd< Vector, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  fn form( mut self ) -> Vector
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

  /// Make a new VectorSubformer. It should be called by a context generated for your structure.
  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< Context >,
    container : core::option::Option< Vector >,
    on_end : ContainerEnd
  ) -> Self
  {
    Self
    {
      context,
      container,
      on_end : Some( on_end ),
      _phantom : core::marker::PhantomData,
      // context,
      // container,
      // on_end,
      // _phantom : core::marker::PhantomData,
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

  // /// Return context of your struct moving container there. Should be called after configuring the container.
  // #[ inline( always ) ]
  // pub fn end( mut self ) -> Context
  // {
  //   let container = self.container.take();
  //   ( self.on_end )( &mut self.context, container );
  //   self.context
  // }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Vector ) -> Self
  {
    self.container = Some( vector );
    self
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

// pub type VectorFormerStdVec< Context, E > =
//   VectorSubformer< E, std::vec::Vec< E >, Context, impl Fn( &mut Context, core::option::Option< std::vec::Vec< E > > ) >;

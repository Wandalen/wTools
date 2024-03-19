use super::*;

// #[ cfg( feature = "use_alloc" ) ]
// extern crate alloc;
// #[ cfg( feature = "use_alloc" ) ]
// #[ allow( unused_imports ) ]
// use alloc::vec::Vec;
// #[ cfg( not( feature = "no_std" ) ) ]
// #[ allow( unused_imports ) ]
// use std::vec::Vec;

#[ allow( unused ) ]
use collection_tools::Vec;

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike< E >
{
  /// Appends an element to the back of a container.
  fn push( &mut self, element : E );
}

impl< E > VectorLike< E > for Vec< E >
{
  fn push( &mut self, element : E )
  {
    Vec::push( self, element );
  }
}

/// A builder for constructing `VectorLike` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer` leverages the `VectorLike` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.
///
/// # Example
/// ```rust
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithVec
/// {
///   #[ subformer( former::VectorSubformer ) ]
///   vec : Vec< &'static str >,
/// }
///
/// let instance = StructWithVec::former()
/// .vec()
///   .push( "apple" )
///   .push( "banana" )
///   .end()
/// .form();
///
/// assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );
///```
///
#[ derive( Debug, Default ) ]
pub struct VectorSubformer< E, Container, Context, ContainerEnd >
where
  Container : VectorLike< E > + core::default::Default,
  ContainerEnd : ToSuperFormer< Container, Context >,
{
  container : core::option::Option< Container >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< ContainerEnd >,
  _phantom : core::marker::PhantomData< E >,
}

impl< E, Container, Context, ContainerEnd > VectorSubformer< E, Container, Context, ContainerEnd >
where
  Container : VectorLike< E > + core::default::Default,
  ContainerEnd : ToSuperFormer< Container, Context >,
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

  // /// Initializes a new `VectorSubformer` instance, starting with an empty container.
  // /// This function serves as the entry point for the builder pattern.
  // ///
  // /// # Returns
  // /// A new instance of `VectorSubformer` with an empty internal container.
  // ///
  // #[ inline( always ) ]
  // pub fn new() -> VectorSubformer< E, Container, Container, impl ToSuperFormer< Container, Container > >
  // {
  //   VectorSubformer::begin
  //   (
  //     None,
  //     None,
  //     crate::ReturnContainer,
  //   )
  // }

  /// Begins the building process, optionally initializing with a context and container.
  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< Context >,
    container : core::option::Option< Container >,
    on_end : ContainerEnd
  ) -> Self
  {
    Self
    {
      context,
      container,
      on_end : Some( on_end ),
      _phantom : core::marker::PhantomData,
    }
  }

  /// Finalizes the building process, returning the container or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let container = self.form();
    on_end.call( container, context )
  }

  /// Replaces the current container with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Container ) -> Self
  {
    self.container = Some( vector );
    self
  }

}

impl< E, Container > VectorSubformer< E, Container, Container, crate::ReturnContainer >
where
  Container : VectorLike< E > + core::default::Default,
{

  /// Initializes a new `VectorSubformer` instance, starting with an empty container.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `VectorSubformer` with an empty internal container.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin
    (
      None,
      None,
      crate::ReturnContainer,
    )
  }

}

impl< E, Container, Context, ContainerEnd > VectorSubformer< E, Container, Context, ContainerEnd >
where
  Container : VectorLike< E > + core::default::Default,
  ContainerEnd : ToSuperFormer< Container, Context >,
{

  /// Appends an element to the end of the container, expanding the internal collection.
  #[ inline( always ) ]
  pub fn push< E2 >( mut self, element : E2 ) -> Self
  where E2 : core::convert::Into< E >,
  {
    if self.container.is_none()
    {
      self.container = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.push( element.into() );
    }
    self
  }

}

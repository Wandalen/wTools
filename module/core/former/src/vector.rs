use super::*;

#[ allow( unused ) ]
use collection_tools::Vec;

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike< E >
{
  /// Appends an element to the back of a formed.
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
pub struct VectorSubformer< E, Formed, Context, ContainerEnd >
where
  Formed : VectorLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context >,
{
  formed : core::option::Option< Formed >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< ContainerEnd >,
  _phantom : core::marker::PhantomData< E >,
}

impl< E, Formed, Context, ContainerEnd > VectorSubformer< E, Formed, Context, ContainerEnd >
where
  Formed : VectorLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> Formed
  {
    let formed = if self.formed.is_some()
    {
      self.formed.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    formed
  }

  // /// Initializes a new `VectorSubformer` instance, starting with an empty formed.
  // /// This function serves as the entry point for the builder pattern.
  // ///
  // /// # Returns
  // /// A new instance of `VectorSubformer` with an empty internal formed.
  // ///
  // #[ inline( always ) ]
  // pub fn new() -> VectorSubformer< E, Formed, Formed, impl FormingEnd< Formed, Formed > >
  // {
  //   VectorSubformer::begin
  //   (
  //     None,
  //     None,
  //     crate::ReturnFormed,
  //   )
  // }

  /// Begins the building process, optionally initializing with a context and formed.
  #[ inline( always ) ]
  pub fn begin
  (
    formed : core::option::Option< Formed >,
    context : core::option::Option< Context >,
    on_end : ContainerEnd
  ) -> Self
  {
    Self
    {
      context,
      formed,
      on_end : Some( on_end ),
      _phantom : core::marker::PhantomData,
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  /// Replaces the current formed with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Formed ) -> Self
  {
    self.formed = Some( vector );
    self
  }

}

impl< E, Formed > VectorSubformer< E, Formed, Formed, crate::ReturnFormed >
where
  Formed : VectorLike< E > + core::default::Default,
{

  /// Initializes a new `VectorSubformer` instance, starting with an empty formed.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `VectorSubformer` with an empty internal formed.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin
    (
      None,
      None,
      crate::ReturnFormed,
    )
  }

}

impl< E, Formed, Context, ContainerEnd > VectorSubformer< E, Formed, Context, ContainerEnd >
where
  Formed : VectorLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context >,
{

  /// Appends an element to the end of the formed, expanding the internal collection.
  #[ inline( always ) ]
  pub fn push< E2 >( mut self, element : E2 ) -> Self
  where E2 : core::convert::Into< E >,
  {
    if self.formed.is_none()
    {
      self.formed = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut formed ) = self.formed
    {
      formed.push( element.into() );
    }
    self
  }

}

//

impl< E, Formed, Context, End > FormerBegin< Formed, Context >
for VectorSubformer< E, Formed, Context, End >
where
  End : FormingEnd< Formed, Context >,
  Formed : VectorLike< E > + Default,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    formed : core::option::Option< Formed >,
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self::begin( formed, context, on_end )
  }

}

use super::*;
use axiomatic2::*;

#[ allow( unused ) ]
use collection_tools::Vec;

/// Trait for containers that behave like a vector, providing an interface for element addition.
///
/// This trait enables the use of custom or standard vector-like containers within the builder pattern,
/// allowing for a unified and flexible approach to constructing collections.
///
pub trait VectorLike2< E >
{
  /// Appends an element to the back of a formed.
  fn push( &mut self, element : E );
}

impl< E > VectorLike2< E > for Vec< E >
{
  fn push( &mut self, element : E )
  {
    Vec::push( self, element );
  }
}

impl< E > StoragePerform for Vec< E >
{
  type Formed = Self;
  fn preform( self ) -> Self::Formed
  {
    self
  }
}
pub struct VectorSubformerDescriptor< E >
{
  _phantom : core::marker::PhantomData< E >,
}

impl< E > VectorSubformerDescriptor< E >
{
  fn new() -> Self
  {
    Self { _phantom : core::marker::PhantomData }
  }
}

impl< E > FormerDescriptor
for VectorSubformerDescriptor< E >
{
  type Storage = Vec< E >;
  type Formed = Vec< E >;
  // type Former = VectorSubformer2< E, Context, End >;
}

/// A builder for constructing `VectorLike2` containers, facilitating a fluent and flexible interface.
///
/// `VectorSubformer2` leverages the `VectorLike2` trait to enable the construction and manipulation
/// of vector-like containers in a builder pattern style, promoting readability and ease of use.
///
#[ derive( Debug, Default ) ]
pub struct VectorSubformer2< E, Context, End >
where
  End : FormingEnd2< VectorSubformerDescriptor< E >, Context >,
{
  formed : core::option::Option< < VectorSubformerDescriptor< E > as axiomatic2::FormerDescriptor >::Formed >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

impl< E, Context, End > VectorSubformer2< E, Context, End >
where
  End : FormingEnd2< VectorSubformerDescriptor< E >, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> < VectorSubformerDescriptor< E > as axiomatic2::FormerDescriptor >::Formed
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

  /// Begins the building process, optionally initializing with a context and formed.
  #[ inline( always ) ]
  pub fn begin
  (
    formed : core::option::Option< < VectorSubformerDescriptor< E > as axiomatic2::FormerDescriptor >::Formed >,
    context : core::option::Option< Context >,
    on_end : End
  ) -> Self
  {
    Self
    {
      context,
      formed,
      on_end : Some( on_end ),
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> < VectorSubformerDescriptor< E > as axiomatic2::FormerDescriptor >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  /// Replaces the current formed with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : < VectorSubformerDescriptor< E > as axiomatic2::FormerDescriptor >::Formed ) -> Self
  {
    self.formed = Some( vector );
    self
  }

}

impl< E > VectorSubformer2< E, (), ReturnStorage2 >
where
{

  /// Initializes a new `VectorSubformer2` instance, starting with an empty formed.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `VectorSubformer2` with an empty internal formed.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    Self::begin
    (
      None,
      None,
      ReturnStorage2,
    )
  }

}

impl< E, Context, End > VectorSubformer2< E, Context, End >
where
  End : FormingEnd2< VectorSubformerDescriptor< E >, Context >,
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

// impl< Former, Context, End > FormerBegin< Formed, Formed, Context >
// for VectorSubformer2< Former, Context, End >
// where
//   End : FormingEnd2< VectorSubformerDescriptor< E >, Context >,
//   // Formed : VectorLike2< E > + Default,
//   Former : FormerDescriptor,
// {
//   type End = End;
//
//   #[ inline( always ) ]
//   fn _begin
//   (
//     formed : core::option::Option< Formed >,
//     context : core::option::Option< Context >,
//     on_end : End,
//   ) -> Self
//   {
//     Self::begin( formed, context, on_end )
//   }
//
// }

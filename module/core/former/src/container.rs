//! Interface for containers.

use crate::*;

/// A trait defining the capability to add elements to a container.
///
/// This trait should be implemented by container types that require a generic interface
/// for adding new elements. It abstracts over the specific details of how elements are
/// added to the container, providing a consistent API regardless of the underlying
/// container's structure.
///
/// # Type Parameters
///
/// - There are no explicit type parameters for the trait itself, but implementers will
///   specify their own types as needed.
///
/// # Associated Types
///
/// * `Element`: The type of elements that can be added to the container. This type is
///   defined by the implementer of the trait, allowing for flexibility in the kinds of
///   elements different containers can accept.
///
pub trait ContainerAdd
{
  /// The type of elements to be added to the container.
  type Element;

  /// Adds an element to the container.
  ///
  /// Implementations of this function should add the provided element to the container,
  /// respecting the container's specific semantics for element addition (e.g., handling
  /// duplicates or maintaining order). The function returns a boolean indicating whether
  /// the addition was successful.
  ///
  /// # Parameters
  ///
  /// * `e`: The element to be added to the container. The type of the element is specified
  ///   by the associated `Element` type.
  ///
  /// # Returns
  ///
  /// Returns `true` if the element was successfully added to the container, or `false` if
  /// the addition failed. Failure conditions are defined by the implementer but may include
  /// situations like the container being at capacity or the element already existing in a
  /// set.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use former::ContainerAdd;
  ///
  /// struct MyContainer
  /// {
  ///   elements : Vec< i32 >,
  /// }
  ///
  /// impl ContainerAdd for MyContainer
  /// {
  ///   type Element = i32;
  ///
  ///   fn add( &mut self, e : Self::Element ) -> bool
  ///   {
  ///     if self.elements.contains( &e )
  ///     {
  ///       false
  ///     }
  ///     else
  ///     {
  ///       self.elements.push( e );
  ///       true
  ///     }
  ///   }
  /// }
  ///
  /// let mut container = MyContainer { elements : vec![] };
  /// assert!( container.add( 10 ) ); // Returns true, element added
  /// assert!( !container.add( 10 ) ); // Returns false, element already exists
  /// ```
  ///
  /// This example demonstrates a simple container that does not allow duplicate elements.
  /// The `add` method checks for the existence of the element before adding it, returning
  /// `false` if the element is already present.
  ///
  fn add( &mut self, e : Self::Element ) -> bool;

}

// qqq : implement for other containers

/// A trait defining the capability to replface all elements.
pub trait ContainerAssign
{
  /// The type of elements to be added to the container.
  type Element;

  /// Agging elements to the container.
  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Element >;

}

// =

/// A builder for constructing containers, facilitating a fluent and flexible interface.
#[ derive( Default ) ]
pub struct ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{
  storage : < Definition::Types as FormerDefinitionTypes >::Storage,
  context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

// zzz : cover by test
use std::fmt;
impl< E, Definition > fmt::Debug for ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "ContainerSubformer" )
    .field( "storage", &"Storage Present" )
    .field( "context", &self.context.as_ref().map( |_| "Context Present" ) )
    .field( "on_end", &self.on_end.as_ref().map( |_| "End Present" ) )
    .finish()
  }
}

impl< E, Definition > ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{

  /// Begins the building process, optionally initializing with a context and storage.
  #[ inline( always ) ]
  pub fn begin
  (
    mut storage : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  )
  -> Self
  {
    if storage.is_none()
    {
      storage = Some( core::default::Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context,
      on_end : Some( on_end ),
    }
  }

  /// zzz : update description
  #[ inline( always ) ]
  pub fn begin_coercing< IntoEnd >
  (
    mut storage : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
    on_end : IntoEnd,
  )
  -> Self
  where
    IntoEnd : Into< Definition::End >,
  {
    if storage.is_none()
    {
      storage = Some( core::default::Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context,
      on_end : Some( on_end.into() ),
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    // let storage = self.storage();
    on_end.call( self.storage, context )
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  /// Replaces the current storage with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : < Definition::Types as FormerDefinitionTypes >::Storage ) -> Self
  {
    self.storage = vector;
    self
  }

}

impl< E, Storage, Formed, Types, Definition > ContainerSubformer< E, Definition >
where
  Types : FormerDefinitionTypes< Context = (), Storage = Storage, Formed = Formed >,
  Definition : FormerDefinition< Types = Types >,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{

  /// Initializes a new `ContainerSubformer` instance, starting with an empty formed.
  /// This function serves as the entry point for the builder pattern.
  ///
  /// # Returns
  /// A new instance of `ContainerSubformer` with an empty internal formed.
  ///
  // zzz : update description
  #[ inline( always ) ]
  pub fn new( end : Definition::End ) -> Self
  {
    Self::begin
    (
      None,
      None,
      end,
    )
  }

  // zzz : update description
  #[ inline( always ) ]
  pub fn new_coercing< IntoEnd >( end : IntoEnd ) -> Self
  where
    IntoEnd : Into< Definition::End >,
  {
    Self::begin
    (
      None,
      None,
      end.into(),
    )
  }

}

impl< E, Definition > ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{

  /// Appends an element to the end of the storage, expanding the internal collection.
  #[ inline( always ) ]
  pub fn add< IntoElement >( mut self, element : IntoElement ) -> Self
  where IntoElement : core::convert::Into< E >,
  {
    ContainerAdd::add( &mut self.storage, element.into() );
    self
  }

}

//

impl< E, Definition > FormerBegin< Definition >
for ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{
  // type End = Definition::End;

  #[ inline( always ) ]
  fn former_begin
  (
    storage : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  )
  -> Self
  {
    Self::begin( storage, context, on_end )
  }

}

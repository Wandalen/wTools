// File container.rs

//! Interface for containers.

use crate::*;

/// Represents a container by defining the types of elements and values it handles.
///
/// This trait abstracts the nature of containers in data structures, facilitating the handling of contained
/// elements and values, especially in scenarios where the structure of the container allows for complex relationships,
/// such as `HashMap`s. It not only identifies what constitutes an element and a value in the context of the container
/// but also provides utility for converting between these two, which is critical in operations involving element manipulation
/// and value retrieval.

pub trait Container
{
  /// The type of elements that can be added to the container. This type can differ from `Val` in containers like `HashMap`,
  /// where an element might represent a key-value pair, and `Val` could represent just the value or the key.
  type Element;

  /// The type of values stored in the container. This might be distinct from `Element` in complex containers.
  /// For example, in a `HashMap`, while `Element` might be a (key, value) tuple, `Val` might only be the value part.
  type Val;

  /// Converts an element to its corresponding value within the container. This function is essential for abstracting
  /// the container's internal representation from the values it manipulates.
  fn element_to_val( e : Self::Element ) -> Self::Val;
}

/// Facilitates the conversion of container elements to their corresponding value representations.
///
/// This trait is utilized to transform an element of a container into a value, abstracting the operation of containers
/// like vectors or hash maps. It ensures that even in complex container structures, elements can be seamlessly managed
/// and manipulated as values.
pub trait ElementToVal<Container>
{
  type Val;

  /// Converts an element into a value representation specific to the type of container. This conversion is crucial
  /// for handling operations on elements, especially when they need to be treated or accessed as individual values,
  /// such as retrieving the value part from a key-value pair in a hash map.
  fn element_to_val( self ) -> Self::Val;
}

impl< C, E > ElementToVal< C > for E
where
  C : Container< Element = E >,
{
  type Val = C::Val;

  fn element_to_val( self ) -> Self::Val
  {
    C::element_to_val( self )
  }
}

/// Provides a mechanism for converting values back to container-specific elements.
///
/// This trait is crucial for operations that require the insertion or modification of elements based on values,
/// especially in complex data structures where the element's structure is more intricate than the value it represents,
/// such as inserting a new entry in a `HashMap` where the element consists of a key-value pair.
pub trait ValToElement< Container >
{
  type Element;

  /// Converts a value back into an element of the container. This function is essential for operations like insertion
  /// or modification, where a value needs to be transformed into a container-compatible element, such as converting
  /// a value into a (key, value) tuple for insertion into a `HashMap`.
  fn val_to_element( self ) -> Self::Element;
}

/// A trait defining the capability to add elements to a container.
///
/// This trait should be implemented by container types that require a generic interface
/// for adding new elements. It abstracts over the specific details of how elements are
/// added to the container, providing a consistent API regardless of the underlying
/// container's structure.
///

// zzz : update description
pub trait ContainerAdd : Container
{

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

// zzz : extend documentation
/// A trait defining the capability to replface all elements.
pub trait ContainerAssign : Container
{

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
  Definition::Storage : ContainerAdd< Element = E >,
{
  storage : Definition::Storage,
  context : core::option::Option< Definition::Context >,
  on_end : core::option::Option< Definition::End >,
}

// zzz : cover by test
use std::fmt;
impl< E, Definition > fmt::Debug for ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f
    .debug_struct( "ContainerSubformer" )
    .field( "storage", &"Storage Present" )
    .field( "context", &self.context.as_ref().map( |_| "Context Present" ) )
    .field( "on_end", &self.on_end.as_ref().map( |_| "End Present" ) )
    .finish()
  }
}

impl< E, Definition > ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  Definition::Storage : ContainerAdd< Element = E >,
{

  /// Begins the building process, optionally initializing with a context and storage.
  #[ inline( always ) ]
  pub fn begin
  (
    mut storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
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
    mut storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
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
  pub fn end( mut self ) -> Definition::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    // let storage = self.storage();
    on_end.call( self.storage, context )
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn form( self ) -> Definition::Formed
  {
    self.end()
  }

  /// Replaces the current storage with a provided one, allowing for a reset or redirection of the building process.
  #[ inline( always ) ]
  pub fn replace( mut self, vector : Definition::Storage ) -> Self
  {
    self.storage = vector;
    self
  }

}

impl< E, Storage, Formed, Types, Definition > ContainerSubformer< E, Definition >
where
  Types : FormerDefinitionTypes< Context = (), Storage = Storage, Formed = Formed >,
  Definition : FormerDefinition< Types = Types >,
  Definition::Storage : ContainerAdd< Element = E >,
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
  Definition::Storage : ContainerAdd< Element = E >,
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
  Definition::Storage : ContainerAdd< Element = E >,
{
  // type End = Definition::End;

  #[ inline( always ) ]
  fn former_begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : Definition::End,
  )
  -> Self
  {
    Self::begin( storage, context, on_end )
  }

}

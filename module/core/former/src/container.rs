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

impl< T > ContainerAdd for collection_tools::Vec< T >
{
  type Element = T;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Element ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > ContainerAdd for collection_tools::HashSet< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  type Element = E;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Element ) -> bool
  {
    self.insert( e )
  }

}

impl< K, V > ContainerAdd for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Element = ( K, V );

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Element ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

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

impl< T > ContainerAssign for collection_tools::Vec< T >
{
  type Element = T;

  #[ inline( always ) ]
  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Element >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }

}

impl< T > ContainerAssign for collection_tools::HashSet< T >
where
  T : core::cmp::Eq + core::hash::Hash,
{
  type Element = T;

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Element >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

impl< K, V > ContainerAssign for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Element = ( K, V );

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Element >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

// =

/// A builder for constructing containers, facilitating a fluent and flexible interface.
#[ derive( Default ) ]
pub struct ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{
  storage : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Storage >,
  context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

impl< E, Definition > ContainerSubformer< E, Definition >
where
  Definition : FormerDefinition,
  < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn storage( mut self ) -> < Definition::Types as FormerDefinitionTypes >::Storage
  {
    let storage = if self.storage.is_some()
    {
      self.storage.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    storage
  }

  /// Begins the building process, optionally initializing with a context and storage.
  #[ inline( always ) ]
  pub fn begin
  (
    storage : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    Self
    {
      storage,
      context,
      on_end : Some( on_end ),
    }
  }

  /// Finalizes the building process, returning the formed or a context incorporating it.
  #[ inline( always ) ]
  pub fn end( mut self ) -> < Definition::Types as FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let storage = self.storage();
    on_end.call( storage, context )
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
    self.storage = Some( vector );
    self
  }

}

// impl< E, T, Types, Definition > ContainerSubformer< E, Definition >
// where
//   Types : FormerDefinitionTypes< Context = (), Storage = T, Formed = T >,
//   Definition : FormerDefinition< Types = Types, End = ReturnStorage >,
//   < Definition::Types as FormerDefinitionTypes >::Storage : ContainerAdd< Element = E >,
//   < Definition::Types as FormerDefinitionTypes >::Storage : StoragePerform< Formed = < Definition::Types as FormerDefinitionTypes >::Formed >,
// {
//
//   // xxx : update description
//   /// Initializes a new `ContainerSubformer` instance, starting with an empty formed.
//   /// This function serves as the entry point for the builder pattern.
//   ///
//   /// # Returns
//   /// A new instance of `ContainerSubformer` with an empty internal formed.
//   ///
//   #[ inline( always ) ]
//   pub fn new_returning_storage() -> Self
//   {
//     Self::begin
//     (
//       None,
//       None,
//       ReturnStorage,
//     )
//   }
//
// }

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
  // xxx : update description
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

  // xxx : update description
  #[ inline( always ) ]
  pub fn new_with< IntoEnd >( end : IntoEnd ) -> Self
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
  pub fn push< IntoElement >( mut self, element : IntoElement ) -> Self
  where IntoElement : core::convert::Into< E >,
  {
    if self.storage.is_none()
    {
      self.storage = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut storage ) = self.storage
    {
      ContainerAdd::add( storage, element.into() );
      // storage.push( element.into() );
    }
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
  fn _begin
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

//! # HashSetLike Trait and HashSetSubformer Struct
//!
//! This part of the crate provides a flexible interface (`HashSetLike`) and a builder pattern implementation (`HashSetSubformer`) for `HashSet`-like containers. It's designed to extend the builder pattern, allowing for fluent and dynamic construction of sets within custom data structures.

use super::*;
use collection_tools::HashSet;

/// A trait for containers behaving like a `HashSet`, allowing insertion operations.
///
/// Implementing this trait enables the associated formed to be used with `HashSetSubformer`,
/// facilitating a builder pattern that is both intuitive and concise.
///
/// # Example Implementation
///
/// Implementing `HashSetLike` for `std::collections::HashSet`:
///

pub trait HashSetLike< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, element : K ) -> Option< K >;
}

impl< K > HashSetLike< K > for HashSet< K >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  fn insert( &mut self, element : K ) -> Option< K >
  {
    HashSet::replace( self, element )
  }
}

//

#[ derive( Debug ) ]
pub struct HashSetDescriptor< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  _phantom : ::core::marker::PhantomData< ( K, K ) >,
}

impl< K > HashSetDescriptor< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  pub fn new() -> Self
  {
    Self { _phantom : ::core::marker::PhantomData }
  }
}

impl< K > Storage
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Descriptor = HashSetDescriptor< K >;
  // fn preform( self ) -> < < Self as Storage >::Descriptor as FormerDescriptor >::Formed
  // {
  //   self
  // }
}

impl< K > StoragePerform
for HashSet< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn preform( self ) -> < < Self as Storage >::Descriptor as FormerDescriptor >::Formed
  {
    self
  }
}

impl< K > FormerDescriptor
for HashSetDescriptor< K >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashSet< K >;
  type Formed = HashSet< K >;
}

/// Facilitates building `HashSetLike` containers with a fluent API.
///
/// `HashSetSubformer` leverages the `HashSetLike` trait to enable a concise and expressive way
/// of populating `HashSet`-like containers. It exemplifies the crate's builder pattern variation for sets.
///
/// # Example Usage
///
/// Using `HashSetSubformer` to populate a `HashSet` within a struct:
///
/// ```rust
/// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
/// # {
/// # use test_tools::exposed::*;
///
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithSet
/// {
///   #[ subformer( former::HashSetSubformer ) ]
///   set : std::collections::HashSet< &'static str >,
/// }
///
/// let instance = StructWithSet::former()
/// .set()
///   .insert( "apple" )
///   .insert( "banana" )
///   .end()
/// .form();
///
/// assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
/// # }
/// ```

#[ derive( Debug, Default ) ]
pub struct HashSetSubformer< K, Descriptor, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  // Formed : HashSetLike< K > + core::default::Default,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = K >,
{
  storage : core::option::Option< Descriptor::Storage >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
  _e_phantom : core::marker::PhantomData< K >,
}

impl< K, Descriptor, Context, End >
HashSetSubformer< K, Descriptor, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  // Formed : HashSetLike< K > + core::default::Default,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = K >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn storage( mut self ) -> Descriptor::Storage
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
  // xxx

  /// Begins the building process with an optional context and storage.
  ///
  /// This method is typically called internally by the builder but can be used directly
  /// to initialize the builder with specific contexts or containers.
  ///
  /// # Parameters
  /// - `context`: An optional context for the building process.
  /// - `storage`: An optional initial storage to populate.
  /// - `on_end`: A handler to be called at the end of the building process.
  ///
  #[ inline( always ) ]
  pub fn begin
  (
    storage : core::option::Option< Descriptor::Storage >,
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self
    {
      storage,
      context : context,
      on_end : Some( on_end ),
      _e_phantom : core::marker::PhantomData,
    }
  }

  /// Finalizes the building process and returns the constructed formed or a context.
  ///
  /// This method concludes the building process by applying the `on_end` handler to transform
  /// the formed or incorporate it into a given context. It's typically called at the end
  /// of the builder chain to retrieve the final product of the building process.
  ///
  /// # Returns
  /// Depending on the `on_end` handler's implementation, this method can return either the
  /// constructed formed or a context that incorporates the formed.
  ///
  #[ inline( always ) ]
  pub fn form( self ) -> Descriptor::Formed
  {
    self.end()
  }

  /// Finalizes the building process and returns the constructed formed or a context.
  ///
  /// This method concludes the building process by applying the `on_end` handler to transform
  /// the formed or incorporate it into a given context. It's typically called at the end
  /// of the builder chain to retrieve the final product of the building process.
  ///
  /// # Returns
  /// Depending on the `on_end` handler's implementation, this method can return either the
  /// constructed formed or a context that incorporates the formed.
  ///
  #[ inline( always ) ]
  pub fn end( mut self ) -> Descriptor::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let storage = self.storage();
    on_end.call( storage, context )
  }

  /// Replaces the current storage with a new one.
  ///
  /// This method allows for replacing the entire set being built with a different one.
  /// It can be useful in scenarios where a pre-populated set needs to be modified or
  /// replaced entirely during the building process.
  ///
  /// # Parameters
  /// - `storage`: The new storage to use for subsequent builder operations.
  ///
  /// # Returns
  /// The builder instance with the storage replaced, enabling further chained operations.
  ///
  #[ inline( always ) ]
  pub fn replace( mut self, storage : Descriptor::Storage ) -> Self
  {
    self.storage = Some( storage );
    self
  }

}

// impl< K > VectorSubformer< K, Formed, crate::ReturnStorage >
// where
//   Formed : VectorLike< K > + core::default::Default,
// {

impl< K, Descriptor >
HashSetSubformer< K, Descriptor, (), crate::ReturnStorage >
where
  K : core::cmp::Eq + core::hash::Hash,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = K >,
  // Formed : HashSetLike< K > + core::default::Default,
  // End : FormingEnd< Descriptor, Context >,
{

  /// Initializes a new instance of the builder with default settings.
  ///
  /// This method provides a starting point for forming a `HashSetLike` using
  /// a fluent interface.
  ///
  /// # Returns
  /// A new instance of `HashSetSubformer` with no elements.
  ///
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    HashSetSubformer::begin
    (
      None,
      None,
      crate::ReturnStorage,
    )
  }

}

impl< K, Descriptor, Context, End >
HashSetSubformer< K, Descriptor, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = K >,
{

  /// Inserts an element into the set, possibly replacing an existing element.
  ///
  /// This method ensures that the set contains the given element, and if the element
  /// was already present, it might replace it depending on the storage's behavior.
  ///
  /// # Parameters
  /// - `element`: The element to insert into the set.
  ///
  /// # Returns
  /// - `Some(element)` if the element was replaced.
  /// - `None` if the element was newly inserted without replacing any existing element.
  ///
  #[ inline( always ) ]
  pub fn insert< E2 >( mut self, element : E2 ) -> Self
  where
    E2 : core::convert::Into< K >,
  {
    if self.storage.is_none()
    {
      self.storage = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut storage ) = self.storage
    {
      ContainerAdd::add( storage, element.into() );
    }
    self
  }

}

//
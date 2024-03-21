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

pub trait HashSetLike< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  /// Inserts a key-value pair into the map.
  fn insert( &mut self, element : E ) -> Option< E >;
}

impl< E > HashSetLike< E > for HashSet< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  fn insert( &mut self, element : E ) -> Option< E >
  {
    HashSet::replace( self, element )
  }
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

// xxx2 : change sequence
#[ derive( Debug, Default ) ]
pub struct HashSetSubformer< E, Formed, Context, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  Formed : HashSetLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context, Formed >,
{
  formed : core::option::Option< Formed >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< ContainerEnd >,
  _e_phantom : core::marker::PhantomData< E >,
}

impl< E, Formed, Context, ContainerEnd >
HashSetSubformer< E, Formed, Context, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  Formed : HashSetLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context, Formed >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn preform( mut self ) -> Formed
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

  /// Begins the building process with an optional context and formed.
  ///
  /// This method is typically called internally by the builder but can be used directly
  /// to initialize the builder with specific contexts or containers.
  ///
  /// # Parameters
  /// - `context`: An optional context for the building process.
  /// - `formed`: An optional initial formed to populate.
  /// - `on_end`: A handler to be called at the end of the building process.
  ///
  #[ inline( always ) ]
  pub fn begin
  (
    formed : core::option::Option< Formed >,
    context : core::option::Option< Context >,
    on_end : ContainerEnd,
  ) -> Self
  {
    Self
    {
      formed,
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
  pub fn form( self ) -> Formed
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
  pub fn end( mut self ) -> Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.preform();
    on_end.call( formed, context )
  }

  /// Replaces the current formed with a new one.
  ///
  /// This method allows for replacing the entire set being built with a different one.
  /// It can be useful in scenarios where a pre-populated set needs to be modified or
  /// replaced entirely during the building process.
  ///
  /// # Parameters
  /// - `formed`: The new formed to use for subsequent builder operations.
  ///
  /// # Returns
  /// The builder instance with the formed replaced, enabling further chained operations.
  ///
  #[ inline( always ) ]
  pub fn replace( mut self, formed : Formed ) -> Self
  {
    self.formed = Some( formed );
    self
  }

}

impl< E, Formed >
HashSetSubformer< E, Formed, (), crate::ReturnStorage >
where
  E : core::cmp::Eq + core::hash::Hash,
  Formed : HashSetLike< E > + core::default::Default,
{

  /// Initializes a new instance of the builder with default settings.
  ///
  /// This method provides a starting point for building a `HashSetLike` formed using
  /// a fluent interface. It sets up an empty formed ready to be populated.
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

impl< E, Formed, Context, ContainerEnd >
HashSetSubformer< E, Formed, Context, ContainerEnd >
where
  E : core::cmp::Eq + core::hash::Hash,
  Formed : HashSetLike< E > + core::default::Default,
  ContainerEnd : FormingEnd< Formed, Context, Formed >,
{

  /// Inserts an element into the set, possibly replacing an existing element.
  ///
  /// This method ensures that the set contains the given element, and if the element
  /// was already present, it might replace it depending on the formed's behavior.
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
    E2 : core::convert::Into< E >,
  {
    if self.formed.is_none()
    {
      self.formed = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut formed ) = self.formed
    {
      formed.insert( element.into() );
    }
    self
  }

}

//
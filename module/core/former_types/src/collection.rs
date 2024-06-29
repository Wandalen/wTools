//!
//! This module defines traits and structures that facilitate the management and manipulation
//! of collection data structures within a builder pattern context. It provides a comprehensive
//! interface for adding, managing, and converting elements within various types of collections,
//! such as vectors, hash maps, and custom collection implementations.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;

  /// Facilitates the conversion of collection entries to their corresponding value representations.
  ///
  /// This trait is utilized to transform an entry of a collection into a value, abstracting the operation of collections
  /// like vectors or hash maps. It ensures that even in complex collection structures, entries can be seamlessly managed
  /// and manipulated as values.
  pub trait EntryToVal< Collection >
  {
    /// The type of values stored in the collection. This might be distinct from `Entry` in complex collections.
    /// For example, in a `HashMap`, while `Entry` might be a ( key, value ) tuple, `Val` might only be the value part.
    type Val;

    /// Converts an entry into a value representation specific to the type of collection. This conversion is crucial
    /// for handling operations on entries, especially when they need to be treated or accessed as individual values,
    /// such as retrieving the value part from a key-value pair in a hash map.
    fn entry_to_val( self ) -> Self::Val;
  }

  impl< C, E > EntryToVal< C > for E
  where
    C : Collection< Entry = E >,
  {
    type Val = C::Val;

    fn entry_to_val( self ) -> Self::Val
    {
      C::entry_to_val( self )
    }
  }

  /// Provides a mechanism for transforming a value back into a collection-specific entry format.
  ///
  /// This trait is particularly valuable in scenarios where the operations on a collection require
  /// not just the manipulation of values but also the re-integration of these values as entries.
  /// It is especially crucial in complex data structures, such as `HashMap`s, where entries
  /// often involve a key-value pair, and simple values need to be restructured to fit this model
  /// for operations like insertion or update.

  pub trait CollectionValToEntry< Val >
  {
    /// The specific type of entry that corresponds to the value within the collection.
    /// For example, in a `HashMap`, this might be a tuple of a key and a value.
    type Entry;

    /// Converts a value into a collection-specific entry, facilitating operations that modify
    /// the collection. This method is key for ensuring that values can be correctly integrated
    /// back into the collection, particularly when the entry type is more complex than the value.
    ///
    /// # Parameters
    /// * `val` - The value to be converted into an entry.
    ///
    /// # Returns
    /// Returns the entry constructed from the provided value, ready for insertion or other modifications.
    ///
    /// # Example
    /// ```
    /// use former_types::CollectionValToEntry; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
    ///
    /// struct PairMap;
    ///
    /// impl CollectionValToEntry< ( i32, i32 ) > for PairMap
    /// {
    ///   type Entry = ( String, i32 );
    ///
    ///   fn val_to_entry( val : ( i32, i32 ) ) -> Self::Entry
    ///   {
    ///     (val.0.to_string(), val.1)
    ///   }
    /// }
    /// ```
    fn val_to_entry( val : Val ) -> Self::Entry;
  }

  /// Facilitates the conversion of values back into entries for specific collection types.
  ///
  /// This trait wraps the functionality of `CollectionValToEntry`, providing a more ergonomic
  /// interface for converting values directly within the type they pertain to. It is useful
  /// in maintaining the integrity of collection operations, especially when dealing with
  /// sophisticated structures that separate the concept of values and entries, such as `HashMap`s
  /// and other associative collections.
  pub trait ValToEntry< Collection >
  {
    /// Represents the type of entry that corresponds to the value within the collection.
    type Entry;

    /// Transforms the instance (value) into an entry compatible with the specified collection.
    /// This conversion is essential for operations like insertion or modification within the collection,
    /// where the value needs to be formatted as an entry.
    ///
    /// # Returns
    /// Returns the entry constructed from the instance of the value, ready for integration into the collection.
    ///
    /// # Example
    /// ```
    /// use former_types::ValToEntry; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
    ///
    /// struct PairMap;
    ///
    /// impl ValToEntry< PairMap > for (i32, i32)
    /// {
    ///   type Entry = ( String, i32 );
    ///
    ///   fn val_to_entry( self ) -> Self::Entry
    ///   {
    ///     (self.0.to_string(), self.1)
    ///   }
    /// }
    /// ```
    fn val_to_entry( self ) -> Self::Entry;
  }

  impl< C, Val > ValToEntry< C > for Val
  where
    C : CollectionValToEntry< Val >,
  {
    type Entry = C::Entry;

    /// Invokes the `val_to_entry` function of the `CollectionValToEntry` trait to convert the value to an entry.
    fn val_to_entry( self ) -> C::Entry
    {
      C::val_to_entry( self )
    }
  }

  /// Represents a collection by defining the types of entries and values it handles.
  ///
  /// This trait abstracts the nature of collections in data structures, facilitating the handling of contained
  /// entries and values, especially in scenarios where the structure of the collection allows for complex relationships,
  /// such as `HashMap`s. It not only identifies what constitutes an entry and a value in the context of the collection
  /// but also provides utility for converting between these two, which is critical in operations involving entry manipulation
  /// and value retrieval.

  pub trait Collection
  {
    /// The type of entries that can be added to the collection. This type can differ from `Val` in collections like `HashMap`,
    /// where an entry might represent a key-value pair, and `Val` could represent just the value or the key.
    type Entry;

    /// The type of values stored in the collection. This might be distinct from `Entry` in complex collections.
    /// For example, in a `HashMap`, while `Entry` might be a ( key, value ) tuple, `Val` might only be the value part.
    type Val;

    /// Converts an entry to its corresponding value within the collection. This function is essential for abstracting
    /// the collection's internal representation from the values it manipulates.
    fn entry_to_val( e : Self::Entry ) -> Self::Val;
  }

  /// Provides functionality to add individual entries to a collection.
  ///
  /// This trait extends the basic `Collection` trait by introducing a method to add entries to a collection.
  /// It is designed to handle the collection's specific requirements and rules for adding entries, such as
  /// managing duplicates, maintaining order, or handling capacity constraints.
  pub trait CollectionAdd : Collection
  {
    /// Adds an entry to the collection and returns a boolean indicating the success of the operation.
    ///
    /// Implementations should ensure that the entry is added according to the rules of the collection,
    /// which might involve checking for duplicates, ordering, or capacity limits.
    ///
    /// # Parameters
    ///
    /// * `e`: The entry to be added to the collection, where the type `Entry` is defined by the `Collection` trait.
    ///
    /// # Returns
    ///
    /// Returns `true` if the entry was successfully added, or `false` if not added due to reasons such as
    /// the entry already existing in the collection or the collection reaching its capacity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    /// use former_types::{ Collection, CollectionAdd }; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
    ///
    /// struct MyCollection
    /// {
    ///   entries : Vec< i32 >,
    /// }
    ///
    /// impl Collection for MyCollection
    /// {
    ///   type Entry = i32;
    ///   type Val = i32;
    ///
    ///   #[ inline( always ) ]
    ///   fn entry_to_val( e : Self::Entry ) -> Self::Val
    ///   {
    ///     e
    ///   }
    ///
    /// }
    ///
    /// impl CollectionAdd for MyCollection
    /// {
    ///   fn add( &mut self, e : Self::Entry ) -> bool
    ///   {
    ///     if self.entries.contains( &e )
    ///     {
    ///       false
    ///     }
    ///     else
    ///     {
    ///       self.entries.push( e );
    ///       true
    ///     }
    ///   }
    /// }
    ///
    /// let mut collection = MyCollection { entries : vec![] };
    /// assert!( collection.add( 10 ) ); // Returns true, entry added
    /// assert!( !collection.add( 10 ) ); // Returns false, entry already exists
    /// ```
    fn add( &mut self, e : Self::Entry ) -> bool;
  }

  /// Defines the capability to replace all entries in a collection with a new set of entries.
  ///
  /// This trait extends the `Collection` trait by providing a method to replace the existing entries in
  /// the collection with a new set. This can be useful for resetting the collection's contents or bulk-updating
  /// them based on external criteria or operations.
  pub trait CollectionAssign : Collection
  where
    Self : IntoIterator< Item = Self::Entry >,
  {
    /// Replaces all entries in the collection with the provided entries and returns the count of new entries added.
    ///
    /// This method clears the existing entries and populates the collection with new ones provided by an iterator.
    /// It is ideal for scenarios where the collection needs to be refreshed or updated with a new batch of entries.
    ///
    /// # Parameters
    ///
    /// * `entries` : An iterator over the entries to be added to the collection. The entries must conform to
    ///   the `Entry` type defined by the `Collection` trait.
    ///
    /// # Returns
    ///
    /// Returns the number of entries successfully added to the collection. This count may differ from the total
    /// number of entries in the iterator if the collection imposes restrictions such as capacity limits or duplicate
    /// handling.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use former_types::{ Collection, CollectionAssign }; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
    ///
    /// struct MyCollection
    /// {
    ///   entries : Vec< i32 >,
    /// }
    ///
    /// impl Collection for MyCollection
    /// {
    ///   type Entry = i32;
    ///   type Val = i32;
    ///
    ///   #[ inline( always ) ]
    ///   fn entry_to_val( e : Self::Entry ) -> Self::Val
    ///   {
    ///     e
    ///   }
    ///
    /// }
    ///
    /// impl IntoIterator for MyCollection
    /// {
    ///   type Item = i32;
    ///   // type IntoIter = std::vec::IntoIter< i32 >;
    ///   type IntoIter = collection_tools::vec::IntoIter< i32 >;
    ///   // qqq : zzz : make sure collection_tools has itearators -- done
    ///
    ///   fn into_iter( self ) -> Self::IntoIter
    ///   {
    ///     self.entries.into_iter() // Create an iterator from the internal HashSet.
    ///   }
    /// }
    ///
    /// impl CollectionAssign for MyCollection
    /// {
    ///   fn assign< Entries >( &mut self, entries : Entries ) -> usize
    ///   where
    ///     Entries : IntoIterator< Item = Self::Entry >,
    ///   {
    ///     self.entries.clear();
    ///     self.entries.extend( entries );
    ///     self.entries.len()
    ///   }
    /// }
    ///
    /// let mut collection = MyCollection { entries : vec![ 1, 2, 3 ] };
    /// let new_elements = vec![ 4, 5, 6 ];
    /// assert_eq!( collection.assign( new_elements ), 3 ); // Collection now contains [ 4, 5, 6 ]
    /// ```
    fn assign< Entries >( &mut self, entries : Entries ) -> usize
    where
      Entries : IntoIterator< Item = Self::Entry >;
  }

  // =

  /// A builder structure for constructing collections with a fluent and flexible interface.
  #[ derive( Default ) ]
  pub struct CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition,
    Definition::Storage : CollectionAdd< Entry = E >,
  {
    storage : Definition::Storage,
    context : core::option::Option< Definition::Context >,
    on_end : core::option::Option< Definition::End >,
  }

  use core::fmt;
  impl< E, Definition > fmt::Debug for CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition,
    Definition::Storage : CollectionAdd< Entry = E >,
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f
      .debug_struct( "CollectionFormer" )
      .field( "storage", &"Storage Present" )
      .field( "context", &self.context.as_ref().map( |_| "Context Present" ) )
      .field( "on_end", &self.on_end.as_ref().map( |_| "End Present" ) )
      .finish()
    }
  }

  impl< E, Definition > CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition,
    Definition::Storage : CollectionAdd< Entry = E >,
  {
    /// Begins the construction process of a collection with optional initial storage and context,
    /// setting up an `on_end` completion handler to finalize the collection's construction.
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

    /// Provides a variation of the `begin` method allowing for coercion of the end handler,
    /// facilitating ease of integration with different end conditions.
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
      on_end.call( self.storage, context )
    }

    /// Alias for the `end` method to align with typical builder pattern terminologies.
    #[ inline( always ) ]
    pub fn form( self ) -> Definition::Formed
    {
      self.end()
    }

    /// Replaces the current storage with a provided storage, allowing for resetting or
    /// redirection of the building process.
    #[ inline( always ) ]
    pub fn replace( mut self, storage : Definition::Storage ) -> Self
    {
      self.storage = storage;
      self
    }
  }

  impl< E, Storage, Formed, Definition > CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition< Context = (), Storage = Storage, Formed = Formed >,
    Definition::Storage : CollectionAdd< Entry = E >,
  {
    /// Constructs a new `CollectionFormer` instance, starting with an empty storage.
    /// This method serves as the entry point for the builder pattern, facilitating the
    /// creation of a new collection.
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

    /// Variant of the `new` method allowing for end condition coercion, providing flexibility
    /// in specifying different types of end conditions dynamically.
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

  impl< E, Definition > CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition,
    Definition::Storage : CollectionAdd< Entry = E >,
  {

    /// Appends an entry to the end of the storage, expanding the internal collection.
    #[ inline( always ) ]
    pub fn add< IntoElement >( mut self, entry : IntoElement ) -> Self
    where IntoElement : core::convert::Into< E >,
    {
      CollectionAdd::add( &mut self.storage, entry.into() );
      self
    }

  }

  //

  impl< E, Definition > FormerBegin< Definition >
  for CollectionFormer< E, Definition >
  where
    Definition : FormerDefinition,
    Definition::Storage : CollectionAdd< Entry = E >,
  {

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

}

/// Former of a binary tree map.
mod btree_map;
/// Former of a binary tree set.
mod btree_set;
/// Former of a binary heap.
mod binary_heap;
/// Former of a hash map.
mod hash_map;
/// Former of a hash set.
mod hash_set;
/// Former of a linked list.
mod linked_list;
/// Former of a vector.
mod vector;
/// Former of a vector deque.
mod vector_deque;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {

    EntryToVal,
    CollectionValToEntry,
    ValToEntry,

    Collection,
    CollectionAdd,
    CollectionAssign,
    CollectionFormer,

  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    btree_map::*,
    btree_set::*,
    binary_heap::*,
    hash_map::*,
    hash_set::*,
    linked_list::*,
    vector::*,
    vector_deque::*,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

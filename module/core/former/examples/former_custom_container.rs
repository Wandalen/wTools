//! Example former_custom_container.rs
//!
//! This example demonstrates how to define and use a custom container with former.
//! The custom container implemented here is a `LoggingSet`, which extends the basic `HashSet` behavior
//! by logging each addition. This example illustrates how to integrate such custom containers with the
//! Former trait system for use in structured data types.

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashSet;

  // Custom container that logs additions.
  #[ derive( Debug, PartialEq ) ]
  pub struct LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    set : HashSet< K >, // Internal HashSet to store the elements.
  }

  // Implement default for the custom container.
  impl< K > Default for LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      Self
      {
        set : Default::default() // Initialize the internal HashSet.
      }
    }
  }

  // Allow the custom container to be converted into an iterator, to iterate over the elements.
  impl< K > IntoIterator for LoggingSet< K >
  where
    K : std::cmp::Eq + std::hash::Hash,
  {
    type Item = K;
    type IntoIter = std::collections::hash_set::IntoIter< K >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.set.into_iter() // Create an iterator from the internal HashSet.
    }
  }

  // Similar iterator functionality but for borrowing the elements.
  impl<'a, K> IntoIterator for &'a LoggingSet< K >
  where
    K : std::cmp::Eq + std::hash::Hash,
  {
    type Item = &'a K;
    type IntoIter = std::collections::hash_set::Iter< 'a, K >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.set.iter() // Borrow the elements via an iterator.
    }
  }

  // Implement the Container trait to integrate with the former system.
  impl< K > former::Container for LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    type Entry = K;
    type Val = K;

    #[ inline( always ) ]
    fn entry_to_val( e : Self::Entry ) -> Self::Val
    {
      e // Direct mapping of entries to values.
    }
  }

  // Implement ContainerAdd to handle adding elements to the custom container.
  impl< K > former::ContainerAdd for LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    #[ inline( always ) ]
    fn add( &mut self, e : Self::Entry ) -> bool
    {
      self.set.insert( e ) // Log the addition and add the element to the internal HashSet.
    }
  }

  // Implement ContainerAssign to handle bulk assignment of elements.
  impl< K > former::ContainerAssign for LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    fn assign< Elements >( &mut self, elements : Elements ) -> usize
    where
      Elements : IntoIterator< Item = Self::Entry >
    {
      let initial_len = self.set.len();
      self.set.extend( elements ); // Extend the set with a collection of elements.
      self.set.len() - initial_len // Return the number of elements added.
    }
  }

  // Implement ContainerValToEntry to convert values back to entries.
  impl< K > former::ContainerValToEntry< K > for LoggingSet< K >
  where
    K : core::cmp::Eq + core::hash::Hash,
  {
    type Entry = K;
    #[ inline( always ) ]
    fn val_to_entry( val : K ) -> Self::Entry
    {
      val // Direct conversion of value to entry.
    }
  }

  // = storage

  // Define storage behavior for the custom container.
  impl< K > former::Storage
  for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
    type Preformed = LoggingSet< K >; // Define the type after the forming process.
  }

  // Implement the preforming behavior to finalize the storage.
  impl< K > former::StoragePreform
  for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
    fn preform( self ) -> Self::Preformed
    {
      self // Return the container as is.
    }
  }

  // = definition types

  // Definitions related to the type settings for the LoggingSet, which detail how the container should behave with former.

  /// Holds generic parameter types for forming operations related to `LoggingSet`.
  #[ derive( Debug, Default ) ]
  pub struct LoggingSetDefinitionTypes< K, Context = (), Formed = LoggingSet< K > >
  {
    _phantom : core::marker::PhantomData< ( K, Context, Formed ) >, // PhantomData is used to handle generic parameters safely.
  }

  /// Specifies the storage, formed type, and context for the `LoggingSet` when used in a forming process.
  impl< K, Context, Formed > former::FormerDefinitionTypes
  for LoggingSetDefinitionTypes< K, Context, Formed >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
    type Storage = LoggingSet< K >; // Specifies that `LoggingSet<K>` is used as the storage.
    type Formed = Formed;           // The final formed type after the forming process.
    type Context = Context;         // The context required for forming, can be specified by the user.
  }

  // = definition

  /// Provides a complete definition for `LoggingSet` including the end condition of the forming process.
  #[ derive( Debug, Default ) ]
  pub struct LoggingSetDefinition< K, Context = (), Formed = LoggingSet< K >, End = former::ReturnStorage >
  {
    _phantom : core::marker::PhantomData< ( K, Context, Formed, End ) >,
  }

  /// Associates the `LoggingSet` with a specific forming process and defines its behavior.
  impl< K, Context, Formed, End > former::FormerDefinition
  for LoggingSetDefinition< K, Context, Formed, End >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
    End : former::FormingEnd< LoggingSetDefinitionTypes< K, Context, Formed > >,
  {
    type Storage = LoggingSet< K >; // The storage type during the formation process.
    type Formed = Formed;           // The type resulting from the formation process.
    type Context = Context;         // The context used during the formation process.
    type Types = LoggingSetDefinitionTypes< K, Context, Formed >; // The associated type settings.
    type End = End;                 // The ending condition for the forming process.
  }

  // = mutator

  /// Optional: Implements mutating capabilities to modify the forming process of `LoggingSet` if needed.
  impl< K, Context, Formed > former::FormerMutator
  for LoggingSetDefinitionTypes< K, Context, Formed >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
  }

  // = Entity To

  /// Associates the `LoggingSet` with a specific `Former` for use in forming processes.
  impl< K, Definition > former::EntityToFormer< Definition > for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
    Definition : former::FormerDefinition
    <
      Storage = LoggingSet< K >,
      Types = LoggingSetDefinitionTypes
      <
        K,
        < Definition as former::FormerDefinition >::Context,
        < Definition as former::FormerDefinition >::Formed,
      >,
    >,
    Definition::End : former::FormingEnd< Definition::Types >,
  {
    type Former = LoggingSetAsSubformer< K, Definition::Context, Definition::Formed, Definition::End >;
  }

  /// Specifies the storage for `LoggingSet`.
  impl< K > former::EntityToStorage
  for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
    type Storage = LoggingSet< K >;
  }

  /// Defines the relationship between `LoggingSet` and its formal definition within the forming system.
  impl< K, Context, Formed, End > former::EntityToDefinition< Context, Formed, End >
  for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
    End : former::FormingEnd< LoggingSetDefinitionTypes< K, Context, Formed > >,
  {
    type Definition = LoggingSetDefinition< K, Context, Formed, End >;
    type Types = LoggingSetDefinitionTypes< K, Context, Formed >;
  }

  /// Provides type-specific settings for the formation process related to `LoggingSet`.
  impl< K, Context, Formed > former::EntityToDefinitionTypes< Context, Formed >
  for LoggingSet< K >
  where
    K : ::core::cmp::Eq + ::core::hash::Hash,
  {
    type Types = LoggingSetDefinitionTypes< K, Context, Formed >;
  }

  // = subformer

  // Subformer type alias simplifies the usage of `ContainerFormer` with `LoggingSet`.
  pub type LoggingSetAsSubformer< K, Context, Formed, End > =
  former::ContainerFormer::< K, LoggingSetDefinition< K, Context, Formed, End > >;

  // == use custom container

  /// Parent required for the template.
  #[ derive( Debug, Default, PartialEq, former::Former ) ]
  pub struct Parent
  {
    #[ container ]
    children : LoggingSet< i32 >,
  }

  // Using the builder pattern provided by Former to manipulate Parent
  let parent = Parent::former()
  .children()
    .add(10)
    .add(20)
    .add(10)
    .end()
  .form();

  println!("Got: {:?}", parent);
  // > Parent { children: LoggingSet { set: {10, 20} } }

}

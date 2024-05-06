// Example former_custom_subformer.rs

//!
//! This example illustrates the implementation of nested builder patterns in Rust using the `Former` trait, emphasizing a parent-child relationship. Here, the `Parent` struct utilizes `ChildFormer` as a custom subformer to dynamically manage its `child` field—a `HashMap`. Each child in the `HashMap` is uniquely identified and configured via the `ChildFormer`.
//!
//! The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.
//!
//! #### Types of Setters
//!
//! It's crucial to understand the differences among subform setters, container setters, and scalar setters:
//!
//! - **Scalar Setter**: Directly sets scalar values or simple fields within the forming entity. Unlike subform or container setters that manage complex objects or collections, scalar setters handle basic data types or individual fields. These are typically straightforward setter methods that do not involve nested formers or additional structuring.
//!
//! - **Container Setter**: Returns a former of the container itself, offering an interface to manage the container as a whole rather than its individual elements. This type of setter is useful for applying configurations or validations to the entire collection, such as a `HashMap` of children.
//!
//! - **Subform Setter**: Returns a former of an element within a container, providing an interface to individually form each element. For example, the `child` method acts as a subform setter, allowing for the addition and configuration of individual `Child` entities within the `Parent`'s `HashMap`.
//!
//! Each type of setter is designed to address different needs in the formation process, ensuring that users can build complex, nested structures or simply set individual field values as required.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Parent
  {
    // Use `hint = true` to gennerate sketch of setter.
    #[ subform( setter = false, hint = false ) ]
    child : HashMap< String, Child >,
  }

  /// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
  /// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
  /// integrating them into the formation process of the parent entity.
  ///
  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  {

    #[ inline( always ) ]
    pub fn child( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      self._child_add::< ChildFormer< _ >, _, >()
      .name( name )
    }

  }

  // Required to define how `value` is converted into pair `( key, value )`
  impl former::ValToEntry< HashMap< String, Child > > for Child
  {
    type Entry = ( String, Child );
    #[ inline( always ) ]
    fn val_to_entry( self ) -> Self::Entry
    {
      ( self.name.clone(), self )
    }
  }

  let ca = Parent::former()
  .child( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .child( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  dbg!( &ca );
  // > &ca = Parent {
  // >     child: {
  // >          "echo": Child {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Child {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}

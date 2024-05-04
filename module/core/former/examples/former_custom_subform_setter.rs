// example former_custom_subformer.rs

//! This example demonstrates the use of the `Former` trait to implement nested builder patterns
//! in Rust using a parent-child relationship. The `Parent` struct uses `ChildFormer` as a custom
//! subformer to dynamically construct its `child` field, which is a `HashMap`. Each entry in the
//! `HashMap` represents a child with unique attributes managed through the `ChildFormer`.
//!
//! The `child` function in `ParentFormer` is particularly noteworthy as it leverages the
//! `ChildFormer` to add and configure children by their names directly within the builder pattern
//! of the `Parent`. This approach showcases the flexibility of the `former` crate in handling
//! complex nested data structures and providing a clear, fluent interface for object construction.

// xxx2 : description is not good enough. it should be made stress that example show how to write custom subform setter. also dedicate a paragraph to explain difference between subform setter which returns former of element of container exposing interface to form an element and container setter which returns container former exposing interface to containet itself, not its element
// xxx2 : finish example former_custom_subformer
// xxx : zzz : implement example former_custom_container

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use std::collections::HashMap;
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
    #[ subform( setter = false, hint = true ) ]
    child : HashMap< String, Child >,
  }

  /// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
  /// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
  /// integrating them into the formation process of the parent entity. After creation, the name of the child is immediately set,
  /// facilitating the customization and integration of child entities within the overall structure of the parent.
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

  impl former::ValToElement< HashMap< String, Child > > for Child
  {
    type Element = ( String, Child );
    #[ inline( always ) ]
    fn val_to_element( self ) -> Self::Element
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

// Example former_custom_container_setter.rs

//!
//! This example demonstrates the use of container setters to manage complex nested data structures with the `Former` trait, focusing on a parent-child relationship structured around a container `HashMap`. Unlike typical builder patterns that add individual elements using subform setters, this example uses a container setter to manage the entire collection of children.
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

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // Use `#[ debug ]` to expand and debug generate code.
  // #[ debug ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  // Use `#[ debug ]` to expand and debug generate code.
  // #[ debug ]
  pub struct Parent
  {
    // Use `hint = true` to gennerate sketch of setter.
    #[ container( setter = false, hint = false ) ]
    children : HashMap< String, Child >,
  }

  /// The containr setter provides a container setter that returns a ContainerFormer tailored for managing a collection of child entities. It employs a generic container definition to facilitate operations on the entire collection, such as adding or updating elements.
  impl< Definition, > ParentFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = ParentFormerStorage >,
  {

    #[ inline( always ) ]
    pub fn children( self ) -> former::ContainerFormer::
    <
      ( String, Child ),
      former::HashMapDefinition< String, Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, >
    >
    {
      self._children_container_former()
    }

  }

  let echo = Child { name : "echo".to_string(), description : "prints all subjects and properties".to_string() };
  let exit = Child { name : "exit".to_string(), description : "just exit".to_string() };
  let ca = Parent::former()
  .children()
    .add( ( echo.name.clone(), echo ) )
    .add( ( exit.name.clone(), exit ) )
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

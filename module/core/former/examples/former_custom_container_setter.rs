// Example former_custom_container_setter.rs

//!
//! This example demonstrates the use of container setters to manage complex nested data structures with the `Former` trait, focusing on a parent-child relationship structured around a container `HashMap`. Unlike typical builder patterns that add individual elements using subform setters, this example uses a container setter to manage the entire collection of children.
//!
//! #### Custom Subform Setter
//!
//! The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.
//!
//! xxx : extend by information about Scalar Setter
//! #### Subform Setter vs. Container Setter
//!
//! It's important to distinguish between a subform setter and a container setter:
//! - **Subform Setter**: This returns a former of an element within a container, providing an interface to individually form each element. In this example, `child` acts as a subform setter, allowing for the addition and configuration of individual `Child` entities within the `Parent`'s `HashMap`.
//! - **Container Setter**: Conversely, a container setter returns a former of the container itself, offering an interface to manage the container as a whole rather than its individual elements. It would be used if one needed to apply configurations or validations to the entire `HashMap` of children, rather than to individual children.
//!

// zzz : duplicate into readme

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
    #[ container( setter = false, hint = true ) ]
    children : HashMap< String, Child >,
  }

  /// The containr setter provides a container setter that returns a ContainerSubformer tailored for managing a collection of child entities. It employs a generic container definition to facilitate operations on the entire collection, such as adding or updating elements.
  impl< Definition, > ParentFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = ParentFormerStorage >,
  {

    #[ inline( always ) ]
    pub fn children( self ) -> former::ContainerSubformer::
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

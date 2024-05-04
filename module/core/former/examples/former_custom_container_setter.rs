// Example former_custom_container_setter.rs

//!
//! This example illustrates the implementation of nested builder patterns in Rust using the `Former` trait, emphasizing a parent-child relationship. Here, the `Parent` struct utilizes `ChildFormer` as a custom subformer to dynamically manage its `child` field—a `HashMap`. Each child in the `HashMap` is uniquely identified and configured via the `ChildFormer`.
//!
//! #### Custom Subform Setter
//!
//! The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.
//!
//! #### Subform Setter vs. Container Setter
//!
//! It's important to distinguish between a subform setter and a container setter:
//! - **Subform Setter**: This returns a former of an element within a container, providing an interface to individually form each element. In this example, `child` acts as a subform setter, allowing for the addition and configuration of individual `Child` entities within the `Parent`'s `HashMap`.
//! - **Container Setter**: Conversely, a container setter returns a former of the container itself, offering an interface to manage the container as a whole rather than its individual elements. It would be used if one needed to apply configurations or validations to the entire `HashMap` of children, rather than to individual children.
//!

// xxx : improve description of this example. container setter unlike subform setter expose interface of container, not element itself

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
    #[ container( setter = true, hint = true ) ]
    child : HashMap< String, Child >,
  }

  // impl former::ValToElement< HashMap< String, Child > > for Child
  // {
  //   type Element = ( String, Child );
  //   #[ inline( always ) ]
  //   fn val_to_element( self ) -> Self::Element
  //   {
  //     ( self.name.clone(), self )
  //   }
  // }

  let ca = Parent::former()
  .child()
    .add( Child { name : "echo".to_string(), description : "prints all subjects and properties".to_string() } )
    .add( Child { name : "exit".to_string(), description : "just exit".to_string() } )
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

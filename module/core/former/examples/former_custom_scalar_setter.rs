// Example former_custom_scalar_setter.rs

//! ## Example : Custom Scalar Setter
//!
//! Use of a scalar setter within a `Former` trait implementation to directly assign a `HashMap` of `Child` entities to a `Parent` structure using a custom setter function.
//!
//! Unlike the more complex subform and collection setters shown in previous examples, this example focuses on a straightforward approach to directly set a scalar value within a parent entity. The `Parent` struct manages a `HashMap` of `Child` entities, and the scalar setter is used to set the entire `HashMap` directly. The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.
//!
//! #### Types of Setters
//!
//! It's crucial to understand the differences among subform setters, collection setters, and scalar setters:
//!
//! - **Scalar Setter**: Directly sets scalar values or simple fields within the forming entity. Unlike subform or collection setters that manage complex objects or collections, scalar setters handle basic data types or individual fields. These are typically straightforward setter methods that do not involve nested formers or additional structuring.
//!
//! - **Collection Setter**: Returns a former of the collection itself, offering an interface to manage the collection as a whole rather than its individual elements. This type of setter is useful for applying configurations or validations to the entire collection, such as a `HashMap` of children.
//!
//! - **Subform Setter**: Returns a former of an element within a collection, providing an interface to individually form each element. For example, the `child` method acts as a subform setter, allowing for the addition and configuration of individual `Child` entities within the `Parent`'s `HashMap`.
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
    #[ scalar( setter = false, hint = false ) ]
    children : HashMap< String, Child >,
  }

  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = ParentFormerStorage >,
  {
    #[ inline ]
    pub fn children< Src >( mut self, src : Src ) -> Self
    where
      Src : ::core::convert::Into< HashMap< String, Child > >,
    {
      debug_assert!( self.storage.children.is_none() );
      self.storage.children = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
      self
    }
  }

  let echo = Child { name : "echo".to_string(), description : "prints all subjects and properties".to_string() };
  let exit = Child { name : "exit".to_string(), description : "just exit".to_string() };
  let mut children = HashMap::new();
  children.insert( echo.name.clone(), echo );
  children.insert( exit.name.clone(), exit );
  let ca = Parent::former()
  .children( children )
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

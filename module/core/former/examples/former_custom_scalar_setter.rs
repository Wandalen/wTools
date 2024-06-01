// Example former_custom_scalar_setter.rs

//! ## Example : Custom Scalar Setter
//!
//! Use of a scalar setter within a `Former` implementation to directly assign a `HashMap` of `Child` entities to a `Parent` structure using a custom setter function.
//!
//! Unlike the more complex subform and collection setters shown in previous examples, this example focuses on a straightforward approach to directly set a scalar value within a parent entity. The `Parent` struct manages a `HashMap` of `Child` entities, and the scalar setter is used to set the entire `HashMap` directly. The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.
//!
//! #### Types of Setters / Subformers
//!
//! Understanding the distinctions among the types of setters or subformers is essential for effectively employing the builder pattern in object construction. Each type of setter is designed to meet specific needs in building complex, structured data entities:
//!
//! - **Scalar Setter**: Handles the direct assignment of scalar values or simple fields within an entity. These setters manage basic data types or individual fields and do not involve nested formers or complex structuring.
//!
//! - **Subform Collection Setter**: Facilitates the management of a collection as a whole by returning a former that provides an interface to configure the entire collection. This setter is beneficial for applying uniform configurations or validations to all elements in a collection, such as a `HashMap` of children.
//!
//! - **Subform Entry Setter**: This setter allows for the individual formation of elements within a collection. It returns a former for each element, enabling detailed configuration and addition of complex elements within collections, exemplified by managing `Child` entities within a `Parent`'s `HashMap`.
//!
//! - **Subform Scalar Setter**: Similar to the subform entry setter but designed for scalar fields that have a former implementation. This setter does not collect instances into a collection because there is no collection involved, only a scalar field. It is used when the scalar field itself needs to be configured or modified through its dedicated former.
//!
//! These setters ensure that developers can precisely and efficiently set properties, manage collections, and configure complex structures within their applications.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
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
    // Use `debug` to gennerate sketch of setter.
    #[ scalar( setter = false ) ]
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

// Example former_custom_subform_scalar.rs

//!
//! ## Example : Custom Subform Scalar Setter
//!
//! Implementation of a custom subform scalar setter using the `Former`.
//!
//! This example focuses on the usage of a subform scalar setter to manage complex scalar types within a parent structure.
//! Unlike more general subform setters that handle collections, this setter specifically configures scalar fields that have
//! their own formers, allowing for detailed configuration within a nested builder pattern.
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


#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc",not( feature = "no_std" ) ) ) ) ) ]
fn main()
{}

// Ensures the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc",not( feature = "no_std" ) ) ) ) ]
fn main()
{
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // Optional: Use `#[debug]` to expand and debug generated code.
  // #[debug]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct designed to hold a single Child instance using subform scalar
  #[ derive( Debug, PartialEq, Former ) ]
  // Optional: Use `#[debug]` to expand and debug generated code.
  // #[debug]
  pub struct Parent
  {
    // The `subform_scalar` attribute is used to specify that the 'child' field has its own former
    // and can be individually configured via a subform setter. This is not a collection but a single scalar entity.
    #[ subform_scalar( setter = false ) ]
    child : Child,
  }

  /// Extends `ParentFormer` to include a method that initializes and configures a subformer for the 'child' field.
  /// This function demonstrates the dynamic addition of a named child, leveraging a subformer to specify detailed properties.
  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  {
    #[ inline( always ) ]
    pub fn child( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      self._child_subform_scalar::< ChildFormer< _ >, _, >().name( name )
    }
  }

  // Creating an instance of `Parent` using the builder pattern to configure `Child`
  let ca = Parent::former()
    .child( "echo" ) // starts the configuration of the `child` subformer
    .description( "prints all subjects and properties" ) // sets additional properties for the `Child`
    .end() // finalize the child configuration
    .form(); // finalize the Parent configuration

  dbg!( &ca ); // Outputs the structured data for review
  // Expected output:
  //> Parent {
  //>   child: Child {
  //>       name: "echo",
  //>       description: "prints all subjects and properties",
  //>   },
  //> }
}

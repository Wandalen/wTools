// Example former_custom_subform_entry.rs

//! ## Example : Custom Subform Entry Setter
//!
//! This example illustrates the implementation of nested builder patterns using the `Former`, emphasizing a parent-child relationship. Here, the `Parent` struct utilizes `ChildFormer` as a custom subformer to dynamically manage its `child` field—a `HashMap`. Each child in the `HashMap` is uniquely identified and configured via the `ChildFormer`.
//!
//! The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.
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
    #[ subform_entry( setter = false ) ]
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
      self._child_subform_entry::< ChildFormer< _ >, _, >()
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

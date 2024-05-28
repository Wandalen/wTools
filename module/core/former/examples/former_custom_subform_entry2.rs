// Example former_custom_subformer2.rs

//!
//! This example extends the demonstration of nested builder patterns using the `Former` trait, highlighting a parent-child relationship similar to the `former_custom_subformer.rs`. However, this variant, `former_custom_subformer2.rs`, showcases a more flexible but complex approach to managing the `child` field in the `Parent` struct—a `HashMap` of `Child` entities. Instead of relying on a predefined subformer setter (`_child_subform_entry`), this example constructs the subformer logic directly using closures. This method provides greater control over how children are added and managed within the `Parent`.
//!
//! #### Custom Subform Setter
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

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Clone, Debug, PartialEq, Former ) ]
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

  // Use ChildFormer as custom subformer for ParentFormer to add children by name.
  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  {

    /// Adds a named child entity to the `Parent`'s `child` field using a custom subformer setup.
    /// This method simplifies the process of dynamically adding child entities with specified names,
    /// providing a basic yet powerful example of custom subformer implementation.
    ///
    #[ inline( always ) ]
    pub fn child1( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      let on_end = | substorage : ChildFormerStorage, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let preformed = former::StoragePreform::preform( substorage );

        if super_former.storage.child.is_none()
        {
          super_former.storage.child = Some( Default::default() );
        }

        // add instance to the collection
        super_former.storage.child.as_mut().unwrap()
        .entry( preformed.name.clone() )
        .or_insert( preformed.clone() );

        super_former
      };
      let subformer = ChildAsSubformer::< Self, _ >::begin( None, Some( self ), former::FormingEndClosure::new( on_end ) );
      subformer.name( name )
    }

    /// Dynamically adds named child entities to the `Parent` structure using a custom subformer.
    /// Unlike traditional methods that might use predefined setters like `_child_subform_entry`, this function
    /// explicitly constructs a subformer setup through a closure to provide greater flexibility and control.
    ///
    #[ inline( always ) ]
    pub fn child2( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      let on_end = | substorage : ChildFormerStorage, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let preformed = former::StoragePreform::preform( substorage );

        if super_former.storage.child.is_none()
        {
          super_former.storage.child = Some( Default::default() );
        }

        // add instance to the collection
        super_former.storage.child.as_mut().unwrap()
        .entry( preformed.name.clone() )
        .or_insert( preformed.clone() );

        // custom logic to add two instances to the collection
        super_former.storage.child.as_mut().unwrap()
        .entry( format!( "{}_2", preformed.name ) )
        .or_insert( preformed.clone() );

        super_former
      };
      let subformer = ChildAsSubformer::< Self, _ >::begin( None, Some( self ), former::FormingEndClosure::new( on_end ) );
      subformer.name( name )
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
  .child1( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .child2( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  dbg!( &ca );
  // > &ca = Parent {
  // >     child: {
  // >         "echo": Child {
  // >             name: "echo",
  // >             description: "prints all subjects and properties",
  // >         },
  // >         "exit": Child {
  // >             name: "exit",
  // >             description: "just exit",
  // >         },
  // >         "exit_2": Child {
  // >             name: "exit",
  // >             description: "just exit",
  // >         },
  // >     },
  // > }

}

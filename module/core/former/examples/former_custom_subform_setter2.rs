// Example former_custom_subformer2.rs

//!
//! This example extends the demonstration of nested builder patterns using the `Former` trait, highlighting a parent-child relationship similar to the `former_custom_subformer.rs`. However, this variant, `former_custom_subformer2.rs`, showcases a more flexible but complex approach to managing the `child` field in the `Parent` struct—a `HashMap` of `Child` entities. Instead of relying on a predefined subformer setter (`_child_add`), this example constructs the subformer logic directly using closures. This method provides greater control over how children are added and managed within the `Parent`.
//!
//! #### Custom Subform Setter
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

// zzz : duplicate into readme

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Clone, Debug, PartialEq, Former ) ]
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

        // add instance to the container
        super_former.storage.child.as_mut().unwrap()
        .entry( preformed.name.clone() )
        .or_insert( preformed.clone() );

        super_former
      };
      let subformer = ChildAsSubformer::< Self, _ >::begin( None, Some( self ), former::FormingEndClosure::new( on_end ) );
      subformer.name( name )
    }

    /// Dynamically adds named child entities to the `Parent` structure using a custom subformer.
    /// Unlike traditional methods that might use predefined setters like `_child_add`, this function
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

        // add instance to the container
        super_former.storage.child.as_mut().unwrap()
        .entry( preformed.name.clone() )
        .or_insert( preformed.clone() );

        // custom logic to add two instances to the container
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

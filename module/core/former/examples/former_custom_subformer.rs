//! example of how to use former of another structure as subformer of former of current one
//! function `child` integrate `ChildFormer` into `ParentFormer`.
// zzz : improve description

// xxx : zzz : implement example former_custom_container

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use std::collections::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Parent
  {
    #[ subform( setter = false ) ]
    child : HashMap< String, Child >,
  }

  // Use ChildFormer as custom subformer for ParentFormer to add children by name.
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

// xxx2 : finish example former_custom_subformer


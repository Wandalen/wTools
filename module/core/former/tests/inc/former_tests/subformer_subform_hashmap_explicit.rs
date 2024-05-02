#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Child struct with Former derived for builder pattern support
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct Child
{
  name : String,
  description : String,
}

// Parent struct to hold commands
#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Parent
{
  #[ subform( setter = false ) ]
  #[ scalar_setter( false ) ] // xxx : should not be required in this case
  command : HashMap< String, Child >,
}

// Use ChildFormer as custom subformer for ParentFormer to add commands by name.
impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn command( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._command_add::< ChildFormer< _ >, _, >()
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

// == begin of generated

// == end of generated

#[ test ]
fn basic()
{

  let got = Parent::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  a_id!( got.command.len(), 2 );

}

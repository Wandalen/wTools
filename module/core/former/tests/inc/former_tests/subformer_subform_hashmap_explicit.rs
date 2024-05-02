#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Command struct with Former derived for builder pattern support
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct Command
{
  name : String,
  description : String,
}

// Aggregator struct to hold commands
#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Aggregator
{
  #[ subform( setter = false ) ]
  #[ setter( false ) ]
  command : HashMap< String, Command >,
}

// Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
impl< Definition > AggregatorFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Aggregator as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn command( self, name : &str ) -> CommandAsSubformer< Self, impl CommandAsSubformerEnd< Self > >
  {
    self._command_add::< CommandFormer< _ >, _, >()
    .name( name )
  }

}

impl former::ValToElement< HashMap< String, Command > > for Command
{
  type Element = ( String, Command );
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

  let got = Aggregator::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  a_id!( got.command.len(), 2 );

}

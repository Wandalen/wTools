//! example of how to use former of another structure as subformer of former of current one
//! function `command` integrate `CommandFormer` into `AggregatorFormer`.

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use std::collections::HashMap;
  use former::Former;

  // Command struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Command
  {
    name : String,
    description : String,
  }

  // Aggregator struct to hold commands
  #[ derive( Debug, PartialEq, Former ) ]
  pub struct Aggregator
  {
    #[ setter( false ) ]
    command : HashMap< String, Command >,
  }

  // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
  impl< Context, End > AggregatorFormer< Context, End >
  where
    End : former::FormingEnd< Aggregator, Context >,
  {
    #[ inline( always ) ]
    pub fn command< IntoName >( self, name : IntoName ) -> CommandFormer< Self, impl former::FormingEnd< Command, Self > >
    where
      IntoName : core::convert::Into< String >,
    {
      let on_end = | command : Command, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut commands ) = super_former.storage.command
        {
          commands.insert( command.name.clone(), command );
        }
        else
        {
          let mut commands: HashMap< String, Command > = Default::default();
          commands.insert( command.name.clone(), command );
          super_former.storage.command = Some( commands );
        }
        super_former
      };
      let former = CommandFormer::begin( None, Some( self ), on_end );
      former.name( name )
    }
    // xxx : review
  }

  let ca = Aggregator::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  dbg!( &ca );
  // > &ca = Aggregator {
  // >     command: {
  // >          "echo": Command {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Command {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}

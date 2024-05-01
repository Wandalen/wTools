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
pub struct Aggregator
{
  #[ subform ]
  // #[ setter( false ) ]
  command : HashMap< String, Command >,
}

// // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
// impl< Definition > AggregatorFormer< Definition >
// where
//   Definition : former::FormerDefinition< Storage = < Aggregator as former::EntityToStorage >::Storage >,
// {
//
//   #[ inline( always ) ]
//   pub fn command( self, name : &str ) -> CommandAsSubformer< Self, impl CommandAsSubformerEnd< Self > >
//   {
//     self._command_add_subformer::< CommandFormer< _ >, _, >()
//     .name( name )
//   }
//
// }

// // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
// impl< Definition > AggregatorFormer< Definition >
// where
//   End : former::FormingEnd< Aggregator, Context >,
// {
//   #[ inline( always ) ]
//   pub fn command< IntoName >( self, name : IntoName ) -> CommandFormer< Self, impl former::FormingEnd< Command, Self > >
//   where
//     IntoName : core::convert::Into< String >,
//   {
//     let on_end = | command : Command, super_former : core::option::Option< Self > | -> Self
//     {
//       let mut super_former = super_former.unwrap();
//       if let Some( ref mut commands ) = super_former.storage.command
//       {
//         commands.insert( command.name.clone(), command );
//       }
//       else
//       {
//         let mut commands: HashMap< String, Command > = Default::default();
//         commands.insert( command.name.clone(), command );
//         super_former.storage.command = Some( commands );
//       }
//       super_former
//     };
//     let former = CommandFormer::begin( None, Some( self ), on_end );
//     former.name( name )
//   }
//   // xxx : review
// }

// == begin of generated

// == end of generated

// #[ test ]
// fn basic()
// {
//
//   let ca = Aggregator::former()
//   .command( "echo" )
//     .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
//     .end()
//   .command( "exit" )
//     .description( "just exit" ) // Sets additional properties using using custom subformer
//     .end()
//   .form();
//
// }
// xxx
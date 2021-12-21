use std::collections::HashMap;

/*
  var track = [];
  var commandWith = ( e ) => { track.push( 'with' ) };
  var Commands =
  {
    'with' : { ro : commandWith, h : 'with', lh : 'WITH' },
  };

  var aggregator = _.CommandsAggregator
  ({
    commands : Commands,
  }).form();

  aggregator.instructionPerform({ command : '.help' });
 */

#[test]
fn full_interface()
{
  let mut track: Vec<ArgParsed> = vec![];
  let mut command_with = | e : &ArgParsed |
  {
    track.push( e.clone() );
  }
  let commands = Commands::default();
  commands.insert( ".with", CommandDescriptor { ro : Box::new( command_with ), h : "with", lh : "WITH" } );

  let mut aggregator = CommandsAggregator::new()
  .commands().replace( commands ).end()
  .with_help( true )
  .form();

  aggregator.instructionPerform( ".with subject num:1 str:abc array:[ a b c ]" )
  let properties_map = HashMap
  {
    "num" : OpType::Int( 1 ),
    "str" : OpType::Str( "str" ),
    "array" : OpType::Vector( vec![ "a", "b", "c" ] )
  };
  let exp = ArgParsed
  {
    command : ".with subject num:1 str:abc array:[ a b c ]",
    command_name : ".with",
    subject : "subject",
    instruction_argument : "subject num:1 str:abc array:[ a b c ]",
    properties_map,

    /* additional fields */
    // parsed_commands : None,
    // index : None,
    // phrase_descriptor : HashMap{ ... },
    // aggregator
  };
  assert_eq!( track.len(), 1 );
  assert_eq!( track[ 0 ], exp );
}


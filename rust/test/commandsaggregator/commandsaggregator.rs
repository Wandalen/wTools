use std::collections::HashMap;

/*
  var track = [];
  var commandRun = ( e ) => { track.push( e ) };
  commandRun.subjectHint = 'Path to test';
  commandRun.properties =
  {
    'verbosity' : 'verbosity of testing',
    'rapidity' : 'rapidity of testing',
    'routine' : 'routine to test',
  }
  command.propertiesAliases =
  {
    verbosity : [ 'v' ]
  }

  var Commands =
  {
    '.run' : { ro : commandRun, h : 'Run test', lh : 'Run test with path and options' },
  };

  var aggregator = _.CommandsAggregator
  ({
    commands : Commands,
    withHelp : 1,
  }).form();

  aggregator.instructionPerform({ command : '.run proto/ v:7 rapidity:-1 routine:[ test1, test2 ]' });
 */

#[test]
fn full_interface()
{
  let mut track: Vec<ArgParsed> = vec![];
  let mut command_run = | e : &ArgParsed |
  {
    track.push( e.clone() );
  };
  let commands = Commands::default();
  let command_run_descriptor = CommandDescriptor
  {
    ro : Box::new( command_run ),
    h : "Run test",
    lh : "Run test with path and options",

    /* can construct type RoutineDescriptor */
    subject_hint : "Path to tests",
    properties : HashMap
    {
      "v" : "verbosity of output",
      "rapidity" : "rapidity of testing",
      "routine" : "routine to run",
    },
    properties_aliases : HashMap { "verbosity" : [ "v" ] },
  }
  commands.insert( ".with", command_run_descriptor );

  let mut aggregator = CommandsAggregator::new()
  .commands().replace( commands ).end()
  .with_help( true )
  .form();

  aggregator.instructionPerform( ".run proto/ v:7 rapidity:-1 routine:[ test1, test2 ]" );
  let properties_map = HashMap
  {
    "verbosity" : OpType::Int( 7 ),
    "rapidity" : OpType::Int( -7 ),
    "routine" : OpType::Vector( vec![ "test1", "test2" ] ),
  };
  let exp = ArgParsed
  {
    command : ".run proto/ v:7 rapidity:-1 routine:[ test1, test2 ]",
    command_name : ".run",
    subject : "proto/",
    instruction_argument : "proto/ v:7 rapidity:-1 routine:[ test1, test2 ]",
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


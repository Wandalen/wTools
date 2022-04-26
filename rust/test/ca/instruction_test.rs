
use wca::*;
use wstring_tools::string::parse::OpType::{ Primitive, Vector };
use wtest_basic::*;
use std::collections::HashMap;

//

fn _basic()
{
  let instruction = instruction::instruction_parse()
  .instruction( "" )
  .perform();
  let exp = instruction::Instruction
  {
    err : Some( wtools::error::Error::new( "Invalid command" ) ),
    command_name : "".to_string(),
    subject : "".to_string(),
    properties_map : HashMap::new(),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get" )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "".to_string(),
    properties_map : HashMap::new(),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some" )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::new(),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get v:1" )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1" )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 routine:some" )
  .perform();
  let properties_map = HashMap::from
  ([
    ( "v".to_string(), Primitive( "1".to_string() ) ),
    ( "routine".to_string(), Primitive( "some".to_string() ) ),
  ]);
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );

  /* */

  let aggregator_map = HashMap::new();
  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 routine:some" )
  .properties_map( aggregator_map )
  .perform();
  let properties_map = HashMap::from
  ([
    ( "v".to_string(), Primitive( "1".to_string() ) ),
    ( "routine".to_string(), Primitive( "some".to_string() ) ),
  ]);
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );

  let mut aggregator_map = HashMap::new();
  aggregator_map.insert( "ne".to_string(), Primitive( "-2".to_string() ) );
  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 routine:some" )
  .properties_map( aggregator_map )
  .perform();
  let properties_map = HashMap::from
  ([
    ( "v".to_string(), Primitive( "1".to_string() ) ),
    ( "routine".to_string(), Primitive( "some".to_string() ) ),
    ( "ne".to_string(), Primitive( "-2".to_string() ) ),
  ]);
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );
}

fn _with_several_values()
{
  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 v:2" )
  .several_values( false )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Primitive( "2".to_string() ) ) ]),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:[1,2]" )
  .several_values( false )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
  };
  assert_eq!( instruction, exp );

  /* */

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 v:2" )
  .several_values( true )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:[1,2]" )
  .several_values( true )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
  };
  assert_eq!( instruction, exp );

  /* */

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:[1,2] v:3" )
  .several_values( true )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) ) ]),
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:3 v:[1,2]" )
  .several_values( true )
  .perform();
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) ) ]),
  };
  assert_eq!( instruction, exp );
}

//

test_suite!
{
  basic,
  with_several_values,
}

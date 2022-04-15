
use wca::*;
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
  .instruction( ".get some v:1" )
  .perform();
  let mut properties_map = HashMap::new();
  properties_map.insert( "v".to_string(), "1".to_string() );
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 routine:some" )
  .perform();
  let mut properties_map = HashMap::new();
  properties_map.insert( "v".to_string(), "1".to_string() );
  properties_map.insert( "routine".to_string(), "some".to_string() );
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
  let mut properties_map = HashMap::new();
  properties_map.insert( "v".to_string(), "1".to_string() );
  properties_map.insert( "routine".to_string(), "some".to_string() );
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );

  let mut aggregator_map = HashMap::new();
  aggregator_map.insert( "ne".to_string(), "-2".to_string() );
  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1 routine:some" )
  .properties_map( aggregator_map )
  .perform();
  let mut properties_map = HashMap::new();
  properties_map.insert( "v".to_string(), "1".to_string() );
  properties_map.insert( "routine".to_string(), "some".to_string() );
  properties_map.insert( "ne".to_string(), "-2".to_string() );
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );
}

//

test_suite!
{
  basic,
}

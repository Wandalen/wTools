
use super::*;
use wstring_tools::string::parse::OpType::{ Primitive, Vector };

//

tests_impls!
{

  #[ test ]
  fn basic()
  {
    let instruction = wca::instruction::instruction_parse()
    .instruction( "" )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : Some( wtools::error::Error::new( "Invalid command" ) ),
      command_name : "".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get" )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some" )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get v:1" )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1" )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1 routine:some" )
    .perform();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );

    /* */

    let aggregator_map = HashMap::new();
    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1 routine:some" )
    .properties_map( aggregator_map )
    .perform();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );

    let mut aggregator_map = HashMap::new();
    aggregator_map.insert( "ne".to_string(), Primitive( "-2".to_string() ) );
    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1 routine:some" )
    .properties_map( aggregator_map )
    .perform();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
      ( "ne".to_string(), Primitive( "-2".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );
  }

  //

  #[ test ]
  fn with_several_values()
  {
    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1 v:2" )
    .several_values( false )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "2".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:[1,2]" )
    .several_values( false )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    /* */

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:1 v:2" )
    .several_values( true )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:[1,2]" )
    .several_values( true )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    /* */

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:[1,2] v:3" )
    .several_values( true )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get some v:3 v:[1,2]" )
    .several_values( true )
    .perform();
    let exp = wca::instruction::Instruction
    {
      err : None,
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );
  }

}

//

tests_index!
{
  basic,
  with_several_values,
}

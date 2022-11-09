
use super::*;
use wca::string::parse_request::OpType::{ Primitive, Vector };
use wtools::error::BasicError;
use wca::
{
  DefaultInstructionParser,
  InstructionParser,
};

//

tests_impls!
{
  fn basic()
  {
    let parser = DefaultInstructionParser::former().form();

    let err = parser.parse( "" ).unwrap_err();
    a_id!( err, BasicError::new( "Invalid command" ) );

    let instruction = parser
    .parse( ".get" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get v:1" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some v:1" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some v:1 routine:some" )
    .unwrap();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );

    /* */

    let instruction = DefaultInstructionParser::former()
    .properties( vec![] )
    .form()
    .parse( ".get some v:1 routine:some" )
    .unwrap();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );

    let mut properties = vec![ ( "ne".to_string(), Primitive( "-2".to_string() ) ) ];
    let instruction = DefaultInstructionParser::former()
    .properties( properties )
    .form()
    .parse( ".get some v:1 routine:some" )
    .unwrap();
    let properties_map = HashMap::from
    ([
      ( "v".to_string(), Primitive( "1".to_string() ) ),
      ( "routine".to_string(), Primitive( "some".to_string() ) ),
      ( "ne".to_string(), Primitive( "-2".to_string() ) ),
    ]);
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map,
    };
    a_id!( instruction, exp );
  }

  //

  fn with_several_values()
  {
    let parser = DefaultInstructionParser::former()
    .several_values( false )
    .form();

    let instruction = parser
    .parse( ".get some v:1 v:2" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "2".to_string() ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some v:[1,2]" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    /* */
    let parser = DefaultInstructionParser::former()
    .several_values( true )
    .form();

    let instruction = parser
    .parse( ".get some v:1 v:2" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some v:[1,2]" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    /* */

    let instruction = parser
    .parse( ".get some v:[1,2] v:3" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );

    let instruction = parser
    .parse( ".get some v:3 v:[1,2]" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) ) ]),
    };
    a_id!( instruction, exp );
  }

  fn path_subject() {
    let parser = DefaultInstructionParser::former()
    .several_values( true )
    .form();
    let instruction = parser
    .parse( ".get ./tmp/dir v:1" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "./tmp/dir".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "1".to_string() ) ) ]),
    };
    a_id!( instruction, exp );
  }

  fn path_property() {
    let parser = DefaultInstructionParser::former()
    .several_values( true )
    .form();
    let instruction = parser
    .parse( ".get some v:./tmp/dir/" )
    .unwrap();
    let exp = wca::instruction::Instruction
    {
      command_name : ".get".to_string(),
      subject : "some".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "./tmp/dir/".to_string() ) ) ]),
    };
    a_id!( instruction, exp );
  }
}

//

tests_index!
{
  basic,
  with_several_values,
  path_subject,
  path_property,
}

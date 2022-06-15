use wca::*;
use wca::string::parse::OpType; /* qqq : this should work. if does not then fix not this line */

fn main()
{
  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1" )
  .perform();
  let properties_map = std::collections::HashMap::from([ ( "v".to_string(), OpType::Primitive( "1".to_string() ) ) ]);
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );
}

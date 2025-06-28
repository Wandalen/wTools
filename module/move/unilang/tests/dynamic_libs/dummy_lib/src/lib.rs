use unilang::{
  semantic::VerifiedCommand,
  interpreter::ExecutionContext,
  data::{ OutputData, ErrorData },
};

#[ no_mangle ]
pub extern "C" fn dummy_command_routine( _verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  println!( "Dummy dynamic routine executed!" );
  Ok( OutputData { content: "Dummy dynamic routine executed!".to_string(), format: "text".to_string() } )
}

#[ no_mangle ]
pub extern "C" fn dummy_add_routine( verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let a = verified_command.arguments.get( "a" )
  .ok_or_else( || ErrorData { code: "MISSING_ARGUMENT".to_string(), message: "Argument 'a' not found".to_string() } )?
  .as_integer()
  .ok_or_else( || ErrorData { code: "INVALID_ARGUMENT_TYPE".to_string(), message: "Argument 'a' is not an integer".to_string() } )?;
  let b = verified_command.arguments.get( "b" )
  .ok_or_else( || ErrorData { code: "MISSING_ARGUMENT".to_string(), message: "Argument 'b' not found".to_string() } )?
  .as_integer()
  .ok_or_else( || ErrorData { code: "INVALID_ARGUMENT_TYPE".to_string(), message: "Argument 'b' is not an integer".to_string() } )?;
  println!( "Dummy add routine result: {}", a + b );
  Ok( OutputData { content: format!( "Dummy add routine result: {}", a + b ), format: "text".to_string() } )
}

#[ no_mangle ]
pub extern "C" fn dummy_error_routine( _verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  Err( ErrorData { code: "DUMMY_ERROR".to_string(), message: "This is a dummy error from dynamic library".to_string() } )
}
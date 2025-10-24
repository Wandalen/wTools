use unilang::
{
  data::{ ErrorData, OutputData },
  interpreter::ExecutionContext,
  semantic::VerifiedCommand,
};

#[ no_mangle ]
pub extern "C" fn dummy_command_routine( _verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  println!( "Dummy dynamic routine executed!" );
  Ok( OutputData { content : "Dummy dynamic routine executed!".to_string(), format : "text".to_string() } )
}

#[ no_mangle ]
pub extern "C" fn dummy_add_routine( verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  use unilang::data::ErrorCode;
  let a = verified_command.arguments.get( "a" )
  .ok_or_else( || ErrorData::new( ErrorCode::ArgumentMissing, "Argument 'a' not found".to_string() ) )?
  .as_integer()
  .ok_or_else( || ErrorData::new( ErrorCode::TypeMismatch, "Argument 'a' is not an integer".to_string() ) )?;
  let b = verified_command.arguments.get( "b" )
  .ok_or_else( || ErrorData::new( ErrorCode::ArgumentMissing, "Argument 'b' not found".to_string() ) )?
  .as_integer()
  .ok_or_else( || ErrorData::new( ErrorCode::TypeMismatch, "Argument 'b' is not an integer".to_string() ) )?;
  println!( "Dummy add routine result: {}", a + b );
  Ok( OutputData { content : format!( "Dummy add routine result: {}", a + b ), format : "text".to_string() } )
}

#[ no_mangle ]
pub extern "C" fn dummy_error_routine( _verified_command : VerifiedCommand, _context : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  use unilang::data::ErrorCode;
  Err( ErrorData::new( ErrorCode::InternalError, "This is a dummy error from dynamic library".to_string() ) )
}
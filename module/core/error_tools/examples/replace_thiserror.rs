//! A runnable example demonstrating how to use `error_tools::typed`
//! as a replacement for `thiserror`.

use error_tools::typed::Error;
use error_tools::dependency::thiserror;
use std::path::PathBuf;

// Define a custom error type using the derive macro from error_tools.
#[ derive( Debug, Error ) ]
/// Custom error type for data processing operations.
pub enum DataError
{
  #[ error( "I/O error for file: {0}" ) ]
  /// Represents an I/O error with the associated file path.
  Io( std::io::Error, PathBuf ),
  #[ error( "Parsing error: {0}" ) ]
  /// Represents a parsing error with a descriptive message.
  Parse( String ),
}

// Manual implementation of From trait for DataError
impl From< std::io::Error > for DataError
{
  fn from( err : std::io::Error ) -> Self
  {
    DataError::Io( err, PathBuf::new() )
  }
}

fn process_data( path : &PathBuf ) -> Result< i32, DataError >
{
  let content = std::fs::read_to_string( path )
    .map_err( | e | DataError::Io( e, path.clone() ) )?;

  content.trim().parse::< i32 >()
    .map_err( | _ | DataError::Parse( "Could not parse content as integer".into() ) )
}

fn main()
{
  // Create dummy files for the example
  _ = std::fs::write( "data.txt", "123" );
  _ = std::fs::write( "invalid_data.txt", "abc" );

  let path1 = PathBuf::from( "data.txt" );
  match process_data( &path1 )
  {
    Ok( num ) => println!( "Processed data: {num}" ),
    Err( e ) => println!( "An error occurred: {e}" ),
  }

  let path2 = PathBuf::from( "invalid_data.txt" );
  match process_data( &path2 )
  {
    Ok( _ ) => (),
    Err( e ) => println!( "Correctly handled parsing error: {e}" ),
  }

  // Clean up dummy files
  _ = std::fs::remove_file( "data.txt" );
  _ = std::fs::remove_file( "invalid_data.txt" );
}
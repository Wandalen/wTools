//! A trivial example demonstrating basic error handling with `error_tools`.
//!
//! This example shows the fundamental error handling patterns:
//! - Creating Result<T, E> return types
//! - Propagating errors with the `?` operator
//! - Handling success and error cases

use error_tools ::untyped :: { Result, format_err };

/// Attempts to parse a string as a number.
///
/// # Errors
/// Returns an error if the string cannot be parsed as u32.
fn parse_number( input: &str ) -> Result< u32 >
{
  input.trim().parse :: < u32 >()
    .map_err( | e | format_err!( "Failed to parse '{input}': {e}" ) )
}

/// Processes a number and returns its doubled value.
///
/// # Errors
/// Returns an error if the input string cannot be parsed.
fn process_input( input: &str ) -> Result< u32 >
{
  let num = parse_number( input )?;  // Error propagation with ?
  Ok( num * 2 )
}

fn main()
{
  // Success case
  match process_input( "42" )
  {
    Ok( result ) => println!( "Success: 42 * 2 = {result}" ),
    Err( e ) => println!( "Error: {e}" ),
  }

  // Error case
  match process_input( "not a number" )
  {
    Ok( result ) => println!( "Success: {result}" ),
    Err( e ) => println!( "Error: {e}" ),
  }
}

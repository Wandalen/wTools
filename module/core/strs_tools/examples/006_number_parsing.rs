//! Number parsing and conversion examples.
//!
//! This example demonstrates how to parse various number formats from strings,
//! handle different numeric bases, floating point formats, and error conditions.
//! Useful for configuration parsing, data validation, and text processing.

// Note: This example uses standard library parsing methods

fn main()
{
  println!( "=== Number Parsing Examples ===" );
  
  basic_number_parsing();
  different_number_formats();
  error_handling_and_validation();
  real_world_scenarios();
}

/// Demonstrates basic number parsing functionality.
///
/// Shows how to parse integers and floating point numbers
/// from string representations with proper error handling.
fn basic_number_parsing()
{
  println!( "\n--- Basic Number Parsing ---" );
  
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  {
  let number_strings = vec![
   "42",      // Integer
   "-17",     // Negative integer
   "3.14159", // Float
   "-2.5",    // Negative float
   "0",       // Zero
   "1000000", // Large number
 ];
  
  println!( "Parsing basic numeric formats: " );
  
  for num_str in number_strings
  {
   print!( "  '{num_str}' -> " );
   
   // Try parsing as integer first
   match num_str.parse :: < i32 >()
   {
  Ok( int_val ) =>
  {
   println!( "i32: {int_val}" );
 },
  Err( _ ) =>
  {
   // If integer parsing fails, try float
   match num_str.parse :: < f64 >()
   {
  Ok( float_val ) =>
  {
   println!( "f64: {float_val}" );
 },
  Err( e ) =>
  {
   println!( "Parse error: {e:?}" );
 }
 }
 }
 }
 }
  
  // Demonstrate different target types
  println!( "\nParsing to different numeric types: " );
  let test_value = "255";
  
  if let Ok( as_u8 ) = test_value.parse :: < u8 >()
  {
   println!( "  '{test_value}' as u8: {as_u8}" );
 }
  
  if let Ok( as_i16 ) = test_value.parse :: < i16 >()
  {
   println!( "  '{test_value}' as i16: {as_i16}" );
 }
  
  if let Ok( as_f32 ) = test_value.parse :: < f32 >()
  {
   println!( "  '{test_value}' as f32: {as_f32}" );
 }
  
  println!( "✓ Basic number parsing completed" );
 }
}

/// Demonstrates parsing different number formats.
///
/// Shows support for various bases (binary, octal, hexadecimal),
/// scientific notation, and special floating point values.
fn different_number_formats()
{
  println!( "\n--- Different Number Formats ---" );
  
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  {
  let format_examples = vec![
   // Hexadecimal
   ( "0xFF", "Hexadecimal" ),
   ( "0x1a2b", "Hex lowercase" ),
   ( "0X7F", "Hex uppercase" ),
   
   // Binary (if supported)
   ( "0b1010", "Binary" ),
   ( "0B11110000", "Binary uppercase" ),
   
   // Octal 
   ( "0o755", "Octal" ),
   ( "0O644", "Octal uppercase" ),
   
   // Scientific notation
   ( "1.23e4", "Scientific notation" ),
   ( "5.67E-3", "Scientific uppercase" ),
   ( "1e6", "Scientific integer" ),
   
   // Special float values
   ( "inf", "Infinity" ),
   ( "-inf", "Negative infinity" ),
   ( "NaN", "Not a number" ),
 ];
  
  println!( "Testing various number formats: " );
  
  for ( num_str, description ) in format_examples
  {
   print!( "  {description} ('{num_str}') -> " );
   
   // Try parsing as the most appropriate type
   if num_str.starts_with( "0x" ) || num_str.starts_with( "0X" ) ||
  num_str.starts_with( "0b" ) || num_str.starts_with( "0B" ) ||
  num_str.starts_with( "0o" ) || num_str.starts_with( "0O" )
   {
  // Handle different bases by preprocessing
  let parsed_value = if num_str.starts_with( "0x" ) || num_str.starts_with( "0X" )
  {
   // Parse hexadecimal
   u64 ::from_str_radix( &num_str[ 2.. ], 16 )
  .map( | v | v.to_string() )
 }
  else if num_str.starts_with( "0b" ) || num_str.starts_with( "0B" )
  {
   // Parse binary
   u64 ::from_str_radix( &num_str[ 2.. ], 2 )
  .map( | v | v.to_string() )
 }
  else if num_str.starts_with( "0o" ) || num_str.starts_with( "0O" )
  {
   // Parse octal
   u64 ::from_str_radix( &num_str[ 2.. ], 8 )
  .map( | v | v.to_string() )
 }
  else
  {
   Err( "invalid digit".parse :: < i32 >().unwrap_err() )
 };
  
  match parsed_value
  {
   Ok( decimal ) => println!( "decimal: {decimal}" ),
   Err( _ ) => 
   {
  // Fallback to lexical parsing
  match num_str.parse :: < i64 >()
  {
   Ok( val ) => println!( "{val}" ),
   Err( _ ) => println!( "Parse failed" ),
 }
 }
 }
 }
   else
   {
  // Try floating point for scientific notation and special values
  match num_str.parse :: < f64 >()
  {
   Ok( float_val ) => println!( "{float_val}" ),
   Err( _ ) => 
   {
  // Fallback to integer
  match num_str.parse :: < i64 >()
  {
   Ok( int_val ) => println!( "{int_val}" ),
   Err( _ ) => println!( "Parse failed" ),
 }
 }
 }
 }
 }
  
  println!( "✓ Different format parsing completed" );
 }
}

/// Demonstrates error handling and validation.
///
/// Shows how to handle invalid input, range checking,
/// and provide meaningful error messages for parsing failures.
fn error_handling_and_validation()
{
  println!( "\n--- Error Handling and Validation ---" );
  
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  {
  let invalid_inputs = vec![
   "",           // Empty string
   "abc",        // Non-numeric
   "12.34.56",   // Multiple decimal points
   "1,234",      // Comma separator
   "42x",        // Mixed alphanumeric
   " 123 ",      // Leading/trailing whitespace
   "∞",          // Unicode infinity
   "½",          // Unicode fraction
   "2²",         // Superscript
   "999999999999999999999", // Overflow
 ];
  
  println!( "Testing error conditions: " );
  
  for input in invalid_inputs
  {
   print!( "  '{}' -> ", input.replace( ' ', "␣" ) ); // Show spaces clearly
   
   if let Ok( val ) = input.parse :: < i32 >() 
   { println!( "Unexpectedly parsed as:{val}" ) }  else
  {
  // Try with preprocessing (trim whitespace)
  let trimmed = input.trim();
  match trimmed.parse :: < i32 >()
  {
   Ok( val ) => println!( "Parsed after trim: {val}" ),
   Err( _ ) => 
   {
  // Provide specific error classification
  if input.is_empty()
  {
   println!( "Error: Empty input" );
 }
  else if input.chars().any( char ::is_alphabetic )
  {
   println!( "Error: Contains letters" );
 }
  else if input.matches( '.' ).count() > 1
  {
   println!( "Error: Multiple decimal points" );
 }
  else if input.contains( ',' )
  {
   println!( "Error: Contains comma (use period for decimal)" );
 }
  else
  {
   println!( "Error: Invalid format or overflow" );
 }
 }
 }
 }
 }
  
  // Demonstrate range validation
  println!( "\nTesting range validation: " );
  
  let range_tests = vec![
   ( "300", "u8" ),   // Overflow for u8 (max 255)
   ( "-1", "u32" ),   // Negative for unsigned
   ( "70000", "i16" ), // Overflow for i16 (max ~32767)
 ];
  
  for ( value, target_type ) in range_tests
  {
   print!( "  '{value}' as {target_type} -> " );
   
   match target_type
   {
  "u8" =>
  {
   match value.parse :: < u8 >()
   {
  Ok( val ) => println!( "OK: {val}" ),
  Err( _ ) => println!( "Range error: value too large for u8" ),
 }
 },
  "u32" =>
  {
   match value.parse :: < u32 >()
   {
  Ok( val ) => println!( "OK: {val}" ),
  Err( _ ) => println!( "Range error: negative value for u32" ),
 }
 },
  "i16" =>
  {
   match value.parse :: < i16 >()
   {
  Ok( val ) => println!( "OK: {val}" ),
  Err( _ ) => println!( "Range error: value too large for i16" ),
 }
 },
  _ => println!( "Unknown type" ),
 }
 }
  
  println!( "✓ Error handling examples completed" );
 }
}

/// Demonstrates real-world number parsing scenarios.
///
/// Shows practical applications like configuration file parsing,
/// data validation, unit conversion, and user input processing.
#[ allow( clippy ::too_many_lines ) ]
fn real_world_scenarios()
{
  println!( "\n--- Real-World Scenarios ---" );
  
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  {
  // Scenario 1 : Configuration file parsing
  println!( "1. Configuration file parsing: " );
  
  let config_entries = vec![
   "port=8080",
   "timeout=30.5",
   "max_connections=100",
   "buffer_size=4096", 
   "enable_ssl=1",      // Boolean as number
   "retry_delay=2.5",
 ];
  
  for entry in config_entries
  {
   // Parse key=value pairs using standard string operations
   if let Some( equals_pos ) = entry.find( '=' )
   {
  let ( key, rest ) = entry.split_at( equals_pos );
  let value_str = &rest[ 1.. ]; // Skip the '=' character
  print!( "    {key} : '{value_str}' -> " );
  
  // Different parsing strategies based on config key
  match key
  {
  k if k.contains( "port" ) || k.contains( "connections" ) || k.contains( "size" ) =>
  {
   match value_str.parse :: < u32 >()
   {
  Ok( val ) => println!( "u32: {val}" ),
  Err( _ ) => println!( "Invalid integer" ),
 }
 },
  k if k.contains( "timeout" ) || k.contains( "delay" ) =>
  {
   match value_str.parse :: < f64 >()
   {
  Ok( val ) => println!( "f64: {val} seconds" ),
  Err( _ ) => println!( "Invalid float" ),
 }
 },
  k if k.contains( "enable" ) =>
  {
   match value_str.parse :: < i32 >()
   {
  Ok( 1 ) => println!( "boolean: true" ),
  Ok( 0 ) => println!( "boolean: false" ),
  Ok( other ) => println!( "boolean: {other} (non-standard)" ),
  Err( _ ) => println!( "Invalid boolean" ),
 }
 },
  _ =>
  {
   match value_str.parse :: < f64 >()
   {
  Ok( val ) => println!( "f64: {val}" ),
  Err( _ ) => println!( "Not a number" ),
 }
 }
 }
 }
 }
  
  // Scenario 2 : User input validation for a calculator
  println!( "\n2. Calculator input validation: " );
  
  let user_inputs = vec![
   "3.14 + 2.86",     // Simple addition
   "10 * 5",          // Multiplication  
   "100 / 7",         // Division
   "2^8",             // Power (needs special handling)
   "sqrt(16)",        // Function (needs special handling)
 ];
  
  for input in user_inputs
  {
   print!( "    Input: '{input}' -> " );
   
   // Simple operator detection and number extraction
   let operators = vec![ "+", "-", "*", "/", "^" ];
   let mut found_operator = None;
   let mut left_operand = "";
   let mut right_operand = "";
   
   for op in &operators
   {
  if input.contains( op )
  {
   let parts: Vec< &str > = input.splitn( 2, op ).collect();
   if parts.len() == 2
   {
  found_operator = Some( *op );
  left_operand = parts[ 0 ].trim();
  right_operand = parts[ 1 ].trim();
  break;
 }
 }
 }
   
   if let Some( op ) = found_operator
   {
  match ( left_operand.parse :: < f64 >(), 
  right_operand.parse :: < f64 >() )
  {
   ( Ok( left ), Ok( right ) ) =>
   {
  let result = match op
  {
   "+" => left + right,
   "-" => left - right,
   "*" => left * right,
   "/" => if right == 0.0 { f64::NAN } else { left / right },
   "^" => left.powf( right ),
   _ => f64 ::NAN,
 };
  
  if result.is_nan()
  {
   println!( "Mathematical error" );
 }
  else
  {
   println!( "= {result}" );
 }
 },
   _ => println!( "Invalid operands" ),
 }
 }
   else
   {
  // Check for function calls
  if input.contains( '(' ) && input.ends_with( ')' )
  {
   println!( "Function call detected (needs advanced parsing)" );
 }
  else
  {
   println!( "Unrecognized format" );
 }
 }
 }
  
  // Scenario 3 : Data file processing with units
  println!( "\n3. Data with units processing: " );
  
  let measurements = vec![
   "25.5°C",          // Temperature
   "120 km/h",        // Speed
   "1024 MB",         // Storage
   "3.5 GHz",         // Frequency
   "85%",             // Percentage
 ];
  
  for measurement in measurements
  {
   print!( "    '{measurement}' -> " );
   
   // Extract numeric part (everything before first non-numeric/non-decimal character)
   let numeric_part = measurement.chars()
  .take_while( | c | c.is_numeric() || *c == '.' || *c == '-' )
  .collect :: < String >();
   
   let unit_part = measurement[ numeric_part.len().. ].trim();
   
   match numeric_part.parse :: < f64 >()
   {
  Ok( value ) =>
  {
   match unit_part
   {
  "°C" => println!( "{:.1}°C ({:.1}°F)", value, value * 9.0 / 5.0 + 32.0 ),
  "km/h" => println!( "{} km/h ({:.1} m/s)", value, value / 3.6 ),
  "MB" => println!( "{} MB ({} bytes)", value, ( value * 1024.0 * 1024.0 ) as u64 ),
  "GHz" => println!( "{} GHz ({} Hz)", value, ( value * 1_000_000_000.0 ) as u64 ),
  "%" => 
  {
   if (0.0..=100.0).contains(&value)
   {
  println!( "{}% ({:.3} ratio)", value, value / 100.0 );
 }
   else
   {
  println!( "{value}% (out of range)" );
 }
 },
  _ => println!( "{value} {unit_part}" ),
 }
 },
  Err( _ ) => println!( "Invalid numeric value" ),
 }
 }
  
  println!( "\n✓ Real-world scenarios completed successfully" );
 }
}
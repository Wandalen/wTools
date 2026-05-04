// allow: test binary functions are not part of the public API; documentation not required
#![ allow( missing_docs ) ]

use config_hierarchy::detect_and_convert_value;
use serde_json::Value as JsonValue;

#[ test ]
fn test_boolean_true_values()
{
  assert_eq!( detect_and_convert_value( "true" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "True" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "TRUE" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "yes" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "Yes" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "YES" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "1" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "on" ), JsonValue::Bool( true ) );
  assert_eq!( detect_and_convert_value( "ON" ), JsonValue::Bool( true ) );
}

#[ test ]
fn test_boolean_false_values()
{
  assert_eq!( detect_and_convert_value( "false" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "False" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "FALSE" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "no" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "No" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "NO" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "0" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "off" ), JsonValue::Bool( false ) );
  assert_eq!( detect_and_convert_value( "OFF" ), JsonValue::Bool( false ) );
}

#[ test ]
fn test_integer_values()
{
  assert_eq!( detect_and_convert_value( "42" ), JsonValue::Number( 42.into() ) );
  assert_eq!( detect_and_convert_value( "0" ), JsonValue::Bool( false ) ); // "0" is boolean false
  assert_eq!( detect_and_convert_value( "-100" ), JsonValue::Number( ( -100 ).into() ) );
  assert_eq!( detect_and_convert_value( "999999999" ), JsonValue::Number( 999_999_999.into() ) );
}

#[ test ]
#[ allow( clippy::float_cmp ) ]
fn test_float_values()
{
  let val = detect_and_convert_value( "4.56789" );
  assert!( val.is_number() );
  assert_eq!( val.as_f64().unwrap(), 4.567_89 );

  let sci = detect_and_convert_value( "1.23e-4" );
  assert!( sci.is_number() );
  assert_eq!( sci.as_f64().unwrap(), 0.000_123 );
}

#[ test ]
fn test_string_fallback()
{
  assert_eq!( detect_and_convert_value( "hello" ), JsonValue::String( "hello".into() ) );
  assert_eq!( detect_and_convert_value( "world" ), JsonValue::String( "world".into() ) );
  assert_eq!( detect_and_convert_value( "2023-10-19" ), JsonValue::String( "2023-10-19".into() ) );
  assert_eq!( detect_and_convert_value( "maybe" ), JsonValue::String( "maybe".into() ) );
}

#[ test ]
fn test_unicode_strings()
{
  assert_eq!( detect_and_convert_value( "测试" ), JsonValue::String( "测试".into() ) );
  assert_eq!( detect_and_convert_value( "тест" ), JsonValue::String( "тест".into() ) );
  assert_eq!( detect_and_convert_value( "🔥" ), JsonValue::String( "🔥".into() ) );
}

#[ test ]
fn test_special_characters()
{
  assert_eq!( detect_and_convert_value( "key:value" ), JsonValue::String( "key:value".into() ) );
  assert_eq!( detect_and_convert_value( "a=b" ), JsonValue::String( "a=b".into() ) );
  assert_eq!( detect_and_convert_value( "[1,2,3]" ), JsonValue::String( "[1,2,3]".into() ) );
}

#[ test ]
fn test_empty_string()
{
  assert_eq!( detect_and_convert_value( "" ), JsonValue::String( String::new() ) );
}

#[ test ]
fn test_whitespace_string()
{
  assert_eq!( detect_and_convert_value( "   " ), JsonValue::String( "   ".into() ) );
  assert_eq!( detect_and_convert_value( "\n" ), JsonValue::String( "\n".into() ) );
}

// AC-05: Non-finite floats (NaN, ±Inf) fall through to String
//
// ## Root Cause
// algorithm/001 Step 3 specifies: "Success and value is finite → Number;
// Non-finite (NaN, ±Inf) → fall through to string". f64::parse("NaN") and
// f64::parse("Inf") SUCCEED, so without the is_finite() guard they would
// silently produce unserializable JsonValue::Number variants.
//
// ## Why Not Caught
// String fallback tests covered "hello" and unicode but not the specific
// float-parses-but-is-non-finite edge case.
//
// ## Fix Applied
// Tests verify the finite-check guard in type_detection.rs returns String.
//
// ## Prevention
// Every documented edge case in an algorithm doc must have a corresponding test.
//
// ## Pitfall
// f64::parse("NaN") returns Ok. The is_finite() guard is what routes it to String.
// Without the guard, non-finite floats become unserializable JSON.
#[ test ]
fn test_non_finite_float_fallback()
{
  assert_eq!( detect_and_convert_value( "NaN" ),  JsonValue::String( "NaN".into() ) );
  assert_eq!( detect_and_convert_value( "Inf" ),  JsonValue::String( "Inf".into() ) );
  assert_eq!( detect_and_convert_value( "-Inf" ), JsonValue::String( "-Inf".into() ) );
  assert_eq!( detect_and_convert_value( "inf" ),  JsonValue::String( "inf".into() ) );
  assert_eq!( detect_and_convert_value( "nan" ),  JsonValue::String( "nan".into() ) );
}

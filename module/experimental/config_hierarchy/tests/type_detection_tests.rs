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

// AC-07: "0" maps to boolean false, not integer zero
//
// ## Root Cause
// "0" appears in both the boolean-false literal table AND parses as a valid i64.
// The boolean check runs BEFORE the integer check in the algorithm step order.
//
// ## Pitfall
// A naive "is this parseable as i64?" check before the bool table lookup would
// silently break the documented ordering invariant (booleans have higher priority).
#[ test ]
fn test_zero_is_boolean_not_integer()
{
  let result = detect_and_convert_value( "0" );
  assert_eq!( result, JsonValue::Bool( false ), "\"0\" must map to Bool(false) — boolean check precedes integer" );
  assert!( !result.is_number(), "\"0\" must NOT be Number(0) — step order: bool before int" );
}

// AC-08: integer string overflowing i64 cascades to float Number
//
// ## Root Cause
// "99999999999999999999" is too large for i64 but within f64 range.
// i64 parse fails → f64 parse succeeds → is_finite() guard passes → float Number returned.
//
// ## Pitfall
// Without a float cascade after i64 failure, this would silently fall through to String,
// losing numeric type information for large-but-representable numbers.
#[ test ]
fn test_integer_overflow_cascades_to_float()
{
  let result = detect_and_convert_value( "99999999999999999999" );
  assert!( result.is_number(), "i64-overflow string must cascade to float Number, not String" );
  let f = result.as_f64().expect( "cascaded Number must be expressible as f64" );
  assert!( f.is_finite(), "cascaded float must be finite" );
  assert!( f > 0.0, "cascaded float must be positive" );
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

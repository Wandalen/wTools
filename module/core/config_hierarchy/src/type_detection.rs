use serde_json::Value as JsonValue;

/// Intelligently detect value type and convert to appropriate JSON representation
///
/// # Type Detection Rules
///
/// Values are converted based on content, with priority order:
///
/// 1. **Boolean Detection** (case-insensitive):
///    - `true` values: "true", "yes", "1", "on"
///    - `false` values: "false", "no", "0", "off"
///
/// 2. **Integer Detection**: "42", "-100", "999999999999"
/// 3. **Float Detection**: "3.14159", "1.23e-4"
/// 4. **String Fallback**: All other values
///
/// # Examples
///
/// ```
/// use config_hierarchy::detect_and_convert_value;
/// use serde_json::Value as JsonValue;
///
/// assert_eq!( detect_and_convert_value( "true" ), JsonValue::Bool( true ) );
/// assert_eq!( detect_and_convert_value( "YES" ), JsonValue::Bool( true ) );
/// assert_eq!( detect_and_convert_value( "42" ), JsonValue::Number( 42.into() ) );
/// assert_eq!( detect_and_convert_value( "3.14" ), JsonValue::Number( serde_json::Number::from_f64( 3.14 ).unwrap() ) );
/// assert_eq!( detect_and_convert_value( "hello" ), JsonValue::String( "hello".to_string() ) );
/// ```
#[ inline ]
#[ must_use ]
pub fn detect_and_convert_value( value : &str ) -> JsonValue
{
  // Try boolean (case-insensitive, multiple formats)
  let lower = value.to_lowercase();
  match lower.as_str()
  {
    "true" | "yes" | "1" | "on" => return JsonValue::Bool( true ),
    "false" | "no" | "0" | "off" => return JsonValue::Bool( false ),
    _ => {}
  }

  // Try integer
  if let Ok( i ) = value.parse::< i64 >()
  {
    return JsonValue::Number( i.into() );
  }

  // Try float
  if let Ok( f ) = value.parse::< f64 >()
  {
    if let Some( num ) = serde_json::Number::from_f64( f )
    {
      return JsonValue::Number( num );
    }
  }

  // Default to string
  JsonValue::String( value.to_string() )
}

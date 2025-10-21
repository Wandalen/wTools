/// Tests for `TemplateValue` trait and Value enum (FR1, FR2)
use super :: *;

//

#[ test ]
fn template_value_trait_string_conversion()
{
  // FR1: TemplateValue trait must define to_template_string method
  let value = Value ::String( "test".into() );
  assert_eq!( value.to_template_string(), "test" );
}

#[ test ]
fn template_value_trait_from_string()
{
  // FR1: TemplateValue trait must define from_string method
  let value = Value ::from_string( "test value".into() );
  assert_eq!( value.to_template_string(), "test value" );
}

#[ test ]
fn template_value_trait_is_empty()
{
  // FR1: TemplateValue trait must define is_empty method
  let empty = Value ::String( String::new() );
  let non_empty = Value ::String( "value".into() );

  assert!( empty.is_empty() );
  assert!( !non_empty.is_empty() );
}

#[ test ]
fn value_enum_string_variant()
{
  // FR2: Must support String variant
  let value = Value ::String( "hello world".into() );
  assert_eq!( value.to_template_string(), "hello world" );
  assert!( !value.is_empty() );
}

#[ test ]
fn value_enum_number_variant()
{
  // FR2: Must support Number variant (i64)
  let value = Value ::Number( 42 );
  assert_eq!( value.to_template_string(), "42" );
  assert!( !value.is_empty() );

  let negative = Value ::Number( -100 );
  assert_eq!( negative.to_template_string(), "-100" );
}

#[ test ]
fn value_enum_bool_variant()
{
  // FR2: Must support Bool variant
  let true_value = Value ::Bool( true );
  let false_value = Value ::Bool( false );

  assert_eq!( true_value.to_template_string(), "true" );
  assert_eq!( false_value.to_template_string(), "false" );
  assert!( !true_value.is_empty() );
}

#[ test ]
fn value_enum_list_variant()
{
  // FR2: Must support List variant (Vec<String>)
  let list = Value ::List( vec![ "one".into(), "two".into(), "three".into() ] );
  // List should join with commas for template rendering
  assert_eq!( list.to_template_string(), "one, two, three" );
  assert!( !list.is_empty() );

  let empty_list = Value ::List( vec![] );
  assert!( empty_list.is_empty() );
}

#[ test ]
fn value_clone()
{
  // Value must be Clone
  let value = Value ::String( "test".into() );
  let cloned = value.clone();
  assert_eq!( value.to_template_string(), cloned.to_template_string() );
}

//

// Custom value type test to verify trait is implementable
#[ derive( Clone ) ]
struct CustomValue( String );

impl TemplateValue for CustomValue
{
  fn to_template_string( &self ) -> String
  {
    format!( "custom:{}", self.0 )
  }

  fn from_string( s: String ) -> Self
  {
    CustomValue( s )
  }

  fn is_empty( &self ) -> bool
  {
    self.0.is_empty()
  }
}

#[ test ]
fn custom_template_value_implementation()
{
  // FR1: Trait must be implementable for custom value types
  let custom = CustomValue( "test".into() );
  assert_eq!( custom.to_template_string(), "custom:test" );
  assert!( !custom.is_empty() );

  let custom_from = CustomValue ::from_string( "hello".into() );
  assert_eq!( custom_from.to_template_string(), "custom:hello" );
}

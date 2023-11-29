use crate::TheModule::query::
{
    parse,
    Value,
};
use std::collections::HashMap;
use std::str::FromStr;

#[ test ]
fn value_from_str() 
{
  assert_eq!( Value::from_str( "123" ).unwrap(), Value::Int( 123 ) );
  assert_eq!( Value::from_str( "true" ).unwrap(), Value::Bool( true ) );
  assert_eq!( Value::from_str( "'hello'" ).unwrap(), Value::String( "hello".to_string() ) );
}

#[ test ]
fn bool_from_value() 
{
  assert_eq!( bool::from( &Value::Bool( true ) ), true );
  assert_eq!( bool::from( &Value::String( "true".to_string() ) ), true );
  assert_eq!( bool::from( &Value::Int( 1 ) ), true );
}

#[ test ]
fn parse_empty_string() 
{
  let expected_map = HashMap::new();
  assert_eq!( parse( "" ).unwrap(), expected_map );
}

#[test]
fn parse_single_value() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "path".to_string(), Value::String( "test/test".to_string() ) );
  assert_eq!( parse( "'test/test'" ).unwrap(), expected_map );
}

#[ test ]
fn parse_multiple_values() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key1".to_string(), Value::Int( 123 ) );
  expected_map.insert( "key2".to_string(), Value::Bool( true ) );
  assert_eq!( parse( "key1: 123, key2: true" ).unwrap(), expected_map );
}

#[ test ]
#[ should_panic ]
fn parse_mixed_values() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key1".to_string(), Value::Int( 123 ) );
  expected_map.insert( "path".to_string(), Value::String( "test/test".to_string() ) );
  assert_eq!( parse( "key1: 123, 'test/test'" ).unwrap(), expected_map );
}

#[ test ]
fn parse_with_quotes() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello world".to_string() ) );
  assert_eq!( parse( "key: 'hello world'" ).unwrap(), expected_map );
}

#[ test ]
fn parse_with_special_characters() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "!@#$%^&*()".to_string() ) );
  assert_eq!( parse( "key: '!@#$%^&*()'" ).unwrap(), expected_map );
}


#[ test ]
fn parse_with_colon_in_value() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello:world".to_string() ) );
  assert_eq!( parse( "key: 'hello:world'" ).unwrap(), expected_map );
}

#[ test ]
fn with_comma_in_value() 
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello,world".to_string() ) );
  assert_eq!( parse( "key: 'hello,world'" ).unwrap(), expected_map );
}

#[ test ]
fn with_single_quote_escape()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( r#"hello\'test\'test"#.into() ) );
  assert_eq!( parse( r#"key: 'hello\'test\'test'"# ).unwrap(), expected_map );
}

#[ test ]
fn with_multiple_spaces()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "test     ".into() ) );
  expected_map.insert( "key2".to_string(), Value::String( "test".into() ) );
  assert_eq!( parse( r#"key    :    'test     ', key2  :      test     "# ).unwrap(), expected_map );
}


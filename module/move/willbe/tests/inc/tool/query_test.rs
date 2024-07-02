use super::*;
use the_module::query::
{
  parse,
  ParseResult,
  Value,
};
use the_module::collection::HashMap;
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
  assert_eq!( bool::from( &Value::Int( 0 ) ), false);
  assert_eq!( bool::from( &Value::String( "test".to_string() ) ), false);
}

#[ test ]
fn parse_result_convert()
{
  let params = vec![ Value::Int( 1 ), Value::Int( 2 ), Value::Int( 3 ) ];
  let result = ParseResult::Positioning( params );

  let named_map = result.clone().into_map(vec!["var0".into(), "var1".into(),"var2".into() ]);
  let unnamed_map = result.clone().into_map( vec![] );
  let mixed_map = result.clone().into_map( vec![ "var0".into() ] );
  let vec = result.into_vec();

  assert_eq!( HashMap::from( [( "var0".to_string(),Value::Int( 1 )), ( "var1".to_string(),Value::Int( 2 )), ( "var2".to_string(),Value::Int( 3 )) ]), named_map );
  assert_eq!( HashMap::from( [( "1".to_string(),Value::Int( 1 )), ( "2".to_string(),Value::Int( 2 )), ( "3".to_string(),Value::Int( 3 )) ]), unnamed_map );
  assert_eq!( HashMap::from( [( "var0".to_string(),Value::Int( 1 )), ( "1".to_string(),Value::Int( 2 )), ( "2".to_string(),Value::Int( 3 )) ]), mixed_map );
  assert_eq!( vec![ Value::Int( 1 ), Value::Int( 2 ), Value::Int( 3 ) ], vec );
}

#[ test ]
fn parse_empty_string()
{
  assert_eq!( parse( "()" ).unwrap().into_vec(), vec![] );
}

#[test]
fn parse_single_value()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "1".to_string(), Value::String( "test/test".to_string() ) );
  assert_eq!( parse( "('test/test')" ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn parse_multiple_values()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key1".to_string(), Value::Int( 123 ) );
  expected_map.insert( "key2".to_string(), Value::Bool( true ) );
  assert_eq!( parse( "{key1 : 123, key2 : true}" ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn parse_with_quotes()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello world".to_string() ) );
  assert_eq!( parse( "{key : 'hello world'}" ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn parse_with_special_characters()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "!@#$%^&*(),".to_string() ) );
  assert_eq!( parse( "{key : '!@#$%^&*(),'}" ).unwrap().into_map(vec![]), expected_map );
}


#[ test ]
fn parse_with_colon_in_value()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello :world".to_string() ) );
  assert_eq!( parse( "{key : 'hello :world'}" ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn with_comma_in_value()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "hello,world".to_string() ) );
  assert_eq!( parse( "{key : 'hello,world'}" ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn with_single_quote_escape()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( r#"hello\'test\'test"#.into() ) );
  assert_eq!( parse( r#"{ key : 'hello\'test\'test' }"# ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn with_multiple_spaces()
{
  let mut expected_map = HashMap::new();
  expected_map.insert( "key".to_string(), Value::String( "test     ".into() ) );
  expected_map.insert( "key2".to_string(), Value::String( "test".into() ) );
  assert_eq!( parse( r#"{ key    :    'test     ', key2  :      test     }"# ).unwrap().into_map(vec![]), expected_map );
}

#[ test ]
fn many_unnamed()
{
  let expected : HashMap< _, _ > = HashMap::from_iter
  ( [
    ( "1".to_string(), Value::Int( 123 ) ),
    ( "2".to_string(), Value::String( "test_aboba".to_string() ) ),
  ] );
  assert_eq!( parse( "( 123, 'test_aboba' )").unwrap().into_map(vec![]), expected );
}

#[ test ]
fn named_and_unnamed()
{
  let expected : HashMap< _, _ > = HashMap::from_iter
    ( [
      ( "1".to_string(), Value::Int( 123 ) ),
      ( "2".to_string(), Value::String( "test_aboba".to_string() ) ),
      ( "3".to_string(), Value::String("test : true".to_string()))
    ] );
  assert_eq!( parse( r#"(123, 'test_aboba', test : true)"#).unwrap().into_map(vec![]), expected );
}

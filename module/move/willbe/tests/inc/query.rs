use crate::TheModule::query::
{
    parse,
    Value,
};
use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn test_value_from_str() {
  assert_eq!(Value::from_str("123").unwrap(), Value::Int(123));
  assert_eq!(Value::from_str("true").unwrap(), Value::Bool(true));
  assert_eq!(Value::from_str("'hello'").unwrap(), Value::String("hello".to_string()));
}

#[test]
fn test_bool_from_value() {
  assert_eq!(bool::from(&Value::Bool(true)), true);
  assert_eq!(bool::from(&Value::String("true".to_string())), true);
  assert_eq!(bool::from(&Value::Int(1)), true);
}

#[test]
fn test_parse_empty_string() {
  let expected_map = HashMap::new();
  assert_eq!(parse(""), expected_map);
}

#[test]
fn test_parse_single_value() {
  let mut expected_map = HashMap::new();
  expected_map.insert("path".to_string(), Value::String("test/test".to_string()));
  assert_eq!(parse("'test/test'"), expected_map);
}

#[test]
fn test_parse_multiple_values() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key1".to_string(), Value::Int(123));
  expected_map.insert("key2".to_string(), Value::Bool(true));
  assert_eq!(parse("key1: 123, key2: true"), expected_map);
}

#[test]
fn test_parse_mixed_values() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key1".to_string(), Value::Int(123));
  expected_map.insert("path".to_string(), Value::String("test/test".to_string()));
  assert_eq!(parse("key1: 123, 'test/test'"), expected_map);
}

#[test]
fn test_parse_with_quotes() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key".to_string(), Value::String("hello world".to_string()));
  assert_eq!(parse("key: 'hello world'"), expected_map);
}

#[test]
fn test_parse_with_special_characters() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key".to_string(), Value::String("!@#$%^&*()".to_string()));
  assert_eq!(parse("key: '!@#$%^&*()'"), expected_map);
}


#[test]
fn test_parse_with_colon_in_value() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key".to_string(), Value::String("hello:world".to_string()));
  assert_eq!(parse("key: 'hello:world'"), expected_map);
}

#[test]
fn test_parse_with_comma_in_value() {
  let mut expected_map = HashMap::new();
  expected_map.insert("key".to_string(), Value::String("hello,world".to_string()));
  assert_eq!(parse("key: 'hello,world'"), expected_map);
}

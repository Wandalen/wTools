//!
//! Tests for the types module
//!

use unilang::types::*;
use unilang::data::Kind;
use std::path::PathBuf;

#[test]
fn test_value_as_integer_success()
{
  let value = Value::Integer(42);
  assert_eq!(value.as_integer(), Some(&42));
}

#[test] 
fn test_value_as_integer_none()
{
  let value = Value::String("not_integer".to_string());
  assert_eq!(value.as_integer(), None);
}

#[test]
fn test_value_as_path_success()
{
  let path = PathBuf::from("/test/path");
  let value = Value::Path(path.clone());
  assert_eq!(value.as_path(), Some(&path));
}

#[test]
fn test_value_as_path_file_variant()
{
  let path = PathBuf::from("/test/file.txt");
  let value = Value::File(path.clone());
  assert_eq!(value.as_path(), Some(&path));
}

#[test]
fn test_value_as_path_directory_variant()
{
  let path = PathBuf::from("/test/dir");
  let value = Value::Directory(path.clone());
  assert_eq!(value.as_path(), Some(&path));
}

#[test]
fn test_value_as_path_none()
{
  let value = Value::String("not_path".to_string());
  assert_eq!(value.as_path(), None);
}

#[test]
fn test_parse_value_string_success()
{
  let result = parse_value("hello world", &Kind::String);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::String("hello world".to_string()));
}

#[test]
fn test_parse_value_integer_success()
{
  let result = parse_value("42", &Kind::Integer);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::Integer(42));
}

#[test]
fn test_parse_value_integer_negative()
{
  let result = parse_value("-123", &Kind::Integer);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::Integer(-123));
}

#[test]
fn test_parse_value_integer_invalid()
{
  let result = parse_value("not_a_number", &Kind::Integer);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Integer);
  assert!(error.reason.contains("invalid digit"));
}

#[test]
fn test_parse_value_float_success()
{
  let result = parse_value("3.14", &Kind::Float);
  assert!(result.is_ok());
  #[allow(clippy::approx_constant)]
  let expected = 3.14;
  assert_eq!(result.unwrap(), Value::Float(expected));
}

#[test]
fn test_parse_value_float_invalid()
{
  let result = parse_value("not_a_float", &Kind::Float);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Float);
  assert!(error.reason.contains("invalid float"));
}

#[test]
fn test_parse_value_boolean_true_variants()
{
  for input in &["true", "TRUE", "1", "yes", "YES"] {
    let result = parse_value(input, &Kind::Boolean);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Boolean(true));
  }
}

#[test]
fn test_parse_value_boolean_false_variants()
{
  for input in &["false", "FALSE", "0", "no", "NO"] {
    let result = parse_value(input, &Kind::Boolean);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Boolean(false));
  }
}

#[test]
fn test_parse_value_boolean_invalid()
{
  let result = parse_value("maybe", &Kind::Boolean);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Boolean);
  assert_eq!(error.reason, "Invalid boolean value");
}

#[test]
fn test_parse_value_enum_success()
{
  let choices = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
  let kind = Kind::Enum(choices);
  let result = parse_value("green", &kind);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::Enum("green".to_string()));
}

#[test]
fn test_parse_value_enum_invalid_choice()
{
  let choices = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
  let kind = Kind::Enum(choices);
  let result = parse_value("purple", &kind);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert!(error.reason.contains("not one of the allowed choices"));
}

#[test]
fn test_parse_value_path_success()
{
  let result = parse_value("/test/path", &Kind::Path);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::Path(PathBuf::from("/test/path")));
}

#[test]
fn test_parse_value_path_empty()
{
  let result = parse_value("", &Kind::Path);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.reason, "Path cannot be empty");
}

#[test]
fn test_parse_value_url_success()
{
  let result = parse_value("https://example.com", &Kind::Url);
  assert!(result.is_ok());
  if let Value::Url(url) = result.unwrap() {
    assert_eq!(url.as_str(), "https://example.com/");
  } else {
    panic!("Expected URL value");
  }
}

#[test]
fn test_parse_value_url_invalid()
{
  let result = parse_value("not_a_url", &Kind::Url);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Url);
  assert!(error.reason.contains("relative URL"));
}

#[test]
fn test_parse_value_datetime_success()
{
  let result = parse_value("2023-01-01T12:00:00+00:00", &Kind::DateTime);
  assert!(result.is_ok());
  if let Value::DateTime(_) = result.unwrap() {
    // DateTime parsed successfully
  } else {
    panic!("Expected DateTime value");
  }
}

#[test]
fn test_parse_value_datetime_invalid()
{
  let result = parse_value("not_a_datetime", &Kind::DateTime);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::DateTime);
  assert!(error.reason.contains("input contains invalid characters"));
}

#[test]
fn test_parse_value_pattern_success()
{
  let result = parse_value(r"\d+", &Kind::Pattern);
  assert!(result.is_ok());
  if let Value::Pattern(regex) = result.unwrap() {
    assert_eq!(regex.as_str(), r"\d+");
  } else {
    panic!("Expected Pattern value");
  }
}

#[test]
fn test_parse_value_pattern_invalid()
{
  let result = parse_value("[invalid_regex", &Kind::Pattern);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Pattern);
  assert!(error.reason.contains("regex parse error"));
}

#[test]
fn test_parse_value_list_success()
{
  let item_kind = Box::new(Kind::Integer);
  let kind = Kind::List(item_kind, Some(','));
  let result = parse_value("1,2,3", &kind);
  assert!(result.is_ok());
  if let Value::List(items) = result.unwrap() {
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], Value::Integer(1));
    assert_eq!(items[1], Value::Integer(2));
    assert_eq!(items[2], Value::Integer(3));
  } else {
    panic!("Expected List value");
  }
}

#[test]
fn test_parse_value_list_empty()
{
  let item_kind = Box::new(Kind::String);
  let kind = Kind::List(item_kind, None);
  let result = parse_value("", &kind);
  assert!(result.is_ok());
  if let Value::List(items) = result.unwrap() {
    assert!(items.is_empty());
  } else {
    panic!("Expected empty List value");
  }
}

#[test]
fn test_parse_value_list_custom_delimiter()
{
  let item_kind = Box::new(Kind::String);
  let kind = Kind::List(item_kind, Some(';'));
  let result = parse_value("a;b;c", &kind);
  assert!(result.is_ok());
  if let Value::List(items) = result.unwrap() {
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], Value::String("a".to_string()));
    assert_eq!(items[1], Value::String("b".to_string()));
    assert_eq!(items[2], Value::String("c".to_string()));
  } else {
    panic!("Expected List value");
  }
}

#[test]
fn test_parse_value_map_success()
{
  let key_kind = Box::new(Kind::String);
  let value_kind = Box::new(Kind::Integer);
  let kind = Kind::Map(key_kind, value_kind, Some(','), Some('='));
  let result = parse_value("a=1,b=2,c=3", &kind);
  assert!(result.is_ok());
  if let Value::Map(map) = result.unwrap() {
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("a"), Some(&Value::Integer(1)));
    assert_eq!(map.get("b"), Some(&Value::Integer(2)));
    assert_eq!(map.get("c"), Some(&Value::Integer(3)));
  } else {
    panic!("Expected Map value");
  }
}

#[test]
fn test_parse_value_map_empty()
{
  let key_kind = Box::new(Kind::String);
  let value_kind = Box::new(Kind::String);
  let kind = Kind::Map(key_kind, value_kind, None, None);
  let result = parse_value("", &kind);
  assert!(result.is_ok());
  if let Value::Map(map) = result.unwrap() {
    assert!(map.is_empty());
  } else {
    panic!("Expected empty Map value");
  }
}

#[test]
fn test_parse_value_map_invalid_entry()
{
  let key_kind = Box::new(Kind::String);
  let value_kind = Box::new(Kind::String);
  let kind = Kind::Map(key_kind, value_kind, Some(','), Some('='));
  let result = parse_value("a=1,invalid_entry,c=3", &kind);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert!(error.reason.contains("Invalid map entry"));
}

#[test]
fn test_parse_value_json_string_success()
{
  let result = parse_value(r#"{"key": "value"}"#, &Kind::JsonString);
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), Value::JsonString(r#"{"key": "value"}"#.to_string()));
}

#[test]
fn test_parse_value_json_string_invalid()
{
  let result = parse_value("{invalid json", &Kind::JsonString);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::JsonString);
  // JSON parsing error occurred - specific message may vary
  assert!(!error.reason.is_empty());
}

#[test]
fn test_parse_value_object_success()
{
  let result = parse_value(r#"{"key": "value", "number": 42}"#, &Kind::Object);
  assert!(result.is_ok());
  if let Value::Object(obj) = result.unwrap() {
    assert!(obj.is_object());
    assert_eq!(obj["key"], "value");
    assert_eq!(obj["number"], 42);
  } else {
    panic!("Expected Object value");
  }
}

#[test]
fn test_parse_value_object_invalid()
{
  let result = parse_value("{invalid json object", &Kind::Object);
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert_eq!(error.expected_kind, Kind::Object);
  // JSON parsing error occurred - specific message may vary
  assert!(!error.reason.is_empty());
}

#[test]
fn test_value_partial_eq()
{
  // Test string equality
  assert_eq!(Value::String("hello".to_string()), Value::String("hello".to_string()));
  assert_ne!(Value::String("hello".to_string()), Value::String("world".to_string()));

  // Test integer equality
  assert_eq!(Value::Integer(42), Value::Integer(42));
  assert_ne!(Value::Integer(42), Value::Integer(43));

  // Test float equality
  assert_eq!(Value::Float(3.15), Value::Float(3.15));
  assert_ne!(Value::Float(3.15), Value::Float(2.71));

  // Test boolean equality
  assert_eq!(Value::Boolean(true), Value::Boolean(true));
  assert_ne!(Value::Boolean(true), Value::Boolean(false));

  // Test cross-type inequality
  assert_ne!(Value::String("42".to_string()), Value::Integer(42));
}

#[test]
fn test_value_display()
{
  assert_eq!(Value::String("hello".to_string()).to_string(), "hello");
  assert_eq!(Value::Integer(42).to_string(), "42");
  assert_eq!(Value::Float(3.15).to_string(), "3.15");
  assert_eq!(Value::Boolean(true).to_string(), "true");
  assert_eq!(Value::Path(PathBuf::from("/test")).to_string(), "/test");
}

#[test]
fn test_type_error_equality()
{
  let error1 = TypeError {
    expected_kind: Kind::Integer,
    reason: "invalid number".to_string(),
  };
  let error2 = TypeError {
    expected_kind: Kind::Integer,
    reason: "invalid number".to_string(),
  };
  let error3 = TypeError {
    expected_kind: Kind::String,
    reason: "invalid number".to_string(),
  };

  assert_eq!(error1, error2);
  assert_ne!(error1, error3);
}
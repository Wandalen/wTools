//! # Types
//!
//! This module defines the parsing and validation logic for the various argument types (`kind`) supported by `unilang`.
//! It is responsible for converting raw string inputs from the command line into strongly-typed Rust values.

/// Internal namespace.
mod private
{
  use crate::data::Kind;
  use std::path::PathBuf; // Removed `Path`
  use url::Url;
  use chrono::{DateTime, FixedOffset};
  use regex::Regex;
  use core::fmt;
  use std::collections::HashMap; // Added for Map Value
  use serde_json; // Added for JsonString and Object Value

/// Represents a parsed and validated value of a specific kind.
#[ derive( Debug, Clone ) ]
pub enum Value
{
  /// A sequence of characters.
  String( String ),
  /// A whole number.
  Integer( i64 ),
  /// A floating-point number.
  Float( f64 ),
  /// A true or false value.
  Boolean( bool ),
  /// A URI representing a file system path.
  Path( PathBuf ),
  /// A `Path` that must point to a file.
  File( PathBuf ),
  /// A `Path` that must point to a directory.
  Directory( PathBuf ),
  /// A string that must be one of the predefined, case-sensitive choices.
  Enum( String ),
  /// A Uniform Resource Locator.
  Url( Url ),
  /// A date and time.
  DateTime( DateTime< FixedOffset > ),
  /// A regular expression pattern string.
  Pattern( Regex ),
  /// A list of elements of a specified `Type`.
  List( Vec< Value > ),
  /// A key-value map.
  Map( HashMap< String, Value > ),
  /// A JSON string.
  JsonString( String ),
  /// A JSON object.
  Object( serde_json::Value ),
}

impl Value
{
  /// Returns a reference to the inner `i64` if the value is `Integer`, otherwise `None`.
  #[ must_use ]
  pub fn as_integer( &self ) -> Option< &i64 >
  {
    if let Self::Integer( v ) = self
    {
      Some( v )
    }
    else
    {
      None
    }
  }

  /// Returns a reference to the inner `PathBuf` if the value is `Path`, `File`, or `Directory`, otherwise `None`.
  #[ must_use ]
  pub fn as_path( &self ) -> Option< &PathBuf >
  {
    match self
    {
      Self::Path( v ) | Self::File( v ) | Self::Directory( v ) => Some( v ),
      _ => None,
    }
  }
}

impl PartialEq for Value
{
  fn eq( &self, other : &Self ) -> bool
  {
    match ( self, other )
    {
      ( Self::String( l ), Self::String( r ) ) | ( Self::Enum( l ), Self::Enum( r ) ) | ( Self::JsonString( l ), Self::JsonString( r ) ) => l == r, // Merged match arms
      ( Self::Integer( l ), Self::Integer( r ) ) => l == r,
      ( Self::Float( l ), Self::Float( r ) ) => l == r,
      ( Self::Boolean( l ), Self::Boolean( r ) ) => l == r,
      ( Self::Path( l ), Self::Path( r ) ) | ( Self::File( l ), Self::File( r ) ) | ( Self::Directory( l ), Self::Directory( r ) ) => l == r, // Merged match arms
      ( Self::Url( l ), Self::Url( r ) ) => l == r,
      ( Self::DateTime( l ), Self::DateTime( r ) ) => l == r,
      ( Self::Pattern( l ), Self::Pattern( r ) ) => l.as_str() == r.as_str(),
      ( Self::List( l ), Self::List( r ) ) => l == r,
      ( Self::Map( l ), Self::Map( r ) ) => l == r,
      ( Self::Object( l ), Self::Object( r ) ) => l == r,
      _ => false,
    }
  }
}

impl fmt::Display for Value
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Value::String( s ) | Value::Enum( s ) | Value::JsonString( s ) => write!( f, "{s}" ), // Merged match arms
      Value::Integer( i ) => write!( f, "{i}" ),
      Value::Float( fl ) => write!( f, "{fl}" ),
      Value::Boolean( b ) => write!( f, "{b}" ),
      Value::Path( p ) | Value::File( p ) | Value::Directory( p ) => write!( f, "{}", p.to_string_lossy() ),
      Value::Url( u ) => write!( f, "{u}" ),
      Value::DateTime( dt ) => write!( f, "{}", dt.to_rfc3339() ),
      Value::Pattern( r ) => write!( f, "{}", r.as_str() ),
      Value::List( l ) => write!( f, "{l:?}" ),
      Value::Map( m ) => write!( f, "{m:?}" ),
      Value::Object( o ) => write!( f, "{o}" ),
    }
  }
}

/// An error that can occur during type parsing or validation.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct TypeError
{
  /// The expected kind of the value.
  pub expected_kind : Kind,
  /// A message describing the reason for the failure.
  pub reason : String,
}

/// Parses a raw string input into a `Value` based on the specified `Kind`.
///
/// # Errors
///
/// Returns a `TypeError` if the input string cannot be parsed into the
/// specified `Kind` or if it fails validation for that `Kind`.
pub fn parse_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  match kind
  {
    Kind::String | Kind::Integer | Kind::Float | Kind::Boolean | Kind::Enum( _ ) => parse_primitive_value( input, kind ),
    Kind::Path | Kind::File | Kind::Directory => parse_path_value( input, kind ),
    Kind::Url | Kind::DateTime | Kind::Pattern => parse_url_datetime_pattern_value( input, kind ),
    Kind::List( .. ) => parse_list_value( input, kind ),
    Kind::Map( .. ) => parse_map_value( input, kind ),
    Kind::JsonString | Kind::Object => parse_json_value( input, kind ),
  }
}

fn parse_primitive_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  match kind
  {
    Kind::String => Ok( Value::String( input.to_string() ) ),
    Kind::Integer => input.parse::< i64 >().map( Value::Integer ).map_err( | e | TypeError
    {
      expected_kind: kind.clone(),
      reason: e.to_string(),
    }),
    Kind::Float => input.parse::< f64 >().map( Value::Float ).map_err( | e | TypeError
    {
      expected_kind: kind.clone(),
      reason: e.to_string(),
    }),
    Kind::Boolean => match input.to_lowercase().as_str() {
      "true" | "1" | "yes" => Ok(Value::Boolean(true)),
      "false" | "0" | "no" => Ok(Value::Boolean(false)),
      _ => Err(TypeError {
        expected_kind: kind.clone(),
        reason: "Invalid boolean value".to_string(),
      }),
    },
    Kind::Enum(choices) => {
      if choices.contains(&input.to_string()) {
        Ok(Value::Enum(input.to_string()))
      } else {
        Err(TypeError {
          expected_kind: kind.clone(),
          reason: format!("Value '{input}' is not one of the allowed choices: {choices:?}"),
        })
      }
    }
    _ => unreachable!("Called parse_primitive_value with non-primitive kind: {:?}", kind),
  }
}

fn parse_path_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  if input.is_empty() {
    return Err(TypeError {
      expected_kind: kind.clone(),
      reason: "Path cannot be empty".to_string(),
    });
  }
  let path = PathBuf::from(input);
  match kind {
    Kind::Path => Ok(Value::Path(path)),
    Kind::File => {
      if path.is_file() {
        Ok(Value::File(path))
      } else if path.is_dir() {
        Err(TypeError {
          expected_kind: kind.clone(),
          reason: "Expected a file, but found a directory".to_string(),
        })
      } else {
        Err(TypeError {
          expected_kind: kind.clone(),
          reason: format!("File not found at path: {input}"),
        })
      }
    }
    Kind::Directory => {
      if path.is_dir() {
        Ok(Value::Directory(path))
      } else if path.is_file() {
        Err(TypeError {
          expected_kind: kind.clone(),
          reason: "Expected a directory, but found a file".to_string(),
        })
      } else {
        Err(TypeError {
          expected_kind: kind.clone(),
          reason: format!("Directory not found at path: {input}"),
        })
      }
    }
    _ => unreachable!("Called parse_path_value with non-path kind: {:?}", kind),
  }
}

fn parse_url_datetime_pattern_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  match kind {
    Kind::Url => Url::parse(input).map(Value::Url).map_err(|e| TypeError {
      expected_kind: kind.clone(),
      reason: e.to_string(),
    }),
    Kind::DateTime => DateTime::parse_from_rfc3339(input)
      .map(Value::DateTime)
      .map_err(|e| TypeError {
        expected_kind: kind.clone(),
        reason: e.to_string(),
      }),
    Kind::Pattern => Regex::new(input).map(Value::Pattern).map_err(|e| TypeError {
      expected_kind: kind.clone(),
      reason: e.to_string(),
    }),
    _ => unreachable!("Called parse_url_datetime_pattern_value with unsupported kind: {:?}", kind),
  }
}

fn parse_list_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  let Kind::List(item_kind, delimiter_opt) = kind else {
    unreachable!("Called parse_list_value with non-list kind: {:?}", kind)
  };

  if input.is_empty() {
    return Ok(Value::List(Vec::new()));
  }
  let delimiter = delimiter_opt.unwrap_or(',');
  let parts: Vec<&str> = input.split(delimiter).collect();
  let mut parsed_items = Vec::new();
  for part in parts {
    parsed_items.push(parse_value(part, item_kind)?);
  }
  Ok(Value::List(parsed_items))
}

fn parse_map_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  let Kind::Map(_key_kind, value_kind, entry_delimiter_opt, kv_delimiter_opt) = kind else {
    unreachable!("Called parse_map_value with non-map kind: {:?}", kind)
  };

  if input.is_empty() {
    return Ok(Value::Map(HashMap::new()));
  }
  let entry_delimiter = entry_delimiter_opt.unwrap_or(',');
  let kv_delimiter = kv_delimiter_opt.unwrap_or('=');
  let entries: Vec<&str> = input.split(entry_delimiter).collect();
  let mut parsed_map = HashMap::new();
  for entry in entries {
    let parts: Vec<&str> = entry.splitn(2, kv_delimiter).collect();
    if parts.len() != 2 {
      return Err(TypeError {
        expected_kind: kind.clone(),
        reason: format!("Invalid map entry: '{entry}'. Expected 'key{kv_delimiter}value'"),
      });
    }
    let key_str = parts[0];
    let value_str = parts[1];

    // For simplicity, map keys are always String for now.
    // A more robust solution would parse key_kind.
    let parsed_key = key_str.to_string();
    let parsed_value = parse_value(value_str, value_kind)?;
    parsed_map.insert(parsed_key, parsed_value);
  }
  Ok(Value::Map(parsed_map))
}

fn parse_json_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
{
  match kind {
    Kind::JsonString => {
      // Validate that it's a valid JSON string using SIMD-optimized parsing
      crate::simd_json_parser::SIMDJsonParser::parse_to_serde_value( input ).map_err( |e| TypeError {
        expected_kind: kind.clone(),
        reason: e.reason,
      })?;
      Ok( Value::JsonString( input.to_string() ) )
    }
    Kind::Object => crate::simd_json_parser::SIMDJsonParser::parse_to_serde_value( input )
      .map( Value::Object )
      .map_err( |e| TypeError {
        expected_kind: kind.clone(),
        reason: e.reason,
      }),
    _ => unreachable!( "Called parse_json_value with non-JSON kind: {:?}", kind ),
  }
}

}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::data::Kind;
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
    assert_eq!(result.unwrap(), Value::Float(3.14));
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
}

mod_interface::mod_interface!
{
  exposed use private::Value;
  exposed use private::TypeError;
  exposed use private::parse_value;
  
  prelude use private::Value;
  prelude use private::TypeError;
  prelude use private::parse_value;
}

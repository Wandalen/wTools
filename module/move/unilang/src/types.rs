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
#[derive(Debug, Clone)]
pub enum Value {
  /// A sequence of characters.
  String(String),
  /// A whole number.
  Integer(i64),
  /// A floating-point number.
  Float(f64),
  /// A true or false value.
  Boolean(bool),
  /// A URI representing a file system path.
  Path(PathBuf),
  /// A `Path` that must point to a file.
  File(PathBuf),
  /// A `Path` that must point to a directory.
  Directory(PathBuf),
  /// A string that must be one of the predefined, case-sensitive choices.
  Enum(String),
  /// A Uniform Resource Locator.
  Url(Url),
  /// A date and time.
  DateTime(DateTime<FixedOffset>),
  /// A regular expression pattern string.
  Pattern(Regex),
  /// A list of elements of a specified `Type`.
  List(Vec<Value>),
  /// A key-value map.
  Map(HashMap<String, Value>),
  /// A JSON string.
  JsonString(String),
  /// A JSON object.
  Object(serde_json::Value),
}

impl Value {
  /// Returns a reference to the inner `i64` if the value is `Integer`, otherwise `None`.
  #[must_use]
  pub fn as_integer(&self) -> Option<&i64> {
    if let Self::Integer(v) = self {
      Some(v)
    } else {
      None
    }
  }

  /// Returns a reference to the inner `PathBuf` if the value is `Path`, `File`, or `Directory`, otherwise `None`.
  #[must_use]
  pub fn as_path(&self) -> Option<&PathBuf> {
    match self {
      Self::Path(v) | Self::File(v) | Self::Directory(v) => Some(v),
      _ => None,
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::String(l), Self::String(r)) | (Self::Enum(l), Self::Enum(r)) | (Self::JsonString(l), Self::JsonString(r)) => l == r, // Merged match arms
      (Self::Integer(l), Self::Integer(r)) => l == r,
      (Self::Float(l), Self::Float(r)) => l == r,
      (Self::Boolean(l), Self::Boolean(r)) => l == r,
      (Self::Path(l), Self::Path(r)) | (Self::File(l), Self::File(r)) | (Self::Directory(l), Self::Directory(r)) => l == r, // Merged match arms
      (Self::Url(l), Self::Url(r)) => l == r,
      (Self::DateTime(l), Self::DateTime(r)) => l == r,
      (Self::Pattern(l), Self::Pattern(r)) => l.as_str() == r.as_str(),
      (Self::List(l), Self::List(r)) => l == r,
      (Self::Map(l), Self::Map(r)) => l == r,
      (Self::Object(l), Self::Object(r)) => l == r,
      _ => false,
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::String(s) | Value::Enum(s) | Value::JsonString(s) => write!(f, "{s}"), // Merged match arms
      Value::Integer(i) => write!(f, "{i}"),
      Value::Float(fl) => write!(f, "{fl}"),
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Path(p) | Value::File(p) | Value::Directory(p) => write!(f, "{}", p.to_string_lossy()),
      Value::Url(u) => write!(f, "{u}"),
      Value::DateTime(dt) => write!(f, "{}", dt.to_rfc3339()),
      Value::Pattern(r) => write!(f, "{}", r.as_str()),
      Value::List(l) => write!(f, "{l:?}"),
      Value::Map(m) => write!(f, "{m:?}"),
      Value::Object(o) => write!(f, "{o}"),
    }
  }
}

/// An error that can occur during type parsing or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError {
  /// The expected kind of the value.
  pub expected_kind: Kind,
  /// A message describing the reason for the failure.
  pub reason: String,
}

/// Parses a raw string input into a `Value` based on the specified `Kind`.
///
/// # Errors
///
/// Returns a `TypeError` if the input string cannot be parsed into the
/// specified `Kind` or if it fails validation for that `Kind`.
pub fn parse_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
  match kind {
    Kind::String | Kind::Integer | Kind::Float | Kind::Boolean | Kind::Enum(_) => parse_primitive_value(input, kind),
    Kind::Path | Kind::File | Kind::Directory => parse_path_value(input, kind),
    Kind::Url | Kind::DateTime | Kind::Pattern => parse_url_datetime_pattern_value(input, kind),
    Kind::List(..) => parse_list_value(input, kind),
    Kind::Map(..) => parse_map_value(input, kind),
    Kind::JsonString | Kind::Object => parse_json_value(input, kind),
  }
}

fn parse_primitive_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
  match kind {
    Kind::String => Ok(Value::String(input.to_string())),
    Kind::Integer => input.parse::<i64>().map(Value::Integer).map_err(|e| TypeError {
      expected_kind: kind.clone(),
      reason: e.to_string(),
    }),
    Kind::Float => input.parse::<f64>().map(Value::Float).map_err(|e| TypeError {
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

fn parse_path_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
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

fn parse_url_datetime_pattern_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
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

fn parse_list_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
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

fn parse_map_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
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

fn parse_json_value(input: &str, kind: &Kind) -> Result<Value, TypeError> {
  match kind {
    Kind::JsonString => {
      // Validate that it's a valid JSON string, but store it as a raw string.
      serde_json::from_str::<serde_json::Value>(input).map_err(|e| TypeError {
        expected_kind: kind.clone(),
        reason: e.to_string(),
      })?;
      Ok(Value::JsonString(input.to_string()))
    }
    Kind::Object => serde_json::from_str::<serde_json::Value>(input)
      .map(Value::Object)
      .map_err(|e| TypeError {
        expected_kind: kind.clone(),
        reason: e.to_string(),
      }),
    _ => unreachable!("Called parse_json_value with non-JSON kind: {:?}", kind),
  }
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

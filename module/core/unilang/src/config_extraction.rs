//!
//! Config value extraction utilities.
//!
//! **Requires feature**: `json_parser`
//!
//! Generic utilities for extracting typed values from configuration maps.
//! Works with `HashMap<String, (JsonValue, S)>` where `S` is any source-tracking type
//! (e.g., `config_hierarchy::ConfigSource`).
//!
//! # Overview
//!
//! These utilities provide a convenient way to extract typed values from configuration
//! maps that track value sources. The generic design allows integration with any
//! source-tracking system without introducing dependencies.
//!
//! # Example
//!
//! ```rust
//! use std::collections::HashMap;
//! use serde_json::json;
//! use unilang::config_extraction::{ ConfigMap, extract_u8, extract_bool, extract_string };
//!
//! // S can be any type - we use () for simplicity
//! let mut config: ConfigMap<()> = HashMap::new();
//! config.insert("verbosity".into(), (json!(3), ()));
//! config.insert("debug".into(), (json!(true), ()));
//! config.insert("name".into(), (json!("example"), ()));
//!
//! assert_eq!(extract_u8(&config, "verbosity"), Some(3));
//! assert_eq!(extract_bool(&config, "debug"), Some(true));
//! assert_eq!(extract_string(&config, "name"), Some("example".into()));
//! assert_eq!(extract_u8(&config, "missing"), None);
//! ```
//!
//! # Design Rationale
//!
//! The functions are generic over source type `S` to avoid requiring dependencies
//! on specific configuration crates (e.g., `config_hierarchy`). The source type
//! is ignored during extraction (`|(val, _)|` pattern) but preserved in the map
//! for consumers who need source tracking.
//!

/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use serde_json::Value as JsonValue;

  /// Type alias for configuration maps with any source type.
  ///
  /// This type represents the common pattern of storing configuration values
  /// alongside their source (e.g., default, environment, file, CLI).
  ///
  /// # Type Parameters
  ///
  /// * `S` - The source type tracking where each value came from
  pub type ConfigMap<S> = HashMap<String, (JsonValue, S)>;

  /// Extract `u8` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  /// - Value overflows `u8` range (> 255)
  #[inline]
  #[must_use]
  pub fn extract_u8<S>(config: &ConfigMap<S>, key: &str) -> Option<u8>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_u64())
      .and_then(|n| u8::try_from(n).ok())
  }

  /// Extract `u16` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  /// - Value overflows `u16` range (> 65535)
  #[inline]
  #[must_use]
  pub fn extract_u16<S>(config: &ConfigMap<S>, key: &str) -> Option<u16>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_u64())
      .and_then(|n| u16::try_from(n).ok())
  }

  /// Extract `u32` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  /// - Value overflows `u32` range
  #[inline]
  #[must_use]
  pub fn extract_u32<S>(config: &ConfigMap<S>, key: &str) -> Option<u32>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_u64())
      .and_then(|n| u32::try_from(n).ok())
  }

  /// Extract `u64` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  #[inline]
  #[must_use]
  pub fn extract_u64<S>(config: &ConfigMap<S>, key: &str) -> Option<u64>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_u64())
  }

  /// Extract `i32` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  /// - Value overflows `i32` range
  #[inline]
  #[must_use]
  pub fn extract_i32<S>(config: &ConfigMap<S>, key: &str) -> Option<i32>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_i64())
      .and_then(|n| i32::try_from(n).ok())
  }

  /// Extract `i64` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  #[inline]
  #[must_use]
  pub fn extract_i64<S>(config: &ConfigMap<S>, key: &str) -> Option<i64>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_i64())
  }

  /// Extract `f64` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a number
  #[inline]
  #[must_use]
  pub fn extract_f64<S>(config: &ConfigMap<S>, key: &str) -> Option<f64>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_f64())
  }

  /// Extract `bool` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not a boolean
  #[inline]
  #[must_use]
  pub fn extract_bool<S>(config: &ConfigMap<S>, key: &str) -> Option<bool>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_bool())
  }

  /// Extract `String` value from config.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is `null`
  /// - Value is not a string
  #[inline]
  #[must_use]
  pub fn extract_string<S>(config: &ConfigMap<S>, key: &str) -> Option<String>
  {
    config.get(key)
      .and_then(|(v, _)|
      {
        if v.is_null()
        {
          None
        }
        else
        {
          v.as_str().map(|s| s.to_string())
        }
      })
  }

  /// Extract array of strings from config.
  ///
  /// Non-string elements in the array are filtered out.
  ///
  /// Returns `None` if:
  /// - Key is missing
  /// - Value is not an array
  #[inline]
  #[must_use]
  pub fn extract_string_array<S>(config: &ConfigMap<S>, key: &str) -> Option<Vec<String>>
  {
    config.get(key)
      .and_then(|(v, _)| v.as_array())
      .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
  }
}

mod_interface::mod_interface!
{
  exposed use
  {
    private::ConfigMap,
    private::extract_u8,
    private::extract_u16,
    private::extract_u32,
    private::extract_u64,
    private::extract_i32,
    private::extract_i64,
    private::extract_f64,
    private::extract_bool,
    private::extract_string,
    private::extract_string_array,
  };
}

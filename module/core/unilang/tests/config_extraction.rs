//! Config extraction tests.
//!
//! ## Test Matrix
//!
//! | Category | Test | Status |
//! |----------|------|--------|
//! | u8 | extract existing value | ✓ |
//! | u8 | extract missing key | ✓ |
//! | u8 | extract overflow value | ✓ |
//! | u16 | extract existing value | ✓ |
//! | u32 | extract existing value | ✓ |
//! | u64 | extract existing value | ✓ |
//! | i32 | extract positive value | ✓ |
//! | i32 | extract negative value | ✓ |
//! | i64 | extract existing value | ✓ |
//! | f64 | extract existing value | ✓ |
//! | bool | extract true | ✓ |
//! | bool | extract false | ✓ |
//! | string | extract normal string | ✓ |
//! | string | extract null returns None | ✓ |
//! | string_array | extract array | ✓ |
//! | string_array | extract empty array | ✓ |
//! | generic | works with different source types | ✓ |

use std::collections::HashMap;
use serde_json::json;
use unilang::config_extraction::*;

// ============================================================================
// Test Fixtures
// ============================================================================

/// Create test config with various value types.
/// Uses `&'static str` as source type for simplicity.
fn make_config() -> ConfigMap<&'static str>
{
  let mut config = HashMap::new();

  // Unsigned integers
  config.insert("u8_val".into(), (json!(42), "default"));
  config.insert("u8_max".into(), (json!(255), "cli"));
  config.insert("u8_overflow".into(), (json!(300), "default"));
  config.insert("u16_val".into(), (json!(1000), "env"));
  config.insert("u32_val".into(), (json!(100_000), "file"));
  config.insert("u64_val".into(), (json!(10_000_000_000_u64), "default"));

  // Signed integers
  config.insert("i32_positive".into(), (json!(42), "default"));
  config.insert("i32_negative".into(), (json!(-42), "cli"));
  config.insert("i64_val".into(), (json!(-9_000_000_000_i64), "default"));

  // Floating point
  config.insert("f64_val".into(), (json!(42.5), "default"));

  // Booleans
  config.insert("bool_true".into(), (json!(true), "cli"));
  config.insert("bool_false".into(), (json!(false), "env"));

  // Strings
  config.insert("string_val".into(), (json!("hello world"), "file"));
  config.insert("string_null".into(), (json!(null), "default"));
  config.insert("string_empty".into(), (json!(""), "default"));

  // Arrays
  config.insert("array_val".into(), (json!(["a", "b", "c"]), "default"));
  config.insert("array_empty".into(), (json!([]), "default"));
  config.insert("array_mixed".into(), (json!(["str", 123, true]), "default"));

  config
}

// ============================================================================
// u8 extraction tests
// ============================================================================

#[test]
fn extract_u8_existing_value()
{
  let config = make_config();
  assert_eq!(extract_u8(&config, "u8_val"), Some(42));
}

#[test]
fn extract_u8_max_value()
{
  let config = make_config();
  assert_eq!(extract_u8(&config, "u8_max"), Some(255));
}

#[test]
fn extract_u8_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_u8(&config, "nonexistent"), None);
}

#[test]
fn extract_u8_overflow_returns_none()
{
  let config = make_config();
  // 300 > 255, should return None
  assert_eq!(extract_u8(&config, "u8_overflow"), None);
}

// ============================================================================
// u16 extraction tests
// ============================================================================

#[test]
fn extract_u16_existing_value()
{
  let config = make_config();
  assert_eq!(extract_u16(&config, "u16_val"), Some(1000));
}

#[test]
fn extract_u16_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_u16(&config, "nonexistent"), None);
}

// ============================================================================
// u32 extraction tests
// ============================================================================

#[test]
fn extract_u32_existing_value()
{
  let config = make_config();
  assert_eq!(extract_u32(&config, "u32_val"), Some(100_000));
}

#[test]
fn extract_u32_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_u32(&config, "nonexistent"), None);
}

// ============================================================================
// u64 extraction tests
// ============================================================================

#[test]
fn extract_u64_existing_value()
{
  let config = make_config();
  assert_eq!(extract_u64(&config, "u64_val"), Some(10_000_000_000));
}

#[test]
fn extract_u64_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_u64(&config, "nonexistent"), None);
}

// ============================================================================
// i32 extraction tests
// ============================================================================

#[test]
fn extract_i32_positive_value()
{
  let config = make_config();
  assert_eq!(extract_i32(&config, "i32_positive"), Some(42));
}

#[test]
fn extract_i32_negative_value()
{
  let config = make_config();
  assert_eq!(extract_i32(&config, "i32_negative"), Some(-42));
}

#[test]
fn extract_i32_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_i32(&config, "nonexistent"), None);
}

// ============================================================================
// i64 extraction tests
// ============================================================================

#[test]
fn extract_i64_existing_value()
{
  let config = make_config();
  assert_eq!(extract_i64(&config, "i64_val"), Some(-9_000_000_000));
}

#[test]
fn extract_i64_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_i64(&config, "nonexistent"), None);
}

// ============================================================================
// f64 extraction tests
// ============================================================================

#[test]
fn extract_f64_existing_value()
{
  let config = make_config();
  let val = extract_f64(&config, "f64_val").unwrap();
  assert!((val - 42.5).abs() < f64::EPSILON);
}

#[test]
fn extract_f64_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_f64(&config, "nonexistent"), None);
}

// ============================================================================
// bool extraction tests
// ============================================================================

#[test]
fn extract_bool_true_value()
{
  let config = make_config();
  assert_eq!(extract_bool(&config, "bool_true"), Some(true));
}

#[test]
fn extract_bool_false_value()
{
  let config = make_config();
  assert_eq!(extract_bool(&config, "bool_false"), Some(false));
}

#[test]
fn extract_bool_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_bool(&config, "nonexistent"), None);
}

// ============================================================================
// string extraction tests
// ============================================================================

#[test]
fn extract_string_normal_value()
{
  let config = make_config();
  assert_eq!(extract_string(&config, "string_val"), Some("hello world".into()));
}

#[test]
fn extract_string_empty_value()
{
  let config = make_config();
  assert_eq!(extract_string(&config, "string_empty"), Some(String::new()));
}

#[test]
fn extract_string_null_returns_none()
{
  let config = make_config();
  assert_eq!(extract_string(&config, "string_null"), None);
}

#[test]
fn extract_string_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_string(&config, "nonexistent"), None);
}

// ============================================================================
// string_array extraction tests
// ============================================================================

#[test]
fn extract_string_array_normal_value()
{
  let config = make_config();
  assert_eq!(
    extract_string_array(&config, "array_val"),
    Some(vec!["a".into(), "b".into(), "c".into()])
  );
}

#[test]
fn extract_string_array_empty_value()
{
  let config = make_config();
  assert_eq!(extract_string_array(&config, "array_empty"), Some(vec![]));
}

#[test]
fn extract_string_array_mixed_filters_non_strings()
{
  let config = make_config();
  // Only "str" is a string, 123 and true are filtered out
  assert_eq!(
    extract_string_array(&config, "array_mixed"),
    Some(vec!["str".into()])
  );
}

#[test]
fn extract_string_array_missing_key_returns_none()
{
  let config = make_config();
  assert_eq!(extract_string_array(&config, "nonexistent"), None);
}

// ============================================================================
// Generic source type tests
// ============================================================================

/// Custom source type for testing generics.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum TestSource
{
  Default,
  Cli,
  Env,
}

#[test]
fn extract_works_with_custom_source_type()
{
  let mut config: ConfigMap<TestSource> = HashMap::new();
  config.insert("level".into(), (json!(5), TestSource::Cli));
  config.insert("name".into(), (json!("test"), TestSource::Env));

  assert_eq!(extract_u8(&config, "level"), Some(5));
  assert_eq!(extract_string(&config, "name"), Some("test".into()));
}

#[test]
fn extract_works_with_unit_source_type()
{
  let mut config: ConfigMap<()> = HashMap::new();
  config.insert("flag".into(), (json!(true), ()));

  assert_eq!(extract_bool(&config, "flag"), Some(true));
}

// ============================================================================
// Type mismatch tests
// ============================================================================

#[test]
fn extract_u8_from_string_returns_none()
{
  let config = make_config();
  assert_eq!(extract_u8(&config, "string_val"), None);
}

#[test]
fn extract_bool_from_string_returns_none()
{
  let config = make_config();
  assert_eq!(extract_bool(&config, "string_val"), None);
}

#[test]
fn extract_string_from_number_returns_none()
{
  let config = make_config();
  assert_eq!(extract_string(&config, "u8_val"), None);
}

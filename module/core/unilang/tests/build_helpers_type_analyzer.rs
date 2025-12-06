//! Tests for `build_helpers::type_analyzer` module
//!
//! ## Overview
//!
//! This module tests the `TypeAnalyzer` that detects potential type mismatches in YAML
//! command definitions during build time. The analyzer provides helpful suggestions
//! when it detects String-typed arguments that appear to be Boolean or Integer values.
//!
//! ## Test Matrix
//!
//! | Test Name | Purpose | What It Tests | Expected Outcome |
//! |-----------|---------|---------------|------------------|
//! | `detects_boolean_as_string` | Core detection | Boolean literal "true" in String argument with boolean-suggestive name | Hint generated |
//! | `no_false_positive_for_code_template` | False positive prevention | "true" in code template context (non-boolean name) | No hint |
//! | `respects_local_suppression` | Suppression mechanism | `suppress_type_hint` attribute | No hint generated |
//! | `detects_integer_as_string` | Core detection | Integer literal "1" in String argument with integer-suggestive name | Hint generated |
//! | `no_false_positive_for_version` | False positive prevention | Dotted version string "1.0.0" | No hint (contains dot) |
//! | `no_false_positive_for_zero_padded` | False positive prevention | Zero-padded ID "0001" | No hint (leading zero) |
//! | `respects_global_suppression` | Global suppression | `UNILANG_SUPPRESS_TYPE_HINTS` env var | No hints |
//! | `no_hint_for_generic_string_argument` | Normal case | Generic string "Hello" | No hint |
//! | `detects_dry_run_boolean` | Keyword detection | `dry_run` name with "false" default | Hint generated |
//! | `detects_count_integer` | Keyword detection | `retry_count` name with "3" default | Hint generated |
//!
//! ## Test Coverage
//!
//! - ✅ Happy path: Boolean and integer type hints detected correctly
//! - ✅ Edge cases: Zero-padded numbers, version strings, code templates
//! - ✅ Error cases: Not applicable (warnings only, never errors)
//! - ✅ Integration: Tests with `serde_yaml` `Value` objects
//! - ✅ Suppression: Both local (per-argument) and global (env var) suppression
//! - ✅ Context awareness: Name-based heuristics (`dry_run`, count, etc.)
//! - ✅ False positive prevention: Conservative detection to minimize noise
//!
//! ## Known Pitfalls
//!
//! - **Environment pollution**: Tests modify `UNILANG_SUPPRESS_TYPE_HINTS` env var.
//!   Mitigation: Always `remove_var` after test to avoid cross-test pollution.
//!
//! - **Context sensitivity**: Detection relies on argument name keywords. Edge case:
//!   An argument named "count" that legitimately stores a string won't be detected.
//!   Mitigation: User can add `suppress_type_hint: true`.
//!
//! - **YAML structure assumptions**: Tests assume `attributes.default` structure.
//!   Mitigation: Analyzer checks both `attributes.default` and direct default field.
//!
//! ## Dependencies
//!
//! - Requires: `serde_yaml` for `Value` deserialization
//! - Uses: `unilang::build_helpers::type_analyzer` module
//! - Uses: `unilang` `ArgumentDefinition` and `Kind` types

use serde_yaml::{ Value, from_str };
use unilang::build_helpers::type_analyzer::{ TypeAnalyzer, TypeHint };

#[test]
fn detects_boolean_as_string()
{
  let yaml = r#"
    name: "enabled"
    kind: "String"
    attributes:
      default: "true"
    description: "Enable feature"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 1 );
  assert!( matches!( hints[ 0 ], TypeHint::BooleanAsString { .. } ) );
}

#[test]
fn no_false_positive_for_code_template()
{
  let yaml = r#"
    name: "template"
    kind: "String"
    attributes:
      default: "return true;"
    description: "Code template"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  // Should not warn - "template" is not boolean-suggestive
  assert_eq!( hints.len(), 0 );
}

#[test]
fn respects_local_suppression()
{
  let yaml = r#"
    name: "enabled"
    kind: "String"
    attributes:
      default: "true"
      suppress_type_hint: true
    description: "Enable feature"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 0 );
}

#[test]
fn detects_integer_as_string()
{
  let yaml = r#"
    name: "verbosity"
    kind: "String"
    attributes:
      default: "1"
    description: "Verbosity level"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 1 );
  assert!( matches!( hints[ 0 ], TypeHint::IntegerAsString { .. } ) );
}

#[test]
fn no_false_positive_for_version()
{
  let yaml = r#"
    name: "version"
    kind: "String"
    attributes:
      default: "1.0.0"
    description: "Version number"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  // Should not warn - contains dot, not pure integer
  assert_eq!( hints.len(), 0 );
}

#[test]
fn no_false_positive_for_zero_padded()
{
  let yaml = r#"
    name: "id"
    kind: "String"
    attributes:
      default: "0001"
    description: "Identifier"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  // Should not warn - starts with zero, not pure integer
  assert_eq!( hints.len(), 0 );
}

#[test]
fn respects_global_suppression()
{
  std::env::set_var( "UNILANG_SUPPRESS_TYPE_HINTS", "1" );

  let yaml = r#"
    name: "enabled"
    kind: "String"
    attributes:
      default: "true"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 0 );

  std::env::remove_var( "UNILANG_SUPPRESS_TYPE_HINTS" );
}

#[test]
fn no_hint_for_generic_string_argument()
{
  let yaml = r#"
    name: "message"
    kind: "String"
    attributes:
      default: "Hello"
    description: "Message to display"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 0 );
}

#[test]
fn detects_dry_run_boolean()
{
  let yaml = r#"
    name: "dry_run"
    kind: "String"
    attributes:
      default: "false"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 1 );
  assert!( matches!( hints[ 0 ], TypeHint::BooleanAsString { .. } ) );
}

#[test]
fn detects_count_integer()
{
  let yaml = r#"
    name: "retry_count"
    kind: "String"
    attributes:
      default: "3"
  "#;

  let arg : Value = from_str( yaml ).unwrap();
  let analyzer = TypeAnalyzer::new();
  let hints = analyzer.analyze_argument( &arg );

  assert_eq!( hints.len(), 1 );
  assert!( matches!( hints[ 0 ], TypeHint::IntegerAsString { .. } ) );
}

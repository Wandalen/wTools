//! Tests for the `cli_parser` module.
//!
//! Comprehensive test coverage for CLI parameter parsing with message tail collection.
//!
//! # Design Decisions
//!
//! ## Two-Phase Parsing Algorithm
//!
//! The parser uses a two-phase approach:
//! 1. **Parameter phase**: Extract `key::value` pairs until a non-parameter token
//! 2. **Message phase**: Collect remaining tokens as message string
//!
//! This matches the `wplan_agent` pattern where parameters precede the message.
//!
//! ## `CliParams` vs `CliParamsAdvanced`
//!
//! - **`CliParams`**: Simple trait returning `Result<bool, String>` for recognized/unrecognized
//! - **`CliParamsAdvanced`**: Config-aware trait with explicit parameter tracking
//!
//! ### Why `Option<&'static str>` for `CliParamsAdvanced::process_param`?
//!
//! The return type `Result<Option<&'static str>, String>` serves two purposes:
//! - `Some("canonical_name")`: Parameter was recognized, add canonical name to explicit set
//! - `None`: Parameter was not recognized, treat token as message start
//!
//! This enables **alias tracking**: both "v" and "verbosity" map to canonical "verbosity",
//! allowing `apply_defaults()` to check a single name regardless of which alias was used.
//!
//! ### Why `BTreeSet<String>` instead of `HashSet`?
//!
//! - **`no_std` compatibility**: `BTreeSet` is available via `alloc::collections`
//! - **Deterministic iteration**: Useful for debugging and testing
//! - **Tradeoff**: Slightly slower than `HashSet`, but consistent behavior
//!
//! ## Message Validation Timing (Known Pitfall)
//!
//! **Problem**: The trait's `validate()` is called before message is assigned to params.
//!
//! When using `CliParamsAdvanced`, message validation cannot be done in `validate()`
//! because the message field isn't populated until after parsing completes.
//!
//! **Solution**: Perform message validation in your `parse_with_config()` wrapper
//! after extracting the message from `CliParseResultAdvanced`.
//!
//! ```rust,ignore
//! // In your parse_with_config():
//! let result = CliParser::new().with_config(config).parse(args)?;
//! let mut params = result.params;
//! params.message = result.message;  // Assign message FIRST
//! if params.message.is_empty() && !params.interactive {
//!   return Err("Message required".into());  // THEN validate
//! }
//! ```
//!
//! ## Config by Reference Design
//!
//! Config is passed by reference (`&C`) rather than owned because:
//! - Config may be large (e.g., a `HashMap`) and should not be copied
//! - Builder pattern with `with_config()` makes ownership explicit
//! - Lifetime parameter ties result to config lifetime
//!
//! # Test Organization
//!
//! - **Basic tests**: Core parsing functionality
//! - **Advanced tests**: Config-aware parsing with `CliParamsAdvanced`
//! - **Edge cases**: Empty input, no params, no message, validation errors

use unilang_parser::cli_parser::{parse_cli_args, parse_cli_str_args, CliParams, CliParseResult};

/// Test parameter struct for most tests.
#[derive(Default, Debug, Clone, PartialEq)]
struct TestParams
{
  timeout: u64,
  verbose: bool,
  dry_run: bool,
  tags: Vec<String>,
  /// When true, unknown params return Err instead of Ok(false)
  error_on_unknown: bool,
}

impl CliParams for TestParams
{
  fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
  {
    match key
    {
      "timeout" =>
      {
        self.timeout = value.parse()
          .map_err(|e| format!("Invalid timeout: {e}"))?;
      }
      "verbose" =>
      {
        self.verbose = value == "true" || value == "1";
      }
      "dry" =>
      {
        self.dry_run = value != "0";
      }
      "tag" =>
      {
        self.tags.push(value.to_string());
      }
      "error_on_unknown" =>
      {
        self.error_on_unknown = value == "true" || value == "1";
      }
      _ =>
      {
        if self.error_on_unknown
        {
          return Err(format!("Unknown parameter: {key}"));
        }
        return Ok(false);
      }
    }
    Ok(true)
  }

  fn validate(&self) -> Result<(), String>
  {
    // Timeout validation is optional for these tests
    // Only validate if explicitly set to 0 and error_on_unknown is set
    // (to test validation functionality without breaking other tests)
    Ok(())
  }
}

/// Parameter struct that validates timeout > 0
#[derive(Default, Debug)]
struct ValidatingParams
{
  timeout: u64,
}

impl CliParams for ValidatingParams
{
  fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
  {
    match key
    {
      "timeout" =>
      {
        self.timeout = value.parse()
          .map_err(|e| format!("Invalid timeout: {e}"))?;
      }
      _ => return Ok(false),
    }
    Ok(true)
  }

  fn validate(&self) -> Result<(), String>
  {
    if self.timeout == 0
    {
      return Err("timeout must be > 0".into());
    }
    Ok(())
  }
}

/// Parameter struct that errors on unknown params
#[derive(Default, Debug)]
struct StrictParams
{
  timeout: u64,
  verbose: bool,
}

impl CliParams for StrictParams
{
  fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
  {
    match key
    {
      "timeout" =>
      {
        self.timeout = value.parse()
          .map_err(|e| format!("Invalid timeout: {e}"))?;
      }
      "verbose" =>
      {
        self.verbose = value == "true" || value == "1";
      }
      _ => return Err(format!("Unknown parameter: {key}")),
    }
    Ok(true)
  }
}

// =============================================================================
// Basic Parsing Tests
// =============================================================================

#[test]
fn basic_parsing()
{
  let args = vec![
    "timeout::5000".to_string(),
    "hello".to_string(),
    "world".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 5000);
  assert_eq!(result.message, "hello world");
}

#[test]
fn no_message()
{
  let args = vec!["timeout::5000".to_string()];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 5000);
  assert_eq!(result.message, "");
}

#[test]
fn no_params()
{
  let args = vec![
    "hello".to_string(),
    "world".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 0); // Default
  assert!(!result.params.verbose); // Default
  assert_eq!(result.message, "hello world");
}

#[test]
fn empty_input()
{
  let args: Vec<String> = vec![];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 0);
  assert_eq!(result.message, "");
}

// =============================================================================
// Multiple Parameters Tests
// =============================================================================

#[test]
fn multiple_different_params()
{
  let args = vec![
    "timeout::1000".to_string(),
    "verbose::true".to_string(),
    "dry::1".to_string(),
    "run".to_string(),
    "tests".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 1000);
  assert!(result.params.verbose);
  assert!(result.params.dry_run);
  assert_eq!(result.message, "run tests");
}

#[test]
fn multiple_same_param()
{
  let args = vec![
    "tag::a".to_string(),
    "tag::b".to_string(),
    "tag::c".to_string(),
    "message".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.tags, vec!["a", "b", "c"]);
  assert_eq!(result.message, "message");
}

#[test]
fn params_in_any_order()
{
  let args = vec![
    "dry::1".to_string(),
    "timeout::2000".to_string(),
    "verbose::true".to_string(),
    "msg".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 2000);
  assert!(result.params.verbose);
  assert!(result.params.dry_run);
  assert_eq!(result.message, "msg");
}

// =============================================================================
// Boolean Flag Tests
// =============================================================================

#[test]
fn boolean_true_variations()
{
  // verbose::true
  let args1 = vec!["verbose::true".to_string(), "msg".to_string()];
  let result1: CliParseResult<TestParams> = parse_cli_args(&args1).unwrap();
  assert!(result1.params.verbose);

  // verbose::1
  let args2 = vec!["verbose::1".to_string(), "msg".to_string()];
  let result2: CliParseResult<TestParams> = parse_cli_args(&args2).unwrap();
  assert!(result2.params.verbose);
}

#[test]
fn boolean_false_variations()
{
  // verbose::false
  let args1 = vec!["verbose::false".to_string(), "msg".to_string()];
  let result1: CliParseResult<TestParams> = parse_cli_args(&args1).unwrap();
  assert!(!result1.params.verbose);

  // verbose::0
  let args2 = vec!["verbose::0".to_string(), "msg".to_string()];
  let result2: CliParseResult<TestParams> = parse_cli_args(&args2).unwrap();
  assert!(!result2.params.verbose);
}

#[test]
fn dry_run_modes()
{
  // dry::0 = false
  let args1 = vec!["dry::0".to_string(), "msg".to_string()];
  let result1: CliParseResult<TestParams> = parse_cli_args(&args1).unwrap();
  assert!(!result1.params.dry_run);

  // dry::1 = true
  let args2 = vec!["dry::1".to_string(), "msg".to_string()];
  let result2: CliParseResult<TestParams> = parse_cli_args(&args2).unwrap();
  assert!(result2.params.dry_run);
}

// =============================================================================
// Unknown Parameter Tests
// =============================================================================

#[test]
fn unknown_param_as_message()
{
  let args = vec![
    "timeout::100".to_string(),
    "unknown::value".to_string(),
    "text".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 100);
  // Unknown param and following become message
  assert!(result.message.contains("unknown::value"));
  assert!(result.message.contains("text"));
}

#[test]
fn unknown_param_error()
{
  let args = vec![
    "unknown::value".to_string(),
    "message".to_string(),
  ];

  let result: Result<CliParseResult<StrictParams>, String> = parse_cli_args(&args);

  assert!(result.is_err());
  let err = result.unwrap_err();
  assert!(err.contains("Unknown parameter: unknown"));
}

// =============================================================================
// Validation Tests
// =============================================================================

#[test]
fn validation_error()
{
  let args = vec!["timeout::0".to_string()];

  let result: Result<CliParseResult<ValidatingParams>, String> = parse_cli_args(&args);

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("timeout must be > 0"));
}

#[test]
fn validation_passes()
{
  let args = vec!["timeout::100".to_string()];

  let result: CliParseResult<ValidatingParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 100);
}

#[test]
fn validation_on_empty_input()
{
  let args: Vec<String> = vec![];

  // ValidatingParams requires timeout > 0, but default is 0
  let result: Result<CliParseResult<ValidatingParams>, String> = parse_cli_args(&args);

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("timeout must be > 0"));
}

// =============================================================================
// Invalid Value Tests
// =============================================================================

#[test]
fn invalid_timeout_value()
{
  let args = vec![
    "timeout::not_a_number".to_string(),
    "message".to_string(),
  ];

  let result: Result<CliParseResult<TestParams>, String> = parse_cli_args(&args);

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Invalid timeout"));
}

#[test]
fn invalid_numeric_format()
{
  let args = vec![
    "timeout::12.5".to_string(), // u64 cant parse float
    "message".to_string(),
  ];

  let result: Result<CliParseResult<TestParams>, String> = parse_cli_args(&args);

  assert!(result.is_err());
}

// =============================================================================
// Message with Special Content Tests
// =============================================================================

#[test]
fn message_with_colons()
{
  let args = vec![
    "timeout::100".to_string(),
    "check".to_string(),
    "std::vector".to_string(),
    "in".to_string(),
    "C++".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 100);
  assert_eq!(result.message, "check std::vector in C++");
}

#[test]
fn message_with_special_chars()
{
  let args = vec![
    "dry::1".to_string(),
    "what".to_string(),
    "is".to_string(),
    "$PATH?".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert!(result.params.dry_run);
  assert_eq!(result.message, "what is $PATH?");
}

#[test]
fn param_syntax_in_message()
{
  // Once message phase starts, param::value syntax is literal
  let args = vec![
    "msg".to_string(),
    "timeout::100".to_string(),
    "after".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  // timeout::100 should be in message, not parsed as param
  assert_eq!(result.params.timeout, 0); // Default, not 100
  assert_eq!(result.message, "msg timeout::100 after");
}

// =============================================================================
// Convenience Wrapper Tests
// =============================================================================

#[test]
fn parse_str_args_basic()
{
  let args = &["timeout::5000", "hello", "world"];

  let result: CliParseResult<TestParams> = parse_cli_str_args(args).unwrap();

  assert_eq!(result.params.timeout, 5000);
  assert_eq!(result.message, "hello world");
}

#[test]
fn parse_str_args_empty()
{
  let args: &[&str] = &[];

  let result: CliParseResult<TestParams> = parse_cli_str_args(args).unwrap();

  assert_eq!(result.params.timeout, 0);
  assert_eq!(result.message, "");
}

#[test]
fn parse_str_args_with_multiple_params()
{
  let args = &["timeout::1000", "verbose::true", "dry::1", "run", "tests"];

  let result: CliParseResult<TestParams> = parse_cli_str_args(args).unwrap();

  assert_eq!(result.params.timeout, 1000);
  assert!(result.params.verbose);
  assert!(result.params.dry_run);
  assert_eq!(result.message, "run tests");
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn single_word_message()
{
  let args = vec!["hello".to_string()];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.message, "hello");
}

#[test]
fn only_params_no_message()
{
  let args = vec![
    "timeout::1000".to_string(),
    "verbose::true".to_string(),
    "dry::1".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 1000);
  assert!(result.params.verbose);
  assert!(result.params.dry_run);
  assert_eq!(result.message, "");
}

#[test]
fn long_message()
{
  let args = vec![
    "timeout::100".to_string(),
    "this".to_string(),
    "is".to_string(),
    "a".to_string(),
    "very".to_string(),
    "long".to_string(),
    "message".to_string(),
    "with".to_string(),
    "many".to_string(),
    "words".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 100);
  assert_eq!(result.message, "this is a very long message with many words");
}

#[test]
fn numeric_string_in_message()
{
  let args = vec![
    "timeout::100".to_string(),
    "error".to_string(),
    "code".to_string(),
    "42".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.message, "error code 42");
}

// =============================================================================
// Real-world Usage Pattern Tests
// =============================================================================

#[test]
fn wplan_agent_pattern()
{
  // Simulates: wplan .please session::debug timeout::7200000 fix the bug
  let args = vec![
    "timeout::7200000".to_string(),
    "verbose::true".to_string(),
    "dry::0".to_string(),
    "fix".to_string(),
    "the".to_string(),
    "bug".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 7_200_000);
  assert!(result.params.verbose);
  assert!(!result.params.dry_run);
  assert_eq!(result.message, "fix the bug");
}

#[test]
fn all_defaults()
{
  // No params, just message
  let args = vec![
    "analyze".to_string(),
    "this".to_string(),
    "code".to_string(),
  ];

  let result: CliParseResult<TestParams> = parse_cli_args(&args).unwrap();

  assert_eq!(result.params.timeout, 0);
  assert!(!result.params.verbose);
  assert!(!result.params.dry_run);
  assert!(result.params.tags.is_empty());
  assert_eq!(result.message, "analyze this code");
}

// =============================================================================
// Advanced Config-Aware Parsing Tests
// =============================================================================

use unilang_parser::cli_parser::{CliParser, CliParamsAdvanced, CliParseResultAdvanced};
use std::collections::{BTreeSet, HashMap};

/// Test configuration type
type TestConfig = HashMap<String, u64>;

/// Parameters with config-aware defaults
#[derive(Default, Debug)]
struct ConfigParams
{
  timeout: u64,
  verbosity: u8,
  dry_run: bool,
  interactive: bool,
}

impl CliParamsAdvanced<TestConfig> for ConfigParams
{
  fn process_param(&mut self, key: &str, value: &str) -> Result<Option<&'static str>, String>
  {
    match key
    {
      "timeout" =>
      {
        self.timeout = value.parse()
          .map_err(|e| format!("Invalid timeout: {e}"))?;
        Ok(Some("timeout"))
      }
      "v" | "verbosity" =>
      {
        self.verbosity = value.parse()
          .map_err(|e| format!("Invalid verbosity: {e}"))?;
        Ok(Some("verbosity"))
      }
      "dry" =>
      {
        self.dry_run = value != "0";
        Ok(Some("dry_run"))
      }
      "interactive" =>
      {
        self.interactive = value == "1";
        Ok(Some("interactive"))
      }
      _ => Ok(None),
    }
  }

  fn apply_defaults(&mut self, config: &TestConfig, explicit: &BTreeSet<String>)
  {
    if !explicit.contains("timeout")
    {
      self.timeout = *config.get("timeout").unwrap_or(&30000);
    }
    if !explicit.contains("verbosity")
    {
      self.verbosity = (*config.get("verbosity").unwrap_or(&2)).min(255) as u8;
    }
  }

  fn finalize(&mut self, explicit: &BTreeSet<String>, message: &str)
  {
    // Smart default: enable interactive when no message
    if message.is_empty() && !explicit.contains("interactive")
    {
      self.interactive = true;
    }
  }

  fn validate(&self) -> Result<(), String>
  {
    if self.verbosity > 5
    {
      return Err("verbosity must be 0-5".to_string());
    }
    Ok(())
  }
}

#[test]
fn advanced_basic_parsing()
{
  let config: TestConfig = HashMap::new();
  let args = vec![
    "timeout::5000".to_string(),
    "hello".to_string(),
    "world".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  assert_eq!(result.params.timeout, 5000);
  assert_eq!(result.message, "hello world");
  assert!(result.explicit_params.contains("timeout"));
  assert!(!result.explicit_params.contains("verbosity"));
}

#[test]
fn advanced_config_defaults_applied()
{
  let mut config: TestConfig = HashMap::new();
  config.insert("timeout".to_string(), 60000);
  config.insert("verbosity".to_string(), 3);

  let args = vec![
    "run".to_string(),
    "tests".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  // Defaults from config should be applied
  assert_eq!(result.params.timeout, 60000);
  assert_eq!(result.params.verbosity, 3);
  assert_eq!(result.message, "run tests");
  
  // Nothing was explicitly set
  assert!(result.explicit_params.is_empty());
}

#[test]
fn advanced_explicit_overrides_config()
{
  let mut config: TestConfig = HashMap::new();
  config.insert("timeout".to_string(), 60000);
  config.insert("verbosity".to_string(), 3);

  let args = vec![
    "timeout::1000".to_string(),
    "do".to_string(),
    "something".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  // Explicit value overrides config
  assert_eq!(result.params.timeout, 1000);
  // Verbosity uses config default
  assert_eq!(result.params.verbosity, 3);
  
  // Only timeout was explicitly set
  assert!(result.explicit_params.contains("timeout"));
  assert!(!result.explicit_params.contains("verbosity"));
}

#[test]
fn advanced_alias_tracking()
{
  let config: TestConfig = HashMap::new();
  let args = vec![
    "v::4".to_string(), // alias for verbosity
    "message".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  assert_eq!(result.params.verbosity, 4);
  // Canonical name is tracked, not alias
  assert!(result.explicit_params.contains("verbosity"));
  assert!(!result.explicit_params.contains("v"));
}

#[test]
fn advanced_finalize_smart_defaults()
{
  let config: TestConfig = HashMap::new();
  
  // No message - should enable interactive
  let args: Vec<String> = vec![];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  assert!(result.params.interactive);
  assert!(result.message.is_empty());
}

#[test]
fn advanced_finalize_explicit_interactive()
{
  let config: TestConfig = HashMap::new();
  
  // Explicit interactive::0 with no message
  let args = vec![
    "interactive::0".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  // Should NOT enable interactive since explicitly set
  assert!(!result.params.interactive);
  assert!(result.explicit_params.contains("interactive"));
}

#[test]
fn advanced_validation_error()
{
  let config: TestConfig = HashMap::new();
  let args = vec![
    "v::10".to_string(), // Invalid: > 5
    "message".to_string(),
  ];

  let result = CliParser::new()
    .with_config(&config)
    .parse::<ConfigParams>(&args);

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("verbosity must be 0-5"));
}

#[test]
fn advanced_multiple_params()
{
  let config: TestConfig = HashMap::new();
  let args = vec![
    "timeout::5000".to_string(),
    "v::3".to_string(),
    "dry::1".to_string(),
    "run".to_string(),
    "tests".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  assert_eq!(result.params.timeout, 5000);
  assert_eq!(result.params.verbosity, 3);
  assert!(result.params.dry_run);
  assert_eq!(result.message, "run tests");
  
  // All three were explicitly set
  assert!(result.explicit_params.contains("timeout"));
  assert!(result.explicit_params.contains("verbosity"));
  assert!(result.explicit_params.contains("dry_run"));
}

#[test]
fn advanced_no_config_error()
{
  let args = vec!["message".to_string()];

  // Missing with_config() call
  let result = CliParser::<TestConfig>::new()
    .parse::<ConfigParams>(&args);

  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Configuration required"));
}

#[test]
fn advanced_parse_str_convenience()
{
  let config: TestConfig = HashMap::new();
  let args = &["timeout::100", "hello"];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse_str(args)
    .unwrap();

  assert_eq!(result.params.timeout, 100);
  assert_eq!(result.message, "hello");
}

#[test]
fn advanced_unknown_param_starts_message()
{
  let config: TestConfig = HashMap::new();
  let args = vec![
    "timeout::100".to_string(),
    "unknown::value".to_string(),
    "rest".to_string(),
  ];

  let result: CliParseResultAdvanced<ConfigParams> = CliParser::new()
    .with_config(&config)
    .parse(&args)
    .unwrap();

  assert_eq!(result.params.timeout, 100);
  assert_eq!(result.message, "unknown::value rest");
}

#[test]
fn advanced_debug_impl()
{
  let config: TestConfig = HashMap::new();
  let parser = CliParser::new().with_config(&config);
  
  // Should not panic
  let debug_str = format!("{parser:?}");
  assert!(debug_str.contains("CliParser"));
  assert!(debug_str.contains("config"));
}

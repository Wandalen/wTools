//! Convenience API for CLI parameter parsing with message tail collection.
//!
//! This module provides a higher-level interface on top of [`Parser`](crate::Parser)
//! for CLI tools that need to extract `param::value` arguments and collect
//! remaining tokens as a single message string.
//!
//! # Use Case
//!
//! Many CLI tools need to parse arguments in the pattern:
//!
//! ```bash
//! command session::resume timeout::7200000 tell me about the code
//! ```
//!
//! Where named parameters (`session::resume`, `timeout::7200000`) are extracted
//! and the remaining tokens (`tell me about the code`) become a message.
//!
//! # Algorithm
//!
//! Parsing occurs in two phases:
//!
//! 1. **Parameter Phase**: Process `param::value` pairs until:
//!    - A positional argument (no `::`) is encountered, or
//!    - An unknown parameter is encountered (if `process_param` returns `Ok(false)`)
//!
//! 2. **Message Phase**: All remaining tokens are joined into a single string
//!
//! # Example
//!
//! ```rust
//! use unilang_parser::cli_parser::{parse_cli_args, CliParams, CliParseResult};
//!
//! #[derive(Default)]
//! struct MyParams
//! {
//!   timeout: u64,
//!   verbose: bool,
//! }
//!
//! impl CliParams for MyParams
//! {
//!   fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
//!   {
//!     match key
//!     {
//!       "timeout" => { self.timeout = value.parse().map_err(|e| format!("{}", e))?; }
//!       "verbose" => { self.verbose = value.parse().map_err(|e| format!("{}", e))?; }
//!       _ => return Ok(false), // Unknown parameter starts message phase
//!     }
//!     Ok(true)
//!   }
//!
//!   fn validate(&self) -> Result<(), String>
//!   {
//!     if self.timeout == 0
//!     {
//!       return Err("timeout must be > 0".into());
//!     }
//!     Ok(())
//!   }
//! }
//!
//! fn main() -> Result<(), String>
//! {
//!   let args = vec![
//!     "timeout::5000".to_string(),
//!     "verbose::true".to_string(),
//!     "tell".to_string(),
//!     "me".to_string(),
//!     "about".to_string(),
//!     "the".to_string(),
//!     "code".to_string(),
//!   ];
//!
//!   let result: CliParseResult<MyParams> = parse_cli_args(&args)?;
//!
//!   assert_eq!(result.params.timeout, 5000);
//!   assert!(result.params.verbose);
//!   assert_eq!(result.message, "tell me about the code");
//!
//!   Ok(())
//! }
//! ```

use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Result of CLI parsing with separated parameters and message.
///
/// Contains the parsed typed parameters and the remaining tokens
/// joined as a single message string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliParseResult<T>
{
  /// Parsed parameters from `param::value` tokens.
  pub params: T,
  /// Remaining tokens joined as message string.
  /// Empty string if no positional arguments were present.
  pub message: String,
}

/// Trait for parameter sets that can be parsed from CLI arguments.
///
/// Implementors define how to process individual `key::value` parameters
/// and optionally validate the complete parameter set.
///
/// # Return Value Semantics
///
/// The `process_param` method returns `Result<bool, String>`:
///
/// - `Ok(true)` - Parameter was recognized and processed successfully
/// - `Ok(false)` - Parameter was not recognized; starts message phase
/// - `Err(String)` - Parameter was recognized but had an invalid value
///
/// This design allows tools to choose whether unknown parameters should
/// error (return `Err`) or be treated as the start of the message (return `Ok(false)`).
pub trait CliParams: Default
{
  /// Process a single `key::value` parameter.
  ///
  /// Called for each named argument found in the input. The implementor
  /// should match on the key and parse the value accordingly.
  ///
  /// # Parameters
  ///
  /// - `key`: The parameter name (e.g., "timeout" from `timeout::5000`)
  /// - `value`: The parameter value (e.g., "5000" from `timeout::5000`)
  ///
  /// # Returns
  ///
  /// - `Ok(true)` if the parameter was recognized and processed
  /// - `Ok(false)` if the parameter was not recognized
  /// - `Err(String)` if the parameter value was invalid
  ///
  /// # Errors
  ///
  /// Returns `Err(String)` if the parameter value is invalid or cannot be parsed.
  ///
  /// # Example
  ///
  /// ```rust
  /// use unilang_parser::cli_parser::CliParams;
  ///
  /// #[derive(Default)]
  /// struct Params { timeout: u64 }
  ///
  /// impl CliParams for Params
  /// {
  ///   fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
  ///   {
  ///     match key
  ///     {
  ///       "timeout" =>
  ///       {
  ///         self.timeout = value.parse()
  ///           .map_err(|e| format!("Invalid timeout: {}", e))?;
  ///       }
  ///       _ => return Ok(false),
  ///     }
  ///     Ok(true)
  ///   }
  /// }
  /// ```
  fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>;

  /// Validate the complete parameter set after all parameters are processed.
  ///
  /// Called once after all `process_param` calls complete. Use this to
  /// check constraints that span multiple parameters or require all
  /// parameters to be set.
  ///
  /// # Returns
  ///
  /// - `Ok(())` if validation passes
  /// - `Err(String)` with error description if validation fails
  ///
  /// # Errors
  ///
  /// Returns `Err(String)` if validation fails.
  ///
  /// # Default Implementation
  ///
  /// Returns `Ok(())` (no validation).
  fn validate(&self) -> Result<(), String>
  {
    Ok(())
  }
}

/// Parse CLI arguments into typed parameters and message tail.
///
/// Uses the unilang parser to tokenize and classify arguments, then
/// binds named arguments to the parameter type and joins positional
/// arguments into a message string.
///
/// # Parameters
///
/// - `args`: Slice of command-line arguments (typically from `std::env::args()`)
///
/// # Returns
///
/// - `Ok(CliParseResult<T>)` with parsed parameters and message
/// - `Err(String)` if parsing or validation fails
///
/// # Errors
///
/// Returns an error if:
/// - A known parameter has an invalid value
/// - Validation fails (from `CliParams::validate`)
/// - The underlying parser encounters a syntax error
///
/// # Example
///
/// ```rust
/// use unilang_parser::cli_parser::{parse_cli_args, CliParams, CliParseResult};
///
/// #[derive(Default)]
/// struct Params { dry_run: bool }
///
/// impl CliParams for Params
/// {
///   fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
///   {
///     match key
///     {
///       "dry" =>
///       {
///         self.dry_run = value != "0";
///       }
///       _ => return Ok(false),
///     }
///     Ok(true)
///   }
/// }
///
/// let args = vec!["dry::1".to_string(), "run".to_string(), "tests".to_string()];
/// let result: CliParseResult<Params> = parse_cli_args(&args).unwrap();
///
/// assert!(result.params.dry_run);
/// assert_eq!(result.message, "run tests");
/// ```
pub fn parse_cli_args<T: CliParams>(args: &[String]) -> Result<CliParseResult<T>, String>
{
  // Handle empty input early
  if args.is_empty()
  {
    let params = T::default();
    params.validate()?;
    return Ok(CliParseResult
    {
      params,
      message: String::new(),
    });
  }

  // Initialize parameters with defaults
  let mut params = T::default();
  let mut message_parts = Vec::new();
  let mut parsing_params = true;

  // Two-phase parsing:
  // 1. Parameter phase: process key::value pairs
  // 2. Message phase: collect remaining tokens as message
  for arg in args
  {
    if parsing_params
    {
      // Try to parse as parameter (key::value)
      if let Some((key, value)) = arg.split_once("::")
      {
        let processed = params.process_param(key, value)?;
        if !processed
        {
          // Unknown parameter - switch to message phase
          parsing_params = false;
          message_parts.push(arg.clone());
        }
      }
      else
      {
        // No :: delimiter - start message phase
        parsing_params = false;
        message_parts.push(arg.clone());
      }
    }
    else
    {
      // Already in message phase - consume all remaining args
      message_parts.push(arg.clone());
    }
  }

  // Validate the complete parameter set
  params.validate()?;

  // Join message parts
  let message = message_parts.join(" ");

  Ok(CliParseResult { params, message })
}

/// Convenience wrapper for parsing `&str` slices.
///
/// Converts the `&str` slice to `String` slice and calls [`parse_cli_args`].
///
/// # Parameters
///
/// - `args`: Slice of string slices
///
/// # Returns
///
/// Same as [`parse_cli_args`]
///
/// # Errors
///
/// Same as [`parse_cli_args`]
///
/// # Example
///
/// ```rust
/// use unilang_parser::cli_parser::{parse_cli_str_args, CliParams, CliParseResult};
///
/// #[derive(Default)]
/// struct Params { verbose: bool }
///
/// impl CliParams for Params
/// {
///   fn process_param(&mut self, key: &str, value: &str) -> Result<bool, String>
///   {
///     match key
///     {
///       "verbose" => { self.verbose = value == "true" || value == "1"; }
///       _ => return Ok(false),
///     }
///     Ok(true)
///   }
/// }
///
/// let args = &["verbose::true", "hello", "world"];
/// let result: CliParseResult<Params> = parse_cli_str_args(args).unwrap();
///
/// assert!(result.params.verbose);
/// assert_eq!(result.message, "hello world");
/// ```
pub fn parse_cli_str_args<T: CliParams>(args: &[&str]) -> Result<CliParseResult<T>, String>
{
  let owned: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();
  parse_cli_args(&owned)
}

// =============================================================================
// Advanced Config-Aware Parsing API
// =============================================================================

/// Result of config-aware CLI parsing with explicit parameter tracking.
///
/// Contains the parsed parameters, message, and a set of parameter names
/// that were explicitly set by the user (vs defaulted).
#[derive(Debug, Clone)]
pub struct CliParseResultAdvanced<T>
{
  /// Parsed parameters from `param::value` tokens.
  pub params: T,
  /// Remaining tokens joined as message string.
  pub message: String,
  /// Set of canonical parameter names that were explicitly provided.
  ///
  /// Use this to distinguish between user-provided values and defaults.
  /// Aliases are normalized to their canonical names.
  pub explicit_params: BTreeSet<String>,
}

/// Extended trait for config-aware parameter parsing.
///
/// This trait extends the basic [`CliParams`] with support for:
/// - Tracking which parameters were explicitly set
/// - Applying defaults from external configuration
/// - Post-parse finalization
///
/// # Type Parameter
///
/// - `C`: The configuration type (e.g., `HashMap<String, JsonValue>`)
///
/// # Example
///
/// ```rust,ignore
/// use unilang_parser::cli_parser::{CliParamsAdvanced, CliParser};
/// use std::collections::HashMap;
///
/// #[derive(Default)]
/// struct Params
/// {
///   timeout: u64,
///   verbosity: u8,
/// }
///
/// impl CliParamsAdvanced<HashMap<String, u64>> for Params
/// {
///   fn process_param(&mut self, key: &str, value: &str)
///     -> Result<Option<&'static str>, String>
///   {
///     match key
///     {
///       "timeout" =>
///       {
///         self.timeout = value.parse()
///           .map_err(|e| format!("Invalid timeout: {e}"))?;
///         Ok(Some("timeout"))
///       }
///       "v" | "verbosity" =>
///       {
///         self.verbosity = value.parse()
///           .map_err(|e| format!("Invalid verbosity: {e}"))?;
///         Ok(Some("verbosity")) // Canonical name for alias
///       }
///       _ => Ok(None),
///     }
///   }
///
///   fn apply_defaults(
///     &mut self,
///     config: &HashMap<String, u64>,
///     explicit: &BTreeSet<String>,
///   )
///   {
///     if !explicit.contains("timeout")
///     {
///       self.timeout = *config.get("timeout").unwrap_or(&30000);
///     }
///     if !explicit.contains("verbosity")
///     {
///       self.verbosity = *config.get("verbosity").unwrap_or(&2) as u8;
///     }
///   }
/// }
/// ```
pub trait CliParamsAdvanced<C>: Default
{
  /// Process a single `key::value` parameter with tracking.
  ///
  /// Similar to [`CliParams::process_param`] but returns the canonical
  /// parameter name when successfully processed, enabling alias tracking.
  ///
  /// # Parameters
  ///
  /// - `key`: The parameter name (e.g., "timeout" or alias "t")
  /// - `value`: The parameter value
  ///
  /// # Returns
  ///
  /// - `Ok(Some("canonical_name"))` if parameter was processed
  /// - `Ok(None)` if parameter was not recognized (starts message phase)
  /// - `Err(String)` if parameter value was invalid
  ///
  /// # Errors
  ///
  /// Returns `Err(String)` if the parameter value is invalid.
  fn process_param(&mut self, key: &str, value: &str) -> Result<Option<&'static str>, String>;

  /// Apply configuration defaults after parsing.
  ///
  /// Called after all arguments are processed, before validation.
  /// Use the `explicit` set to avoid overwriting user-provided values.
  ///
  /// # Parameters
  ///
  /// - `config`: External configuration to read defaults from
  /// - `explicit`: Set of parameter names that were explicitly set
  fn apply_defaults(&mut self, config: &C, explicit: &BTreeSet<String>);

  /// Finalize parameters after defaults are applied.
  ///
  /// Called after `apply_defaults`, before validation.
  /// Use this for post-processing like smart defaults based on other fields.
  ///
  /// # Parameters
  ///
  /// - `explicit`: Set of parameter names that were explicitly set
  /// - `message`: The parsed message string
  ///
  /// Default implementation does nothing.
  fn finalize(&mut self, _explicit: &BTreeSet<String>, _message: &str) {}

  /// Validate the complete parameter set.
  ///
  /// Called after finalization. Check all constraints here.
  ///
  /// # Returns
  ///
  /// - `Ok(())` if validation passes
  /// - `Err(String)` with error description if validation fails
  ///
  /// # Errors
  ///
  /// Returns `Err(String)` if validation fails.
  ///
  /// Default implementation returns `Ok(())`.
  fn validate(&self) -> Result<(), String>
  {
    Ok(())
  }
}

/// Builder for config-aware CLI parsing.
///
/// Provides a fluent API for parsing CLI arguments with configuration support.
///
/// # Example
///
/// ```rust,ignore
/// use unilang_parser::cli_parser::CliParser;
///
/// let config = load_config();
/// let result = CliParser::new()
///   .with_config(&config)
///   .parse::<MyParams>(&args)?;
///
/// // Check if user explicitly set a parameter
/// if result.explicit_params.contains("timeout")
/// {
///   println!("User specified timeout: {}", result.params.timeout);
/// }
/// ```
pub struct CliParser<'a, C>
{
  config: Option<&'a C>,
}

impl<C> core::fmt::Debug for CliParser<'_, C>
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
  {
    f.debug_struct("CliParser")
      .field("config", &self.config.is_some())
      .finish()
  }
}

impl<'a, C> CliParser<'a, C>
{
  /// Create a new CLI parser builder.
  #[inline]
  #[must_use]
  pub fn new() -> Self
  {
    Self { config: None }
  }

  /// Set the configuration for default value resolution.
  ///
  /// # Parameters
  ///
  /// - `config`: Reference to configuration data
  #[inline]
  #[must_use]
  pub fn with_config(mut self, config: &'a C) -> Self
  {
    self.config = Some(config);
    self
  }

  /// Parse CLI arguments with config support.
  ///
  /// # Parameters
  ///
  /// - `args`: Slice of command-line arguments
  ///
  /// # Returns
  ///
  /// - `Ok(CliParseResultAdvanced<T>)` with parsed parameters, message, and explicit set
  /// - `Err(String)` if parsing or validation fails
  ///
  /// # Errors
  ///
  /// Returns an error if:
  /// - A known parameter has an invalid value
  /// - Validation fails
  /// - Configuration is required but not provided
  pub fn parse<T>(self, args: &[String]) -> Result<CliParseResultAdvanced<T>, String>
  where
    T: CliParamsAdvanced<C>,
  {
    let config = self.config.ok_or_else(||
      "Configuration required for CliParamsAdvanced. Use with_config().".to_string()
    )?;

    // Initialize parameters with defaults
    let mut params = T::default();
    let mut message_parts = Vec::new();
    let mut explicit_params = BTreeSet::new();
    let mut parsing_params = true;

    // Two-phase parsing
    for arg in args
    {
      if parsing_params
      {
        if let Some((key, value)) = arg.split_once("::")
        {
          if let Some(canonical) = params.process_param(key, value)?
          {
            explicit_params.insert(canonical.to_string());
          }
          else
          {
            // Unknown parameter - switch to message phase
            parsing_params = false;
            message_parts.push(arg.clone());
          }
        }
        else
        {
          // No :: delimiter - start message phase
          parsing_params = false;
          message_parts.push(arg.clone());
        }
      }
      else
      {
        message_parts.push(arg.clone());
      }
    }

    // Join message parts
    let message = message_parts.join(" ");

    // Apply configuration defaults
    params.apply_defaults(config, &explicit_params);

    // Finalize (smart defaults based on context)
    params.finalize(&explicit_params, &message);

    // Validate
    params.validate()?;

    Ok(CliParseResultAdvanced
    {
      params,
      message,
      explicit_params,
    })
  }

  /// Convenience method for parsing `&str` slices.
  ///
  /// # Parameters
  ///
  /// - `args`: Slice of string slices
  ///
  /// # Returns
  ///
  /// Same as [`CliParser::parse`]
  ///
  /// # Errors
  ///
  /// Same as [`CliParser::parse`]
  pub fn parse_str<T>(self, args: &[&str]) -> Result<CliParseResultAdvanced<T>, String>
  where
    T: CliParamsAdvanced<C>,
  {
    let owned: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();
    self.parse(&owned)
  }
}

impl<C> Default for CliParser<'_, C>
{
  fn default() -> Self
  {
    Self::new()
  }
}

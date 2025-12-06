//! Type analyzer for command arguments
//!
//! Analyzes YAML command definitions to detect potential type mismatches and
//! provides helpful suggestions during build time.
//!
//! # Philosophy
//!
//! This analyzer follows the principle: **Be helpful, not dictatorial**.
//!
//! - Suggestions are warnings, never errors (build continues)
//! - Easy suppression mechanisms (per-argument or global)
//! - Context-aware detection (low false positive rate)
//! - Clear, actionable messages showing fixes
//!
//! # Example
//!
//! ```yaml
//! # This will trigger a hint:
//! - name: "enabled"
//!   kind: "String"
//!   attributes:
//!     default: "true"  # String literal
//!
//! # Suggestion: Use kind: Boolean with default: true (no quotes)
//! ```

use serde_yaml::Value;
use crate::{ ArgumentDefinition, Kind };

/// Analyzes argument definitions for potential type issues
#[derive(Debug)]
pub struct TypeAnalyzer
{
  suppress_warnings : bool,
}

impl TypeAnalyzer
{
  /// Create a new type analyzer
  ///
  /// Checks `UNILANG_SUPPRESS_TYPE_HINTS` environment variable for global suppression.
  pub fn new() -> Self
  {
    Self
    {
      suppress_warnings : std::env::var( "UNILANG_SUPPRESS_TYPE_HINTS" )
        .map( | v | v == "1" )
        .unwrap_or( false ),
    }
  }

  /// Analyze a single argument for type hints
  ///
  /// Returns a list of type hints (warnings) if potential issues are detected.
  /// Returns empty vec if:
  /// - Warnings are globally suppressed
  /// - Argument has `suppress_type_hint: true`
  /// - No issues detected
  pub fn analyze_argument( &self, arg : &Value ) -> Vec< TypeHint >
  {
    let mut hints = Vec::new();

    let name = arg[ "name" ].as_str().unwrap_or( "" );
    let kind = arg[ "kind" ].as_str().unwrap_or( "" );

    // Get default value - check both direct and nested in attributes
    let default = arg[ "attributes" ][ "default" ].as_str()
      .or_else( || arg[ "default" ].as_str() );

    // Check for local suppression
    let suppress = arg[ "attributes" ][ "suppress_type_hint" ]
      .as_bool()
      .unwrap_or( false );

    // Skip if globally suppressed or locally suppressed
    if self.suppress_warnings || suppress
    {
      return hints;
    }

    // Check 1: Boolean-like default with String kind
    if kind == "String"
    {
      if let Some( def ) = default
      {
        if def == "true" || def == "false"
        {
          // Additional context check to reduce false positives
          if self.context_suggests_boolean( name, arg )
          {
            hints.push( TypeHint::BooleanAsString
            {
              argument_name : name.to_string(),
              default_value : def.to_string(),
              severity : Severity::Warning,
            } );
          }
        }
      }
    }

    // Check 2: Integer-like default with String kind
    if kind == "String"
    {
      if let Some( def ) = default
      {
        // Only pure integers (no dots, no leading zeros, no plus/minus)
        if def.parse::< i64 >().is_ok() &&
           !def.starts_with( '0' ) &&
           !def.contains( '.' ) &&
           def.len() > 0 &&
           def.chars().all( | c | c.is_ascii_digit() )
        {
          if self.context_suggests_integer( name, arg )
          {
            hints.push( TypeHint::IntegerAsString
            {
              argument_name : name.to_string(),
              default_value : def.to_string(),
              severity : Severity::Warning,
            } );
          }
        }
      }
    }

    hints
  }

  /// Analyze an ArgumentDefinition from unilang's data structures
  ///
  /// This is a convenience method for use with CommandDefinition objects
  /// instead of raw YAML Value objects.
  pub fn analyze_argument_definition( &self, arg : &ArgumentDefinition ) -> Vec< TypeHint >
  {
    let mut hints = Vec::new();

    // Skip if globally suppressed
    if self.suppress_warnings
    {
      return hints;
    }

    // Check for local suppression (would need to be in ArgumentAttributes)
    // For now, we'll skip this check as ArgumentAttributes doesn't have suppress_type_hint field

    // Check 1: Boolean-like default with String kind
    if matches!( arg.kind, Kind::String )
    {
      if let Some( ref default_value ) = arg.attributes.default
      {
        if default_value == "true" || default_value == "false"
        {
          if self.context_suggests_boolean_for_arg( &arg.name, &arg.description )
          {
            hints.push( TypeHint::BooleanAsString
            {
              argument_name : arg.name.clone(),
              default_value : default_value.clone(),
              severity : Severity::Warning,
            } );
          }
        }
      }
    }

    // Check 2: Integer-like default with String kind
    if matches!( arg.kind, Kind::String )
    {
      if let Some( ref default_value ) = arg.attributes.default
      {
        // Only pure integers (no dots, no leading zeros)
        if default_value.parse::< i64 >().is_ok() &&
           !default_value.starts_with( '0' ) &&
           !default_value.contains( '.' ) &&
           !default_value.is_empty() &&
           default_value.chars().all( | c | c.is_ascii_digit() )
        {
          if self.context_suggests_integer_for_arg( &arg.name )
          {
            hints.push( TypeHint::IntegerAsString
            {
              argument_name : arg.name.clone(),
              default_value : default_value.clone(),
              severity : Severity::Warning,
            } );
          }
        }
      }
    }

    hints
  }

  /// Check if context suggests boolean (for ArgumentDefinition)
  fn context_suggests_boolean_for_arg( &self, name : &str, description : &str ) -> bool
  {
    let name_lower = name.to_lowercase();

    let boolean_keywords = [
      "enable", "disable", "flag", "is_", "has_", "can_",
      "should_", "dry_run", "dry-run", "force", "quiet", "verbose",
      "clone", "parallel", "recursive", "skip", "ignore",
    ];

    let name_suggests = boolean_keywords.iter()
      .any( | kw | name_lower.contains( kw ) );

    let desc_suggests = description.contains( "true/false" ) ||
                       description.contains( "(true|false)" ) ||
                       description.contains( "true or false" );

    name_suggests || desc_suggests
  }

  /// Check if context suggests integer (for ArgumentDefinition)
  fn context_suggests_integer_for_arg( &self, name : &str ) -> bool
  {
    let name_lower = name.to_lowercase();

    let integer_keywords = [
      "count", "limit", "max", "min", "size", "length",
      "verbosity", "level", "timeout", "retry", "retries",
      "attempts", "depth", "width", "height", "num",
    ];

    integer_keywords.iter().any( | kw | name_lower.contains( kw ) )
  }

  /// Check if context suggests this should be boolean
  ///
  /// Uses argument name and description to infer intent.
  /// Conservative to avoid false positives.
  fn context_suggests_boolean( &self, name : &str, arg : &Value ) -> bool
  {
    let name_lower = name.to_lowercase();
    let desc = arg[ "description" ].as_str().unwrap_or( "" );

    // Boolean-suggestive names
    let boolean_keywords = [
      "enable", "disable", "flag", "is_", "has_", "can_",
      "should_", "dry_run", "dry-run", "force", "quiet", "verbose",
      "clone", "parallel", "recursive", "skip", "ignore",
    ];

    let name_suggests = boolean_keywords.iter()
      .any( | kw | name_lower.contains( kw ) );

    // Boolean-suggestive description (careful: avoid false positives)
    // Only match very specific patterns
    let desc_suggests = desc.contains( "true/false" ) ||
                       desc.contains( "(true|false)" ) ||
                       desc.contains( "true or false" );

    name_suggests || desc_suggests
  }

  /// Check if context suggests this should be integer
  ///
  /// Uses argument name to infer intent.
  /// Conservative to avoid false positives.
  fn context_suggests_integer( &self, name : &str, _arg : &Value ) -> bool
  {
    let name_lower = name.to_lowercase();

    // Integer-suggestive names
    let integer_keywords = [
      "count", "limit", "max", "min", "size", "length",
      "verbosity", "level", "timeout", "retry", "retries",
      "attempts", "depth", "width", "height", "num",
    ];

    integer_keywords.iter().any( | kw | name_lower.contains( kw ) )
  }
}

impl Default for TypeAnalyzer
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Type hint result
#[derive(Debug, Clone)]
pub enum TypeHint
{
  /// Argument has String kind but boolean-like default
  BooleanAsString
  {
    /// Name of the argument with type mismatch
    argument_name : String,
    /// Default value that looks like a boolean
    default_value : String,
    /// Severity level of the type hint
    severity : Severity,
  },
  /// Argument has String kind but integer-like default
  IntegerAsString
  {
    /// Name of the argument with type mismatch
    argument_name : String,
    /// Default value that looks like an integer
    default_value : String,
    /// Severity level of the type hint
    severity : Severity,
  },
}

/// Severity level for type hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity
{
  /// Warning level (only level currently used)
  Warning,
}

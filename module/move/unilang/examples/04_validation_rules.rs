#![allow(clippy::all)]
//! # Validation Rules Demo
//!
//! **⚠️ NOTE:** This example uses runtime registration for demonstration purposes.
//! For production use, define validation rules in YAML and use compile-time generation.
//!
//! This example demonstrates how to apply validation rules to command arguments,
//! including min/max values, string patterns, and length constraints.
//!
//! ## Validation Pipeline Overview
//!
//! Validation in unilang happens in this order:
//! 1. **Parsing** - Raw input is converted to typed values
//! 2. **Validation** - Each argument is checked against its validation rules
//! 3. **Execution** - The command runs with validated arguments
//!
//! If validation fails at step 2, the command will not execute and an error
//! will be returned to the user explaining which validation rule was violated.
//!
//! ## `ValidationRule` Enum Variants
//!
//! The `ValidationRule` enum provides these constraint types:
//!
//! ### Numeric Constraints (for integers and floats):
//! - `Min(f64)` - Value must be >= the specified minimum
//! - `Max(f64)` - Value must be <= the specified maximum
//!
//! ### String/Collection Length Constraints:
//! - `MinLength(usize)` - String length or collection size must be >= minimum
//! - `MaxLength(usize)` - String length or collection size must be <= maximum
//! - `MinItems(usize)` - Collections (lists, maps) must have >= minimum items
//! - `MaxItems(usize)` - Collections (lists, maps) must have <= maximum items
//!
//! ### Pattern Matching:
//! - `Pattern(String)` - String must match the provided regular expression
//!
//! ## Combining Multiple Rules
//!
//! Multiple validation rules can be applied to a single argument. They are
//! evaluated in the order specified, and ALL rules must pass for validation
//! to succeed. This allows for complex constraints like "password must be
//! at least 8 characters AND contain both letters and numbers".

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Validation Rules Demo ===\n" );

  // Create a new command registry to hold our validation demonstration command
  #[allow(deprecated)]
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Define a command that showcases different types of validation rules.
  // This command demonstrates how validation rules are applied to arguments
  // and how they prevent invalid data from reaching the command execution.
  let validation_demo = CommandDefinition::former()
  .name( ".validate" )
  .namespace( "validation".to_string() )
  .description( "Demonstrates argument validation rules".to_string() )
  .hint( "Shows different validation constraints" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".check".to_string() ] )
  .tags( vec![ "validation".to_string(), "demo".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "validation.validate age::25 name::Alice email::alice@example.com".to_string(),
    "validation.validate score::85.5 password::secretkey123".to_string(),
  ])
  .arguments( vec!
  [
    // EXAMPLE 1: Numeric Range Validation (Integer)
    //
    // This demonstrates Min/Max validation rules for numeric types.
    // Both rules must pass for validation to succeed.
    //
    // Valid inputs: age::0, age::25, age::120
    // Invalid inputs: age::-1 (below minimum), age::150 (above maximum)
    //
    // When validation fails, the user will see an error like:
    // "Validation failed for argument 'age': value -1 is below minimum 0"
    ArgumentDefinition {
      name: "age".to_string(),
      description: "Person's age (must be 0-120)".to_string(),
      kind: Kind::Integer, // Integer type supports Min/Max validation
      hint: "Age in years".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        ValidationRule::Min(0.0),   // Must be >= 0 (no negative ages)
        ValidationRule::Max(120.0)  // Must be <= 120 (reasonable maximum)
      ],
      aliases: vec![ "a".to_string() ],
      tags: vec![ "personal".to_string() ],
    },

    // EXAMPLE 2: Float Range Validation
    //
    // Demonstrates Min/Max validation for floating-point numbers.
    // Float validation works the same as integer validation but allows decimals.
    //
    // Valid inputs: score::0.0, score::85.5, score::100.0
    // Invalid inputs: score::-10.5 (below minimum), score::150.7 (above maximum)
    //
    // Note: Float parsing happens before validation, so score::abc would fail
    // at parsing stage, while score::-5.0 would fail at validation stage.
    ArgumentDefinition {
      name: "score".to_string(),
      description: "Test score (must be 0.0 or higher)".to_string(),
      kind: Kind::Float, // Float type supports Min/Max validation with decimals
      hint: "Score as decimal".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        ValidationRule::Min(0.0),   // No negative scores allowed
        ValidationRule::Max(100.0)  // Standard percentage scale maximum
      ],
      aliases: vec![ "s".to_string() ],
      tags: vec![ "academic".to_string() ],
    },

    // EXAMPLE 3: String Length Validation
    //
    // Demonstrates MinLength/MaxLength validation for strings.
    // String length is measured in UTF-8 characters, not bytes.
    //
    // Valid inputs: name::"Alice", name::"John Smith", name::"José María"
    // Invalid inputs: name::"A" (too short), name::"" (empty string, too short)
    //
    // Length validation is useful for:
    // - Ensuring meaningful names (not just single characters)
    // - Database field constraints (preventing overflow)
    // - UI/display requirements (fitting in specific layouts)
    ArgumentDefinition {
      name: "name".to_string(),
      description: "Person's name (2-50 characters)".to_string(),
      kind: Kind::String, // String type supports length-based validation
      hint: "Full name".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        ValidationRule::MinLength(2),  // At least 2 characters (no single-letter names)
        ValidationRule::MaxLength(50)  // At most 50 characters (database/UI constraint)
        // Note: MaxLength validation may not be fully implemented in all contexts
      ],
      aliases: vec![ "n".to_string() ],
      tags: vec![ "personal".to_string() ],
    },

    // EXAMPLE 4: Regular Expression Pattern Validation
    //
    // Demonstrates Pattern validation using regular expressions.
    // The regex is compiled and matched against the entire string.
    //
    // Valid inputs: 
    //   - email::"alice@example.com"
    //   - email::"user.name+tag@domain.co.uk"
    //   - email::"test123@subdomain.example.org"
    //
    // Invalid inputs:
    //   - email::"invalid-email" (no @ symbol)
    //   - email::"@example.com" (missing local part)
    //   - email::"user@" (missing domain)
    //   - email::"user@domain" (missing TLD)
    //
    // Pattern validation is powerful for:
    // - Email addresses, phone numbers, postal codes
    // - API keys, tokens, identifiers with specific formats
    // - Custom business rules (product codes, etc.)
    ArgumentDefinition {
      name: "email".to_string(),
      description: "Email address (must match email pattern)".to_string(),
      kind: Kind::String, // Pattern validation works with string types
      hint: "Valid email format".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        // Regex breakdown:
        // ^[a-zA-Z0-9._%+-]+ - Local part: letters, numbers, dots, underscores, percent, plus, hyphen
        // @ - Required @ symbol
        // [a-zA-Z0-9.-]+ - Domain part: letters, numbers, dots, hyphens
        // \\. - Required dot before TLD (escaped for Rust string)
        // [a-zA-Z]{2,}$ - TLD: at least 2 letters at end
        ValidationRule::Pattern("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$".to_string())
      ],
      aliases: vec![ "e".to_string() ],
      tags: vec![ "contact".to_string() ],
    },

    // EXAMPLE 5: Combining Multiple Validation Rules
    //
    // This demonstrates how to apply multiple validation rules to a single argument.
    // ALL rules must pass for the argument to be considered valid.
    // Rules are evaluated in the order they appear in the vector.
    //
    // This password field combines:
    // 1. Length requirement (MinLength)
    // 2. Pattern requirement (must contain letters AND numbers)
    //
    // Valid inputs:
    //   - password::"password123" (8+ chars, has letters and numbers)
    //   - password::"mySecure8Pass" (meets both requirements)
    //   - password::"abc123def456" (long and has both character types)
    //
    // Invalid inputs:
    //   - password::"short7" (only 6 chars, fails MinLength even though pattern matches)
    //   - password::"verylongpassword" (8+ chars but no numbers, fails Pattern)
    //   - password::"12345678" (8+ chars but no letters, fails Pattern)
    //   - password::"abc" (fails both MinLength and Pattern)
    //
    // Note: This argument is marked as 'sensitive', which means its value
    // will be hidden in logs and debug output for security purposes.
    ArgumentDefinition {
      name: "password".to_string(),
      description: "Password (8+ chars, must contain letters and numbers)".to_string(),
      kind: Kind::String,
      hint: "Secure password".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        sensitive: true, // Hide value in logs/output for security
        ..Default::default()
      },
      validation_rules: vec![
        // Rule 1: Minimum length requirement
        ValidationRule::MinLength(8),
        
        // Rule 2: Pattern requirement using positive lookaheads
        // Regex breakdown:
        // ^ - Start of string
        // (?=.*[A-Za-z]) - Positive lookahead: must contain at least one letter
        // (?=.*\\d) - Positive lookahead: must contain at least one digit
        // .+$ - Match one or more characters to end of string
        ValidationRule::Pattern("^(?=.*[A-Za-z])(?=.*\\d).+$".to_string())
      ],
      aliases: vec![ "pwd".to_string() ],
      tags: vec![ "security".to_string() ],
    },

    // EXAMPLE 6: Collection Size Validation (Lists)
    //
    // Demonstrates MinItems validation for collections like lists.
    // This ensures the list contains a minimum number of elements.
    //
    // The list is comma-separated as specified by Some(',') delimiter.
    //
    // Valid inputs:
    //   - tags::"web,api" (exactly 2 items, meets minimum)
    //   - tags::"rust,cli,validation,demo" (4 items, above minimum)
    //   - tags::"a,b,c,d,e,f,g,h,i,j" (10 items, many elements)
    //
    // Invalid inputs:
    //   - tags::"solo" (only 1 item, below minimum of 2)
    //   - tags::"" (empty string results in empty list, below minimum)
    //
    // Collection validation is useful for:
    // - Ensuring meaningful categorization (multiple tags)
    // - Batch operations (minimum number of items to process)
    // - Data quality (avoiding single-item collections where multiple expected)
    ArgumentDefinition {
      name: "tags".to_string(),
      description: "List of tags (2-10 items)".to_string(),
      kind: Kind::List( Box::new( Kind::String ), Some( ',' ) ), // Comma-separated string list
      hint: "Comma-separated tags".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        ValidationRule::MinItems(2), // Must have at least 2 tags
        // Note: MaxItems could be added here as well: ValidationRule::MaxItems(10)
      ],
      aliases: vec![ "t".to_string() ],
      tags: vec![ "metadata".to_string() ],
    },

    // EXAMPLE 7: URL Validation with Security Requirements
    //
    // Demonstrates pattern validation for URLs with security constraints.
    // Even though the field is Kind::Url (which provides basic URL parsing),
    // we add pattern validation to enforce HTTPS-only for security.
    //
    // Valid inputs:
    //   - website::"https://example.com"
    //   - website::"https://api.mysite.org/v1"
    //   - website::"https://subdomain.example.co.uk:8443/path"
    //
    // Invalid inputs:
    //   - website::"http://example.com" (HTTP not allowed, fails pattern)
    //   - website::"ftp://files.example.com" (FTP not allowed, fails pattern)
    //   - website::"example.com" (missing protocol, fails pattern)
    //   - website::"not-a-url" (not a valid URL format)
    //
    // This layered validation approach:
    // 1. Kind::Url ensures basic URL structure is valid
    // 2. Pattern validation adds business/security rules on top
    //
    // Common use cases:
    // - API endpoints (must be HTTPS for security)
    // - Webhook URLs (enforce secure protocols)
    // - Configuration URLs (specific schemes only)
    ArgumentDefinition {
      name: "website".to_string(),
      description: "Website URL (must be HTTPS)".to_string(),
      kind: Kind::Url, // Basic URL parsing and validation
      hint: "HTTPS URL only".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![
        // Additional security constraint: must use HTTPS
        ValidationRule::Pattern("^https://".to_string())
      ],
      aliases: vec![ "url".to_string() ],
      tags: vec![ "web".to_string(), "security".to_string() ],
    },
  ])
  .end();

  // The validation routine is executed ONLY if all validation rules pass.
  // If any validation rule fails, this function will never be called.
  // The VerifiedCommand contains arguments that have been:
  // 1. Parsed from strings to their correct types
  // 2. Validated against all specified rules
  // 3. Guaranteed to be safe for use in business logic
  let validation_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "✓ All validation rules passed!" );
    println!( "\nValidated arguments received by command:" );

    // Display the validated arguments, with special handling for sensitive data
    for ( name, value ) in &cmd.arguments
    {
      let value_str = match value
      {
        // Special case: hide sensitive argument values for security
        Value::String( s ) if name == "password" => "*".repeat( s.len() ),
        _ => format!( "{value:?}" ),
      };
      println!( "  {name}: {value_str}" );
    }

    // At this point, you can safely use the validated arguments in your business logic:
    // - Numbers are guaranteed to be within specified ranges
    // - Strings are guaranteed to meet length and pattern requirements
    // - Collections are guaranteed to have the required number of items
    // - All data has been parsed and type-checked

    Ok( OutputData
    {
      content : "All arguments validated successfully".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  #[allow(deprecated)]
  registry.command_add_runtime( &validation_demo, validation_routine )?;
  println!( "✓ Registered validation demonstration command" );

  println!( "\n=== Complete ValidationRule Reference ===" );
  
  println!( "\n🔢 Numeric Constraints (Kind::Integer, Kind::Float):" );
  println!( "  • ValidationRule::Min(f64) - Value must be >= minimum" );
  println!( "  • ValidationRule::Max(f64) - Value must be <= maximum" );
  println!( "  Example: age with Min(0.0) + Max(120.0) allows 0-120" );

  println!( "\n📝 String Length Constraints (Kind::String):" );
  println!( "  • ValidationRule::MinLength(usize) - String must have >= N characters" );
  println!( "  • ValidationRule::MaxLength(usize) - String must have <= N characters" );
  println!( "  Example: name with MinLength(2) + MaxLength(50) allows 2-50 chars" );

  println!( "\n🔍 Pattern Matching (Kind::String, Kind::Url, etc.):" );
  println!( "  • ValidationRule::Pattern(String) - Must match regex pattern" );
  println!( "  Example: email with Pattern for email format validation" );
  println!( "  Example: password with Pattern for complexity requirements" );

  println!( "\n📋 Collection Constraints (Kind::List, Kind::Map):" );
  println!( "  • ValidationRule::MinItems(usize) - Collection must have >= N items" );
  println!( "  • ValidationRule::MaxItems(usize) - Collection must have <= N items" );
  println!( "  Example: tags list with MinItems(2) requires at least 2 tags" );

  println!( "\n=== Validation Execution Order ===" );
  println!( "1. **Input Parsing** - Convert string input to typed values" );
  println!( "   - If parsing fails → Error returned, validation not attempted" );
  println!( "2. **Rule Evaluation** - Check each validation rule in order" );
  println!( "   - If any rule fails → Error returned with specific rule violation" );
  println!( "3. **Command Execution** - Run command with validated arguments" );
  println!( "   - All arguments guaranteed to meet their constraints" );

  println!( "\n=== Argument Attributes (affect behavior) ===" );
  println!( "  • optional: true - Argument not required (default: false)" );
  println!( "  • multiple: true - Argument can appear multiple times" );
  println!( "  • sensitive: true - Value hidden in logs/output" );
  println!( "  • interactive: true - May prompt user for input" );
  println!( "  • default: Some(value) - Default when not specified" );

  println!( "\n=== Example Usage with Expected Results ===" );
  
  println!( "\n✅ VALID EXAMPLES (all validation rules pass):" );
  println!( "cargo run --bin unilang_cli validation.validate age::25 name::Alice" );
  println!( "  → age=25 (within 0-120 range), name='Alice' (2+ chars)" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate score::95.5 email::alice@example.com" );
  println!( "  → score=95.5 (within 0-100 range), email matches pattern" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate password::mypass123 website::https://example.com" );
  println!( "  → password=8+ chars with letters+numbers, website uses HTTPS" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate tags::'web,api,rust'" );
  println!( "  → tags list has 3 items (≥ 2 required)" );

  println!( "\n❌ INVALID EXAMPLES (validation will fail with specific error messages):" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate age::150" );
  println!( "  → ERROR: Value 150 exceeds maximum 120 for argument 'age'" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate age::-5" );
  println!( "  → ERROR: Value -5 is below minimum 0 for argument 'age'" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate name::A" );
  println!( "  → ERROR: String 'A' is too short (minimum 2 characters) for argument 'name'" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate email::invalid-email" );
  println!( "  → ERROR: String 'invalid-email' does not match required pattern for argument 'email'" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate password::short" );
  println!( "  → ERROR: String 'short' is too short (minimum 8 characters) for argument 'password'" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate password::verylongpassword" );
  println!( "  → ERROR: String 'verylongpassword' does not match required pattern for argument 'password'" );
  println!( "    (Pattern requires both letters AND numbers)" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate website::http://example.com" );
  println!( "  → ERROR: String 'http://example.com' does not match required pattern for argument 'website'" );
  println!( "    (Must start with 'https://' for security)" );
  println!();
  println!( "cargo run --bin unilang_cli validation.validate tags::solo" );
  println!( "  → ERROR: Collection has 1 items but minimum 2 required for argument 'tags'" );

  println!( "\n=== Tips for Combining Validation Rules ===" );
  println!( "• Rules are evaluated in order - put cheaper checks first" );
  println!( "• Length checks are faster than regex pattern matching" );
  println!( "• Use meaningful error messages in argument descriptions" );
  println!( "• Consider user experience - don't make rules too restrictive" );
  println!( "• Test edge cases: empty strings, boundary values, special characters" );

  Ok(())
}
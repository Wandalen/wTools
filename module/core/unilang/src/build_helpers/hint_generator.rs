//! Hint message generator for type analysis
//!
//! Generates helpful, actionable warning messages for type hints detected during build.
//!
//! # Message Philosophy
//!
//! Messages should be:
//! - **Clear**: Show current code and suggested code
//! - **Actionable**: Explain benefits and how to suppress
//! - **Non-intrusive**: Emphasize build continues normally
//! - **Educational**: Explain why suggestion matters

use crate::build_helpers::type_analyzer::TypeHint;

/// Generates hint messages for build warnings
#[derive(Debug)]
pub struct HintGenerator;

impl HintGenerator
{
  /// Generate a warning message for a type hint
  pub fn generate_warning( hint : &TypeHint ) -> String
  {
    match hint
    {
      TypeHint::BooleanAsString { argument_name, default_value, .. } =>
      {
        format!(
          "💡 Type Hint: Argument '{}' might be better as Boolean kind\n\
           \n\
           Current:\n\
           - name: \"{}\"\n\
             kind: \"String\"\n\
             attributes:\n\
               default: \"{}\"  # String literal\n\
           \n\
           Suggestion:\n\
           - name: \"{}\"\n\
             kind: \"Boolean\"\n\
             attributes:\n\
               default: {}  # Boolean value (no quotes)\n\
           \n\
           Benefits:\n\
           - Automatic validation (rejects invalid values like 'yes', '1')\n\
           - Type-safe: cmd.get_boolean(\"{}\") instead of manual parsing\n\
           - Better error messages for users\n\
           \n\
           If this is intentionally a string literal (e.g., code template):\n\
           - Add: suppress_type_hint: true (in attributes)\n\
           \n\
           This is a hint, not an error. Build continues normally.\n\
           To suppress all type hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n",
          argument_name,
          argument_name,
          default_value,
          argument_name,
          default_value,  // true or false (no quotes)
          argument_name
        )
      },

      TypeHint::IntegerAsString { argument_name, default_value, .. } =>
      {
        format!(
          "💡 Type Hint: Argument '{}' might be better as Integer kind\n\
           \n\
           Current:\n\
           - name: \"{}\"\n\
             kind: \"String\"\n\
             attributes:\n\
               default: \"{}\"  # String literal\n\
           \n\
           Suggestion:\n\
           - name: \"{}\"\n\
             kind: \"Integer\"\n\
             attributes:\n\
               default: {}  # Integer value (no quotes)\n\
             validation_rules:\n\
               - Min: 0  # Add appropriate range constraints\n\
               - Max: 100\n\
           \n\
           Benefits:\n\
           - Automatic range validation\n\
           - Type-safe: cmd.get_integer(\"{}\") instead of manual parsing\n\
           - Better error messages for users\n\
           \n\
           If this is intentionally a string (version, ID, code):\n\
           - Add: suppress_type_hint: true (in attributes)\n\
           \n\
           This is a hint, not an error. Build continues normally.\n\
           To suppress all type hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n",
          argument_name,
          argument_name,
          default_value,
          argument_name,
          default_value,
          argument_name
        )
      },
    }
  }

  /// Emit all hints to stderr
  ///
  /// Formats and displays hints in a user-friendly way.
  /// If no hints, does nothing.
  pub fn emit_hints( hints : Vec< TypeHint > )
  {
    if hints.is_empty()
    {
      return;
    }

    eprintln!();
    eprintln!( "{}", "=".repeat( 80 ) );
    eprintln!( "📋 Unilang Type Hints ({} suggestion{})", hints.len(), if hints.len() == 1 { "" } else { "s" } );
    eprintln!( "{}", "=".repeat( 80 ) );
    eprintln!();

    for hint in hints
    {
      eprintln!( "{}", Self::generate_warning( &hint ) );
      eprintln!( "{}", "-".repeat( 80 ) );
      eprintln!();
    }

    eprintln!(
      "ℹ️  Type hints help you choose appropriate argument types.\n\
       These are suggestions, not errors. Your build continues normally.\n\
       \n\
       See: usage.md Section 5 'Argument Types' for guidance.\n"
    );
  }
}

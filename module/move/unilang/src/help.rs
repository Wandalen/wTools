//!
//! The help generation components for the Unilang framework.
//!
//! This module provides flexible help text generation with configurable verbosity levels,
//! allowing applications to tailor help output to different user preferences and use cases.
//!
//! # Verbosity Levels
//!
//! Help output can be controlled through five verbosity levels (0-4):
//! - **Level 0 (Minimal)**: Single-line output with command name and description
//! - **Level 1 (Basic)**: Add parameter list with types
//! - **Level 2 (Standard)**: Concise format with USAGE, PARAMETERS, EXAMPLES - **DEFAULT**
//! - **Level 3 (Detailed)**: Full metadata including version, aliases, tags, validation rules
//! - **Level 4 (Comprehensive)**: Extensive format with rationale and detailed explanations
//!
//! # Usage Examples
//!
//! ## Basic Usage (Default Verbosity)
//!
//! ```rust
//! use unilang::prelude::*;
//!
//! let registry = CommandRegistry::new();
//! let help_gen = HelpGenerator::new( &registry );
//!
//! // Generates help at default Level 2 (Standard)
//! if let Some( help ) = help_gen.command( ".config" )
//! {
//!   println!( "{}", help );
//! }
//! ```
//!
//! ## Environment Variable Control
//!
//! ```bash
//! # Set verbosity to Minimal (Level 0)
//! UNILANG_HELP_VERBOSITY=0 cargo run
//!
//! # Set verbosity to Comprehensive (Level 4)
//! UNILANG_HELP_VERBOSITY=4 cargo run
//! ```
//!
//! ```rust
//! use unilang::prelude::*;
//!
//! let registry = CommandRegistry::new();
//!
//! // Read verbosity from environment variable UNILANG_HELP_VERBOSITY
//! let help_gen = HelpGenerator::from_env( &registry );
//! ```
//!
//! ## Programmatic Verbosity Control
//!
//! ```rust
//! use unilang::prelude::*;
//! use unilang::help::HelpVerbosity;
//!
//! let registry = CommandRegistry::new();
//!
//! // Create with specific verbosity level
//! let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Comprehensive );
//!
//! // Or set verbosity dynamically
//! let mut help_gen = HelpGenerator::new( &registry );
//! help_gen.set_verbosity( HelpVerbosity::Minimal );
//! ```
//!

/// Internal namespace.
mod private
{
  use crate::registry::CommandRegistry;
  use core::fmt::Write;

///
/// Help verbosity levels controlling output detail.
///
/// Controls the amount of information displayed in help text, from minimal
/// single-line output to comprehensive documentation. The default level is
/// Standard (Level 2), which provides a good balance of conciseness and completeness.
///
/// # Levels
///
/// - **Level 0 (Minimal)**: Command name and one-line description only
///   - Use case: Quick reference, command discovery
///   - Example: `.config - Display current configuration and sources`
///
/// - **Level 1 (Basic)**: Add parameters list with types
///   - Use case: Syntax lookup, remembering parameter names
///   - Shows: Command description + parameter list
///
/// - **Level 2 (Standard)**: Concise help with usage, parameters, examples (DEFAULT)
///   - Use case: Terminal use, getting started quickly
///   - Shows: USAGE, PARAMETERS with descriptions, EXAMPLES
///   - Inspired by unikit-style concise formatting
///
/// - **Level 3 (Detailed)**: Full metadata including validation rules, aliases, tags
///   - Use case: Comprehensive documentation, understanding constraints
///   - Shows: All command metadata, validation rules, version info
///
/// - **Level 4 (Comprehensive)**: Extensive explanations with rationale and use cases
///   - Use case: Learning, documentation generation, detailed references
///   - Shows: Extended format with rationale and explanations
///   - Inspired by runbox-style comprehensive formatting
///
/// # Environment Variable
///
/// The verbosity level can be controlled via the `UNILANG_HELP_VERBOSITY` environment
/// variable (values 0-4). Values above 4 are capped at Comprehensive.
///
/// # Examples
///
/// ```rust
/// use unilang::help::HelpVerbosity;
///
/// // Parse from integer
/// let level = HelpVerbosity::from_level( 2 );
/// assert_eq!( level, HelpVerbosity::Standard );
///
/// // Read from environment variable
/// let level = HelpVerbosity::from_env();
///
/// // Default is Standard (Level 2)
/// assert_eq!( HelpVerbosity::default(), HelpVerbosity::Standard );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord ) ]
pub enum HelpVerbosity
{
  /// Level 0: Just command name and brief description
  Minimal = 0,
  /// Level 1: Add parameters list with types only
  Basic = 1,
  /// Level 2: Standard concise help (USAGE, PARAMETERS, EXAMPLES) - DEFAULT
  Standard = 2,
  /// Level 3: Detailed help with all metadata
  Detailed = 3,
  /// Level 4: Comprehensive with extensive explanations
  Comprehensive = 4,
}

impl Default for HelpVerbosity
{
  fn default() -> Self
  {
    Self::Standard
  }
}

impl HelpVerbosity
{
  /// Parse verbosity level from integer (0-4)
  pub fn from_level( level : u8 ) -> Self
  {
    match level
    {
      0 => Self::Minimal,
      1 => Self::Basic,
      2 => Self::Standard,
      3 => Self::Detailed,
      4.. => Self::Comprehensive,
    }
  }

  /// Read verbosity level from environment variable UNILANG_HELP_VERBOSITY.
  /// Falls back to default (Level 2: Standard) if not set or invalid.
  pub fn from_env() -> Self
  {
    std::env::var( "UNILANG_HELP_VERBOSITY" )
      .ok()
      .and_then( |v| v.parse::< u8 >().ok() )
      .map( Self::from_level )
      .unwrap_or_default()
  }
}

///
/// Generates help information for commands.
///
/// This struct provides methods to create formatted help messages from
/// `CommandDefinition` instances, which can be displayed to the user.
#[ allow( missing_debug_implementations ) ]
pub struct HelpGenerator< 'a >
{
  registry : & 'a CommandRegistry,
  verbosity : HelpVerbosity,
}

impl< 'a > HelpGenerator< 'a >
{
  ///
  /// Creates a new `HelpGenerator` with default verbosity (Level 2: Standard).
  ///
  #[ must_use ]
  pub fn new( registry : & 'a CommandRegistry ) -> Self
  {
    Self
    {
      registry,
      verbosity : HelpVerbosity::default(),
    }
  }

  ///
  /// Creates a new `HelpGenerator` reading verbosity from UNILANG_HELP_VERBOSITY environment variable.
  /// Falls back to default (Level 2: Standard) if not set or invalid.
  ///
  #[ must_use ]
  pub fn from_env( registry : & 'a CommandRegistry ) -> Self
  {
    Self
    {
      registry,
      verbosity : HelpVerbosity::from_env(),
    }
  }

  ///
  /// Creates a new `HelpGenerator` with specified verbosity level.
  ///
  #[ must_use ]
  pub fn with_verbosity( registry : & 'a CommandRegistry, verbosity : HelpVerbosity ) -> Self
  {
    Self { registry, verbosity }
  }

  ///
  /// Sets the verbosity level for help output.
  ///
  pub fn set_verbosity( &mut self, verbosity : HelpVerbosity )
  {
    self.verbosity = verbosity;
  }

  ///
  /// Gets the current verbosity level.
  ///
  #[ must_use ]
  pub fn verbosity( &self ) -> HelpVerbosity
  {
    self.verbosity
  }

  ///
  /// Generates a help string for a single command using current verbosity level.
  ///
  /// The output format depends on the verbosity level (0-4).
  #[ must_use ]
  pub fn command( &self, command_name : &str ) -> Option< String >
  {
    // Try exact match first, then try with dot prefix
    let command = self.registry.command( command_name )
    .or_else( || self.registry.command( &format!( ".{command_name}" ) ) )
    .or_else( ||
    {
      // If command_name is "echo", try ".system.echo"
      // If command_name is "math.add", it should already be found.
      // This handles cases where the user provides just the command name without namespace,
      // or a partial namespace.
      // For now, a simple check for "echo" to ".system.echo"
      if command_name == "echo"
      {
        self.registry.command( ".system.echo" )
      }
      else
      {
        None
      }
    })?;

    match self.verbosity
    {
      HelpVerbosity::Minimal => Some( self.format_minimal( &command ) ),
      HelpVerbosity::Basic => Some( self.format_basic( &command ) ),
      HelpVerbosity::Standard => Some( self.format_standard( &command ) ),
      HelpVerbosity::Detailed => Some( self.format_detailed( &command ) ),
      HelpVerbosity::Comprehensive => Some( self.format_comprehensive( &command ) ),
    }
  }

  /// Helper function to format argument kind as lowercase string
  fn format_kind( kind : &crate::data::Kind ) -> String
  {
    format!( "{:?}", kind ).to_lowercase()
  }

  /// Format Level 0: Minimal - Just name and brief description
  fn format_minimal( &self, command : &crate::CommandDefinition ) -> String
  {
    format!( "{} - {}", command.name, command.description )
  }

  /// Format Level 1: Basic - Add parameters list with types
  fn format_basic( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}", command.name, command.description ).unwrap();

    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nPARAMETERS:" ).unwrap();
      for arg in &command.arguments
      {
        writeln!( &mut help, "  {}::{}", arg.name, Self::format_kind( &arg.kind ) ).unwrap();
      }
    }
    help
  }

  /// Format Level 2: Standard (DEFAULT) - Concise like unikit
  fn format_standard( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{}\n", command.description ).unwrap();

    // USAGE section
    write!( &mut help, "USAGE:\n  {}", command.name ).unwrap();
    if !command.arguments.is_empty()
    {
      for arg in &command.arguments
      {
        if arg.attributes.optional
        {
          write!( &mut help, " [{}::{}]", arg.name, Self::format_kind( &arg.kind ) ).unwrap();
        }
        else
        {
          write!( &mut help, " {}::{}", arg.name, Self::format_kind( &arg.kind ) ).unwrap();
        }
      }
    }
    writeln!( &mut help, "\n" ).unwrap();

    // PARAMETERS section
    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "PARAMETERS:" ).unwrap();
      for arg in &command.arguments
      {
        let optional_marker = if arg.attributes.optional { " (optional)" } else { "" };

        writeln!
        (
          &mut help,
          "  {}::{}{}  {}",
          arg.name,
          Self::format_kind( &arg.kind ),
          optional_marker,
          if arg.hint.is_empty() { &arg.description } else { &arg.hint }
        )
        .unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    // EXAMPLES section
    if !command.examples.is_empty()
    {
      writeln!( &mut help, "EXAMPLES:" ).unwrap();
      for example in &command.examples
      {
        writeln!( &mut help, "  {example}" ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    help
  }

  /// Format Level 3: Detailed - Full metadata (old default behavior)
  fn format_detailed( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!
    (
      &mut help,
      "Usage: {} (v{})",
      command.name,
      command.version
    )
    .unwrap();
    if !command.aliases.is_empty()
    {
      writeln!( &mut help, "Aliases: {}", command.aliases.join( ", " ) ).unwrap();
    }
    if !command.tags.is_empty()
    {
      writeln!( &mut help, "Tags: {}", command.tags.join( ", " ) ).unwrap();
    }
    writeln!( &mut help, "\n  Hint: {}", command.hint ).unwrap();
    writeln!( &mut help, "  {}\n", command.description ).unwrap();
    writeln!( &mut help, "Status: {}", command.status ).unwrap();

    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in &command.arguments
      {
        write!( &mut help, "{}", arg.name ).unwrap();
        write!( &mut help, " (Type: {})", arg.kind ).unwrap();

        let mut status_parts = Vec::new();
        if arg.attributes.optional {
          status_parts.push("Optional");
        }
        if arg.attributes.multiple {
          status_parts.push("Multiple");
        }
        if !status_parts.is_empty() {
          write!( &mut help, " - {}", status_parts.join(", ") ).unwrap();
        }
        writeln!( &mut help ).unwrap();

        if !arg.description.is_empty() {
          writeln!( &mut help, "  {}", arg.description ).unwrap();
          if !arg.hint.is_empty() && arg.hint != arg.description {
            writeln!( &mut help, "  ({})", arg.hint ).unwrap();
          }
        } else if !arg.hint.is_empty() {
          writeln!( &mut help, "  {}", arg.hint ).unwrap();
        }

        if !arg.validation_rules.is_empty() {
          writeln!(
            &mut help,
            "  Rules: [{}]",
            arg.validation_rules.iter().map(|r| format!("{r:?}")).collect::<Vec<_>>().join( ", " )
          ).unwrap();
        }

        writeln!( &mut help ).unwrap();
      }
    }

    help
  }

  /// Format Level 4: Comprehensive - Extensive like runbox
  fn format_comprehensive( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}\n", command.name, command.description ).unwrap();

    // USAGE section
    write!( &mut help, "USAGE:\n  {}", command.name ).unwrap();
    if !command.arguments.is_empty()
    {
      for arg in &command.arguments
      {
        if arg.attributes.optional
        {
          write!( &mut help, " [{}::<value>]", arg.name ).unwrap();
        }
        else
        {
          write!( &mut help, " {}::<value>", arg.name ).unwrap();
        }
      }
    }
    writeln!( &mut help, "\n" ).unwrap();

    // DESCRIPTION section with metadata
    writeln!( &mut help, "DESCRIPTION:" ).unwrap();
    writeln!( &mut help, "  {}", command.description ).unwrap();
    if !command.hint.is_empty() && command.hint != command.description
    {
      writeln!( &mut help, "  {}", command.hint ).unwrap();
    }
    writeln!( &mut help, "\n  Status: {} (v{})", command.status, command.version ).unwrap();
    if !command.aliases.is_empty()
    {
      writeln!( &mut help, "  Aliases: {}", command.aliases.join( ", " ) ).unwrap();
    }
    writeln!( &mut help ).unwrap();

    // PARAMETERS section with detailed explanations
    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "PARAMETERS:\n" ).unwrap();
      for arg in &command.arguments
      {
        writeln!( &mut help, "  {}::<value>", arg.name ).unwrap();

        // Description with indentation
        if !arg.description.is_empty()
        {
          writeln!( &mut help, "    {}", arg.description ).unwrap();
        }
        if !arg.hint.is_empty() && arg.hint != arg.description
        {
          writeln!( &mut help, "    {}", arg.hint ).unwrap();
        }

        // Type and attributes
        writeln!( &mut help, "    Type: {}", arg.kind ).unwrap();
        if arg.attributes.optional
        {
          writeln!( &mut help, "    Optional: yes" ).unwrap();
        }
        if arg.attributes.multiple
        {
          writeln!( &mut help, "    Multiple values: yes" ).unwrap();
        }

        // Validation rules
        if !arg.validation_rules.is_empty()
        {
          writeln!( &mut help, "    Validation:" ).unwrap();
          for rule in &arg.validation_rules
          {
            writeln!( &mut help, "      - {rule:?}" ).unwrap();
          }
        }

        writeln!( &mut help ).unwrap();
      }
    }

    // EXAMPLES section
    if !command.examples.is_empty()
    {
      writeln!( &mut help, "EXAMPLES:" ).unwrap();
      for example in &command.examples
      {
        writeln!( &mut help, "  {example}" ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    // TAGS section if present
    if !command.tags.is_empty()
    {
      writeln!( &mut help, "TAGS: {}", command.tags.join( ", " ) ).unwrap();
    }

    help
  }

  ///
  /// Generates a summary list of all available commands.
  ///
  #[ must_use ]
  pub fn list_commands( &self ) -> String
  {
    let mut summary = String::new();
    writeln!( &mut summary, "Available Commands:" ).unwrap();
    for ( name, command ) in &self.registry.commands()
    {
      writeln!( &mut summary, "  {:<15} {}", name, command.description ).unwrap();
    }
    summary
  }
}

}

mod_interface::mod_interface!
{
  exposed use private::HelpGenerator;
  exposed use private::HelpVerbosity;

  prelude use private::HelpGenerator;
  prelude use private::HelpVerbosity;
}

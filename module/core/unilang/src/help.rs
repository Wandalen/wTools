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

    // Command header with version
    writeln!( &mut help, "Usage: {} (v{})", command.name, command.version ).unwrap();
    writeln!( &mut help, "{}\n", command.description ).unwrap();

    // Status information
    writeln!( &mut help, "Status: {}", command.status ).unwrap();
    if !command.aliases.is_empty()
    {
      writeln!( &mut help, "Aliases: {}", command.aliases.join( ", " ) ).unwrap();
    }

    // Arguments section with improved formatting
    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in &command.arguments
      {
        write!( &mut help, "{}", arg.name ).unwrap();
        write!( &mut help, " (Type: {})", Self::format_kind( &arg.kind ) ).unwrap();

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

        // Show description, or hint if description is empty
        let desc_text = if !arg.description.is_empty() {
          &arg.description
        } else if !arg.hint.is_empty() {
          &arg.hint
        } else {
          ""
        };

        if !desc_text.is_empty() {
          writeln!( &mut help, "  {}", desc_text ).unwrap();
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

    // Examples section
    if !command.examples.is_empty()
    {
      writeln!( &mut help, "Examples:" ).unwrap();
      for (idx, example) in command.examples.iter().enumerate()
      {
        writeln!( &mut help, "  {}. {}", idx + 1, example ).unwrap();
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
    self.list_commands_filtered( None )
  }

  ///
  /// Generates a summary list of commands filtered by prefix.
  ///
  /// # Arguments
  /// * `prefix` - Optional prefix filter (e.g., ".git", ".remove")
  ///
  /// # Returns
  /// Formatted string with categorized command list
  ///
  #[ must_use ]
  pub fn list_commands_filtered( &self, prefix : Option< &str > ) -> String
  {
    use std::collections::BTreeMap;

    let mut summary = String::new();

    // Filter commands by prefix and visibility
    let all_commands = self.registry.commands();
    let commands : Vec< ( &String, &crate::CommandDefinition ) > = all_commands
      .iter()
      .filter( |( name, cmd )|
      {
        // Apply prefix filter if provided
        let matches_prefix = prefix.map_or( true, |p| name.starts_with( p ) );

        // Hide commands marked as hidden_from_list
        let is_visible = !cmd.hidden_from_list;

        matches_prefix && is_visible
      })
      .collect();

    if commands.is_empty()
    {
      if let Some( p ) = prefix
      {
        writeln!( &mut summary, "No commands found matching prefix: {}", p ).unwrap();
      }
      else
      {
        writeln!( &mut summary, "No commands available." ).unwrap();
      }
      return summary;
    }

    // Group commands by category
    let mut by_category : BTreeMap< String, Vec< ( &String, &crate::CommandDefinition ) > > = BTreeMap::new();

    for ( name, cmd ) in commands
    {
      let category = if cmd.category.is_empty()
      {
        // Auto-detect category from command prefix
        self.auto_categorize( name )
      }
      else
      {
        cmd.category.clone()
      };

      by_category.entry( category ).or_default().push( ( name, cmd ) );
    }

    // If only one category and it's empty, show flat list
    if by_category.len() == 1 && by_category.contains_key( "" )
    {
      writeln!( &mut summary, "Available commands:\n" ).unwrap();
      let mut cmds : Vec< _ > = by_category.get( "" ).unwrap().iter().collect();
      cmds.sort_by_key( |( name, cmd )| ( cmd.priority, name.as_str() ) );

      for ( name, cmd ) in cmds
      {
        let desc = if cmd.short_desc.is_empty()
        {
          &cmd.description
        }
        else
        {
          &cmd.short_desc
        };
        writeln!( &mut summary, "  {:<20} {}", name, desc ).unwrap();
      }
    }
    else
    {
      // Show categorized output
      writeln!( &mut summary, "Available commands:\n" ).unwrap();

      for ( category, mut cmds ) in by_category
      {
        if !category.is_empty()
        {
          writeln!( &mut summary, "{}:", self.format_category_name( &category ) ).unwrap();
        }

        // Sort by priority then name
        cmds.sort_by_key( |( name, cmd )| ( cmd.priority, name.as_str() ) );

        for ( name, cmd ) in cmds
        {
          let desc = if cmd.short_desc.is_empty()
          {
            &cmd.description
          }
          else
          {
            &cmd.short_desc
          };
          writeln!( &mut summary, "  {:<20} {}", name, desc ).unwrap();
        }
        writeln!( &mut summary ).unwrap();
      }
    }

    // Add footer with usage hints
    if prefix.is_none()
    {
      writeln!( &mut summary, "Use '<command> help' to get detailed help for a specific command." ).unwrap();
      writeln!( &mut summary, "Example: . .list help" ).unwrap();
    }

    summary
  }

  /// Auto-categorize command based on its name pattern
  fn auto_categorize( &self, name : &str ) -> String
  {
    if name.starts_with( ".git" )
    {
      "git_operations".to_string()
    }
    else if name.starts_with( ".remove" )
    {
      "removal_operations".to_string()
    }
    else if name.starts_with( ".orgs" ) || name.starts_with( ".users" ) || name.starts_with( ".discover" )
    {
      "github_integration".to_string()
    }
    else if name.starts_with( ".add" ) || name.starts_with( ".clone" ) || name.starts_with( ".list" ) || name.starts_with( ".init" )
    {
      "repository_management".to_string()
    }
    else if name == ".status" || name.starts_with( ".pull" ) || name.starts_with( ".push" ) || name.starts_with( ".sync" ) || name.starts_with( ".update" )
    {
      "git_operations".to_string()
    }
    else if name == "." || name == ".help"
    {
      "help".to_string()
    }
    else
    {
      String::new()
    }
  }

  /// Format category name for display
  fn format_category_name( &self, category : &str ) -> String
  {
    match category
    {
      "repository_management" => "REPOSITORY MANAGEMENT".to_string(),
      "git_operations" => "GIT OPERATIONS".to_string(),
      "removal_operations" => "REMOVAL OPERATIONS".to_string(),
      "github_integration" => "GITHUB INTEGRATION".to_string(),
      "help" => "HELP & INFORMATION".to_string(),
      _ => category.to_uppercase(),
    }
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

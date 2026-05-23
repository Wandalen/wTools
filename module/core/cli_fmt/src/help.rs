//! Structured CLI help template: data model + colour-aware renderer.
//!
//! Provides a declarative way to define CLI help screens with command groups,
//! options, and examples. Renders to a formatted string with optional ANSI
//! colour (suppressed when stdout is not a terminal or `NO_COLOR` is set).
//!
//! # Quick Start
//!
//! ```rust
//! use cli_fmt::help::*;
//!
//! let data = CliHelpData
//! {
//!   binary  : "myapp".to_string(),
//!   tagline : "Does useful things.".to_string(),
//!   groups  : vec!
//!   [
//!     CommandGroup
//!     {
//!       name    : "Commands".to_string(),
//!       entries : vec!
//!       [
//!         CommandEntry { name : ".run".to_string(), desc : "Run the thing".to_string() },
//!       ],
//!     },
//!   ],
//!   options  : vec![],
//!   examples : vec![],
//! };
//! let style = CliHelpStyle { use_color : false };
//! let text = CliHelpTemplate::new( style, data ).render();
//! assert!( text.contains( "myapp" ) );
//! ```

use core::fmt::Write as _;

// ── ANSI escape codes ────────────────────────────────────────────────────────

const BOLD  : &str = "\x1b[1m";
const RESET : &str = "\x1b[0m";

// ── Data model ──────────────────────────────────────────────────────────────

/// Complete data for a CLI help screen.
#[ derive( Debug ) ]
pub struct CliHelpData
{
  /// Binary name shown in the header (e.g., `"clp"`).
  pub binary   : String,
  /// One-line purpose description.
  pub tagline  : String,
  /// Command groups (each rendered as a section with a heading).
  pub groups   : Vec< CommandGroup >,
  /// Global options (rendered in an "Options" section).
  pub options  : Vec< OptionEntry >,
  /// Usage examples (rendered in an "Examples" section).
  pub examples : Vec< ExampleEntry >,
}

/// Named group of related commands.
#[ derive( Debug ) ]
pub struct CommandGroup
{
  /// Section heading (e.g., `"Account management"`).
  pub name    : String,
  /// Commands in this group.
  pub entries : Vec< CommandEntry >,
}

/// Single command entry within a group.
#[ derive( Debug ) ]
pub struct CommandEntry
{
  /// Command name (e.g., `".accounts"`).
  pub name : String,
  /// Short description.
  pub desc : String,
}

/// Global option displayed in the "Options" section.
#[ derive( Debug ) ]
pub struct OptionEntry
{
  /// Option syntax (e.g., `"format::text|json"`).
  pub name : String,
  /// Short description.
  pub desc : String,
}

/// Usage example.
#[ derive( Debug ) ]
pub struct ExampleEntry
{
  /// Full invocation string (e.g., `"clp .accounts"`).
  pub invocation : String,
  /// Optional annotation rendered after the invocation.
  pub desc       : Option< String >,
}

// ── Style ───────────────────────────────────────────────────────────────────

/// Controls whether ANSI colour codes are emitted.
#[ derive( Debug ) ]
pub struct CliHelpStyle
{
  /// When `true`, ANSI bold/reset sequences are included in rendered output.
  pub use_color : bool,
}

impl Default for CliHelpStyle
{
  /// Detect colour support: `true` when stdout is a terminal AND `NO_COLOR` is unset.
  fn default() -> Self
  {
    use std::io::IsTerminal;
    let use_color = std::io::stdout().is_terminal()
      && std::env::var_os( "NO_COLOR" ).is_none();
    Self { use_color }
  }
}

// ── Template ────────────────────────────────────────────────────────────────

/// Rendered help screen combining data + style.
#[ derive( Debug ) ]
pub struct CliHelpTemplate
{
  style : CliHelpStyle,
  data  : CliHelpData,
}

impl CliHelpTemplate
{
  /// Construct a new template from style and data.
  #[ inline ]
  #[ must_use ]
  pub fn new( style : CliHelpStyle, data : CliHelpData ) -> Self
  {
    Self { style, data }
  }

  /// Render the help screen to a `String`.
  #[ must_use ]
  pub fn render( &self ) -> String
  {
    let mut out = String::new();

    // Header: `binary — tagline` + usage line
    self.bold( &mut out, &self.data.binary );
    let _ = write!( out, " \u{2014} {}\n", self.data.tagline );
    let _ = writeln!( out, "\nUsage: {} <command> [param::value ...]", self.data.binary );

    // Command groups
    for group in &self.data.groups
    {
      out.push( '\n' );
      self.bold( &mut out, &group.name );
      out.push( '\n' );

      let width = group.entries.iter()
        .map( | e | e.name.len() )
        .max()
        .unwrap_or( 0 );
      for entry in &group.entries
      {
        let _ = writeln!( out, "  {:width$}  {}", entry.name, entry.desc );
      }
    }

    // Options
    if !self.data.options.is_empty()
    {
      out.push( '\n' );
      self.bold( &mut out, "Options" );
      out.push( '\n' );

      let width = self.data.options.iter()
        .map( | o | o.name.len() )
        .max()
        .unwrap_or( 0 );
      for opt in &self.data.options
      {
        let _ = writeln!( out, "  {:width$}  {}", opt.name, opt.desc );
      }
    }

    // Examples
    if !self.data.examples.is_empty()
    {
      out.push( '\n' );
      self.bold( &mut out, "Examples" );
      out.push( '\n' );

      for ex in &self.data.examples
      {
        let _ = write!( out, "  {}", ex.invocation );
        if let Some( ref desc ) = ex.desc
        {
          let _ = write!( out, "  # {desc}" );
        }
        out.push( '\n' );
      }
    }

    out
  }

  /// Write `text` wrapped in ANSI bold when colour is enabled.
  fn bold( &self, out : &mut String, text : &str )
  {
    if self.style.use_color
    {
      out.push_str( BOLD );
      out.push_str( text );
      out.push_str( RESET );
    }
    else
    {
      out.push_str( text );
    }
  }
}

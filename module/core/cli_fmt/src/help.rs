//! Typed, configurable CLI help text renderer.
//!
//! Provides `CliHelpStyle`, `CliHelpData`, and `CliHelpTemplate` for building
//! column-aligned, ANSI-colored help text from structured data without coupling
//! to `data_fmt`.

use core::fmt::Write as _;
use std::io::IsTerminal;

// ─── Style ───────────────────────────────────────────────────────────────────

/// Visual and color style parameters for CLI help rendering.
///
/// `CliHelpStyle::default()` reproduces the layout and ANSI codes used by
/// `claude_profile::print_usage()` (`cmd_indent=4`, `cmd_name_width=20`, etc.).
#[ derive( Debug, Clone ) ]
pub struct CliHelpStyle
{
  /// Left margin (spaces) before command names in group entries.
  pub cmd_indent     : usize,
  /// Minimum column width for command names (padding only, not truncation).
  pub cmd_name_width : usize,
  /// Left margin (spaces) before group header lines.
  pub grp_indent     : usize,
  /// Left margin (spaces) before option name entries.
  pub opt_indent     : usize,
  /// Minimum column width for option names (padding only, not truncation).
  pub opt_name_width : usize,
  /// Gap (spaces) between the name column and the description column.
  pub col_gap        : usize,
  /// Left margin (spaces) before example invocation lines.
  pub example_indent : usize,
  /// ANSI code for section headers and the usage line (bold).
  pub color_tagline  : &'static str,
  /// ANSI codes for group header lines (yellow+bold).
  pub color_group    : &'static str,
  /// ANSI code for command and option names (bold cyan).
  pub color_option   : &'static str,
  /// ANSI code for example invocation lines (dim).
  pub color_example  : &'static str,
  /// ANSI reset sequence applied after each colored span.
  pub color_reset    : &'static str,
  /// When `true`, suppress ANSI codes when stdout is not a terminal; when `false`, always suppress.
  pub tty_detect     : bool,
}

impl Default for CliHelpStyle
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      cmd_indent     : 4,
      cmd_name_width : 20,
      grp_indent     : 2,
      opt_indent     : 2,
      opt_name_width : 18,
      col_gap        : 2,
      example_indent : 2,
      color_tagline  : "\x1b[1m",
      color_group    : "\x1b[33m\x1b[1m",
      color_option   : "\x1b[1;36m",
      color_example  : "\x1b[2m",
      color_reset    : "\x1b[0m",
      tty_detect     : true,
    }
  }
}

// ─── Data ────────────────────────────────────────────────────────────────────

/// A group of related commands shown together under a shared header.
#[ derive( Debug, Clone ) ]
pub struct CommandGroup
{
  /// Display name for this command group (e.g., `"Account management"`).
  pub name    : String,
  /// Ordered list of command entries within this group.
  pub entries : Vec<CommandEntry>,
}

/// A single command entry within a `CommandGroup`.
#[ derive( Debug, Clone ) ]
pub struct CommandEntry
{
  /// Command name as typed by the user (e.g., `".account.save"`).
  pub name : String,
  /// Short one-line description displayed in the adjacent column.
  pub desc : String,
}

/// A single global option entry shown in the Options section.
#[ derive( Debug, Clone ) ]
pub struct OptionEntry
{
  /// Option name or syntax string (e.g., `"format::text|json"`).
  pub name : String,
  /// Short description displayed in the adjacent column.
  pub desc : String,
}

/// A single usage example shown in the Examples section.
#[ derive( Debug, Clone ) ]
pub struct ExampleEntry
{
  /// The example invocation string shown to the user.
  pub invocation : String,
  /// Optional annotation line appended after the invocation.
  pub desc       : Option<String>,
}

/// A named group of option entries rendered as its own section.
#[ derive( Debug, Clone ) ]
pub struct OptionGroup
{
  /// Display name for this option group (e.g., `"RUNNER OPTIONS"`).
  pub name    : String,
  /// Ordered list of option entries within this group.
  pub entries : Vec<OptionEntry>,
}

/// Structured content for all sections of the help output.
///
/// `#[non_exhaustive]` blocks all struct expressions (including struct update
/// syntax) from outside the defining crate. Use `Default` + field assignment:
///
/// ```
/// use cli_fmt::help::*;
/// let mut data = CliHelpData::default();
/// data.binary  = "myapp".into();
/// data.tagline = "A useful tool".into();
/// ```
///
/// Struct expressions — including struct update syntax — fail to compile from
/// external crates (E0639). The doctest below verifies this:
///
/// ```compile_fail
/// use cli_fmt::help::*;
/// let _data = CliHelpData
/// {
///   binary        : String::new(),
///   tagline       : String::new(),
///   groups        : vec![],
///   options       : vec![],
///   examples      : vec![],
///   usage_lines   : vec![],
///   arguments     : vec![],
///   option_groups : vec![],
/// };
/// ```
#[ non_exhaustive ]
#[ derive( Default, Debug, Clone ) ]
pub struct CliHelpData
{
  /// Binary name used in the usage line (e.g., `"clp"`).
  pub binary        : String,
  /// One-line description shown below the usage line.
  pub tagline       : String,
  /// Ordered list of command groups.
  pub groups        : Vec<CommandGroup>,
  /// Global options; the Options section is omitted when this is empty.
  pub options       : Vec<OptionEntry>,
  /// Usage examples; the Examples section is omitted when this is empty.
  pub examples      : Vec<ExampleEntry>,
  /// Custom usage lines; when non-empty, replaces the default `"Usage: {binary} <command>"` header.
  pub usage_lines   : Vec<String>,
  /// Argument entries rendered between the Commands label and command group entries.
  pub arguments     : Vec<OptionEntry>,
  /// Named option groups; when non-empty, the legacy `options` field is suppressed.
  pub option_groups : Vec<OptionGroup>,
}

// ─── Template ────────────────────────────────────────────────────────────────

/// Renders CLI help text from a `CliHelpStyle` and `CliHelpData` pair.
///
/// Separating style from data allows either to be substituted independently
/// for testing, customization, or localization.
#[ derive( Debug ) ]
pub struct CliHelpTemplate
{
  style : CliHelpStyle,
  data  : CliHelpData,
}

impl CliHelpTemplate
{
  /// Create a new template from style and data parameters.
  #[ inline ]
  #[ must_use ]
  pub fn new( style : CliHelpStyle, data : CliHelpData ) -> Self
  {
    Self { style, data }
  }

  /// Render the full help text to a `String`.
  ///
  /// When `style.tty_detect` is `true` and stdout is not a TTY, all ANSI
  /// color codes are suppressed. Set `tty_detect = false` to always suppress
  /// ANSI codes regardless of TTY state (color fields are ignored).
  #[ inline ]
  #[ must_use ]
  pub fn render( &self ) -> String
  {
    let use_color = self.style.tty_detect && std::io::stdout().is_terminal();
    let s         = &self.style;
    let c         = | code : &'static str | -> &str { if use_color { code } else { "" } };
    let bold      = c( s.color_tagline );
    let grp       = c( s.color_group   );
    let opt       = c( s.color_option  );
    let ex        = c( s.color_example );
    let rst       = c( s.color_reset   );
    let mut out   = String::new();
    self.emit_header( &mut out, bold, rst );
    self.emit_arguments( &mut out, bold, opt, rst );
    self.emit_groups( &mut out, grp, opt, rst );
    self.emit_option_groups( &mut out, bold, opt, rst );
    if self.data.option_groups.is_empty() && !self.data.options.is_empty()
    {
      self.emit_options( &mut out, bold, opt, rst );
    }
    if !self.data.examples.is_empty() { self.emit_examples( &mut out, bold, ex, rst ); }
    out
  }

  fn emit_header( &self, out : &mut String, bold : &str, rst : &str )
  {
    if !self.data.usage_lines.is_empty()
    {
      for line in &self.data.usage_lines
      {
        let _ = writeln!( out, "  {line}" );
      }
    }
    else
    {
      let _ = writeln!( out, "{bold}Usage:{rst} {} <command>", self.data.binary );
    }
    let _ = writeln!( out );
    let _ = writeln!( out, "{}", self.data.tagline );
    let _ = writeln!( out );
    let _ = writeln!( out, "{bold}Commands:{rst}" );
  }

  fn emit_arguments( &self, out : &mut String, bold : &str, opt_color : &str, rst : &str )
  {
    if self.data.arguments.is_empty() { return; }
    let max_len = self.data.arguments.iter().map( |e| e.name.len() ).max().unwrap_or( 0 );
    let _ = writeln!( out, "\n{bold}Arguments:{rst}" );
    for e in &self.data.arguments
    {
      let _ = writeln!( out, "  {opt_color}{:<width$}{rst}  {}", e.name, e.desc, width = max_len );
    }
  }

  fn emit_groups( &self, out : &mut String, grp_color : &str, opt_color : &str, rst : &str )
  {
    let s  = &self.style;
    let gi = " ".repeat( s.grp_indent );
    let ci = " ".repeat( s.cmd_indent );
    let gp = " ".repeat( s.col_gap    );
    for group in &self.data.groups
    {
      let _ = writeln!( out, "\n{gi}{grp_color}{}{rst}", group.name );
      for entry in &group.entries
      {
        let padded = format!( "{:<width$}", entry.name, width = s.cmd_name_width );
        let _ = writeln!( out, "{ci}{opt_color}{padded}{rst}{gp}{}", entry.desc );
      }
    }
  }

  fn emit_option_group( &self, out : &mut String, bold : &str, opt_color : &str, rst : &str,
                        name : &str, entries : &[ OptionEntry ] )
  {
    if entries.is_empty() { return; }
    let max_len = entries.iter().map( |e| e.name.len() ).max().unwrap_or( 0 );
    let _ = writeln!( out, "\n{bold}{name}:{rst}" );
    for e in entries
    {
      let _ = writeln!( out, "  {opt_color}{:<width$}{rst}  {}", e.name, e.desc, width = max_len );
    }
  }

  fn emit_option_groups( &self, out : &mut String, bold : &str, opt_color : &str, rst : &str )
  {
    for group in &self.data.option_groups
    {
      self.emit_option_group( out, bold, opt_color, rst, &group.name, &group.entries );
    }
  }

  fn emit_options( &self, out : &mut String, bold : &str, opt_color : &str, rst : &str )
  {
    let s  = &self.style;
    let oi = " ".repeat( s.opt_indent );
    let gp = " ".repeat( s.col_gap    );
    let _ = writeln!( out );
    let _ = writeln!( out, "{bold}Options:{rst}" );
    for opt in &self.data.options
    {
      let padded = format!( "{:<width$}", opt.name, width = s.opt_name_width );
      let _ = writeln!( out, "{oi}{opt_color}{padded}{rst}{gp}{}", opt.desc );
    }
  }

  // BUG-007 task/bug/closed/007_example_desc_silent_drop.md — desc field ignored in emit_examples
  // Fix(BUG-007): render ExampleEntry.desc when Some — was silently dropped
  // Root cause: emit_examples() emitted only ex.invocation unconditionally,
  //   ignoring desc: Option<String> despite being documented as annotation field
  // Pitfall: Option-typed renderer fields need a test asserting the Some branch
  //   appears in output — compiling without error is not proof it renders
  fn emit_examples( &self, out : &mut String, bold : &str, ex_color : &str, rst : &str )
  {
    let s  = &self.style;
    let ei = " ".repeat( s.example_indent );
    let _ = writeln!( out );
    let _ = writeln!( out, "{bold}Examples:{rst}" );
    for ex in &self.data.examples
    {
      if let Some( ref desc ) = ex.desc
      {
        let _ = writeln!( out, "{ei}{ex_color}{}  # {desc}{rst}", ex.invocation );
      }
      else
      {
        let _ = writeln!( out, "{ei}{ex_color}{}{rst}", ex.invocation );
      }
    }
  }
}

// ─── Namespaces ──────────────────────────────────────────────────────────────

/// Own namespace of the module.
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
}

/// Namespace to include with `use cli_fmt::help::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use super::
  {
    CliHelpStyle,
    CommandGroup,
    CommandEntry,
    OptionEntry,
    ExampleEntry,
    CliHelpData,
    CliHelpTemplate,
    OptionGroup,
  };
}

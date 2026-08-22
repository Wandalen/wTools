//! Typed, configurable CLI help text renderer.
//!
//! Provides `CliHelpStyle`, `CliHelpData`, and `CliHelpTemplate` for building
//! column-aligned, ANSI-colored help text from structured data without coupling
//! to `data_fmt`.

use core::fmt::Write as _;
use std::io::IsTerminal;
use color_tools::{ Color, DecoratedText };

// ─── Style ───────────────────────────────────────────────────────────────────

/// Visual and color style parameters for CLI help rendering.
///
/// `CliHelpStyle::default()` reproduces the layout and ANSI codes used by
/// `claude_profile::print_usage()` (`cmd_indent=4`, `cmd_name_width=20`, etc.).
///
/// Color fields are `color_tools::DecoratedText` style descriptors: only their
/// `color`/`bold`/`dim` fields are meaningful here. `text` is always empty and
/// is replaced with the actual span content at render time via struct-update
/// syntax (see the private `decorate()` helper below).
#[ derive( Debug, Clone ) ]
pub struct CliHelpStyle
{
  /// Left margin (spaces) before command names in group entries.
  pub cmd_indent     : usize,
  /// Floor for the command-name column width. The column automatically grows to the
  /// longest entry name, so alignment is preserved regardless of this setting.
  pub cmd_name_width : usize,
  /// Left margin (spaces) before group header lines.
  pub grp_indent     : usize,
  /// Left margin (spaces) before option name entries.
  pub opt_indent     : usize,
  /// Floor for the option-name column width. The column automatically grows to the
  /// longest option name, so alignment is preserved regardless of this setting.
  pub opt_name_width : usize,
  /// Gap (spaces) between the name column and the description column.
  pub col_gap        : usize,
  /// Left margin (spaces) before example invocation lines.
  pub example_indent : usize,
  /// Style for section headers and the usage line (bold).
  pub color_tagline  : DecoratedText,
  /// Style for group header lines (yellow+bold).
  pub color_group    : DecoratedText,
  /// Style for command and option names (bold cyan).
  pub color_option   : DecoratedText,
  /// Style for example invocation lines (dim).
  pub color_example  : DecoratedText,
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
      color_tagline  : DecoratedText::default().with_bold(),
      color_group    : DecoratedText::default().with_bold().with_color_named( Color::Yellow ),
      color_option   : DecoratedText::default().with_bold().with_color_named( Color::Cyan ),
      color_example  : DecoratedText::default().with_dim(),
      tty_detect     : true,
    }
  }
}

/// Apply a style role to a text span, honoring `use_color`.
///
/// `role.text` is discarded — the caller's `text` becomes the actual content via
/// struct-update. When `use_color` is `false`, returns `text` unchanged with no
/// escape codes injected, regardless of what `role` carries.
fn decorate( use_color : bool, role : &DecoratedText, text : &str ) -> String
{
  if use_color
  {
    DecoratedText { text : text.to_owned(), ..role.clone() }.render()
  }
  else
  {
    text.to_owned()
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

/// A titled, column-aligned name/description section on a detail page.
///
/// `#[non_exhaustive]` blocks external struct literals; construct via
/// [`DetailSection::new`] or `Default` + field assignment.
///
/// ```
/// use cli_fmt::help::*;
/// let section = DetailSection::new( "Possible values", vec!
/// [
///   OptionEntry { name : "local".into(), desc : "current directory only".into() },
/// ] );
/// assert_eq!( section.title, "Possible values" );
/// ```
#[ non_exhaustive ]
#[ derive( Default, Debug, Clone ) ]
pub struct DetailSection
{
  /// Section header text; an empty title renders the entries as a bare aligned block with no header line.
  pub title   : String,
  /// Ordered name/description entries; the whole section is omitted when this is empty.
  pub entries : Vec<OptionEntry>,
}

impl DetailSection
{
  /// Create a section from a title and its entries.
  #[ inline ]
  #[ must_use ]
  pub fn new( title : impl Into<String>, entries : Vec<OptionEntry> ) -> Self
  {
    Self { title : title.into(), entries }
  }
}

/// Structured content for a single-subject detail page (one parameter, one
/// command, one topic) — as opposed to `CliHelpData`, which models a full
/// command-listing page.
///
/// The renderer is domain-free: what the subject *is* (a parameter, a command)
/// lives entirely in the caller's `label`/`sections` content.
///
/// `#[non_exhaustive]` blocks all struct expressions from outside the defining
/// crate. Use `Default` + field assignment:
///
/// ```
/// use cli_fmt::help::*;
/// let mut page = DetailPageData::default();
/// page.label = "Parameter".into();
/// page.name  = "scope".into();
/// page.description.push( "Discovery strategy selector.".into() );
/// page.sections.push( DetailSection::new( "Possible values", vec!
/// [
///   OptionEntry { name : "local".into(), desc : "current directory only".into() },
/// ] ) );
/// let style = CliHelpStyle { tty_detect : false, ..Default::default() };
/// let text  = DetailPageTemplate::new( style, page ).render();
/// assert!( text.contains( "Parameter: scope" ) );
/// assert!( text.contains( "Possible values:" ) );
/// ```
///
/// Struct expressions — including struct update syntax — fail to compile from
/// external crates (E0639). The doctest below verifies this:
///
/// ```compile_fail
/// use cli_fmt::help::*;
/// let _page = DetailPageData
/// {
///   label       : String::new(),
///   name        : String::new(),
///   usage       : vec![],
///   description : vec![],
///   sections    : vec![],
///   examples    : vec![],
/// };
/// ```
#[ non_exhaustive ]
#[ derive( Default, Debug, Clone ) ]
pub struct DetailPageData
{
  /// Kind label for the page subject (e.g., `"Parameter"`, `"Command"`); rendered as `"{label}: {name}"`.
  /// When empty, the header line shows `name` alone; when both are empty, no header line is emitted.
  pub label       : String,
  /// Subject name (e.g., `"scope"`, `".rollup"`).
  pub name        : String,
  /// Usage/syntax lines rendered directly under the header in example color; omitted when empty.
  pub usage       : Vec<String>,
  /// Free-form description lines rendered as a plain paragraph; omitted when empty.
  pub description : Vec<String>,
  /// Ordered detail sections; sections with no entries are skipped entirely.
  pub sections    : Vec<DetailSection>,
  /// Usage examples; the Examples section is omitted when this is empty.
  pub examples    : Vec<ExampleEntry>,
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
    let mut out   = String::new();
    self.emit_header( &mut out, use_color );
    self.emit_arguments( &mut out, use_color );
    self.emit_groups( &mut out, use_color );
    self.emit_option_groups( &mut out, use_color );
    if self.data.option_groups.is_empty() && !self.data.options.is_empty()
    {
      self.emit_options( &mut out, use_color );
    }
    if !self.data.examples.is_empty() { self.emit_examples( &mut out, use_color ); }
    out
  }

  fn emit_header( &self, out : &mut String, use_color : bool )
  {
    let s = &self.style;
    if !self.data.usage_lines.is_empty()
    {
      for line in &self.data.usage_lines
      {
        let _ = writeln!( out, "  {line}" );
      }
    }
    else
    {
      let usage = decorate( use_color, &s.color_tagline, "Usage:" );
      let _ = writeln!( out, "{usage} {} <command>", self.data.binary );
    }
    let _ = writeln!( out );
    let _ = writeln!( out, "{}", self.data.tagline );
    let _ = writeln!( out );
    let commands = decorate( use_color, &s.color_tagline, "Commands:" );
    let _ = writeln!( out, "{commands}" );
  }

  fn emit_arguments( &self, out : &mut String, use_color : bool )
  {
    let s = &self.style;
    if self.data.arguments.is_empty() { return; }
    let max_len = self.data.arguments.iter().map( |e| e.name.len() ).max().unwrap_or( 0 );
    let header = decorate( use_color, &s.color_tagline, "Arguments:" );
    let _ = writeln!( out, "\n{header}" );
    for e in &self.data.arguments
    {
      let padded = format!( "{:<width$}", e.name, width = max_len );
      let name   = decorate( use_color, &s.color_option, &padded );
      let _ = writeln!( out, "  {name}  {}", e.desc );
    }
  }

  fn emit_groups( &self, out : &mut String, use_color : bool )
  {
    let s  = &self.style;
    let gi = " ".repeat( s.grp_indent );
    let ci = " ".repeat( s.cmd_indent );
    let gp = " ".repeat( s.col_gap    );
    // cmd_name_width is a floor, not a fixed width : the column grows to the longest
    // entry name so alignment survives names longer than the preset ( misuse-proof )
    let width = self.data.groups.iter()
      .flat_map( |g| g.entries.iter() )
      .map( |e| e.name.len() )
      .max()
      .unwrap_or( 0 )
      .max( s.cmd_name_width );
    for group in &self.data.groups
    {
      let name = decorate( use_color, &s.color_group, &group.name );
      let _ = writeln!( out, "\n{gi}{name}" );
      for entry in &group.entries
      {
        let padded = format!( "{:<width$}", entry.name, width = width );
        let opt    = decorate( use_color, &s.color_option, &padded );
        let _ = writeln!( out, "{ci}{opt}{gp}{}", entry.desc );
      }
    }
  }

  fn emit_option_group( &self, out : &mut String, use_color : bool, name : &str, entries : &[ OptionEntry ] )
  {
    let s = &self.style;
    if entries.is_empty() { return; }
    let max_len = entries.iter().map( |e| e.name.len() ).max().unwrap_or( 0 );
    let header = decorate( use_color, &s.color_tagline, &format!( "{name}:" ) );
    let _ = writeln!( out, "\n{header}" );
    for e in entries
    {
      let padded = format!( "{:<width$}", e.name, width = max_len );
      let opt    = decorate( use_color, &s.color_option, &padded );
      let _ = writeln!( out, "  {opt}  {}", e.desc );
    }
  }

  fn emit_option_groups( &self, out : &mut String, use_color : bool )
  {
    for group in &self.data.option_groups
    {
      self.emit_option_group( out, use_color, &group.name, &group.entries );
    }
  }

  fn emit_options( &self, out : &mut String, use_color : bool )
  {
    let s  = &self.style;
    let oi = " ".repeat( s.opt_indent );
    let gp = " ".repeat( s.col_gap    );
    // opt_name_width is a floor, not a fixed width : the column grows to the longest
    // option name so alignment survives names longer than the preset ( misuse-proof )
    let width = self.data.options.iter()
      .map( |o| o.name.len() )
      .max()
      .unwrap_or( 0 )
      .max( s.opt_name_width );
    let _ = writeln!( out );
    let header = decorate( use_color, &s.color_tagline, "Options:" );
    let _ = writeln!( out, "{header}" );
    for opt in &self.data.options
    {
      let padded = format!( "{:<width$}", opt.name, width = width );
      let name   = decorate( use_color, &s.color_option, &padded );
      let _ = writeln!( out, "{oi}{name}{gp}{}", opt.desc );
    }
  }

  fn emit_examples( &self, out : &mut String, use_color : bool )
  {
    emit_examples_section( out, &self.style, use_color, &self.data.examples );
  }
}

// BUG-007 task/bug/closed/007_example_desc_silent_drop.md — desc field ignored in emit_examples
// Fix(BUG-007): render ExampleEntry.desc when Some — was silently dropped
// Root cause: emit_examples() emitted only ex.invocation unconditionally,
//   ignoring desc: Option<String> despite being documented as annotation field
// Pitfall: Option-typed renderer fields need a test asserting the Some branch
//   appears in output — compiling without error is not proof it renders
fn emit_examples_section( out : &mut String, style : &CliHelpStyle, use_color : bool, examples : &[ ExampleEntry ] )
{
  let ei = " ".repeat( style.example_indent );
  let _ = writeln!( out );
  let header = decorate( use_color, &style.color_tagline, "Examples:" );
  let _ = writeln!( out, "{header}" );
  for ex in examples
  {
    // invocation + optional annotation share ONE color span, matching the
    // pre-typed-style behaviour where both sat between one open code and one reset
    let line_text = match &ex.desc
    {
      Some( desc ) => format!( "{}  # {desc}", ex.invocation ),
      None         => ex.invocation.clone(),
    };
    let line = decorate( use_color, &style.color_example, &line_text );
    let _ = writeln!( out, "{ei}{line}" );
  }
}

/// Renders a single-subject detail page from a `CliHelpStyle` and `DetailPageData` pair.
///
/// Same style/data separation as `CliHelpTemplate`; the two templates share the
/// `CliHelpStyle` vocabulary so a binary's listing page and detail pages stay
/// visually consistent without duplicated configuration.
#[ derive( Debug ) ]
pub struct DetailPageTemplate
{
  style : CliHelpStyle,
  data  : DetailPageData,
}

impl DetailPageTemplate
{
  /// Create a new template from style and data parameters.
  #[ inline ]
  #[ must_use ]
  pub fn new( style : CliHelpStyle, data : DetailPageData ) -> Self
  {
    Self { style, data }
  }

  /// Render the full detail page to a `String`.
  ///
  /// Infallible: performs no I/O beyond a single TTY probe and cannot fail.
  /// A fully-empty `DetailPageData` renders to an empty string. When
  /// `style.tty_detect` is `true` and stdout is not a TTY, all ANSI color
  /// codes are suppressed; `tty_detect = false` always suppresses them.
  #[ inline ]
  #[ must_use ]
  pub fn render( &self ) -> String
  {
    let use_color = self.style.tty_detect && std::io::stdout().is_terminal();
    let s         = &self.style;
    let oi        = " ".repeat( s.opt_indent );
    let ei        = " ".repeat( s.example_indent );
    let gp        = " ".repeat( s.col_gap );
    let mut out   = String::new();
    match ( self.data.label.is_empty(), self.data.name.is_empty() )
    {
      ( true, true )   => {},
      ( true, false )  =>
      {
        let name = decorate( use_color, &s.color_option, &self.data.name );
        let _ = writeln!( out, "{name}" );
      },
      ( false, true )  =>
      {
        let label = decorate( use_color, &s.color_tagline, &format!( "{}:", self.data.label ) );
        let _ = writeln!( out, "{label}" );
      },
      ( false, false ) =>
      {
        let label = decorate( use_color, &s.color_tagline, &format!( "{}:", self.data.label ) );
        let name  = decorate( use_color, &s.color_option, &self.data.name );
        let _ = writeln!( out, "{label} {name}" );
      },
    }
    for line in &self.data.usage
    {
      let text = decorate( use_color, &s.color_example, line );
      let _ = writeln!( out, "{ei}{text}" );
    }
    if !self.data.description.is_empty()
    {
      let _ = writeln!( out );
      for line in &self.data.description
      {
        let _ = writeln!( out, "{line}" );
      }
    }
    for section in &self.data.sections
    {
      if section.entries.is_empty() { continue; }
      // width is content-driven per section : facts blocks and value lists each
      // align to their own longest name, never to another section's ( misuse-proof )
      let width = section.entries.iter().map( |e| e.name.len() ).max().unwrap_or( 0 );
      let _ = writeln!( out );
      if !section.title.is_empty()
      {
        let title = decorate( use_color, &s.color_tagline, &format!( "{}:", section.title ) );
        let _ = writeln!( out, "{title}" );
      }
      for e in &section.entries
      {
        if e.desc.is_empty()
        {
          let name = decorate( use_color, &s.color_option, &e.name );
          let _ = writeln!( out, "{oi}{name}" );
        }
        else
        {
          let padded = format!( "{:<width$}", e.name, width = width );
          let name   = decorate( use_color, &s.color_option, &padded );
          let _ = writeln!( out, "{oi}{name}{gp}{}", e.desc );
        }
      }
    }
    if !self.data.examples.is_empty()
    {
      emit_examples_section( &mut out, s, use_color, &self.data.examples );
    }
    out
  }
}

// ─── Namespaces ──────────────────────────────────────────────────────────────

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
    DetailSection,
    DetailPageData,
    DetailPageTemplate,
  };
}

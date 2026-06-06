//! Proc-macro crate providing compile-time markdown file inclusion.
//!
//! Exports two proc-macros:
//! - `include_md!` — include a complete markdown file as a compile-time string constant
//! - `include_md_section!` — include a single named section from a markdown file

#![ cfg_attr( not( feature = "enabled" ), allow( unused ) ) ]

#[ cfg( feature = "enabled" ) ]
use macro_tools ::
{
  quote ::quote,
  syn,
};
#[ cfg( feature = "enabled" ) ]
use proc_macro::TokenStream;
#[ cfg( feature = "enabled" ) ]
use std ::
{
  env,
  fs,
  path ::Path,
};

/// Returns the heading level (number of `#` chars) for a markdown heading line.
///
/// Returns `None` if the line is not a valid heading (no leading `#`, or `#` not followed
/// by a space or end-of-line).
#[ cfg( feature = "enabled" ) ]
fn heading_level( line : &str ) -> Option< usize >
{
  if !line.starts_with( '#' )
  {
    return None;
  }
  let count = line.chars().take_while( | c | *c == '#' ).count();
  let after = &line[ count.. ];
  if after.is_empty() || after.starts_with( ' ' )
  {
    Some( count )
  }
  else
  {
    None
  }
}

/// Extracts the section identified by `heading` from `content`.
///
/// The heading must match exactly (case-sensitive). The extracted text starts at the
/// heading line and ends just before the next heading of equal or higher level. Nested
/// subsections are included. First occurrence wins when the heading appears multiple times.
/// Lines inside fenced code blocks (delimited by ` ``` ` or `~~~`) are never treated as
/// section boundaries even if they begin with `#`. Returns `None` if `heading` is not found.
#[ cfg( feature = "enabled" ) ]
fn extract_section( content : &str, heading : &str ) -> Option< String >
{
  let target_level = heading_level( heading )?;
  let mut result = String ::new();
  let mut in_section = false;
  // Fix(BUG-005) — track fence state so heading-like lines inside code blocks do not
  // terminate the section.
  // Root cause: no in_code_block variable existed; heading_level() was called unconditionally
  //   on every line, causing premature section break on fenced headings.
  // Pitfall: Fenced code blocks can contain heading-like lines (## x); boundary detection
  //   must be suppressed while inside a fence.
  let mut in_code_block = false;

  for line in content.lines()
  {
    if in_section
    {
      // Toggle fence state on ``` or ~~~ delimiter lines.
      if line.starts_with( "```" ) || line.starts_with( "~~~" )
      {
        in_code_block = !in_code_block;
      }
      // Only check for section boundary when we are NOT inside a fenced code block.
      if !in_code_block
      {
        if let Some( level ) = heading_level( line )
        {
          if level <= target_level
          {
            break;
          }
        }
      }
      result.push_str( line );
      result.push( '\n' );
    }
    else if line == heading
    {
      in_section = true;
      result.push_str( line );
      result.push( '\n' );
    }
  }

  if in_section { Some( result ) } else { None }
}

/// Include a complete markdown file as a compile-time `&'static str`.
///
/// Accepts a single string literal argument — the path to the file,
/// resolved relative to the source file containing the invocation
/// (identical semantics to the standard compile-time file inclusion built-in).
/// Rejects files larger than 10 MB at compile time.
///
/// # Examples
///
/// ```rust,ignore
/// let content = include_md!( "../readme.md" );
/// ```
#[ cfg( feature = "enabled" ) ]
#[ proc_macro ]
pub fn include_md( input : TokenStream ) -> TokenStream
{
  let path = match syn ::parse ::< syn ::LitStr >( input )
  {
    Ok( lit ) => lit,
    Err( e ) => return e.to_compile_error().into(),
  };
  let path_str = path.value();
  quote!
  {{
    const _ : () = assert!
    (
      :: core :: mem :: size_of_val( include_bytes!( #path_str ) ) <= 10_000_000_usize,
      "include_md: file exceeds 10 MB limit"
    );
    include_str!( #path_str )
  }}
  .into()
}

/// Argument structure for `include_md_section!` — path and heading literals.
#[ cfg( feature = "enabled" ) ]
struct SectionArgs
{
  path    : syn ::LitStr,
  heading : syn ::LitStr,
}

#[ cfg( feature = "enabled" ) ]
impl syn ::parse ::Parse for SectionArgs
{
  fn parse( input : syn ::parse ::ParseStream< '_ > ) -> syn ::Result< Self >
  {
    let path    = input.parse ::< syn ::LitStr >()?;
    input.parse ::< syn ::Token![ , ] >()?;
    let heading = input.parse ::< syn ::LitStr >()?;
    Ok( SectionArgs { path, heading } )
  }
}

/// Include a single named section from a markdown file as a compile-time `&'static str`.
///
/// Accepts two string literal arguments — the file path and the heading string
/// (verbatim, including leading `#` characters). The path is resolved relative to
/// `CARGO_MANIFEST_DIR` (the crate root of the crate containing the invocation).
/// Section boundary is level-aware and inclusive; first occurrence wins for
/// duplicate headings.
///
/// # Examples
///
/// ```rust,ignore
/// let section = include_md_section!( "readme.md", "## Quick Start" );
/// ```
#[ cfg( feature = "enabled" ) ]
#[ proc_macro ]
pub fn include_md_section( input : TokenStream ) -> TokenStream
{
  let args = match syn ::parse ::< SectionArgs >( input )
  {
    Ok( a ) => a,
    Err( e ) => return e.to_compile_error().into(),
  };

  let path_lit    = args.path;
  let heading_lit = args.heading;
  let path_str    = path_lit.value();
  let heading_str = heading_lit.value();

  // Resolve path relative to the invoking crate's root.
  // NOTE: source-file-relative resolution requires the unstable `proc_macro_span` feature;
  // CARGO_MANIFEST_DIR is the stable alternative for proc-macro file I/O.
  let Ok( manifest_dir ) = env ::var( "CARGO_MANIFEST_DIR" ) else
  {
    return syn ::Error ::new( path_lit.span(), "include_md_section: CARGO_MANIFEST_DIR not set" )
    .to_compile_error()
    .into();
  };

  let abs_path = Path ::new( &manifest_dir ).join( &path_str );

  // NOTE: `proc_macro::tracked_path::path()` would register `abs_path` for incremental-rebuild
  // invalidation, but that API requires `#![feature(track_path)]` (nightly only as of Rust 1.94).
  // On stable Rust, changes to included markdown files require `cargo clean` to take effect.
  // This is a known limitation documented in `docs/invariant/001_path_resolution.md`.

  // Read the file.
  let content = match fs ::read_to_string( &abs_path )
  {
    Ok( c ) => c,
    Err( e ) =>
    {
      return syn ::Error ::new
      (
        path_lit.span(),
        format!( "include_md_section: cannot read `{}`: {}", abs_path.display(), e ),
      )
      .to_compile_error()
      .into();
    }
  };

  // Enforce the 10 MB size limit (byte count on UTF-8 source, consistent with api/001).
  if content.len() > 10_000_000
  {
    return syn ::Error ::new( path_lit.span(), "include_md_section: file exceeds 10 MB limit" )
    .to_compile_error()
    .into();
  }

  // Extract the requested section.
  let Some( section ) = extract_section( &content, &heading_str ) else
  {
    return syn ::Error ::new
    (
      heading_lit.span(),
      format!
      (
        "include_md_section: heading `{}` not found in `{}`",
        heading_str,
        abs_path.display(),
      ),
    )
    .to_compile_error()
    .into();
  };

  let lit = syn ::LitStr ::new( &section, heading_lit.span() );
  quote!( #lit ).into()
}

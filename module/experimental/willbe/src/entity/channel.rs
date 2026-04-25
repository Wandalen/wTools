#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;
  use std ::
  {
  fmt ::Formatter,
  ffi ::OsString,
 };
  use std ::path ::Path;
  use collection_tools ::collection ::HashSet;
  use error ::untyped :: { Error };

  use process_tools ::process :: *;

  /// The `Channel` enum represents different release channels for rust.
  #[ derive( Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd ) ]
  pub enum Channel
  {
  /// Represents the stable release channel.
  #[ default ]
  Stable,
  /// Represents the nightly release channel.
  Nightly,
 }

  impl std ::fmt ::Display for Channel
  {
  fn fmt( &self, f: &mut Formatter< '_ > ) -> std ::fmt ::Result
  {
   match self
   {
  Self ::Stable => write!( f, "stable" ),
  Self ::Nightly => write!( f, "nightly" ),
 }
 }
 }

  impl TryFrom< String > for Channel
  {
  type Error = error ::untyped ::Error;
  fn try_from( value: String ) -> Result< Self, Self ::Error >
  {
   Ok( match value.as_ref()
   {
  "stable" => Self ::Stable,
  "nightly" => Self ::Nightly,
  other => error ::untyped ::bail!( "Unexpected channel value. Expected [stable, channel]. Got: `{other}`" ),
 })
 }
 }

  /// Classify one line from `rustup toolchain list` into a `Channel`, if recognised.
  ///
  /// Handles both aliased toolchains (`stable-aarch64-…`, `nightly-…`) and
  /// version-pinned toolchains (`1.94.1-aarch64-…`).
  fn classify_toolchain( line: &str ) -> Option< Channel >
  {
  // Strip trailing annotations such as "(active, default)".
  let name = line.split_whitespace().next().unwrap_or( "" );
  if name.is_empty() { return None; }

  if name.starts_with( "stable" )  { return Some( Channel ::Stable  ); }
  if name.starts_with( "nightly" ) { return Some( Channel ::Nightly ); }

  // Fix(issue-NNN): Detect version-pinned stable toolchains (e.g. "1.94.1-aarch64-unknown-linux-gnu").
  // Root cause: such names start with a digit, not "stable", so the old split_once('-') check missed them.
  // Pitfall: split_once('-') on a version-pinned name yields the major-version digit, not the channel name.
  if name.chars().next().is_some_and( | c | c.is_ascii_digit() )
  && !name.contains( "nightly" )
  && !name.contains( "beta" )
  {
   return Some( Channel ::Stable );
  }

  None
  }

  /// Retrieves the set of rust channels available via the local rustup installation.
  ///
  /// # Errors
  /// qqq: doc
  // qqq: typed error
  pub fn available_channels< P >( path: P ) -> error ::untyped ::Result< HashSet< Channel > >
  where
  P: AsRef< Path >,
  {
  let ( program, options ) = ( "rustup", [ "toolchain", "list" ] );
  let report = Run ::former()
  .bin_path( program )
  .args( options.into_iter().map( OsString ::from ).collect :: < Vec< _ > >() )
  .current_path( path.as_ref().to_path_buf() )
  .run().map_err :: < Error, _ >( | report | error ::untyped ::format_err!( report.to_string() ) )?;

  Ok( report.out.lines().filter_map( classify_toolchain ).collect() )
  }

  /// Returns the rustup toolchain identifier to pass to `rustup run` for the given channel.
  ///
  /// When the named alias (e.g. `stable-aarch64-unknown-linux-gnu`) is installed, returns the
  /// bare channel string (`"stable"`) so that `rustup run stable` keeps working.  When only a
  /// version-pinned toolchain is installed (e.g. `1.94.1-aarch64-unknown-linux-gnu`), returns
  /// its full name so `rustup run 1.94.1-aarch64-unknown-linux-gnu` is used instead.
  ///
  /// # Errors
  /// Returns an error when no toolchain is found for the requested channel.
  // qqq: typed error
  pub fn toolchain_name< P >( channel: Channel, path: P ) -> error ::untyped ::Result< String >
  where
  P: AsRef< Path >,
  {
  let ( program, options ) = ( "rustup", [ "toolchain", "list" ] );
  let report = Run ::former()
  .bin_path( program )
  .args( options.into_iter().map( OsString ::from ).collect :: < Vec< _ > >() )
  .current_path( path.as_ref().to_path_buf() )
  .run().map_err :: < Error, _ >( | report | error ::untyped ::format_err!( report.to_string() ) )?;

  let channel_str = channel.to_string();

  // Prefer the channel alias (e.g. "stable-aarch64-…") — `rustup run stable` then works.
  for line in report.out.lines()
  {
   let name = line.split_whitespace().next().unwrap_or( "" );
   if name.starts_with( &channel_str ) { return Ok( channel_str ); }
  }

  // Fall back to a version-pinned toolchain that maps to the requested channel.
  for line in report.out.lines()
  {
   let name = line.split_whitespace().next().unwrap_or( "" );
   let matches = match channel
   {
  Channel ::Stable =>
   name.chars().next().is_some_and( | c | c.is_ascii_digit() )
   && !name.contains( "nightly" )
   && !name.contains( "beta" ),
  Channel ::Nightly => name.contains( "nightly" ),
   };
   if matches { return Ok( name.to_string() ); }
  }

  error ::untyped ::bail!
  (
  "No installed toolchain for channel `{channel_str}`. \
Try to install it with `rustup install {channel_str}`"
  )
  }
}

//

crate ::mod_interface!
{
  own use Channel;
  own use available_channels;
  own use toolchain_name;
}

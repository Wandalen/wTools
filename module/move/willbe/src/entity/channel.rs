mod private
{
  use crate::*;
  use std::
  {
    fmt::Formatter,
    path::Path,
    collections::HashSet,
  };
  use std::ffi::OsString;
  use error_tools::for_app::Error;
  use wtools::error::Result;
  use process_tools::process::*;

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

  impl std::fmt::Display for Channel
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        Self::Stable => write!( f, "stable" ),
        Self::Nightly => write!( f, "nightly" ),
      }
    }
  }
  
  impl TryFrom< String > for Channel
  {
    type Error = error::for_app::Error;
    fn try_from( value : String ) -> Result< Self, Self::Error >
    {
      Ok( match value.as_ref()
      {
        "stable" => Self::Stable,
        "nightly" => Self::Nightly,
        other => error::for_app::bail!( "Unexpected channel value. Expected [stable, channel]. Got: `{other}`" ),
      })
    }
  }

  /// Retrieves a list of available channels.
  ///
  /// This function takes a path and returns a `Result` with a vector of strings representing the available channels.
  pub fn available_channels< P >( path : P ) -> Result< HashSet< Channel > >
  where
    P : AsRef< Path >,
  {
    let ( program, options ) = ( "rustup", [ "toolchain", "list" ] );
    let report = Run::former()
    .bin_path( program )
    .args( options.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
    .current_path( path.as_ref().to_path_buf() )
    .run().map_err::< Error, _ >( | report | err!( report.to_string() ) )?;

    let list = report
    .out
    .lines()
    .map( | l | l.split_once( '-' ).unwrap().0 )
    .filter_map( | c | match c
    {
      "stable" => Some( Channel::Stable ),
      "nightly" => Some( Channel::Nightly ),
      _ => None
    } )
    .collect();

    Ok( list )
  }
}

//

crate::mod_interface!
{
  protected use Channel;
  protected use available_channels;
}

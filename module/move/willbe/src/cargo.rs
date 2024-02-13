mod private
{
  use crate::*;

  use std::{ fmt::Formatter, path::Path };

  use process::CmdReport;
  use wtools::error::Result;
  use former::Former;

  ///
  /// Assemble the local package into a distributable tarball.
  ///
  /// # Args:
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  pub fn package< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "cargo", [ "package" ] );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start2_sync( program, args, path )
    }
  }
  
 /// Upload a package to the registry
  pub fn publish< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "cargo", [ "publish" ] );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start2_sync( program, args, path )
    }
  }

  /// The `Channel` enum represents different release channels for rust.
  #[ derive( Debug, Default, Copy, Clone ) ]
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

  /// Represents the arguments for the test.
  #[ derive( Debug, Former, Clone ) ]
  pub struct TestArgs
  {
    /// Specifies the release channels for rust.
    channel : Channel,
    /// Determines whether to use default features in the test.
    /// Enabled by default.
    #[ default( true ) ]
    with_default_features : bool,
    /// Determines whether to use all available features in the test.
    /// Disabled by default.
    #[ default( false ) ]
    with_all_features : bool,
    /// Specifies a list of features to be enabled in the test.
    enable_features : Vec< String >,
  }

  impl TestArgs
  {
    fn as_rustup_args(&self ) -> Vec< String >
    {
      [ "run".into(), self.channel.to_string(), "cargo".into(), "test".into() ]
      .into_iter()
      .chain( if self.with_default_features { None } else { Some( "--no-default-features".into() ) } )
      .chain( if self.with_all_features { Some( "--all-features".into() ) } else { None } )
      .chain( if self.enable_features.is_empty() { None } else { Some([ "--features".into(), self.enable_features.join( "," ) ]) }.into_iter().flatten() )
      .collect()
    }
  }

  /// Executes a test command with the given arguments.
  ///
  /// # Arguments
  ///
  /// * `path` - The path to the test command.
  /// * `args` - The arguments for the test command.
  /// * `dry` - A boolean indicating whether to perform a dry run or not.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `CmdReport` if the command is executed successfully,
  /// or an error if the command fails to execute.
  pub fn test< P >( path : P, args : TestArgs, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "rustup", args.as_rustup_args() );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start2_sync( program, args, path )
    }
  }
}

//

crate::mod_interface!
{
  protected use package;
  protected use publish;
  protected use Channel;
  protected use TestArgs;
  protected use test;
}

mod private
{
  use crate::*;

  use std::path::PathBuf;
  use wca::Args;

  use wtools::error::Result;

  /// Command to display the differences between a local and remote package versions.
  ///
  /// # Arguments
  ///
  /// * `args` - Command line arguments.
  ///
  /// # Returns
  ///
  /// Returns a `Result` indicating success or failure.
  ///
  /// # Errors
  ///
  /// Returns an error if there is an issue with the command.
  pub fn publish_diff( args : Args ) -> Result< () >
  {
    let path : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir()? );

    println!( "{}", action::publish_diff( path )? );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  // use core::
  // {
  //   fmt,
  //   ops::
  //   {
  //     Deref,
  //     DerefMut,
  //   },
  // };

  use std::
  {
    io,
  };
  use error::
  {
    // Result,
    // thiserror,
    typed::Error,
  };
  // // use path::AbsolutePath;

  /// `PathError` enum represents errors when creating a `CrateDir` object.
  #[ derive( Debug, Error ) ]
  pub enum PathError
  {
    /// Indicates a validation error with a descriptive message.
    #[ error( "Failed to create a `CrateDir` object due to `{0}`" ) ]
    Validation( String ),
    /// Try to read or write
    #[ error( "IO operation failed. Details : {0}" ) ]
    Io( #[ from ] io::Error ),
  }

}

//

mod crate_dir;
mod manifest_file;

//

crate::mod_interface!
{
  exposed use super::crate_dir::CrateDir;
  exposed use super::manifest_file::ManifestFile;
  exposed use super::crate_dir::PathEither;
  exposed use PathError;
}

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

  // use std::
  // {
  //   path::{ Path, PathBuf },
  // };
  use wtools::error::
  {
    // Result,
    thiserror,
    for_lib::Error,
  };
  // use _path::AbsolutePath;

  /// `PathError` enum represents errors when creating a `CrateDir` object.
  #[ derive( Debug, Error ) ]
  pub enum PathError
  {
    /// Indicates a validation error with a descriptive message.
    #[ error( "Failed to create a `CrateDir` object due to `{0}`" ) ]
    Validation( String ),
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
  orphan use PathError;
}

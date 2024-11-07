/// Define a private namespace for all its items.
#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  use std::
  {
    io,
  };
  use error::
  {
    typed::Error,
  };

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
mod source_file;
mod either;

//

crate::mod_interface!
{
  exposed use super::
  {
    crate_dir::CrateDir,
    manifest_file::ManifestFile,
    source_file::
    {
      SourceFile,
      Entries,
      Sources,
      // Items,
    },
    either::EitherDirOrFile
  };
  exposed use PathError;
}

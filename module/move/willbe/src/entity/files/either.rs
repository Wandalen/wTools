#[ allow( clippy::wildcard_imports ) ]
use crate::*;
use core::
{
  ops::
  {
    Deref,
    DerefMut,
  },
};
use std::
{
  path::Path,
};
// use error::
// {
//   Result,
// };

/// Wrapper over `data_type::Either< CrateDir, ManifestFile >` with util methods.
#[ derive( Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug ) ]
pub struct EitherDirOrFile( data_type::Either< CrateDir, ManifestFile > );

impl EitherDirOrFile
{
  /// Returns inner type which is an `data_type::Either`< `CrateDir`, `ManifestFile` >.
  #[ must_use ]
  pub fn inner( self ) -> data_type::Either< CrateDir, ManifestFile >
  {
    self.0
  }

}

impl TryFrom< &Path > for EitherDirOrFile
{
  type Error = PathError;

  fn try_from( value : &Path ) -> Result< Self, Self::Error >
  {
    if value.file_name() == Some( "Cargo.toml".as_ref() )
    {
      Ok( Self( data_type::Either::Right( ManifestFile::try_from( value )? ) ) )
    }
    else
    {
      Ok( Self( data_type::Either::Left( CrateDir::try_from( value )? ) ) )
    }
  }
}

impl AsRef< Path > for EitherDirOrFile
{
  fn as_ref( &self ) -> &Path
  {
    match &self.0
    {
      data_type::Either::Left( crate_dir ) => crate_dir.as_ref(),
      data_type::Either::Right( manifest_path ) => manifest_path.as_ref(),
    }
  }
}

impl AsMut< Path > for EitherDirOrFile
{
  fn as_mut( &mut self ) -> &mut Path
  {
    match &mut self.0
    {
      data_type::Either::Left( crate_dir ) => crate_dir.as_mut(),
      data_type::Either::Right( manifest_path ) => manifest_path.as_mut(),
    }
  }
}

impl Deref for EitherDirOrFile
{
  type Target = Path;

  #[ allow( clippy::explicit_deref_methods ) ]
  fn deref( &self ) -> &Self::Target
  {
    self.0.deref()
  }
}

impl DerefMut for EitherDirOrFile
{
  #[ allow( clippy::explicit_deref_methods ) ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    self.0.deref_mut()
  }
}
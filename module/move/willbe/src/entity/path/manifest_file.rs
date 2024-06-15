use crate::*;

use entity::PathError;

use core::
{
  fmt,
  ops::
  {
    Deref,
    DerefMut,
  },
};

use std::
{
  path::{ Path, PathBuf },
};
use wtools::error::
{
  Result,
};
use _path::AbsolutePath;

/// Path to crate directory
#[ derive( Debug, Clone ) ]
pub struct ManifestFile( AbsolutePath );

impl ManifestFile
{
  // qqq : bad : for Petro : why clone?
  // /// Returns an absolute path.
  // pub fn absolute_path( &self ) -> AbsolutePath
  // {
  //   self.0.clone()
  // }

  /// Returns inner type what is an absolute path.
  #[ inline( always ) ]
  pub fn inner( self ) -> AbsolutePath
  {
    self.0
  }

  /// Returns path to manifest aka cargo file.
  #[ inline( always ) ]
  pub fn manifest_path( self ) -> AbsolutePath
  {
    self.inner().join( "Cargo.toml" )
  }

}

impl fmt::Display for ManifestFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "crate dir :: {}", self.0.display() )
  }
}

// impl AsRef< AbsolutePath > for ManifestFile
// {
//   fn as_ref( &self ) -> &AbsolutePath
//   {
//     &self.0
//   }
// }

impl TryFrom< AbsolutePath > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
  {
    if !crate_dir_path.as_ref().join( "Cargo.toml" ).exists()
    {
      return Err( PathError::Validation( "The path is not a crate directory path".into() ) );
    }
    Ok( Self( crate_dir_path ) )
  }
}

impl TryFrom< PathBuf > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
  {
    if !crate_dir_path.join( "Cargo.toml" ).exists()
    {
      return Err( PathError::Validation( "The path is not a crate directory path".into() ) );
    }

    Ok( Self( AbsolutePath::try_from( crate_dir_path ).unwrap() ) )
  }
}

impl AsRef< Path > for ManifestFile
{
  fn as_ref( &self ) -> &Path
  {
    self.0.as_ref()
  }
}

impl AsMut< Path > for ManifestFile
{
  fn as_mut( &mut self ) -> &mut Path
  {
    self.0.as_mut()
  }
}

impl Deref for ManifestFile
{
  type Target = AbsolutePath;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl DerefMut for ManifestFile
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

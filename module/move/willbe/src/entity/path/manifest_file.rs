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
  io,
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

  /// Returns inner type whicj is an absolute path.
  #[ inline( always ) ]
  pub fn inner( self ) -> AbsolutePath
  {
    self.0
  }

  /// Returns path to crate dir.
  #[ inline( always ) ]
  pub fn manifest_path( self ) -> CrateDir
  {
    self.inner().parent().unwrap().try_into().unwrap()
  }

}

impl fmt::Display for ManifestFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "manifest file :: {}", self.0.display() )
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
  fn try_from( manifest_file : AbsolutePath ) -> Result< Self, Self::Error >
  {
    if !manifest_file.as_ref().is_file()
    {
      let err =  io::Error::new( io::ErrorKind::InvalidData, format!( "Cannot find crate dir at {manifest_file:?}" ) );
      return Err( PathError::Io( err ) );
    }
    Ok( Self( manifest_file ) )
  }
}

impl TryFrom< PathBuf > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file ).unwrap() )
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

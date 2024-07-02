use crate::*;

use entity::
{
  PathError,
  CrateDir,
};
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

use path::{ AbsolutePath, Utf8Path, Utf8PathBuf };

use error::
{
  Result,
};

/// Path to crate directory
#[ derive( Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
pub struct ManifestFile( AbsolutePath );

impl ManifestFile
{
  // aaa : bad : for Petro : why clone?
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
  pub fn crate_dir( self ) -> CrateDir
  {
    self.into()
  }

}

impl fmt::Display for ManifestFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "{}", self.0.display() )
  }
}

impl fmt::Debug for ManifestFile
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

impl From< CrateDir > for ManifestFile
{
  fn from( src : CrateDir ) -> Self
  {
    Self( src.absolute_path().join( "Cargo.toml" ) )
  }
}

impl From< ManifestFile > for AbsolutePath
{
  fn from( src : ManifestFile ) -> Self
  {
    src.inner()
  }
}

impl From< ManifestFile > for PathBuf
{
  fn from( src : ManifestFile ) -> Self
  {
    src.inner().inner()
  }
}

// impl From< &ManifestFile > for &str
// {
//   fn from( src : &ManifestFile ) -> Self
//   {
//     src.to_str()
//   }
// }

impl< 'a > TryFrom< &'a ManifestFile > for &'a str
{
  type Error = std::io::Error;
  fn try_from( src : &'a ManifestFile ) -> Result< &'a str, Self::Error >
  {
    ( &src.0 ).try_into()
  }
}

impl TryFrom< &ManifestFile > for String
{
  type Error = std::io::Error;
  fn try_from( src : &ManifestFile ) -> Result< String, Self::Error >
  {
    let src2 : &str = src.try_into()?;
    Ok( src2.into() )
  }
}

impl TryFrom< &AbsolutePath > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : &AbsolutePath ) -> Result< Self, Self::Error >
  {
    manifest_file.clone().try_into()
  }
}

impl TryFrom< AbsolutePath > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : AbsolutePath ) -> Result< Self, Self::Error >
  {

    if !manifest_file.as_ref().ends_with( "Cargo.toml" )
    {
      let err = io::Error::new( io::ErrorKind::Other, format!( "File path does not end with Cargo.toml as it should {manifest_file:?}" ) );
      return Err( PathError::Io( err ) );
    }

    if !manifest_file.as_ref().is_file()
    {
      let err = io::Error::new( io::ErrorKind::InvalidData, format!( "Cannot find crate dir at {manifest_file:?}" ) );
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
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
  }
}

impl TryFrom< &PathBuf > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : &PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
  }
}

impl TryFrom< &Path > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : &Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
  }
}

impl TryFrom< &str > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &str ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< Utf8PathBuf > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
  }
}

impl TryFrom< &Utf8PathBuf > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : &Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
  }
}

impl TryFrom< &Utf8Path > for ManifestFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( manifest_file : &Utf8Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( manifest_file )? )
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

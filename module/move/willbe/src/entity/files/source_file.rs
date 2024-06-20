use crate::*;

use entity::
{
  PathError,
  ManifestFile,
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
use error::
{
  Result,
};
use path::{ AbsolutePath, Utf8Path, Utf8PathBuf };

/// Path to a source file
#[ derive( Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
pub struct SourceFile( AbsolutePath );

impl SourceFile
{

  /// Returns inner type which is an absolute path.
  #[ inline( always ) ]
  pub fn inner( self ) -> AbsolutePath
  {
    self.0
  }

}

impl fmt::Display for SourceFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "{}", self.0.display() )
  }
}

impl fmt::Debug for SourceFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "source file :: {}", self.0.display() )
  }
}

impl From< ManifestFile > for SourceFile
{
  fn from( src : ManifestFile ) -> Self
  {
    Self ( src.inner().parent().unwrap() )
  }
}

impl From< SourceFile > for AbsolutePath
{
  fn from( src : SourceFile ) -> Self
  {
    src.inner()
  }
}

impl From< SourceFile > for PathBuf
{
  fn from( src : SourceFile ) -> Self
  {
    src.inner().inner()
  }
}

impl< 'a > TryFrom< &'a SourceFile > for &'a str
{
  type Error = std::io::Error;
  fn try_from( src : &'a SourceFile ) -> Result< &'a str, Self::Error >
  {
    ( &src.0 ).try_into()
  }
}

impl TryFrom< &SourceFile > for String
{
  type Error = std::io::Error;
  fn try_from( src : &SourceFile ) -> Result< String, Self::Error >
  {
    let src2 : &str = src.try_into()?;
    Ok( src2.into() )
  }
}

impl TryFrom< &AbsolutePath > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &AbsolutePath ) -> Result< Self, Self::Error >
  {
    crate_dir_path.clone().try_into()
  }
}

impl TryFrom< AbsolutePath > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
  {
    if !crate_dir_path.as_ref().join( "Cargo.toml" ).is_file()
    {
      let err =  io::Error::new( io::ErrorKind::InvalidData, format!( "Cannot find crate dir at {crate_dir_path:?}" ) );
      return Err( PathError::Io( err ) );
    }
    Ok( Self( crate_dir_path ) )
  }
}

impl TryFrom< PathBuf > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< &Path > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< &str > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &str ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< Utf8PathBuf > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl TryFrom< &Utf8Path > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : &Utf8Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path )? )
  }
}

impl AsRef< Path > for SourceFile
{
  fn as_ref( &self ) -> &Path
  {
    self.0.as_ref()
  }
}

impl AsMut< Path > for SourceFile
{
  fn as_mut( &mut self ) -> &mut Path
  {
    self.0.as_mut()
  }
}

impl Deref for SourceFile
{
  type Target = AbsolutePath;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl DerefMut for SourceFile
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

// =

pub trait Sources
{
  fn sources( &self ) -> impl Iterator< Item = SourceFile >;
}

pub trait Entries
{
  fn entries( &self ) -> impl Iterator< Item = SourceFile >;
}

// =
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
use path::AbsolutePath;
// xxx

/// Path to crate directory
#[ derive( Debug, Clone ) ]
pub struct CrateDir( AbsolutePath );

impl CrateDir
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

  /// Returns path to manifest aka cargo file.
  #[ inline( always ) ]
  pub fn manifest_path( self ) -> AbsolutePath
  {
    self.inner().join( "Cargo.toml" )
  }

}

impl fmt::Display for CrateDir
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "crate dir :: {}", self.0.display() )
  }
}

// impl AsRef< AbsolutePath > for CrateDir
// {
//   fn as_ref( &self ) -> &AbsolutePath
//   {
//     &self.0
//   }
// }

impl TryFrom< AbsolutePath > for CrateDir
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

impl TryFrom< PathBuf > for CrateDir
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( crate_dir_path ).unwrap() )
  }
}

impl AsRef< Path > for CrateDir
{
  fn as_ref( &self ) -> &Path
  {
    self.0.as_ref()
  }
}

impl AsMut< Path > for CrateDir
{
  fn as_mut( &mut self ) -> &mut Path
  {
    self.0.as_mut()
  }
}

impl Deref for CrateDir
{
  type Target = AbsolutePath;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl DerefMut for CrateDir
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

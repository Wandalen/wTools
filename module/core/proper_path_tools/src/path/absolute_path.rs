/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::
  {
    fmt,
    path::{ Path, PathBuf },
  };

  /// Absolute path.
  #[ derive( Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct AbsolutePath( PathBuf );

  impl fmt::Display for AbsolutePath
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "{}", self.0.display() )
    }
  }

  impl< 'a > TryFrom< &'a str > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &'a str ) -> Result< Self, Self::Error >
    {
      Ok( Self( path::canonicalize( value )? ) )
    }
  }

  impl TryFrom< PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : PathBuf ) -> Result< Self, Self::Error >
    {
      Ok( Self( path::canonicalize( value )? ) )
    }
  }

  // xxx : qqq : use Into< Path >
  impl TryFrom< &Path > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &Path ) -> Result< Self, Self::Error >
    {
      Ok( Self( path::canonicalize( value )? ) )
    }
  }

//   impl TryFrom< Utf8PathBuf > for AbsolutePath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( value : Utf8PathBuf ) -> Result< Self, Self::Error >
//     {
//       AbsolutePath::try_from( value.as_std_path() )
//     }
//   }

//   impl TryFrom< &Utf8Path > for AbsolutePath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( value : &Utf8Path ) -> Result< Self, Self::Error >
//     {
//       AbsolutePath::try_from( value.as_std_path() )
//     }
//   }

  // xxx : use derives
  impl AsRef< Path > for AbsolutePath
  {
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl AbsolutePath
  {
    /// Returns the Path without its final component, if there is one.
    /// Returns None if the path terminates in a root or prefix, or if it's the empty string.
    pub fn parent( &self ) -> Option< AbsolutePath >
    {
      self.0.parent().map( PathBuf::from ).map( AbsolutePath )
    }

    /// Creates an owned `AbsolutePath` with path adjoined to self.
    pub fn join< P >( &self, path : P ) -> AbsolutePath
    where
      P : AsRef< Path >,
    {
      Self::try_from( self.0.join( path ) ).unwrap()
    }
  }

}

crate::mod_interface!
{
  exposed use AbsolutePath;
}

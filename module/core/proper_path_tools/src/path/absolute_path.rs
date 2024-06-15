/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;

  use std::
  {
    borrow::Cow,
    path::{ Path, PathBuf },
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

  #[cfg(feature="no_std")]
  extern crate std;

  #[ cfg( feature = "derive_serde" ) ]
  use serde::{ Serialize, Deserialize };

  #[ cfg( feature = "path_utf8" ) ]
  use camino::{ Utf8Path, Utf8PathBuf };

  /// Absolute path.
  #[ cfg_attr( feature = "derive_serde", derive( Serialize, Deserialize ) ) ]
  #[ derive( Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct AbsolutePath( PathBuf );

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

    /// Converts a `AbsolutePath` to a `Cow<str>`
    pub fn to_string_lossy( &self ) -> Cow< '_, str >
    {
      self.0.to_string_lossy()
    }

    /// Returns inner type which is PathBuf.
    #[ inline( always ) ]
    pub fn inner( self ) -> PathBuf
    {
      self.0
    }

  }

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

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( value.as_std_path() )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8Path > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &Utf8Path ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( value.as_std_path() )
    }
  }

  impl From< AbsolutePath > for PathBuf
  {
    fn from( abs_path: AbsolutePath ) -> Self
    {
      abs_path.0
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

  // // xxx : use derives
  // impl AsRef< Path > for AbsolutePath
  // {
  //   fn as_ref( &self ) -> &Path
  //   {
  //     self.0.as_ref()
  //   }
  // }

  impl AsRef< Path > for AbsolutePath
  {
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl AsMut< Path > for AbsolutePath
  {
    fn as_mut( &mut self ) -> &mut Path
    {
      &mut self.0
    }
  }

  impl Deref for AbsolutePath
  {
    type Target = Path;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for AbsolutePath
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

}

crate::mod_interface!
{
  exposed use AbsolutePath;
}

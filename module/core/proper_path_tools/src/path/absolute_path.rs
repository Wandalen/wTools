/// Internal namespace.
mod private
{

  use crate::*;

  use std::
  {
    // borrow::Cow,
    path::{ Path, PathBuf },
    io,
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
    #[ inline ]
    pub fn parent( &self ) -> Option< AbsolutePath >
    {
      self.0.parent().map( PathBuf::from ).map( AbsolutePath )
    }

    /// Creates an owned `AbsolutePath` with path adjoined to self.
    #[ inline ]
    pub fn join< P >( &self, path : P ) -> AbsolutePath
    where
      P : AsRef< Path >,
    {
      Self::try_from( self.0.join( path ) ).unwrap()
    }

    // /// Converts a `AbsolutePath` to a `Cow<str>`
    // pub fn to_string_lossy( &self ) -> Cow< '_, str >
    // {
    //   self.0.to_string_lossy()
    // }

    /// Determines whether base is a prefix of self.
    ///
    /// Only considers whole path components to match.
    #[ inline ]
    pub fn starts_with< P : AsRef< Path > >( &self, base : P ) -> bool
    {
      self.0.starts_with( base )
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
    #[ inline ]
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "{}", self.0.display() )
    }
  }

  #[ inline ]
  fn is_absolute( path : &Path ) -> bool
  {
    // None - not absolute
    // with `.` or `..` at the first component - not absolute
    !path.components().next().is_some_and( | c | c.as_os_str() == "." || c.as_os_str() == ".." )
  }

  impl TryFrom< PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : PathBuf ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( &src.as_path() )
    }
  }

  impl TryFrom< &PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &PathBuf ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( &src.as_path() )
    }
  }

  // xxx : qqq : use Into< Path >
  impl TryFrom< &Path > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &Path ) -> Result< Self, Self::Error >
    {
      // < Self as TryFrom< &str > >::try_from( src.to_string_lossy() )
      let path = path::canonicalize( src )?;

      // xxx
      if !is_absolute( &path )
      {
        return Err( io::Error::new( io::ErrorKind::InvalidData, "Path expected to be absolute, but it's not {path}" ) )
      }

      Ok( Self( path ) )
    }
  }

  impl< 'a > TryFrom< &'a str > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &'a str ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( src.as_ref() )
    }
  }

//   impl TryFrom< &str > for AbsolutePath
//   {
//     type Error = std::io::Error;
//     // type Error = PathError;
//
//     #[ inline( always ) ]
//     fn try_from( src : &str ) -> Result< Self, Self::Error >
//     {
//       Self::try_from( AbsolutePath::try_from( src )? )
//     }
//   }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( src.as_std_path() )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( src.as_std_path() )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8Path > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &Utf8Path ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( src.as_std_path() )
    }
  }

  impl From< AbsolutePath > for PathBuf
  {
    #[ inline ]
    fn from( src : AbsolutePath ) -> Self
    {
      src.0
    }
  }

  impl< 'a > TryFrom< &'a AbsolutePath > for &'a str
  {
    type Error = std::io::Error;
    #[ inline ]
    fn try_from( src : &'a AbsolutePath ) -> Result< &'a str, Self::Error >
    {
      src
      .to_str()
      .ok_or_else
      (
        move || io::Error::new( io::ErrorKind::Other, format!( "Can't convert &PathBuf into &str {src}" ) )
      )
    }
  }

  impl TryFrom< &AbsolutePath > for String
  {
    type Error = std::io::Error;
    #[ inline ]
    fn try_from( src : &AbsolutePath ) -> Result< String, Self::Error >
    {
      let src2 : &str = src.try_into()?;
      Ok( src2.into() )
    }
  }

//   impl TryFrom< Utf8PathBuf > for AbsolutePath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( src : Utf8PathBuf ) -> Result< Self, Self::Error >
//     {
//       AbsolutePath::try_from( src.as_std_path() )
//     }
//   }

//   impl TryFrom< &Utf8Path > for AbsolutePath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( src : &Utf8Path ) -> Result< Self, Self::Error >
//     {
//       AbsolutePath::try_from( src.as_std_path() )
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
    #[ inline ]
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl AsMut< Path > for AbsolutePath
  {
    #[ inline ]
    fn as_mut( &mut self ) -> &mut Path
    {
      &mut self.0
    }
  }

  impl Deref for AbsolutePath
  {
    type Target = Path;
    #[ inline ]
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for AbsolutePath
  {
    #[ inline ]
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

//   /// Convertable into absolute path entity should implement the trait.
//   pub trait TryIntoAbsolutePath
//   {
//     /// Error returned if conversion is failed.
//     type Error;
//     /// Method to convert the type into absolute path.
//     fn into_absolute_path( self ) -> Result< AbsolutePath, Self::Error >;
//   }
//
//   // impl TryIntoAbsolutePath for AbsolutePath
//   // {
//   //   type Error = std::io::Error;
//   //   #[ inline ]
//   //   fn into_absolute_path( self ) -> Result< AbsolutePath, Self::Error >
//   //   {
//   //     Ok( self )
//   //   }
//   // }
//
//   impl< TryIntoAbsolutePathType > TryIntoAbsolutePath for TryIntoAbsolutePathType
//   where
//     TryIntoAbsolutePathType : TryInto< AbsolutePath >,
//   {
//     type Error = < Self as TryInto< AbsolutePath > >::Error;
//     #[ inline ]
//     fn into_absolute_path( self ) -> Result< AbsolutePath, Self::Error >
//     {
//       self.try_into()
//     }
//   }

}

crate::mod_interface!
{
  exposed use AbsolutePath;
  // exposed use TryIntoAbsolutePath;
}

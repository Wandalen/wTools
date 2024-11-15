/// Define a private namespace for all its items.
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

  // qqq : xxx : redo
  #[ cfg( feature="no_std" ) ]
  extern crate std;

  #[ cfg( feature="no_std" ) ]
  use alloc::string::String;

  #[ cfg( feature = "derive_serde" ) ]
  use serde::{ Serialize, Deserialize };

  // #[ cfg( feature = "path_utf8" ) ]
  // use camino::{ Utf8Path, Utf8PathBuf };

  /// Caninical path.
  #[ cfg_attr( feature = "derive_serde", derive( Serialize, Deserialize ) ) ]
  #[ derive( Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct CanonicalPath( PathBuf );

  impl CanonicalPath
  {

    /// Returns the Path without its final component, if there is one.
    /// Returns None if the path terminates in a root or prefix, or if it's the empty string.
    #[ inline ]
    pub fn parent( &self ) -> Option< CanonicalPath >
    {
      self.0.parent().map( PathBuf::from ).map( CanonicalPath )
    }

    /// Creates an owned `CanonicalPath` with path adjoined to self.
    #[ inline ]
    pub fn join< P >( &self, path : P ) -> CanonicalPath
    where
      P : AsRef< Path >,
    {
      Self::try_from( self.0.join( path ) ).unwrap()
    }

    // /// Converts a `CanonicalPath` to a `Cow<str>`
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

  impl fmt::Display for CanonicalPath
  {
    #[ inline ]
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "{}", self.0.display() )
    }
  }

  // fn is_absolute( path : &Path ) -> bool
  // {
  //   // None - not absolute
  //   // with `.` or `..` at the first component - not absolute
  //   !path.components().next().is_some_and( | c | c.as_os_str() == "." || c.as_os_str() == ".." )
  // }

  impl< 'a > TryFrom< &'a str > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : &'a str ) -> Result< Self, Self::Error >
    {
      let path = path::canonicalize( value )?;
      // if !is_absolute( &path )
      // {
      //   return Err( io::Error::new( io::ErrorKind::InvalidData, "Path expected to be absolute" ) )
      // }
      Ok( Self( path ) )
    }
  }

  impl< 'a > TryFrom< &'a String > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &'a String ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( src.as_ref() )
    }
  }

  impl< 'a > TryFrom< String > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : String ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( src.as_ref() )
    }
  }

  impl TryFrom< PathBuf > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : PathBuf ) -> Result< Self, Self::Error >
    {
      let path = path::canonicalize( value )?;

      // if !is_absolute( &path ) { return Err( io::Error::new( io::ErrorKind::InvalidData, "Path expected to be absolute" ) ) }

      Ok( Self( path ) )
    }
  }

  // xxx : qqq : use Into< Path >
  impl TryFrom< &Path > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : &Path ) -> Result< Self, Self::Error >
    {
      let path = path::canonicalize( value )?;

      // if !is_absolute( &path ) { return Err( io::Error::new( io::ErrorKind::InvalidData, "Path expected to be absolute" ) ) }

      Ok( Self( path ) )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< Utf8PathBuf > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      CanonicalPath::try_from( value.as_std_path() )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8PathBuf > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : &Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      CanonicalPath::try_from( value.as_std_path() )
    }
  }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8Path > for CanonicalPath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( value : &Utf8Path ) -> Result< Self, Self::Error >
    {
      CanonicalPath::try_from( value.as_std_path() )
    }
  }

  impl From< CanonicalPath > for PathBuf
  {
    #[ inline ]
    fn from( src : CanonicalPath ) -> Self
    {
      src.0
    }
  }

  impl< 'a > TryFrom< &'a CanonicalPath > for &'a str
  {
    type Error = std::io::Error;
    #[ inline ]
    fn try_from( src : &'a CanonicalPath ) -> Result< &'a str, Self::Error >
    {
      src
      .to_str()
      .ok_or_else
      (
        move || io::Error::new( io::ErrorKind::Other, format!( "Can't convert &PathBuf into &str {src}" ) )
      )
    }
  }

  impl TryFrom< &CanonicalPath > for String
  {
    type Error = std::io::Error;
    #[ inline ]
    fn try_from( src : &CanonicalPath ) -> Result< String, Self::Error >
    {
      let src2 : &str = src.try_into()?;
      Ok( src2.into() )
    }
  }

  impl TryIntoPath for CanonicalPath
  {
    #[ inline ]
    fn try_into_path( self ) -> Result< PathBuf, io::Error >
    {
      Ok( self.0 )
    }
  }

  impl< 'a > TryIntoCowPath< 'a > for CanonicalPath
  {
    #[ inline ]
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( self.0 ) )
    }
  }

  // impl AsPath for CanonicalPath
  // {
  //   fn as_path( &self ) -> &Path
  //   {
  //     self.0.as_path()
  //   }
  // }

//   impl TryFrom< Utf8PathBuf > for CanonicalPath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( value : Utf8PathBuf ) -> Result< Self, Self::Error >
//     {
//       CanonicalPath::try_from( value.as_std_path() )
//     }
//   }

//   impl TryFrom< &Utf8Path > for CanonicalPath
//   {
//     type Error = std::io::Error;
//
//     fn try_from( value : &Utf8Path ) -> Result< Self, Self::Error >
//     {
//       CanonicalPath::try_from( value.as_std_path() )
//     }
//   }

  // // xxx : use derives
  // impl AsRef< Path > for CanonicalPath
  // {
  //   fn as_ref( &self ) -> &Path
  //   {
  //     self.0.as_ref()
  //   }
  // }

  impl AsRef< Path > for CanonicalPath
  {
    #[ inline ]
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl AsMut< Path > for CanonicalPath
  {
    #[ inline ]
    fn as_mut( &mut self ) -> &mut Path
    {
      &mut self.0
    }
  }

  impl Deref for CanonicalPath
  {
    type Target = Path;
    #[ inline ]
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for CanonicalPath
  {
    #[ inline ]
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

}

crate::mod_interface!
{
  exposed use CanonicalPath;
}

/// Define a private namespace for all its items.
mod private
{
  use crate::*;
  use std::
  {
    path::{ Path, PathBuf },
    io,
  };
  use core::
  {
    fmt,
    ops::{ Deref, DerefMut },
  };
  #[ cfg( feature = "no_std" ) ]
  extern crate std;
  #[ cfg( feature = "no_std" ) ]
  use alloc::string::String;
  #[ cfg( feature = "derive_serde" ) ]
  use serde::{ Serialize, Deserialize };

  // #[ cfg( feature = "path_utf8" ) ]
  // use camino::{ Utf8Path, Utf8PathBuf };

  /// A new type representing an absolute path.
  ///
  /// The `AbsolutePath` type ensures that paths are absolute, which helps reduce issues and maintenance costs associated with relative paths.
  /// Relative paths can be problematic as they introduce additional variables and complexities, making code analysis, integration, refactoring, and testing more difficult.
  /// By using absolute paths, software architecture can be improved, similar to how avoiding global variables can enhance code quality.
  /// It is recommended to use relative paths only at the outskirts of an application.
  #[ cfg_attr( feature = "derive_serde", derive( Serialize, Deserialize ) ) ]
  #[ derive( Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct AbsolutePath( PathBuf );

  impl AbsolutePath
  {
    /// Returns the parent directory as an `AbsolutePath`, if it exists.
    ///
    /// Returns `None` if the path terminates in a root or prefix, or if it's the empty string.
    #[ inline ]
    pub fn parent( &self ) -> Option< AbsolutePath >
    {
      self.0.parent().map( PathBuf::from ).map( AbsolutePath )
    }

    /// Creates an owned `AbsolutePath` by joining a given path to `self`.
    #[ inline ]
    pub fn join< P >( &self, path : P ) -> AbsolutePath
    where
      P : AsRef< Path >,
    {
      Self::try_from( self.0.join( path ) ).unwrap()
    }

    /// Checks if the path starts with a given base path.
    ///
    /// Only considers whole path components to match.
    #[ inline ]
    pub fn starts_with< P : AsRef< Path > >( &self, base : P ) -> bool
    {
      self.0.starts_with( base )
    }

    /// Returns the inner `PathBuf`.
    #[inline(always)]
    pub fn inner( self ) -> PathBuf
    {
      self.0
    }

    /// Creates an `AbsolutePath` from an iterator over items that implement `TryIntoCowPath`.
    ///
    /// This function joins all path segments into a single path and attempts to convert it
    /// into an `AbsolutePath`. The resulting path must be absolute.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator over path segments.
    ///
    /// # Returns
    ///
    /// * `Ok(AbsolutePath)` if the joined path is absolute.
    /// * `Err(io::Error)` if the joined path is not absolute.
    pub fn from_iter< 'a, I, P >( iter : I ) -> Result< Self, io::Error >
    where
      I : Iterator< Item = P >,
      P : TryIntoCowPath< 'a >,
    {
      let joined_path = iter_join( iter );
      AbsolutePath::try_from( joined_path )
    }

    /// Joins path components into a `PathBuf`.
    ///
    /// This function leverages the `PathJoined` trait to join multiple path components into a single `PathBuf`.
    ///
    /// # Arguments
    ///
    /// * `paths` - A tuple of path components implementing the `PathJoined` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - The joined path as a `PathBuf`.
    /// * `Err(io::Error)` - An error if any component fails to convert.
    pub fn from_paths< Paths : PathJoined >( paths : Paths ) -> Result< Self, io::Error >
    {
      Self::try_from( paths.iter_join()? )
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

  impl TryFrom< &Path > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &Path ) -> Result< Self, Self::Error >
    {
      let path = path::canonicalize( src )?;

      if !is_absolute( &path )
      {
        return Err( io::Error::new( io::ErrorKind::Other, format!( "Path expected to be absolute, but it's not {path:?}" ) ) );
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

  impl< 'a > TryFrom< &'a String > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &'a String ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( src.as_ref() )
    }
  }

  impl< 'a > TryFrom< String > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : String ) -> Result< Self, Self::Error >
    {
      < Self as TryFrom< &Path > >::try_from( src.as_ref() )
    }
  }

  #[cfg( feature = "path_utf8" )]
  impl TryFrom< Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( src.as_std_path() )
    }
  }

  #[cfg( feature = "path_utf8" )]
  impl TryFrom< &Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : &Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( src.as_std_path() )
    }
  }

  #[cfg( feature = "path_utf8" )]
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
      src.to_str().ok_or_else( || io::Error::new( io::ErrorKind::Other, format!( "Can't convert &PathBuf into &str {src}" ) ) )
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

  impl TryIntoPath for AbsolutePath
  {
    #[ inline ]
    fn try_into_path( self ) -> Result< PathBuf, io::Error >
    {
      Ok( self.0 )
    }
  }

  impl< 'a > TryIntoCowPath< 'a > for AbsolutePath
  {
    #[ inline ]
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( self.0 ) )
    }
  }

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
}

crate::mod_interface!
{
  exposed use AbsolutePath;
}
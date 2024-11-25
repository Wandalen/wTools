//!
//! Paths in agents graph.
//!

mod private
{
  use std::
  {
    io,
    fmt,
    ops::Deref,
  };

  /// New type for paths in agents graph. Use `TryFrom` implementation
  /// to create `Path`s.
  ///
  /// Paths resemble filesystem path, path separator is `::`.
  /// Absolute path starts with `::`.
  #[ derive( Debug, Clone, Eq, PartialEq, Hash ) ]
  pub struct Path( String );

  impl Path
  {
    /// Returns the parent directory, if it exists.
    ///
    /// Returns `None` if the `Path` terminates in a root or if it's the empty string.
    #[ inline ]
    pub fn parent( &self ) -> Option< Path >
    {
      todo!()
    }

    /// Returns whether the `Path` is relative (does not start with `::`).
    pub fn is_relative( &self ) -> bool
    {
      !self.is_absolute()
    }

    /// Returns whether the `Path` is absolute (starts with `::`).
    pub fn is_absolute( &self ) -> bool
    {
      todo!()
    }

    /// Creates an owned `Path` by joining a given path to `self`.
    ///
    /// Returns `Err(io::Error)` is the `path` is an absolute path.
    #[ inline ]
    pub fn join( &self, path : &Path ) -> Result< Self, io::Error >
    {
      todo!()
    }

    /// Checks if the `Path` starts with a given base path.
    ///
    /// Only considers whole path components to match.
    #[ inline ]
    pub fn starts_with( &self, base : &Path ) -> bool
    {
      todo!()
    }

    /// Returns the inner `String`.
    #[ inline( always ) ]
    pub fn inner( self ) -> String
    {
      self.0
    }

    /// Creates an `Path` from an iterator over items that implement `AsRef<str>`.
    ///
    /// Returns `Err(io::Error)` is the `Path` is not well-formed.
    pub fn from_iter< I, P >( iter : I ) -> Result< Self, io::Error >
    where
      I : Iterator< Item = P >,
      P : AsRef< str >,
    {
      todo!()
    }
  }

  impl fmt::Display for Path
  {
    #[ inline ]
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl TryFrom< &str > for Path
  {
    type Error = io::Error;

    fn try_from( src : &str ) -> Result< Self, Self::Error >
    {
      todo!()
    }
  }

  impl AsRef< str > for Path
  {
    #[ inline ]
    fn as_ref( &self ) -> &str
    {
      self.0.as_ref()
    }
  }

  impl Into< String > for Path
  {
    #[ inline ]
    fn into( self ) -> String
    {
      self.0
    }
  }

  impl Deref for Path
  {
    type Target = str;

    #[ inline ]
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }
}

crate::mod_interface!
{
  own use Path;
}
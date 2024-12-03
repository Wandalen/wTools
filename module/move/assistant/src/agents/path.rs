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
    sync::LazyLock,
  };

  use serde::
  {
    Serialize,
    Deserialize,
  };

  use regex::Regex;

  /// Path separator string.
  pub const PATH_SEPARATOR : &str = "::";

  /// Regular expression for `Path` items. Represented in `&str`. 
  /// It is not anchored to start and end of the string.
  ///
  /// If you want to match against this expression, use `PATH_ITEM_REGEX`.
  pub const PATH_ITEM_REGEX_STR : &str = r"[a-zA-Z0-9_ -]+";

  /// Regular expression for `Path`. You can match whole `&str` with this type.
  pub static PATH_REGEX : LazyLock< Regex > = LazyLock::new( || 
  {
    let regex = format!
    (
        r"^({sep})?({item}({sep}{item})*({sep})?)?$",
        sep = PATH_SEPARATOR,
        item = PATH_ITEM_REGEX_STR,
    );

    Regex::new( &regex ).unwrap()
  });

  /// New type for paths in agents graph. Use `TryFrom` implementation
  /// to create `Path`s.
  ///
  /// Paths resemble filesystem path, path separator is `::`.
  /// Absolute path starts with `::`.
  #[ derive( Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize ) ]
  pub struct Path( String );

  impl Path
  {
    /// Returns the parent directory, if it exists.
    ///
    /// Returns `None` if the `Path` terminates in a root or if it's the empty string.
    #[ inline ]
    pub fn parent( &self ) -> Option< Path >
    {
      /// Find parent of a `Path`.
      ///
      /// This method uses `&str` as an argument instead of `Path`
      /// in order to be more general and handle trailing `::` case.
      fn find_parent( s : &str ) -> Option< &str >
      {
        s.rfind( PATH_SEPARATOR )
        .map( | sep_pos | 
        {
          if sep_pos == 0
          {
            // We found root. We should not return string before `::`,
            // as it will be empty.
            Some( PATH_SEPARATOR )
          }
          else if sep_pos == s.len() - PATH_SEPARATOR.len()
          {
            // We found trailing `::`. We should continue looking for last separator.
            find_parent( &s[ .. sep_pos ] )
          }
          else
          {
            Some( &s[ .. sep_pos ] )
          }
        })
        .flatten()
      }

      find_parent( self.0.as_str() )
      .map( | s | Self( s.to_string() ) )
    }

    /// Returns whether the `Path` is relative (does not start with `::`).
    pub fn is_relative( &self ) -> bool
    {
      !self.is_absolute()
    }

    /// Returns whether the `Path` is absolute (starts with `::`).
    pub fn is_absolute( &self ) -> bool
    {
      self.0.starts_with( PATH_SEPARATOR )
    }

    /// Creates an owned `Path` by joining a given path to `self`.
    ///
    /// If path is joined with an absolute path, then this absolute
    /// path will be returned.
    #[ inline ]
    pub fn join( &self, path : &Path ) ->  Self
    {
      if path.is_absolute()
      {
        path.clone()
      }
      else
      {
        if self.0.ends_with( PATH_SEPARATOR )
        {
          Self( format!( "{}{}", self.0, path.0 ) )
        }
        else
        {
          Self( format!( "{}::{}", self.0, path.0 ) )
        }
      }
    }

    /// Checks if the `Path` starts with a given base path.
    #[ inline ]
    pub fn starts_with( &self, base : &Path ) -> bool
    {
      self.0.starts_with( base.0.as_str() )
    }

    /// Returns the inner `String`.
    #[ inline( always ) ]
    pub fn inner( self ) -> String
    {
      self.0
    }

    /// Iterate over components of a `Path`. If the `Path` is absolute, then the first
    /// element will be `::`.
    pub fn components( &self ) -> impl Iterator< Item = &str >
    {
      self.0.split( PATH_SEPARATOR ).map( | c |
      {
        if c.is_empty()
        {
          PATH_SEPARATOR
        }
        else
        {
          c
        }
      })
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

  impl TryFrom< String > for Path
  {
    type Error = io::Error;

    fn try_from( src : String ) -> Result< Self, Self::Error >
    {
      if PATH_REGEX.is_match( src.as_str() )
      {
        Ok( Self ( src ) )
      }
      else
      {
        Err( io::Error::from( io::ErrorKind::InvalidData ) )
      }
    }
  }

  impl TryFrom< &str > for Path
  {
    type Error = io::Error;

    fn try_from( src : &str ) -> Result< Self, Self::Error >
    {
      if PATH_REGEX.is_match( src )
      {
        Ok( Self ( src.to_string() ) )
      }
      else
      {
        Err( io::Error::from( io::ErrorKind::InvalidData ) )
      }
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
  own use
  {
    Path,
    PATH_SEPARATOR,
  };
}
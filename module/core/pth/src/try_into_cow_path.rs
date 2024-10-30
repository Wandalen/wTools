/// Internal namespace.
mod private
{
  use crate::*;
  use std::
  {
    borrow::Cow,
    io,
    path::{ Component, Path, PathBuf },
  };
  // use camino::{ Utf8Path, Utf8PathBuf };

  /// A trait for converting various types into a `Cow<Path>`.
  ///
  /// This trait is designed to avoid redundant memory allocation.
  /// Unlike `TryIntoPath`, it does not allocate memory on the heap if it's not necessary.
  /// Unlike `AsPath`, it is implemented for a wider number of path-like types, similar to `TryIntoPath`.
  /// The drawback is the necessity to differentiate borrowed and owned paths at runtime.
  pub trait TryIntoCowPath<'a>
  {
    /// Converts the implementing type into a `Cow<Path>`.
    ///
    /// # Returns
    ///
    /// * `Ok(Cow<Path>)` - A `Cow` that may be either borrowed or owned, depending on the input type.
    /// * `Err(io::Error)` - An error if the conversion fails.
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >;
  }

  /// Implementation of `TryIntoCowPath` for `String`.
  impl<'a> TryIntoCowPath<'a> for &'a str
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Borrowed( self.as_path() ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for `String`.
  impl<'a> TryIntoCowPath<'a> for String
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( PathBuf::from( self ) ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for `PathBuf`.
  impl<'a> TryIntoCowPath<'a> for PathBuf
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( self ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for a reference to `Path`.
  impl<'a> TryIntoCowPath<'a> for &'a Path
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Borrowed( self ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for a reference to `Utf8Path`.
  #[cfg( feature = "path_utf8" )]
  impl< 'a > TryIntoCowPath< 'a > for &'a Utf8Path
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Borrowed( self.as_std_path() ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for `Utf8PathBuf`.
  #[cfg( feature = "path_utf8" )]
  impl<'a> TryIntoCowPath<'a> for Utf8PathBuf
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( self.as_std_path().to_path_buf() ) )
    }
  }

  /// Implementation of `TryIntoCowPath` for `std::path::Component`.
  impl<'a> TryIntoCowPath<'a> for Component<'a>
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Owned( PathBuf::from( self.as_os_str() ) ) )
    }
  }

  /// Blanket implementation of `TryIntoCowPath` for references to types implementing `AsPath`.
  impl<'a, T> TryIntoCowPath< 'a > for &'a T
  where
    T : AsPath,
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      Ok( Cow::Borrowed( self.as_path() ) )
    }
  }

}

crate::mod_interface!
{
  orphan use TryIntoCowPath;
}
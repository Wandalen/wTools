/// Internal namespace.
mod private
{
  #[ allow( unused_imports ) ]
  use crate::*;
  #[ cfg( feature = "no_std" ) ]
  extern crate std;

  use std::path::Path;

  /// A trait for converting various types into a reference to a `Path`.
  ///
  /// This trait is used to avoid redundant allocation of memory by providing a reference to a `Path`.
  /// It is implemented only for types that can either be referenced or are references to `Path` itself.
  /// Unlike `TryIntoPath`, it does not allocate memory on the heap. However, `TryIntoPath` is implemented for a wider range of types because it is not restricted from allocating memory.
  /// Unlike `AsRef<Path>`, `AsPath` is implemented for a wider number of types, including those that are not directly convertible to a `Path` using `AsRef`.
  /// This is because `AsPath` is designed to provide a more flexible interface for path-like types, accommodating various representations that can logically be treated as paths.
  pub trait AsPath
  {
    /// Converts the implementing type into a reference to a `Path`.
    ///
    /// # Returns
    ///
    /// A reference to a `Path`.
    fn as_path( &self ) -> &Path;
  }

  /// Implementation of `AsPath` for `str`.
  impl AsPath for str
  {
    fn as_path( &self ) -> &Path
    {
      Path::new( self )
    }
  }

  /// Implementation of `AsPath` for `Path`.
  impl AsPath for Path
  {
    fn as_path( &self ) -> &Path
    {
      self
    }
  }

  /// Implementation of `AsPath` for `Utf8Path`.
  #[cfg( feature = "path_utf8" )]
  impl AsPath for Utf8Path
  {
    fn as_path( &self ) -> &Path
    {
      self.as_std_path()
    }
  }

  /// Blanket implementation of `AsPath` for all types that implement `AsRef<Path>`.
  impl< T > AsPath for T
  where
    T : AsRef< Path >,
  {
    fn as_path( &self ) -> &Path
    {
      self.as_ref()
    }
  }
}

crate::mod_interface!
{
  orphan use AsPath;
}
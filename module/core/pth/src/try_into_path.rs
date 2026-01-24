/// Define a private namespace for all its items.
mod private
{
  #[ allow( unused_imports, clippy ::wildcard_imports ) ]
  use crate :: *;
  use std ::
  {
  io,
  path :: { Component, Path, PathBuf },
  string ::String,
 };

  /// A trait for converting various types into a `PathBuf`.
  ///
  /// This trait is used to convert any path-like type into an owned `PathBuf`.
  /// Unlike `TryIntoCowPath`, it always returns an owned `PathBuf`, so there is no need to differentiate between borrowed and owned paths at runtime.
  /// Unlike `AsPath`, it is implemented for a wider range of path-like types, similar to `TryIntoCowPath`.
  pub trait TryIntoPath
  {
  /// Converts the implementing type into a `PathBuf`.
  ///
  /// # Returns
  ///
  /// * `Ok(PathBuf)` - The owned path buffer.
  /// * `Err(io ::Error)` - An error if the conversion fails.
  ///
  /// # Errors
  ///
  /// Currently, all standard implementations (`&str`, `String`, `PathBuf`, `&Path`, `Component`,
  /// and types implementing `AsRef<Path>`) never return an error - they always succeed. The `Result`
  /// return type is maintained for:
  /// - API consistency with `TryIntoCowPath` and other fallible conversions
  /// - Future implementations that may need to validate or transform paths
  /// - Custom types that may have fallible path conversions
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >;
 }

  /// Implementation of `TryIntoPath` for `&str`.
  impl TryIntoPath for &str
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( PathBuf ::from( self ) )
 }
 }

  /// Implementation of `TryIntoPath` for `String`.
  impl TryIntoPath for String
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( PathBuf ::from( self ) )
 }
 }

  /// Implementation of `TryIntoPath` for a reference to `Path`.
  impl TryIntoPath for &Path
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.to_path_buf() )
 }
 }

  /// Implementation of `TryIntoPath` for `PathBuf`.
  impl TryIntoPath for PathBuf
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self )
 }
 }

  /// Implementation of `TryIntoPath` for a reference to `Utf8Path`.
  #[ cfg( feature = "path_utf8" ) ]
  impl TryIntoPath for &Utf8Path
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.as_std_path().to_path_buf() )
 }
 }

  /// Implementation of `TryIntoPath` for `Utf8PathBuf`.
  #[ cfg( feature = "path_utf8" ) ]
  impl TryIntoPath for Utf8PathBuf
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.as_std_path().to_path_buf() )
 }
 }

  /// Implementation of `TryIntoPath` for `std ::path ::Component`.
  impl TryIntoPath for Component< '_ >
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.as_os_str().into() )
 }
 }

  /// Blanket implementation of `TryIntoPath` for references to types implementing `AsRef< Path >`.
  impl< T > TryIntoPath for &T
  where
  T: AsRef< Path >,
  {
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.as_ref().to_path_buf() )
 }
 }
}

crate ::mod_interface!
{
  orphan use TryIntoPath;
}
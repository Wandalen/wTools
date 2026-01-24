/// Define a private namespace for all its items.
mod private
{
  use crate :: *;
  use std ::
  {
  borrow ::Cow,
  path :: { Path, PathBuf },
  io,
 };
  use core ::
  {
  fmt,
  ops ::
  {
   Deref,
   DerefMut,
 },
 };
  #[ cfg( feature = "derive_serde" ) ]
  use serde :: { Serialize, Deserialize };


  /// A path that has been normalized via syntactic canonicalization.
  ///
  /// This type represents a path that has been processed through `path::canonicalize()`,
  /// which resolves symbolic links and normalizes the path to an absolute form.
  ///
  /// **Historical Note**: Previously implemented as separate `CanonicalPath` and `NativePath`
  /// types, but these were functionally identical and have been unified as of v0.30.0.
  /// Use the deprecated type aliases for backward compatibility during migration.
  #[ cfg_attr( feature = "derive_serde", derive( Serialize, Deserialize ) ) ]
  #[ derive( Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct NormalizedPath( PathBuf );

  impl NormalizedPath
  {

  /// Returns the Path without its final component, if there is one.
  /// Returns None if the path terminates in a root or prefix, or if it's the empty string.
  #[ inline ]
  pub fn parent( &self ) -> Option< NormalizedPath >
  {
   self.0.parent().map( PathBuf ::from ).map( NormalizedPath )
 }

  /// Creates an owned `NormalizedPath` with path adjoined to self.
  ///
  /// # Errors
  ///
  /// Returns an error if the resulting path cannot be canonicalized. This occurs when:
  /// - The resulting path does not exist in the filesystem
  /// - Permission is denied to access the path
  /// - I/O errors occur during canonicalization
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use pth::NormalizedPath;
  /// use std::path::Path;
  ///
  /// let base = NormalizedPath::try_from("/tmp")?;
  /// let joined = base.join("subdir")?;
  /// # Ok::<(), std::io::Error>(())
  /// ```
  #[ inline ]
  pub fn join< P >( &self, path: P ) -> Result< NormalizedPath, io::Error >
  where
   P: AsRef< Path >,
  {
   Self ::try_from( self.0.join( path ) )
 }


  /// Determines whether base is a prefix of self.
  ///
  /// Only considers whole path components to match.
  #[ inline ]
  pub fn starts_with< P: AsRef< Path > >( &self, base: P ) -> bool
  {
   self.0.starts_with( base )
 }

  /// Returns inner type which is `PathBuf`.
  #[ inline( always ) ]
  #[ must_use ]
  pub fn inner( self ) -> PathBuf
  {
   self.0
 }

 }

  impl fmt ::Display for NormalizedPath
  {
  #[ inline ]
  fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
  {
   write!( f, "{}", self.0.display() )
 }
 }


  impl< 'a > TryFrom< &'a str > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: &'a str ) -> Result< Self, Self ::Error >
  {
   let path = path ::normalize_unchecked( value );
   // if !is_absolute( &path )
   // {
   //   return Err( io ::Error ::new( io ::ErrorKind ::InvalidData, "Path expected to be absolute" ) )
   // }
   Ok( Self( path ) )
 }
 }

  impl< 'a > TryFrom< &'a String > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( src: &'a String ) -> Result< Self, Self ::Error >
  {
   < Self as TryFrom< &Path > > ::try_from( src.as_ref() )
 }
 }

  #[ allow( clippy ::extra_unused_lifetimes ) ]
  impl< 'a > TryFrom< String > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( src: String ) -> Result< Self, Self ::Error >
  {
   < Self as TryFrom< &Path > > ::try_from( src.as_ref() )
 }
 }

  impl TryFrom< PathBuf > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: PathBuf ) -> Result< Self, Self ::Error >
  {
   let path = path ::normalize_unchecked( value );

   // if !is_absolute( &path ) { return Err( io ::Error ::new( io ::ErrorKind ::InvalidData, "Path expected to be absolute" ) ) }

   Ok( Self( path ) )
 }
 }

  impl TryFrom< &Path > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: &Path ) -> Result< Self, Self ::Error >
  {
   let path = path ::normalize_unchecked( value );

   // if !is_absolute( &path ) { return Err( io ::Error ::new( io ::ErrorKind ::InvalidData, "Path expected to be absolute" ) ) }

   Ok( Self( path ) )
 }
 }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< Utf8PathBuf > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: Utf8PathBuf ) -> Result< Self, Self ::Error >
  {
   NormalizedPath ::try_from( value.as_std_path() )
 }
 }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8PathBuf > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: &Utf8PathBuf ) -> Result< Self, Self ::Error >
  {
   NormalizedPath ::try_from( value.as_std_path() )
 }
 }

  #[ cfg( feature = "path_utf8" ) ]
  impl TryFrom< &Utf8Path > for NormalizedPath
  {
  type Error = std ::io ::Error;

  #[ inline ]
  fn try_from( value: &Utf8Path ) -> Result< Self, Self ::Error >
  {
   NormalizedPath ::try_from( value.as_std_path() )
 }
 }

  impl From< NormalizedPath > for PathBuf
  {
  #[ inline ]
  fn from( src: NormalizedPath ) -> Self
  {
   src.0
 }
 }

  impl< 'a > TryFrom< &'a NormalizedPath > for &'a str
  {
  type Error = std ::io ::Error;
  #[ inline ]
  fn try_from( src: &'a NormalizedPath ) -> Result< &'a str, Self ::Error >
  {
   src
   .to_str()
   .ok_or_else
   (
  move || io ::Error ::other( format!( "Can't convert &PathBuf into &str {}", src.display() ) )
 )
 }
 }

  impl TryFrom< &NormalizedPath > for String
  {
  type Error = std ::io ::Error;
  #[ inline ]
  fn try_from( src: &NormalizedPath ) -> Result< String, Self ::Error >
  {
   let src2: &str = src.try_into()?;
   Ok( src2.into() )
 }
 }

  impl TryIntoPath for NormalizedPath
  {
  #[ inline ]
  fn try_into_path( self ) -> Result< PathBuf, io ::Error >
  {
   Ok( self.0 )
 }
 }

  impl< 'a > TryIntoCowPath< 'a > for NormalizedPath
  {
  #[ inline ]
  fn try_into_cow_path( self ) -> Result< Cow<'a, Path >, io ::Error >
  {
   Ok( Cow ::Owned( self.0 ) )
 }
 }




  impl AsRef< Path > for NormalizedPath
  {
  #[ inline ]
  fn as_ref( &self ) -> &Path
  {
   self.0.as_ref()
 }
 }

  impl AsMut< Path > for NormalizedPath
  {
  #[ inline ]
  fn as_mut( &mut self ) -> &mut Path
  {
   &mut self.0
 }
 }

  impl Deref for NormalizedPath
  {
  type Target = Path;
  #[ inline ]
  fn deref( &self ) -> &Self ::Target
  {
   &self.0
 }
 }

  impl DerefMut for NormalizedPath
  {
  #[ inline ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
   &mut self.0
 }
 }

}

crate ::mod_interface!
{
  exposed use NormalizedPath;
}

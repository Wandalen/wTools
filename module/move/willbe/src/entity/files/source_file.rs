use crate::*;

use entity::
{
  PathError,
  ManifestFile,
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
use std::
{
  fs,
  path::{ Path, PathBuf },
  borrow::Cow,
};
// use error::
// {
//   Result,
// };
use path::{ AbsolutePath, Utf8Path, Utf8PathBuf };

/// Path to a source file
#[ derive( Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
pub struct SourceFile( AbsolutePath );

impl SourceFile
{

  /// Returns inner type which is an absolute path.
  #[ inline( always ) ]
  pub fn inner( self ) -> AbsolutePath
  {
    self.0
  }

}

impl fmt::Display for SourceFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "{}", self.0.display() )
  }
}

impl fmt::Debug for SourceFile
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    write!( f, "source file :: {}", self.0.display() )
  }
}

impl From< ManifestFile > for SourceFile
{
  fn from( src : ManifestFile ) -> Self
  {
    Self ( src.inner().parent().unwrap() )
  }
}

impl From< SourceFile > for AbsolutePath
{
  fn from( src : SourceFile ) -> Self
  {
    src.inner()
  }
}

impl From< SourceFile > for PathBuf
{
  fn from( src : SourceFile ) -> Self
  {
    src.inner().inner()
  }
}

impl< 'a > TryFrom< &'a SourceFile > for &'a str
{
  type Error = std::io::Error;
  fn try_from( src : &'a SourceFile ) -> Result< &'a str, Self::Error >
  {
    ( &src.0 ).try_into()
  }
}

impl TryFrom< &SourceFile > for String
{
  type Error = std::io::Error;
  fn try_from( src : &SourceFile ) -> Result< String, Self::Error >
  {
    let src2 : &str = src.try_into()?;
    Ok( src2.into() )
  }
}

impl TryFrom< &AbsolutePath > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : &AbsolutePath ) -> Result< Self, Self::Error >
  {
    src.clone().try_into()
  }
}

impl TryFrom< AbsolutePath > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : AbsolutePath ) -> Result< Self, Self::Error >
  {
    Ok( Self( src ) )
  }
}

impl TryFrom< PathBuf > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl TryFrom< &Path > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : &Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl TryFrom< &str > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : &str ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl TryFrom< Utf8PathBuf > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl TryFrom< &Utf8PathBuf > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : &Utf8PathBuf ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl TryFrom< &Utf8Path > for SourceFile
{
  type Error = PathError;

  #[ inline( always ) ]
  fn try_from( src : &Utf8Path ) -> Result< Self, Self::Error >
  {
    Self::try_from( AbsolutePath::try_from( src )? )
  }
}

impl AsRef< Path > for SourceFile
{
  fn as_ref( &self ) -> &Path
  {
    self.0.as_ref()
  }
}

impl AsMut< Path > for SourceFile
{
  fn as_mut( &mut self ) -> &mut Path
  {
    self.0.as_mut()
  }
}

impl Deref for SourceFile
{
  type Target = AbsolutePath;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl DerefMut for SourceFile
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

// =

impl CodeItems for SourceFile
{
  fn items( &self ) -> impl IterTrait< '_, syn::Item >
  {
    // xxx : use closures instead of expect
    let content = fs::read_to_string( self.as_ref() ).expect( &format!( "Failed to parse file {self}" ) );
    let parsed : syn::File = syn::parse_file( &content ).expect( &format!( "Failed to parse file {self}" ) );
    parsed.items.into_iter()
  }
}

impl AsCode for SourceFile
{
  fn as_code< 'a >( &'a self ) -> std::io::Result< Cow< 'a, str > >
  {
    Ok( Cow::Owned( std::fs::read_to_string( self.as_ref() )? ) )
  }
}

// =

/// A trait that defines a method for retrieving an iterator over entries.
///
/// The `Entries` trait is used to represent objects that can provide an iterator over their
/// contained entries, which are represented as source files. This can be useful in scenarios
/// where you need to access or process all entries associated with an object.
pub trait Entries
{
  /// Returns an iterator over the entries.
  fn entries( &self ) -> impl IterTrait< '_, SourceFile >;
}

/// A trait that defines a method for retrieving an iterator over source files.
///
/// The `Sources` trait is used to represent objects that can provide an iterator over their
/// contained source files. This can be useful in scenarios where you need to access or process
/// all source files associated with an object.
pub trait Sources
{
  /// Returns an iterator over the source files.
  fn sources( &self ) -> impl IterTrait< '_, SourceFile >;
}

// =

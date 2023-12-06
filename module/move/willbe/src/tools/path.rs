/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };
  use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };

  /// Absolute path.
  #[ derive( Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct AbsolutePath( PathBuf );

  impl TryFrom< PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : PathBuf ) -> Result< Self, Self::Error >
    {
      Ok( Self( canonicalize( value )? ) )
    }
  }

  impl TryFrom< &Path > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &Path ) -> Result< Self, Self::Error >
    {
      Ok( Self( canonicalize( value )? ) )
    }
  }

  impl TryFrom< Utf8PathBuf > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : Utf8PathBuf ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( value.as_std_path() )
    }
  }

  impl TryFrom< &Utf8Path > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &Utf8Path ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( value.as_std_path() )
    }
  }

  impl AsRef< Path > for AbsolutePath
  {
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  impl AbsolutePath
  {
    /// Returns the Path without its final component, if there is one.
    /// Returns None if the path terminates in a root or prefix, or if it's the empty string.
    pub fn parent( &self ) -> Option< AbsolutePath >
    {
      self.0.parent().map( PathBuf::from ).map( AbsolutePath )
    }

    /// Creates an owned `AbsolutePath` with path adjoined to self.
    pub fn join< P >( &self, path : P ) -> AbsolutePath
    where
      P : AsRef< Path >,
    {
      Self::try_from( self.0.join( path ) ).unwrap()
    }
  }

  /// Check if path is valid.
  pub fn valid_is( path: &str ) -> bool
  {
    std::fs::metadata( path ).is_ok()
  }

  /// Check if path has a glob.
  #[ allow( dead_code ) ]
  pub fn glob_is( path : &str ) -> bool
  {
    let glob_chars = "*?[{";
    let mut last_char = ' ';
    for char in path.chars()
    {
      if last_char != '\\' && glob_chars.contains( char )
      {
        return true;
      }

      last_char = char;
    }

    false
  }

  /// qqq : for Bohdan : explain how does it work?
  pub fn canonicalize( path : impl AsRef< Path > ) -> std::io::Result< PathBuf >
  {
    let path = path.as_ref().canonicalize()?;

    // qqq : for Bohdan : explain why is it necessary? Add relevant links.
    #[ cfg( target_os = "windows" ) ] // canonicalization on windows adds `\\?\` prefix
    let path =
    {
      const VERBATIM_PREFIX : &str = r#"\\?\"#;
      let p = path.display().to_string();
      if p.starts_with( VERBATIM_PREFIX )
      {
        PathBuf::from( &p[ VERBATIM_PREFIX.len() .. ] )
      }
      else
      {
        path.into()
      }
    };

    Ok( path )
  }

}

crate::mod_interface!
{
  protected use glob_is;
  protected use valid_is;
  protected use canonicalize;

  protected use AbsolutePath;
}

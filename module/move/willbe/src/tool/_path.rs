/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };
  use std::time::{ SystemTime, UNIX_EPOCH };
  use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };

  /// Absolute path.
  #[ derive( Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash ) ]
  pub struct AbsolutePath( PathBuf );

  impl TryFrom< &str > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : &str ) -> Result< Self, Self::Error >
    {
      let value = PathBuf::from( value );
      Ok( Self( canonicalize( value )? ) )
    }
  }

  impl TryFrom< String > for AbsolutePath
  {
    type Error = std::io::Error;

    fn try_from( value : String ) -> Result< Self, Self::Error >
    {
      let value = PathBuf::from( value );
      Ok( Self( canonicalize( value )? ) )
    }
  }

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

  // qqq : for Petro : for Bohdan : bad. move out

  /// Check if path has a glob.
  #[ allow( dead_code ) ]
  fn glob_is( path : &str ) -> bool
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

  /// Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
  pub fn canonicalize( path : impl AsRef< Path > ) -> std::io::Result< PathBuf >
  {
    let path = path.as_ref().canonicalize()?;

    // In Windows the regular/legacy paths (C :\foo) are supported by all programs, but have lots of bizarre restrictions for backwards compatibility with MS-DOS.
    // And there are Windows NT UNC paths (\\?\C :\foo), which are more robust and with fewer gotchas, but are rarely supported by Windows programs. Even Microsoft’s own!
    //
    // https://github.com/rust-lang/rust/issues/42869
    #[ cfg( target_os = "windows" ) ]
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

  /// Generate name based on system time
  pub fn unique_folder_name() -> crate::wtools::error::Result< String >
  {
    let timestamp = SystemTime::now()
    .duration_since( UNIX_EPOCH )?
    .as_nanos();

    Ok( format!( "{}", timestamp ) )
  }

}

crate::mod_interface!
{
  // qqq : remove this?
  // protected use glob_is;
  protected use canonicalize;
  protected use unique_folder_name;

  protected use AbsolutePath;
}

// qqq : for Petro : for Bohdan : rid off this file. use proper_path_tools

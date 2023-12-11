/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };

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

  /// Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
  pub fn canonicalize( path : impl AsRef< Path > ) -> std::io::Result< PathBuf >
  {
    let path = path.as_ref().canonicalize()?;

    // In Windows the regular/legacy paths (C:\foo) are supported by all programs, but have lots of bizarre restrictions for backwards compatibility with MS-DOS.
    // And there are Windows NT UNC paths (\\?\C:\foo), which are more robust and with fewer gotchas, but are rarely supported by Windows programs. Even Microsoft’s own!
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

}

crate::mod_interface!
{
  protected use glob_is;
  protected use valid_is;
  protected use canonicalize;
}

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
  // It's enough to check if path is valid.
  // https://stackoverflow.com/questions/42283009/check-if-string-is-a-glob-pattern
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

  //

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
}

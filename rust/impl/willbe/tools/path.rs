/// Internal namespace.
pub( crate ) mod private
{
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
}

crate::mod_interface!
{
  prelude use glob_is;
  prelude use valid_is;
}

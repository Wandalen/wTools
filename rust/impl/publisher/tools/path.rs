/// Internal namespace.
pub( crate ) mod private
{
  /// Check if path has a glob.
  pub fn glob_is( path : &str ) -> bool
  {
    path.chars().any( | char | char == '*' || char == '?' || char == '[' )
  }
}

crate::mod_interface!
{
  prelude use glob_is;
}

/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  /// Searches for a README file in specific subdirectories of the given directory path.
  ///
  /// This function attempts to find a README file in the following subdirectories: ".github",
  /// the root directory, and "./docs". It returns the path to the first found README file, or
  /// `None` if no README file is found in any of these locations.
  pub fn readme_path( dir_path : &std::path::Path ) -> Result< std::path::PathBuf, std::io::Error >
  {
    if let Some( path ) = readme_in_dir_find( &dir_path.join( ".github" ) )
    {
      Ok( path )
    }
    else if let Some( path )  = readme_in_dir_find( dir_path )
    {
      Ok( path )
    }
    else if let Some( path )  = readme_in_dir_find( &dir_path.join( "docs" ) )
    {
      Ok( path )
    }
    else
    {
      Err( std::io::Error::new( std::io::ErrorKind::NotFound, format!( "Fail to find README.md at {}", &dir_path.display() ) ) )
    }
  }

  /// Searches for a file named "readme.md" in the specified directory path.
  ///
  /// Given a directory path, this function searches for a file named "readme.md" in the specified
  /// directory.
  fn readme_in_dir_find( path : &std::path::Path ) -> Option< std::path::PathBuf >
  {
    std::fs::read_dir( path )
    .ok()?
    .filter_map( Result::ok )
    .filter( | p | p.path().is_file() )
    .filter_map( | f |
    {
      let l_f = f.file_name().to_ascii_lowercase();
      if l_f == "readme.md"
      {
        return Some( f.file_name() )
      }
      None
    })
    .max()
    .map( std::path::PathBuf::from )
  }

}


crate::mod_interface!
{
  protected use readme_path;
}

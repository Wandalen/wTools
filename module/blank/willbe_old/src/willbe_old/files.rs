/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };

  use path_absolutize::*;

  use iter_tools::Itertools;

  ///
  /// Iterate over unique files in directory using globs 
  ///

  pub fn unique_walk< P, S >( base_dir : P, patterns : &[ S ] ) -> impl Iterator< Item = PathBuf >
  where
    P: AsRef< Path >,
    S: AsRef< str >,
  {
    globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .follow_links( true )
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .filter_map( | s | s.path().absolutize().map( | p | p.to_path_buf() ).ok() )
    .unique()
  }
}

//

crate::mod_interface!
{
  prelude use unique_walk;
}

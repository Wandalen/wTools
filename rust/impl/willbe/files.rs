/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };

  use path_absolutize::*;

  use wtools::Itertools;

  ///
  /// Iterate over unique files in directory using globs 
  ///

  pub fn unique_walk< P, S >( base_dir : P, patterns : &[ S ], depth : std::ops::Range< usize > ) -> impl Iterator< Item = PathBuf >
  where
    P: AsRef< Path >,
    S: AsRef< str >,
  {
    globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .follow_links( true )
    .min_depth( depth.start )
    .max_depth( depth.end )
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .filter_map( | s | s.path().absolutize().map( | p | p.to_path_buf() ).ok() )
    .unique()
  }
}

//

wtools::mod_interface!
{
  prelude use unique_walk;
}

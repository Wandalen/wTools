

/// Internal namespace.
pub( crate ) mod private
{
  use std::path::{ Path, PathBuf };

  ///
  /// Find paths.
  ///

  /* rrr : Dmytro : dubious prototype */
  pub fn find< P, S >( base_dir : P, patterns : &[ S ] ) -> Vec< PathBuf >
  where
    P: AsRef< Path >,
    S: AsRef< str >,
  {
    let paths = globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .follow_links( false )
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .map( | s | s.path().to_path_buf() )
    .collect::< Vec< PathBuf > >();
    paths
  }
}

//

crate::mod_interface!
{
  prelude use find;
}

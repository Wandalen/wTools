
/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use std::path::{ Path, PathBuf };

  ///
  /// Find paths.
  ///

  /* xxx : check */
  pub fn find< P, S >( base_dir : P, patterns : &[ S ] ) -> Vec< PathBuf >
  where
    P : AsRef< Path >,
    S : AsRef< str >,
  {
    globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .follow_links( false )
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .map( | s | s.path().to_path_buf() )
    .collect()
  }

  /// Check if path is valid.
  pub fn valid_is( path : &str ) -> bool
  {
    std::fs::metadata( path ).is_ok()
  }
}

//

crate::mod_interface!
{
  own use valid_is;
  orphan use find;
}

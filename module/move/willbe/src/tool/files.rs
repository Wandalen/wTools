
/// Define a private namespace for all its items.
#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( unused_imports, clippy::wildcard_imports ) ]
  use crate::tool::*;

  use std::path::{ Path, PathBuf };

  ///
  /// Find paths.
  ///
  /// # Panics
  /// qqq: doc

  /* xxx : check */
  #[ allow( clippy::useless_conversion ) ]
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
  #[ must_use ]
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

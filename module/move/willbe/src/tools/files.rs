

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
    globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .follow_links( false )
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .map( | s | s.path().to_path_buf() )
    .collect::< Vec< PathBuf > >()
  }

  /// Searches for files in the specified directory using glob patterns with a depth limit of zero.
  ///
  /// This function utilizes the `globwalk` crate to perform a search for files matching the provided
  /// glob patterns within the specified directory (`base_dir`). The search is restricted to files
  /// directly within the specified directory (depth limit of zero), and symbolic links are not
  /// followed.
  pub fn find_with_depth_zero< P, S >( base_dir : P, patterns : &[ S ] ) -> Vec< PathBuf>
  where
    P: AsRef< Path >,
    S: AsRef< str >,
  {
    globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
    .max_depth(0)
    .follow_links(false)
    .build().unwrap()
    .into_iter()
    .filter_map( Result::ok )
    .map( | s | s.path().to_path_buf() )
    .collect::< Vec< PathBuf > >()
  }

  
}

//

crate::mod_interface!
{
  prelude use find;
}

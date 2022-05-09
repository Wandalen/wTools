#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Operate over files.
//!

use std::path::{ Path, PathBuf };

///
/// Find paths.
///

/* qqq : dubious prototype */
pub fn find<P, S>( base_dir : P, patterns : &[ S ] ) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
  let paths = globwalk::GlobWalkerBuilder::from_patterns( base_dir, patterns )
  .follow_links( false )
  .build().unwrap()
  .into_iter()
  .filter_map( Result::ok )
  .map( | s | s.path().to_path_buf() )
  .collect::<Vec<PathBuf>>();
  paths
}

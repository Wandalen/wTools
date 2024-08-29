/// Internal namespace.
mod private
{
  use crate::prelude::*;
  // use core::fmt::Debug;
  // use core::iter::Iterator;

  ///
  /// Implementation of depth-first search algorithm.
  ///

  pub trait DfsAlgorithm
  where
    Self : NodeBasicInterface,
  {
    // fn dfs( roots : Iterator< IdInterface > )
    // {
    //
    // }
  }

}

//

crate::mod_interface!
{
  prelude use DfsAlgorithm;
}

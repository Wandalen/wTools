/// Internal namespace.
pub( crate ) mod private
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
//
//     fn dfs( roots : Iterator< IdInterface > )
//     {
//
//     }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

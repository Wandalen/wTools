/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt;
  use core::hash::Hash;

  ///
  /// Kind of a edge.
  ///

  pub trait EdgeKindInterface
  where
    Self :
      'static +
      Copy +
      fmt::Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  impl< T > EdgeKindInterface for T
  where
    T :
      'static +
      Copy +
      fmt::Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  ///
  /// No kind for edges.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  pub struct EdgeKindless();

  ///
  /// Edge of a graph.
  ///

  pub trait EdgeBasicInterface
  where
    Self :
      HasId +
  {
  }
}

//

crate::mod_interface!
{
  exposed use EdgeKindless;
  prelude use EdgeKindInterface;
  prelude use EdgeBasicInterface;
}


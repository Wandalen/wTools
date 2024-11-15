/// Define a private namespace for all its items.
mod private
{
  use crate::prelude::*;
  // use core::fmt;
  // use core::hash::Hash;

//   ///
//   /// Kind of a node.
//   ///
//
//   pub trait NodeKindInterface
//   where
//     Self :
//       'static +
//       Copy +
//       fmt::Debug +
//       PartialEq +
//       // Eq +
//       // xxx
//       Hash  +
//       Default +
//     ,
//   {
//   }
//
//   impl< T > NodeKindInterface for T
//   where
//     T :
//       'static +
//       Copy +
//       fmt::Debug +
//       PartialEq +
//       // Eq +
//       Hash  +
//       Default +
//     ,
//   {
//   }

//   ///
//   /// No kind for nodes.
//   ///
//
//   #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash, Default ) ]
//   pub struct NodeKindless();

  ///
  /// Node of a graph.
  ///

  pub trait NodeBasicInterface
  where
    Self :
      HasId +
  {
  }

}

//

crate::mod_interface!
{

  // exposed use NodeKindless;
  prelude use super::private::
  {
    // NodeKindInterface,
    NodeBasicInterface,
  };
}

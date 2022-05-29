/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  use core::fmt;

  ///
  /// Kind of an edge.
  ///

  pub trait EdgeKindInterface
  where
    Self :
      'static +
      Copy +
      fmt::Debug +
      PartialEq +
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
    ,
  {
  }

  ///
  /// No kind for edges.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone, Hash, Default ) ]
  pub struct EdgeKindless();

//   ///
//   /// Edge iterator.
//   ///
//
//   #[ derive( Debug ) ]
//   pub struct EdgesIterator< Edge >
//   where
//     Edge : EdgeInterface,
//   {
//     _p : std::marker::PhantomData< Edge >,
//   }
//
//   impl< Edge > EdgesIterator< Edge >
//   where
//     Edge : EdgeInterface,
//   {
//     pub fn make() -> Self
//     {
//       Self
//       {
//         _p : std::marker::PhantomData,
//       }
//     }
//   }
//
//   impl< Edge > Iterator for EdgesIterator< Edge >
//   where
//     Edge : EdgeInterface,
//   {
//     type Item = Edge;
//     fn next( &mut self ) -> Option< Self::Item >
//     {
//       None
//     }
//   }

  ///
  /// Edge of a graph.
  ///

  pub trait EdgeInterface
  {

    // /// Get kind of the edge.
    // fn kind< Kind : EdgeKindInterface >() -> Kind;

  }

  ///
  /// Get kind of an edge .
  ///

  pub trait EdgeKindGetterInterface< Kind >
  where
    Kind : EdgeKindInterface,
    Self : EdgeInterface,
  {
    /// Get kind of the edge.
    fn kind() -> Kind;
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::private as i;
  pub use super::prelude::*;
  pub use super::private::EdgeKindless;
  // pub use super::private::EdgesIterator;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::private as i;
  pub use super::private::EdgeKindInterface;
  pub use super::private::EdgeInterface;
  pub use super::private::EdgeKindGetterInterface;
}

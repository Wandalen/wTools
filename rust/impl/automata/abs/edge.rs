/// Internal namespace.
mod internal
{
  // use crate::prelude::*;
  use core::fmt::Debug;

  ///
  /// Kind of an edge.
  ///

  pub trait EdgeKindInterface
  where
    Self :
      'static +
      Copy +
      Debug +
      PartialEq +
    ,
  {
  }

  impl< T > EdgeKindInterface for T
  where
    T :
      'static +
      Copy +
      Debug +
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

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  // use super::internal as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use super::prelude::*;
  pub use i::EdgeKindless;
  // pub use i::EdgesIterator;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::EdgeKindInterface;
  pub use i::EdgeInterface;
  pub use i::EdgeKindGetterInterface;
}

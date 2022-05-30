/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;

  ///
  /// Canonical implementation of edge.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone ) ]
  pub struct Edge< 'a, Node, Kind = crate::EdgeKindless >
  where
    Node : NodeBasicInterface,
    Kind : EdgeKindInterface,
  {
    /// Input node.
    pub in_node : &'a Node,
    /// Output node.
    pub out_node : &'a Node,
    /// Kind of the edge.
    pub kind : Kind,
  }

  //

//   impl< 'a, Id, Kind > HasId
//   for Edge< 'a, Id, Kind >
//   where
//     Id : IdentityInterface,
//     Kind : NodeKindInterface,
//   {
//     type Id = Id;
//     fn id( &self ) -> Self::Id
//     {
//       self.name
//     }
//   }
//
//   //
//
//   impl< 'a, Node, Kind > EdgeBasicInterface
//   for Edge< 'a, Node, Kind >
//   where
//     Node : NodeBasicInterface,
//     Kind : EdgeKindInterface,
//   {
//   }

  // impl< 'a, Node, Kind > EdgeInterface
  // for Edge< 'a, Node, Kind >
  // where
  //   Node : NodeBasicInterface,
  //   Kind : EdgeKindInterface,
  // {
  // }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  pub use super::private::Edge;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

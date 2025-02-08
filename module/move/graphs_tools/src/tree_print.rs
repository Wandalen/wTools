
/// Define a private namespace for all its items.
mod private
{

  use crate::*;
  pub use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

  use std::
  {
    hash::Hash,
    fmt,
  };

//   /// Represent directed graph. Can be zero-sized structure if nodes own all the information.
//   pub trait GraphDirected< 'a >
//   {
//     /// Uniquely identify a node.
//     type NodeId : NodeId;
//     /// Node itself.
//     type Node : Node + 'a;
//
//     /// Get a reference on a node by its id.
//     fn node_ref( &'a self, node_id : Self::NodeId ) -> &'a Self::Node;
//     /// Get id by its node reference.
//     fn node_id( &self, node_id : &'a Self::Node ) -> Self::NodeId;
//
//     /// Iterate over out nodes of
//     fn node_out_nodes( &'a self, node_id : Self::NodeId ) -> BoxedIter< 'a, Self::NodeId >;
//
//   }

  /// Print directed graph as a tree.
  pub trait GraphDirectedPrintAsTree< 'g >
  where
    Self : abs::GraphDirected< 'g >,
  {

    /// Print directed graph as a tree.
    fn _print_as_tree< 'w >( &self, write : &'w mut ( dyn core::fmt::Write + 'w ), node : Self::NodeId ) -> fmt::Result
    {
      // let node = self.node_ref( node );
      write.write_fmt( format_args!( "{:?}", node ) )
    }

    /// Print directed graph as a tree.
    fn print_as_tree< 'w >( &self, node : Self::NodeId ) -> String
    {
      // let node = self.node_ref( node );
      let mut result = String::new();
      self._print_as_tree( &mut result, node ).unwrap();
      result
    }

  }

  impl< 'g, T > GraphDirectedPrintAsTree< 'g > for T
  where
    Self : abs::GraphDirected< 'g >,
  {
  }

  // impl fmt::Debug for Context< '_ >
  // {
  //   fn fmt( &self, c : &mut fmt::Formatter< '_ > ) -> fmt::Result
  //   {
  //     c
  //     .debug_struct( "Context" )
  //     .field( "buf", &"dyn fmt::Write" )
  //     .field( "printer", &self.printer )
  //     .finish()
  //   }
  // }

}

crate::mod_interface!
{
  own use
  {
    GraphDirectedPrintAsTree,
  };
}

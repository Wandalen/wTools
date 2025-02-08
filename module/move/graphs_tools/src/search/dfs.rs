//! Depth first search method.

mod private
{
  use crate::*;
  use search::{ Method, ForGraphDirected, Options };

  /// Depth-first search method.
  #[ derive( Debug, Default ) ]
  pub struct Dfs;

  impl Method for Dfs
  {
    type ExtraOptions = ();

    /// Perform depth-first search on a graph.
    fn _search< 'a, Graph, Visit >
    (
      graph : &'a Graph,
      mut o : Options< 'a, Self, Graph, Visit >,
    )
    where
      Visit : FnMut( &'a Graph::Node ),
      Graph : ForGraphDirected< 'a > + ?Sized,
    {
      let mut visited = collection_tools::HashSet::new();
      let mut stack = collection_tools::Vec::new();
      stack.push( o.start_id );

      while let Some( node_id ) = stack.pop()
      {
        let node = graph.node_ref( node_id );
        if visited.insert( node_id )
        {
          ( o.visit )( node );
          for child_id in graph.node_out_nodes( node_id )
          {
            stack.push( child_id );
          }
        }
      }
    }

  }

}

crate::mod_interface!
{
  orphan use
  {
    Dfs,
  };
}

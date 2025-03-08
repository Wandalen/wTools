//! Breadth  first search method.

mod private
{
  use crate::*;
  use search::{ Method, ForGraphDirected, Options, OnVisit };

  /// Breadth-first search strategy.
  #[ derive( Debug, Default ) ]
  pub struct Bfs;

  impl Method for Bfs
  {
    type ExtraOptions = ();

    /// Perform breadth-first search on a graph.
    fn _search< 'a, Graph, PreVisit, PostVisit >
    (
      graph : &'a Graph,
      mut o : Options< 'a, Self, Graph, PreVisit, PostVisit >,
    )
    where
      // PreVisit : FnMut( &'a Graph::Node ),
      // PostVisit : FnMut( &'a Graph::Node ),
      PreVisit : OnVisit< 'a, Graph::Node >,
      PostVisit : OnVisit< 'a, Graph::Node >,
      Graph : ForGraphDirected< 'a > + ?Sized,
    {
      let mut visited = collection_tools::HashSet::new();
      let mut queue = collection_tools::VecDeque::new();
      queue.push_back( o.start_id );

      while let Some( node_id ) = queue.pop_front()
      {
        let node = graph.node_ref( node_id );
        if visited.insert( node_id )
        {
          o.pre_visit.call( node );
          for child_id in graph.node_out_nodes( node_id )
          {
            queue.push_back( child_id );
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
    Bfs,
  };
}

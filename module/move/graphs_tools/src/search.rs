mod private
{
  use crate::*;

  /// Former of Options for searching.
  pub fn options< 'a, Method, Graph, Visit >() -> OptionsFormer< 'a, Method, Graph, Visit >
  where
    Graph : crate::abs::GraphDirected< 'a > + ?Sized,
    Visit : FnMut( &'a Graph::Node ),
    Method : super::Method,
  {
    Options::former()
  }

  /// Options for configuring a graph search.
  #[ derive( Debug, Default, Former ) ]
  pub struct Options< 'a, Method, Graph, Visit >
  where
    Graph : crate::abs::GraphDirected< 'a > + ?Sized,
    Visit : FnMut( &'a Graph::Node ),
    Method : super::Method,
  {
    /// Starting node ID for the search.
    pub start_id : Graph::NodeId,
    /// Function to call on each visited node.
    pub visit : Visit,
    /// Method of searhcing.
    pub method : Method,
    /// Additional options specific to the search method.
    pub _extra : Method::ExtraOptions,
    /// Phantom data to associate types and lifetimes.
    pub _phantom : std::marker::PhantomData< ( &'a (), ) >,
  }

  impl< 'a, Method, Graph, Visit > Options< 'a, Method, Graph, Visit >
  where
    Graph : ForGraphDirected< 'a > + ?Sized,
    Visit : FnMut( &'a Graph::Node ),
    Method : super::Method,
  {
    pub fn search( self, graph : &'a Graph )
    {
      graph.search( self )
    }
  }

  // xxx : adjust Former to eliminate need in this
  impl< 'a, Method, Graph, Visit > OptionsFormer< 'a, Method, Graph, Visit >
  where
    Graph : ForGraphDirected< 'a > + ?Sized,
    Visit : FnMut( &'a Graph::Node ),
    Method : super::Method,
  {
    pub fn visit_set( mut self, visit : Visit ) -> Self
    {
      self.storage.visit = Some( visit );
      self
    }
    pub fn method_set( mut self, method : Method ) -> Self
    {
      self.storage.method = Some( method );
      self
    }
  }

  /// Trait for performing searches on directed graphs.
  pub trait ForGraphDirected< 'a > : crate::abs::GraphDirected< 'a >
  {
    /// Perform a search using specified options and method.
    fn search< Visit, Method >
    (
      &'a self,
      o : Options< 'a, Method, Self, Visit >,
    )
    where
      Visit : FnMut( &'a Self::Node ),
      Method : super::Method,
    {
      Method::_search( self, o )
    }
  }

  impl< 'a, T > ForGraphDirected< 'a > for T
  where
    T : crate::abs::GraphDirected< 'a >,
  {
  }

  /// Trait for defining specific search strategies like DFS or BFS.
  pub trait Method : Default
  {
    /// Additional options for the search method.
    type ExtraOptions : Default;

    /// Execute the search on a graph.
    fn _search< 'a, Graph, Visit >
    (
      graph : &'a Graph,
      o : Options< 'a, Self, Graph, Visit >,
    )
    where
      Visit : FnMut( &'a Graph::Node ),
      Graph : ForGraphDirected< 'a > + ?Sized,
      Self : Sized;
  }

  /// Depth-first search strategy.
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

  /// Breadth-first search strategy.
  #[ derive( Debug, Default ) ]
  pub struct Bfs;

  impl Method for Bfs
  {
    type ExtraOptions = ();

    /// Perform breadth-first search on a graph.
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
      let mut queue = collection_tools::VecDeque::new();
      queue.push_back( o.start_id );

      while let Some( node_id ) = queue.pop_front()
      {
        let node = graph.node_ref( node_id );
        if visited.insert( node_id )
        {
          ( o.visit )( node );
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
  own use
  {
    options,
    Method,
    Options,
    ForGraphDirected,
    Dfs,
    Bfs,
  };
}

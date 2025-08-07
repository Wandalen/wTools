mod private
{
  use crate::*;

  /// Former of Options for searching.
  pub fn options< 'a, Method, Graph, PreVisit, PostVisit >() -> OptionsFormer< 'a, Method, Graph, PreVisit, PostVisit >
  where
    Graph : crate::abs::GraphDirected< 'a > + ?Sized,
    Method : super::Method,
    PreVisit : OnVisit< 'a, Graph::Node >,
    PostVisit : OnVisit< 'a, Graph::Node >,
  {
    Options::former()
  }

  /// Options for configuring a graph search.
  #[ derive( Debug, Default, Former ) ]
  pub struct Options< 'a, Method, Graph, PreVisit = NopVisit, PostVisit = NopVisit >
  where
    Graph : crate::abs::GraphDirected< 'a > + ?Sized,
    Method : super::Method,
    PreVisit : OnVisit< 'a, Graph::Node >,
    PostVisit : OnVisit< 'a, Graph::Node >,
  {
    /// Starting node ID for the search.
    pub start_id : Graph::NodeId,

    /// Function to call on each pre-order visit of node.
    pub pre_visit : PreVisit,
    /// Function to call on each post-order visit of node.
    pub post_visit : PostVisit,

    /// Method of searhcing.
    pub method : Method,
    /// Additional options specific to the search method.
    pub _extra : Method::ExtraOptions,
    /// Phantom data to associate types and lifetimes.
    pub _phantom : std::marker::PhantomData< ( &'a (), ) >,
  }

  impl< 'a, Method, Graph, PreVisit, PostVisit > Options< 'a, Method, Graph, PreVisit, PostVisit >
  where
    Graph : ForGraphDirected< 'a > + ?Sized,
    Method : super::Method,
    PreVisit : OnVisit< 'a, Graph::Node >,
    PostVisit : OnVisit< 'a, Graph::Node >,
  {
    /// Search traversing each node in an order specified by method.
    pub fn search( self, graph : &'a Graph )
    {
      graph.search( self )
    }
  }

  // xxx : adjust Former to eliminate need in this
  impl< 'a, Method, Graph, PreVisit, PostVisit > OptionsFormer< 'a, Method, Graph, PreVisit, PostVisit >
  where
    Graph : ForGraphDirected< 'a > + ?Sized,
    Method : super::Method,
    PreVisit : OnVisit< 'a, Graph::Node >,
    PostVisit : OnVisit< 'a, Graph::Node >,
  {

    pub fn pre_visit_set( mut self, pre_visit : PreVisit ) -> Self
    {
      self.storage.pre_visit = Some( pre_visit );
      self
    }

    pub fn post_visit_set( mut self, post_visit : PostVisit ) -> Self
    {
      self.storage.post_visit = Some( post_visit );
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
    fn search< Method, PreVisit, PostVisit >
    (
      &'a self,
      o : Options< 'a, Method, Self, PreVisit, PostVisit >,
    )
    where
      Method : super::Method,
      PreVisit : OnVisit< 'a, Self::Node >,
      PostVisit : OnVisit< 'a, Self::Node >,
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
    fn _search< 'a, Graph, PreVisit, PostVisit >
    (
      graph : &'a Graph,
      o : Options< 'a, Self, Graph, PreVisit, PostVisit >,
    )
    where
      PreVisit : OnVisit< 'a, Graph::Node >,
      PostVisit : OnVisit< 'a, Graph::Node >,
      Graph : ForGraphDirected< 'a > + ?Sized,
      Self : Sized;
  }

  /// A function to call on visit, either pre-order or post-order.
  pub trait OnVisit< 'a, Node >
  {
    /// Call itself.
    fn call( &mut self, node : &'a Node );
  }

  /// No-op visit
  #[ derive( Debug, Default ) ]
  pub struct NopVisit;
  impl< 'a, Node > OnVisit< 'a, Node > for NopVisit
  {
    fn call( &mut self, _node : &'a Node )
    {
    }
  }

  impl< 'a, Node, F > OnVisit< 'a, Node > for F
  where
    Node : 'a,
    F : FnMut( &'a Node ),
  {
    fn call( &mut self, node : &'a Node )
    {
      self( node );
    }
  }

}

crate::mod_interface!
{
  layer
  {
    dfs,
    bfs,
  };
  own use
  {
    options,
    Method,
    Options,
    ForGraphDirected,
    OnVisit,
    NopVisit
  };
}

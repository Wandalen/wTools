use super::*;

// #[ path = "../graph.rs" ]
// mod graph;

use graph::map_of_nodes::
{
  Node, NodeId, Graph,
};

// use derive_tools::From;
// use the_module::abs;
// use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

// #[ derive( Debug ) ]
// struct Node< 'a >
// {
//   id : NodeId,
//   children : Vec< &'a Node< 'a > >,
// }
//
// impl< 'a > the_module::abs::Node for Node< 'a > {}
//
// impl< 'a > Node< 'a >
// {
//   fn new< IntoId : Into< NodeId > >( id : IntoId ) -> Node< 'a >
//   {
//     Node
//     {
//       id : id.into(),
//       children : Vec::new(),
//     }
//   }
//
//   fn child_add( &mut self, child : &'a Node< 'a > ) -> &mut Self
//   {
//     self.children.push( child );
//     self
//   }
// }
//
// #[ derive( Default ) ]
// struct Graph< 'a >
// {
//   nodes : HashMap< NodeId, &'a Node< 'a > >,
// }
//
// impl< 'a > Graph< 'a >
// {
//
//   // fn new() -> Graph< 'a >
//   // {
//   //   Graph
//   //   {
//   //     nodes : HashMap::new(),
//   //   }
//   // }
//
//   fn add_node( &mut self, node : &'a Node< 'a > )
//   {
//     self.nodes.insert( node.id, node );
//   }
//
// }
//
// impl< 'a > abs::GraphDirected< 'a > for Graph< 'a >
// {
//
//   type NodeId = NodeId;
//   type Node = Node< 'a >;
//
//   fn node_ref( &self, node_id : NodeId ) -> &'a Node< 'a >
//   {
//     self.nodes.get( &node_id ).expect( "If id exist then node shoudl also exist" )
//   }
//
//   fn node_id( &self, node : &'a Node< 'a > ) -> NodeId
//   {
//     node.id
//   }
//
//   fn node_out_nodes( &self, node_id : NodeId ) -> BoxedIter< 'a, Self::NodeId >
//   {
//     if let Some( node ) = self.nodes.get( &node_id )
//     {
//       Box::new( node.children.iter().map( | child | child.id ) )
//     }
//     else
//     {
//       Box::new( std::iter::empty() )
//     }
//   }
// }
//
// #[ derive( Debug, Copy, Clone, Hash, PartialEq, Eq, From ) ]
// struct NodeId( usize );
//
// impl the_module::abs::NodeId for NodeId {}

// =

#[ test ]
fn test_dfs()
{
  // use the_module::search;
  // use the_module::abs;
  use the_module::search::{ ForGraphDirected, PassVisit };
  let graph = Graph::triplet_with_double_legs();

  // Prepare a vector to collect visited nodes
  let mut pre_visited_nodes = Vec::new();
  let pre_visit = | node : &Node |
  {
    pre_visited_nodes.push( node.id );
    println!( "pre visiting {:?}", node.id );
  };

  let mut post_visited_nodes = Vec::new();
  let post_visit = | node : &Node |
  {
    post_visited_nodes.push( node.id );
    println!( "post visiting {:?}", node.id );
  };

  // // Create search options
  // let search_options = the_module::search::Options
  // {
  //   start_id : NodeId( 1 ),
  //   visit,
  //   method : search::Dfs,
  //   // ..Default::default()
  //   _extra : (),
  //   _phantom : Default::default(),
  // };

//   // Create search options
//   let search_options = the_module::search::options()
//   .start_id( 1 )
//   .visit_set( visit )
//   .method_set( search::Dfs )
//   .form()
//   ;
//
//   // Perform DFS
//   graph.search( search_options );
//
//   // Assert the order of visited nodes
//   assert_eq!( visited_nodes, vec![ NodeId( 1 ), NodeId( 4 ), NodeId( 3 ), NodeId( 2 ) ] );

  // Create search options
  the_module::search::options()
  .start_id( 0 )
  .pre_visit_set( pre_visit )
  .post_visit_set( post_visit )
  .method_set( the_module::search::Dfs )
  .form()
  .search( &graph )
  ;

  // Assert the order of visited nodes
  assert_eq!( pre_visited_nodes, into_vec![ 0, 1, 4, 5, 2, 3, 6, 7 ] );
  assert_eq!( post_visited_nodes, into_vec![ 4, 5, 1, 2, 6, 7, 3, 0 ] );

  // node::0
  // ├─ node::1
  // │  ├─ node::4
  // │  ├─ node::5
  // ├─ node::2
  // ├─ node::3
  // │  ├─ node::6
  // │  ├─ node::7

}

// xxx
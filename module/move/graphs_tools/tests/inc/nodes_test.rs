// use super::*;
//
// use derive_tools::From;
//
// #[ derive( Debug ) ]
// struct Node< 'a >
// {
//   id : NodeId,
//   children : Vec< &'a Node< 'a > >,
// }
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
// struct Graph< 'a >
// {
//   nodes : HashMap< NodeId, &'a Node< 'a > >,
// }
//
// impl< 'a > Graph< 'a >
// {
//   fn new() -> Graph< 'a >
//   {
//     Graph
//     {
//       nodes : HashMap::new(),
//     }
//   }
//
//   fn add_node( &mut self, node : &'a Node< 'a > )
//   {
//     self.nodes.insert( node.id, node );
//   }
//
//   fn node_ref( &self, node_id : NodeId ) -> Option< &'a Node< 'a > >
//   {
//     self.nodes.get( &node_id ).copied()
//   }
//
//   fn node_id( node : &'a Node< 'a > ) -> NodeId
//   {
//     node.id
//   }
//
//   fn node_out_nodes( &self, node_id : NodeId ) -> Box< dyn Iterator< Item = NodeId > + 'a >
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
//
// #[ test ]
// fn basic()
// {
//
//   // test
//
//   let mut node1 = Node::new( NodeId( 1 ) );
//   let node2 = Node::new( NodeId( 2 ) );
//   let node3 = Node::new( NodeId( 3 ) );
//   let node4 = Node::new( NodeId( 4 ) );
//
//   node1
//   .child_add( &node2 )
//   .child_add( &node3 )
//   .child_add( &node4 );
//
//   let mut graph = Graph::new();
//   graph.add_node( &node1 );
//   graph.add_node( &node2 );
//   graph.add_node( &node3 );
//   graph.add_node( &node4 );
//
//   // Assert that the root node is correctly retrieved
//   assert_eq!( graph.node_ref( NodeId( 1 ) ).unwrap().id, NodeId( 1 ) );
//
//   // Assert that the root node has the correct children
//   let out_nodes : Vec< NodeId > = graph.node_out_nodes( NodeId( 1 ) ).collect();
//   assert_eq!( out_nodes, vec![ NodeId( 2 ), NodeId( 3 ), NodeId( 4 ) ] );
//
//   // Print statements for debugging
//   println!( "{:?}", graph.node_ref( NodeId( 1 ) ) );
//   println!( "{:?}", out_nodes );
//
//   // Assert that the root node structure is as expected
//   assert_eq!( node1.id, NodeId( 1 ) );
//   assert_eq!( node1.children.len(), 3 );
//   assert_eq!( node1.children[ 0 ].id, NodeId( 2 ) );
//   assert_eq!( node1.children[ 1 ].id, NodeId( 3 ) );
//   assert_eq!( node1.children[ 2 ].id, NodeId( 4 ) );
//
//   println!( "{:?}", node1 );
// }

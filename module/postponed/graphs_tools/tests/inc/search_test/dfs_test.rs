use super :: *;

// #[ path = "../graph.rs" ]
// mod graph;

use graph ::map_of_nodes ::
{
  Node, NodeId, Graph,
};

// =

#[ test ]
fn test_dfs_manual()
{
  // use the_module ::search;
  // use the_module ::abs;
  use the_module ::search :: { ForGraphDirected, NopVisit };
  let graph = Graph ::triplet_with_double_legs();

  // Prepare a vector to collect visited nodes
  let mut pre_visited_nodes = Vec ::new();
  let pre_visit = | node: &Node |
  {
  pre_visited_nodes.push( node.id );
  println!( "pre visiting {:?}", node.id );
 };

  let mut post_visited_nodes = Vec ::new();
  let post_visit = | node: &Node |
  {
  post_visited_nodes.push( node.id );
  println!( "post visiting {:?}", node.id );
 };

  // Create search options
  let search_options = the_module ::search ::Options
  {
  start_id: 0.into(),
  pre_visit,
  post_visit,
  method: the_module ::search ::Dfs,
  _extra: (),
  _phantom: Default ::default(),
 };

  // Perform DFS
  graph.search( search_options );

  // Assert the order of visited nodes
  assert_eq!( pre_visited_nodes, into_vec![ 0, 1, 4, 5, 2, 3, 6, 7 ] );
  assert_eq!( post_visited_nodes, into_vec![ 4, 5, 1, 2, 6, 7, 3, 0 ] );

}

// =

#[ test ]
fn test_dfs()
{
  // use the_module ::search;
  // use the_module ::abs;
  use the_module ::search :: { ForGraphDirected, NopVisit };
  let graph = Graph ::triplet_with_double_legs();

  // Prepare a vector to collect visited nodes
  let mut pre_visited_nodes = Vec ::new();
  let pre_visit = | node: &Node |
  {
  pre_visited_nodes.push( node.id );
  println!( "pre visiting {:?}", node.id );
 };

  let mut post_visited_nodes = Vec ::new();
  let post_visit = | node: &Node |
  {
  post_visited_nodes.push( node.id );
  println!( "post visiting {:?}", node.id );
 };

  // Create search options
  the_module ::search ::options()
  .start_id( 0 )
  .pre_visit_set( pre_visit )
  .post_visit_set( post_visit )
  .method_set( the_module ::search ::Dfs )
  .form()
  .search( &graph )
  ;

  // Assert the order of visited nodes
  assert_eq!( pre_visited_nodes, into_vec![ 0, 1, 4, 5, 2, 3, 6, 7 ] );
  assert_eq!( post_visited_nodes, into_vec![ 4, 5, 1, 2, 6, 7, 3, 0 ] );

  // node :: 0
  // ├─ node :: 1
  // │  ├─ node :: 4
  // │  ├─ node :: 5
  // ├─ node :: 2
  // ├─ node :: 3
  // │  ├─ node :: 6
  // │  ├─ node :: 7

}

// xxx
//! Depth first search method.

mod private
{
  use crate :: *;
  use search :: { Method, ForGraphDirected, Options, OnVisit };

  /// Depth-first search method.
  #[ derive( Debug, Default ) ]
  pub struct Dfs;

  impl Method for Dfs
  {
  type ExtraOptions = ();

// node :: 0
// ├─ node :: 1
// │  ├─ node :: 4
// │  ├─ node :: 5
// ├─ node :: 2
// ├─ node :: 3
// │  ├─ node :: 6
// │  ├─ node :: 7

  /// Perform depth-first search on a graph.
  fn _search< 'a, Graph, PreVisit, PostVisit >
  (
   graph: &'a Graph,
   mut o: Options< 'a, Self, Graph, PreVisit, PostVisit >,
 )
  where
   PreVisit: OnVisit< 'a, Graph ::Node >,
   PostVisit: OnVisit< 'a, Graph ::Node >,
   Graph: ForGraphDirected< 'a > + ?Sized,
  {
   let mut visited = collection_tools ::HashSet ::new();
   let mut stack = collection_tools ::Vec ::new();
   stack.push( ( o.start_id, true ) );

   // while let Some( node_id ) = stack.pop()
   while let Some( ( node_id, is_preorder ) ) = stack.pop()
   {
  let node = graph.node_ref( node_id );

  if !is_preorder
  {
   o.post_visit.call( node );
   continue;
 }

  if visited.insert( node_id )
  {
   stack.push( ( node_id, false ) );
   o.pre_visit.call( node );
   for child_id in graph.node_out_nodes( node_id ).rev()
   {
  // o.post_visit.call( node );
  stack.push( ( child_id, true ) );
 }
 }
 }
 }

 }

}

crate ::mod_interface!
{
  orphan use
  {
  Dfs,
 };
}

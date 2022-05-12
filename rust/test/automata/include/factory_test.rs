use std::collections::HashSet;
use test_tools::*;
use wtools::prelude::*;

//

fn node_test()
{
  use wautomata::*;

  let mut factory = wautomata::canonical::NodeFactory::make();

  let n1 = factory.node_making( 1 );
  let n1b = factory.node( 1 );
  assert_eq!( n1, n1b.id() );
  dbg!( &n1 );

  let node1a = factory.node( 1 );
  let node1b = factory.node( 1 );
  assert_eq!( node1a, node1b );

  let node1a = factory.node( &1 );
  let node1b = factory.node( &&1 );
  assert_eq!( node1a, node1b );

}

//

fn basic_test()
{
  use wautomata::*;

  let mut factory = wautomata::canonical::NodeFactory::make();

  let a = factory.node_making( 1 );
  let b = factory.node_making( 2 );

  factory.node_extend_out_node( a, b );
  factory.node_extend_out_nodes( b, [ a, b ].into_iter() );

  dbg!( factory.node( a ) );
  dbg!( factory.node( b ) );

  let exp = hset![ b ];
  let got : HashSet< _ > = factory.out_nodes( a ).collect();
  assert_eq!( got, exp );

  let exp = hset![ a, b ];
  let got : HashSet< _ > = factory.out_nodes( b ).collect();
  assert_eq!( got, exp );

}

//

fn make_edge_list_test()
{
  use wautomata::*;

  let mut factory = wautomata::canonical::NodeFactory::make();

  factory.make_edge_list
  ([
    1, 2,
    2, 1,
    2, 2,
  ])
  ;

  dbg!( factory.node( 1 ) );
  dbg!( factory.node( 2 ) );

  let exp = hset![ 2 ];
  let got : HashSet< _ > = factory.out_nodes( 1 ).collect();
  assert_eq!( got, exp );

  let exp = hset![ 1, 2 ];
  let got : HashSet< _ > = factory.out_nodes( 2 ).collect();
  assert_eq!( got, exp );

}

//

test_suite!
{
  node,
  basic,
  make_edge_list,
}

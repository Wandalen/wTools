use std::collections::HashSet;
use test_tools::*;
use wtools::prelude::*;

//

fn basic_test()
{
  use wautomata::*;

  let mut factory = wautomata::canonical::CellNodeFactory::make();

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

test_suite!
{
  basic,
}

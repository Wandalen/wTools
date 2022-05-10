use std::collections::HashSet;
use test_tools::*;
use wtools::prelude::*;

//

fn basic_test()
{
  use wautomata::*;

  let mut factory = wautomata::canonical::NodeFactory::make();

  let a = factory.node_making_id( "a" );
  let b = factory.node_making_id( "b" );

  // dbg!( &a );
  // dbg!( &b );

  factory.node( a ).borrow_mut().extend( core::iter::once( b ) );
  factory.node( b ).borrow_mut().extend( [ a, b ].into_iter() );

  dbg!( factory.node( a ) );
  dbg!( factory.node( b ) );

  let exp = vec![ b ];
  let got : Vec< _ > = factory.node( a ).borrow().out_nodes().collect();
  assert_eq!( got, exp );

  let exp = hset![ a, b ];
  let got : HashSet< _ > = factory.node( b ).borrow().out_nodes().collect();
  assert_eq!( got, exp );

  // a.borrow_mut().extend( core::iter::once( b.borrow().id() ) );
  // b.borrow_mut().extend( core::iter::once( a.borrow().id() ) );

  // dbg!( &a );
  // dbg!( &b );

}

//

test_suite!
{
  basic,
}

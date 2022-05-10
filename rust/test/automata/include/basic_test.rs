use test_tools::*;

//

fn basic_test()
{
  use wautomata::*;
  // use wautomata::NodeFactory;

  // let node1 = wautomata::Node::make_labeled();

  let mut factory = wautomata::canonical::NodeFactory::make();

  let a = factory.node_making_id( "a" );
  let b = factory.node_making_id( "b" );

  // dbg!( &a );
  // dbg!( &b );

  factory.node( a ).borrow_mut().extend( core::iter::once( b ) );
  factory.node( b ).borrow_mut().extend( core::iter::once( a ) );

  dbg!( factory.node( a ) );
  dbg!( factory.node( b ) );

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

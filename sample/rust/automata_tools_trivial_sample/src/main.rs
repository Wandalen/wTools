
fn main()
{
  use automata_tools::prelude::*;
  use wtools::prelude::*;
  let node : automata_tools::canonical::Node = make!( 13 );
  assert_eq!( node.id(), 13.into() );
  println!( "{:?}", node );
  /* print : node::13 */
}


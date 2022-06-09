
fn main()
{
  #[ cfg( all( feature = "cell_factory", feature = "use_std" ) ) ]
  {
    use graphs_tools::prelude::*;
    use wtools::prelude::*;
    let node : graphs_tools::canonical::Node = make!( 13 );
    assert_eq!( node.id(), 13.into() );
    println!( "{:?}", node );
    /* print : node::13 */
  }
}


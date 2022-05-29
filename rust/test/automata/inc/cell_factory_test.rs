use super::*;
use TheModule::canonical::CellNodeFactory as TheFactory;
include!( "./factory_impls.rs" );

tests_impls!
{

  fn nodecell_make()
  {
    use TheModule::prelude::*;

    let node : TheModule::canonical::Node = make!( 13 );
    a_id!( node.id(), 13.into() );
    let cellnode : < TheModule::canonical::CellNodeFactory as GraphBasicInterface >::NodeHandle = make!( node );

  }

}

//

tests_index!
{

  node,
  basic,
  make_default,
  make_with_edge_list,
  make_with_edge_list_string,
  graph_print,

  nodecell_make,

}

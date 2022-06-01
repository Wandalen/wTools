use super::*;

tests_impls!
{

  fn node_make()
  {
    use TheModule::prelude::*;

    let node : TheModule::canonical::Node = make!( 13 );
    a_id!( node.id(), 13.into() );

  }

  fn nodecell_make()
  {
    use TheModule::prelude::*;

    let node : TheModule::canonical::Node = make!( 13 );
    a_id!( node.id(), 13.into() );
    let cellnode : TheModule::NodeCell< _ > = make!( node );

  }

}

//

tests_index!
{

  node_make,
  nodecell_make,

}

use super::*;
use TheModule::canonical::NodeFactory as TheFactory;
include!( "./factory_impl.rs" );

//

tests_index!
{
  node,
  basic,
  make_default,
  make_with_edge_list,
  make_with_edge_list_string,
  graph_print,
}

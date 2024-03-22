use super::*;
use the_module::canonical::ReadableNodeFactory as ReadableNodeFactory;
use the_module::canonical::GenerativeNodeFactory as GenerativeNodeFactory;

include!( "./factory_impls.rs" );

//

tests_index!
{
  // node,
  // basic,
  // make_default,
  // make_with_edge_list,
  // make_with_edge_list_string,
  // graph_print,
}

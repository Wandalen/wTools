
use super::*;

mod_interface!
{
  #![ debug ]

  /// layer_a
  layer layer_a;

}

//

include!( "../../only_test/layer_single_only_test.rs" );

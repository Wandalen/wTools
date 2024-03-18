
use super::*;

/// Layer X
pub mod layer_x;

// the_module::mod_interface!
// {
//   #![ debug ]
//
//   /// layer_a
//   use super::layer_x as layer_a;
// }

include!( "./manual_only.rs" );

//

include!( "../../only_test/layer_single_only_test.rs" );

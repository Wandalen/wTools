
use super::*;

/// Layer X
pub mod layer_x;

mod private {}

the_module::mod_interface!
{
  // #![ debug ]

  /// layer_a
  use super::layer_x as layer_a;

  // /// layer_a
  // pub use super::layer_x as layer_a;
  // zzz : make that working

}

// include!( "./manual_only.rs" );

//

include!( "../../only_test/layer_single_only_test.rs" );

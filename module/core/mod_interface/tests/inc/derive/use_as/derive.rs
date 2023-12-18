
use super::*;

/// Layer X
pub mod layer_x;

mod_interface!
{
  // #![ debug ]

  /// layer_a
  use super::layer_x as layer_a;

  // /// layer_a
  // pub use super::layer_x as layer_a;
  // xxx : make that working

}

// include!( "./manual_only.rs" );

//

include!( "../../only_test/layer_single_only_test.rs" );

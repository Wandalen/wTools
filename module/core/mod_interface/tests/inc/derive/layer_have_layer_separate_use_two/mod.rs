
use super::*;
mod tools
{
  #[ allow( unused_imports ) ]
  pub use super::super::*;
}

/// Private namespace of the module.
mod private
{
}

/// layer_a
pub mod layer_a;
/// layer_b
pub mod layer_b;

the_module::mod_interface!
{

  // zzz : test with `layer { layer_a, layer_a };`
  // zzz : test with `use { layer_a, layer_a };`

  // zzz : make it working
  // use super::
  // {
  //   layer_a,
  //   layer_b,
  // };

  use super::layer_a;
  use super::layer_b;

}

mod mod1
{

  // use super::{ layer_b };
  // pub use super::{ layer_b }::orphan::*;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );

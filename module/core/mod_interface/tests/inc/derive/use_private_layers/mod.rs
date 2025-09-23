#![allow(dead_code)]
#![allow(clippy ::doc_markdown)]
#![allow(unused_imports)]

use super :: *;
use test_tools ::a_id;

// private layer
mod layer_a;
// private layer
mod layer_b;

mod private {}

// xxx: qqq: make it working

// the_module ::mod_interface!
// {
//
//   /// layer_a
//   priv use super ::layer_a;
//
//   /// layer_b
//   priv use super ::layer_b;
//
// }
//
// //
//
// include!( "../../only_test/layer_simple_only_test.rs" );

//! This example demonstrates how to use the `mod_interface` crate to organize a Rust program into structured namespaces. The code is divided into a library file (`child.rs`) and a main function. The library file defines a module with private functions and uses the `mod_interface` macro to specify which functions should be exposed in different namespaces. The main function then tests the visibility and accessibility of these functions.

use mod_interface::mod_interface;

/// Children.
pub mod child;

// Priave namespaces is necessary.
mod private {}

crate::mod_interface!
{
  /// Inner.
  use super::child;
}


fn main()
{

  assert!( child::prelude_thing(), "prelude thing of child is there" );
  assert!( prelude_thing(), "and here" );
  assert!( own::prelude_thing(), "and here" );
  assert!( orphan::prelude_thing(), "and here" );
  assert!( exposed::prelude_thing(), "and here" );
  assert!( prelude::prelude_thing(), "and here" );

  assert!( child::exposed_thing(), "exposed thing of child is there" );
  assert!( exposed_thing(), "and here" );
  assert!( own::exposed_thing(), "and here" );
  assert!( orphan::exposed_thing(), "and here" );
  assert!( exposed::exposed_thing(), "and here" );
  // assert!( prelude::exposed_thing(), "but not here" );

  assert!( child::orphan_thing(), "orphan thing of child is there" );
  assert!( orphan_thing(), "orphan thing of child is here" );
  assert!( own::orphan_thing(), "and here" );
  // assert!( orphan::orphan_thing(), "but not here" );
  // assert!( exposed::orphan_thing(), "and not here" );
  // assert!( prelude::orphan_thing(), "and not here" );

  assert!( child::my_thing(), "own thing of child is only there" );
  // assert!( my_thing(), "and not here" );
  // assert!( own::my_thing(), "and not here" );
  // assert!( orphan::my_thing(), "and not here" );
  // assert!( exposed::my_thing(), "and not here" );
  // assert!( prelude::my_thing(), "and not here" );

}

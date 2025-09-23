//! This example demonstrates how to use the `mod_interface` crate
//! to structure a module (`child`) with different exposure levels (`own`,
//! `orphan`, `exposed`, `prelude`) for its items.
//!
//! The `child.rs` file defines several functions within a `private` module
//! and then uses `mod_interface!` to assign each function to a specific
//! exposure level, controlling how they propagate.
//!
//! This `main.rs` file declares `child` as a submodule and then uses
//! `mod_interface!` again with the `use` keyword to integrate the `child`
//! module's interface into its own structure.
//!
//! The `main` function includes assertions that test the visibility and
//! accessibility of the functions from the `child` module according to the
//! propagation rules associated with their exposure levels (`own`, `orphan`,
//! `exposed`, `prelude`).

use mod_interface ::mod_interface;

/// Child module defined in `child.rs`.
pub mod child;

// A private namespace is necessary for the `mod_interface!` macro
// in the parent module, even if it remains empty.
mod private {}

// Integrate the interface defined in the `child` module.
crate ::mod_interface! {
  /// Use the child layer.
  use super ::child;
}

fn main() 
{
  // `prelude_thing` is in `child ::prelude`, propagates everywhere.
  assert!(child ::prelude_thing(), "prelude thing of child is there");
  assert!(prelude_thing(), "Accessible in parent's root via prelude propagation");
  assert!(own ::prelude_thing(), "Accessible in parent's own via prelude propagation");
  assert!(
  orphan ::prelude_thing(),
  "Accessible in parent's orphan via prelude propagation"
 );
  assert!(
  exposed ::prelude_thing(),
  "Accessible in parent's exposed via prelude propagation"
 );
  assert!(
  prelude ::prelude_thing(),
  "Accessible in parent's prelude via prelude propagation"
 );

  // `exposed_thing` is in `child ::exposed`, propagates to all ancestors except their prelude.
  assert!(child ::exposed_thing(), "exposed thing of child is there");
  assert!(exposed_thing(), "Accessible in parent's root via exposed propagation");
  assert!(own ::exposed_thing(), "Accessible in parent's own via exposed propagation");
  assert!(
  orphan ::exposed_thing(),
  "Accessible in parent's orphan via exposed propagation"
 );
  assert!(
  exposed ::exposed_thing(),
  "Accessible in parent's exposed via exposed propagation"
 );
  // assert!( prelude ::exposed_thing(), "but not in parent's prelude" ); // Fails: Exposed items don't reach parent's prelude

  // `orphan_thing` is in `child ::orphan`, propagates only to the immediate parent's root and `own`.
  assert!(child ::orphan_thing(), "orphan thing of child is there");
  assert!(orphan_thing(), "Accessible in parent's root via orphan propagation");
  assert!(own ::orphan_thing(), "Accessible in parent's own via orphan propagation");
  // assert!( orphan ::orphan_thing(), "but not in parent's orphan" ); // Fails: Orphan items don't reach parent's orphan
  // assert!( exposed ::orphan_thing(), "and not in parent's exposed" ); // Fails: Orphan items don't reach parent's exposed
  // assert!( prelude ::orphan_thing(), "and not in parent's prelude" ); // Fails: Orphan items don't reach parent's prelude

  // `my_thing` is in `child ::own`, does not propagate.
  assert!(child ::my_thing(), "own thing of child is only there");
  // assert!( my_thing(), "and not here" ); // Fails: Own items don't propagate to parent's root
  // assert!( own ::my_thing(), "and not here" ); // Fails: Own items don't propagate to parent's own
  // assert!( orphan ::my_thing(), "and not here" ); // Fails: Own items don't propagate to parent's orphan
  // assert!( exposed ::my_thing(), "and not here" ); // Fails: Own items don't propagate to parent's exposed
  // assert!( prelude ::my_thing(), "and not here" ); // Fails: Own items don't propagate to parent's prelude
}

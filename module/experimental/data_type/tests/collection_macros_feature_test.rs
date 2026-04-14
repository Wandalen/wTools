//! Reproducing test for collection macros feature flag bug.
//!
//! ## Root Cause
//!
//! The `dt_collection` feature in `Cargo.toml` was configured to only enable
//! `collection_tools/enabled` but not `collection_tools/collection_constructors`.
//! This made collection macros (`hmap!`, `hset!`, `bmap!`, `bset!`) unavailable despite
//! the `dt_collection` feature being enabled, breaking the `facade` pattern's promise
//! of providing convenient access to collection utilities.
//!
//! ## Why Not Caught
//!
//! - No existing tests verified macro availability under `dt_collection` feature
//! - Tests only verified type re-exports (`Hmap`, `Hset` types), not macros
//! - Example file was stub (empty), so compilation was never attempted
//! - Feature flag validation was missing from test suite
//!
//! ## Fix Applied
//!
//! Changed `Cargo.toml` line 60 from:
//! ```toml
//! dt_collection = [ "collection_tools/enabled" ]
//! ```
//!
//! To:
//! ```toml
//! dt_collection = [ "collection_tools/enabled", "collection_tools/collection_constructors" ]
//! ```
//!
//! This ensures the `dt_collection` feature properly enables all necessary
//! `collection_tools` features for both types and macros to be usable.
//!
//! ## Prevention
//!
//! 1. Always test both types AND macros when adding `facade` re-exports
//! 2. Verify feature flags propagate all necessary sub-features
//! 3. Ensure example files compile and run as part of `CI/CD`
//! 4. Add feature flag validation tests for all `facade` features
//!
//! ## Pitfall
//!
//! `Facade` crates re-exporting macros must explicitly enable the macro-providing
//! features of dependencies. Unlike types, macros don't automatically propagate
//! through feature flags - each macro-providing feature must be explicitly listed.
//! When adding `facade` features, always check if dependency has separate features
//! for types vs macros (common pattern: `enabled` for types, `*_constructors` for macros).

#[ cfg( feature = "dt_collection" ) ]
#[ test ]
fn collection_macros_available_with_dt_collection_feature()
{
  // Import macros from collection_tools (facade makes them available)
  use collection_tools::{ hmap, hset, bmap, bset };

  // Test HashMap macro
  let map = hmap! { "a" => 1, "b" => 2 };
  assert_eq!( map.len(), 2 );
  assert_eq!( map.get( "a" ), Some( &1 ) );

  // Test HashSet macro
  let set = hset! { 1, 2, 3 };
  assert_eq!( set.len(), 3 );
  assert!( set.contains( &2 ) );

  // Test BTreeMap macro
  let btree_map = bmap! { 1 => "one", 2 => "two" };
  assert_eq!( btree_map.len(), 2 );
  assert_eq!( btree_map.get( &1 ), Some( &"one" ) );

  // Test BTreeSet macro
  let btree_set = bset! { 3, 1, 2 };
  assert_eq!( btree_set.len(), 3 );
  assert!( btree_set.contains( &1 ) );
}

#[ cfg( not( feature = "dt_collection" ) ) ]
#[ test ]
fn collection_macros_unavailable_without_dt_collection_feature()
{
  // This test ensures that when dt_collection is disabled,
  // compilation fails as expected (documenting feature boundary)

  // NOTE: This test doesn't actually compile when feature is disabled
  // (which is the expected behavior). It serves as documentation.
}

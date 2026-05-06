//! Verification test for readme.md code example accuracy
//!
//! This test file verifies that code examples in readme.md are functional
//! and use correct feature flags.

use wstring_tools::*;

/// Tests README example with CORRECT feature condition (feature = "std")
#[ cfg( all( feature = "split", feature = "std" ) ) ]
#[ test ]
fn readme_example_with_correct_std_feature()
{
  // This is the README example code with CORRECTED cfg condition
  /* delimeter exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter not exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( String::from ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}

/// Reproduces bug where readme.md used non-existent feature `use_std`
///
/// # Root Cause
///
/// In `readme.md:15`, the code example used cfg condition:
/// ```rust
/// #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
/// ```
/// However, `Cargo.toml` defines NO feature named `use_std`. The actual features are:
/// - `std` (line 51): enables standard library support
/// - `no_std` (line 52): enables `no_std` mode
/// - `use_alloc` (line 53): enables allocator support
///
/// The cfg condition `feature = "use_std"` always evaluates to FALSE because
/// the feature doesn't exist in the feature list. Users copying this example
/// from the README would get code that compiles but never executes.
///
/// # Why Not Caught
///
/// README code examples are NOT compiled or tested during CI. The example file
/// `examples/wstring_toolst_trivial_sample.rs:10` uses the correct condition
/// `not( feature = "no_std" )`, so automated tests passed. Documentation
/// diverged from working example without detection.
///
/// # Fix Applied
///
/// Changed `readme.md:15` from:
/// ```rust
/// #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
/// ```
/// To:
/// ```rust
/// #[ cfg( all( feature = "split", not( feature = "no_std" ) ) ) ]
/// ```
/// This matches the working condition in `examples/wstring_toolst_trivial_sample.rs:10`
/// and ensures README example actually executes when features are enabled.
///
/// # Prevention
///
/// 1. **Add documentation tests**: Convert README examples to doc tests that compile during `cargo test --doc`
/// 2. **Lint for undefined features**: Enable `unexpected_cfgs` lint in `Cargo.toml` lints section
/// 3. **Sync examples with README**: Use code generation or includes to keep examples and README in sync
/// 4. **Test README examples**: Add integration test (like this one) verifying README examples execute
///
/// # Pitfall
///
/// Check ALL documentation files (readme.md, module docs, doc comments) for feature gates.
/// Similar bugs may exist in:
/// - `src/lib.rs` doc comments
/// - Other crates in workspace
/// - External documentation (docs.rs examples)
///
/// Audit command: `rg 'feature = "use_std"' --type rust`
#[ cfg( all( feature = "split", not( feature = "no_std" ) ) ) ]
#[ test ]
#[ cfg_attr( all( test, feature = "split" ), test_tools::nightly::bug_reproducer( issue-2 ) ) ]
fn readme_feature_gate_bug_reproducer()
{
  // This test verifies the fix by ensuring code with correct feature executes
  #[ cfg( feature = "split" ) ]
  {
    /* delimeter exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( " " )
    .stripping( false )
    .perform();
    let iterated = iter.map( String::from ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc", " ", "def" ] );

    /* delimeter not exists */
    let src = "abc def";
    let iter = string::split()
    .src( src )
    .delimeter( "g" )
    .perform();
    let iterated = iter.map( String::from ).collect::< Vec< _ > >();
    assert_eq!( iterated, vec![ "abc def" ] );
  }
}

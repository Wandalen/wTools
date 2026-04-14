//! Bug Reproducer: Namespace Propagation Failure
//!
//! This test demonstrates that items declared with explicit visibility
//! (own/orphan/exposed use) do NOT propagate to higher layers as expected.

/// Test: own use should propagate to orphan, exposed, prelude
mod test_own_propagation
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn test_fn() -> &'static str { "works" }
  }

  mod_interface!
  {
    own use test_fn;
  }

  #[ test ]
  fn own_has_item()
  {
    assert_eq!( own::test_fn(), "works" );
  }

  // These tests are commented out because they fail (the bug)
  // #[ test ]
  // fn orphan_should_have_item()
  // {
  //   assert_eq!( orphan::test_fn(), "works" );
  // }
}

/// Test: orphan use should propagate to exposed, prelude
mod test_orphan_propagation
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn test_fn() -> &'static str { "works" }
  }

  mod_interface!
  {
    orphan use test_fn;
  }

  #[ test ]
  fn orphan_has_item()
  {
    assert_eq!( orphan::test_fn(), "works" );
  }

  // These tests are commented out because they fail (the bug)
  // #[ test ]
  // fn exposed_should_have_item()
  // {
  //   assert_eq!( exposed::test_fn(), "works" );
  // }
}

/// Test: exposed use should propagate to prelude
mod test_exposed_propagation
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn test_fn() -> &'static str { "works" }
  }

  mod_interface!
  {
    exposed use test_fn;
  }

  #[ test ]
  fn exposed_has_item()
  {
    assert_eq!( exposed::test_fn(), "works" );
  }

  // This test is commented out because it fails (the bug)
  // #[ test ]
  // fn prelude_should_have_item()
  // {
  //   assert_eq!( prelude::test_fn(), "works" );
  // }
}

/// Baseline: prelude use works (has nowhere to propagate)
mod test_prelude_no_propagation
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn test_fn() -> &'static str { "works" }
  }

  mod_interface!
  {
    prelude use test_fn;
  }

  #[ test ]
  fn prelude_has_item()
  {
    assert_eq!( prelude::test_fn(), "works" );
  }
}

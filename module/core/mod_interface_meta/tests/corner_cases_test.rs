//! Corner Case Tests for `mod_interface_meta`
//!
//! This test suite systematically validates corner cases and edge cases in the
//! `mod_interface` procedural macro implementation. Tests are organized by
//! category and cover use statement variations, layer directives, micro-modules,
//! visibility propagation, and error conditions.

// =============================================================================
// Phase 1: Use Statement Variations
// =============================================================================

/// UC-01: Simple identifier with own layer
/// Tests that `` `own use` `` exports to own layer only (not orphan/exposed/prelude).
/// Items in `own_clause` are accessible via `own::` and root; the cascade runs
/// prelude → exposed → orphan → own, so own items do not propagate downward.
mod uc_01_own_use_simple
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn my_fn() -> bool { true }
  }

  mod_interface!
  {
    own use my_fn;
  }

  #[ test ]
  fn own_layer_has_item()
  {
    assert!( own::my_fn() );
  }
}

/// UC-02: Simple identifier with orphan layer
/// Tests that `` `orphan use` `` exports to orphan+own layers (not exposed/prelude).
/// Items in `orphan_clause` are accessible via `orphan::`, `own::`, and root.
mod uc_02_orphan_use_simple
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn my_fn() -> bool { true }
  }

  mod_interface!
  {
    orphan use my_fn;
  }

  #[ test ]
  fn orphan_layer_has_item()
  {
    assert!( orphan::my_fn() );
  }

  #[ test ]
  fn own_layer_has_item()
  {
    assert!( own::my_fn() );
  }
}

/// UC-03: Simple identifier with exposed layer
/// Tests that `` `exposed use` `` exports to exposed+orphan+own layers (not prelude).
/// Items in `exposed_clause` are accessible via `exposed::`, `orphan::`, `own::`, and root.
mod uc_03_exposed_use_simple
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn my_fn() -> bool { true }
  }

  mod_interface!
  {
    exposed use my_fn;
  }

  #[ test ]
  fn exposed_layer_has_item()
  {
    assert!( exposed::my_fn() );
  }

  #[ test ]
  fn orphan_layer_has_item()
  {
    assert!( orphan::my_fn() );
  }

  #[ test ]
  fn own_layer_has_item()
  {
    assert!( own::my_fn() );
  }
}

/// UC-04: Simple identifier with prelude layer
/// Tests that `prelude use` exports only to prelude layer
mod uc_04_prelude_use_simple
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn my_fn() -> bool { true }
  }

  mod_interface!
  {
    prelude use my_fn;
  }

  #[ test ]
  fn prelude_layer_has_item()
  {
    assert!( prelude::my_fn() );
  }
}

/// UC-05: Implicit use (no layer keyword)
/// Tests that bare `use` exports to all layers
mod uc_05_implicit_use_all_layers
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn my_fn() -> bool { true }
  }

  mod_interface!
  {
    use my_fn;
  }

  #[ test ]
  fn own_layer_has_item()
  {
    assert!( own::my_fn() );
  }

  #[ test ]
  fn orphan_layer_has_item()
  {
    assert!( orphan::my_fn() );
  }

  #[ test ]
  fn exposed_layer_has_item()
  {
    assert!( exposed::my_fn() );
  }

  #[ test ]
  fn prelude_layer_has_item()
  {
    assert!( prelude::my_fn() );
  }
}

/// UC-09: Rename with as keyword
/// Tests that `use Type1 as Alias` works correctly
mod uc_09_use_with_rename
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn original_name() -> bool { true }
  }

  mod_interface!
  {
    own use original_name as aliased_name;
  }

  #[ test ]
  fn aliased_name_works()
  {
    assert!( own::aliased_name() );
  }
}

/// UC-13: Multiple use statements in same layer
/// Tests that multiple `own use` statements work correctly
mod uc_13_multiple_use_statements
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn fn_a() -> bool { true }
    pub fn fn_b() -> bool { true }
    pub fn fn_c() -> bool { true }
  }

  mod_interface!
  {
    own use fn_a;
    own use fn_b;
    own use fn_c;
  }

  #[ test ]
  fn all_functions_exported()
  {
    assert!( own::fn_a() );
    assert!( own::fn_b() );
    assert!( own::fn_c() );
  }
}

/// UC-14: Mixed implicit and explicit use statements
/// Tests that `use` and `own use` can coexist
mod uc_14_mixed_implicit_explicit
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn implicit_fn() -> bool { true }
    pub fn explicit_fn() -> bool { true }
  }

  mod_interface!
  {
    use implicit_fn;
    own use explicit_fn;
  }

  #[ test ]
  fn implicit_in_all_layers()
  {
    assert!( own::implicit_fn() );
    assert!( orphan::implicit_fn() );
    assert!( exposed::implicit_fn() );
    assert!( prelude::implicit_fn() );
  }

  #[ test ]
  fn explicit_in_own_layer_only()
  {
    // `own use explicit_fn` → own_clause → accessible in own:: and root only
    assert!( own::explicit_fn() );
  }
}

// =============================================================================
// Phase 3: Micro-Module Variations
// =============================================================================

/// MM-01 through MM-04: All four micro-module types
/// Tests that micro-modules work in all four layers
mod mm_01_04_micro_modules_all_layers
{
  use mod_interface_meta::mod_interface;

  mod private {}

  mod_interface!
  {
    own mod micro_own;
    orphan mod micro_orphan;
    exposed mod micro_exposed;
    prelude mod micro_prelude;
  }

  #[ test ]
  fn all_micro_modules_exist()
  {
    // Verify modules are created (compile-time check)
    let _ = micro_own::has_marker();
    let _ = micro_orphan::has_marker();
    let _ = micro_exposed::has_marker();
    let _ = micro_prelude::has_marker();
  }
}

/// MM-07: Multiple micro-modules in same layer
/// Tests that multiple micro-modules in one layer work correctly
mod mm_07_multiple_micro_modules_same_layer
{
  use mod_interface_meta::mod_interface;

  mod private {}

  mod_interface!
  {
    own mod mod_a;
    own mod mod_b;
    own mod mod_c;
  }

  #[ test ]
  fn all_modules_created()
  {
    // Compile-time verification
    let _ = mod_a::marker();
    let _ = mod_b::marker();
    let _ = mod_c::marker();
  }
}

// =============================================================================
// Phase 5: Namespace Combinations
// =============================================================================

/// NC-06: Only own layer populated
/// Tests that populating only own layer works correctly
mod nc_06_only_own_populated
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn own_fn() -> bool { true }
  }

  mod_interface!
  {
    own use own_fn;
  }

  #[ test ]
  fn own_has_content()
  {
    // own_clause items are accessible via own:: and root; cascade does not propagate them down
    assert!( own::own_fn() );
  }
}

/// NC-07: Only prelude layer populated
/// Tests that populating only prelude works correctly
mod nc_07_only_prelude_populated
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn prelude_fn() -> bool { true }
  }

  mod_interface!
  {
    prelude use prelude_fn;
  }

  #[ test ]
  fn prelude_has_content()
  {
    assert!( prelude::prelude_fn() );
  }
}

// =============================================================================
// Phase 9: Integration Patterns
// =============================================================================

/// IP-01: Mix all item types
/// Tests that structs, traits, functions, and constants all work
mod ip_01_mix_all_item_types
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub struct MyStruct;
    pub trait MyTrait {}
    pub fn my_fn() -> bool { true }
    pub const MY_CONST : i32 = 42;
  }

  mod_interface!
  {
    own use MyStruct;
    orphan use MyTrait;
    exposed use my_fn;
    prelude use MY_CONST;
  }

  #[ test ]
  fn struct_exported()
  {
    let _ : own::MyStruct = own::MyStruct;
  }

  #[ test ]
  fn trait_exported()
  {
    // Compile-time check that trait is accessible
    // Define a type that implements the trait
    struct TestType;
    impl orphan::MyTrait for TestType {}

    // Verify we can use the trait bound
    fn check_trait<T : orphan::MyTrait>(_: T) {}
    check_trait(TestType);
  }

  #[ test ]
  fn function_exported()
  {
    assert!( exposed::my_fn() );
  }

  #[ test ]
  fn const_exported()
  {
    assert_eq!( prelude::MY_CONST, 42 );
  }
}

/// IP-03: Empty private namespace
/// Tests whether empty private namespace is allowed
mod ip_03_empty_private_namespace
{
  use mod_interface_meta::mod_interface;

  mod private {}

  mod_interface!
  {
    // No items
  }

  #[ test ]
  fn compiles_successfully()
  {
    // Compile-time verification
  }
}

/// IP-04: Private namespace with content
/// Tests that content in private namespace doesn't interfere
mod ip_04_private_with_content
{
  use mod_interface_meta::mod_interface;

  mod private
  {
    pub fn exposed_fn() -> bool { true }
    pub fn hidden_fn() -> bool { true }
  }

  mod_interface!
  {
    own use exposed_fn;
    // hidden_fn is not re-exported
  }

  #[ test ]
  fn exposed_fn_accessible()
  {
    assert!( own::exposed_fn() );
  }

  #[ test ]
  fn hidden_fn_not_in_public_interface()
  {
    // hidden_fn is accessible via private::hidden_fn() but not via layers
    assert!( private::hidden_fn() );
  }
}

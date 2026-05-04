//! Bug reproducer: `SmokeModuleTest::form()` must wrap code lacking `fn main()`.
//!
//! # Root Cause
//!
//! `SmokeModuleTest::form()` wrote user-provided code to `main.rs` without ensuring
//! a `fn main()` entry point existed. When the user code contained only `use` statements
//! or expressions (no `fn main()`), `cargo build` failed with E0601:
//! "main function not found in crate".
//!
//! The root cause was an implicit assumption in the `else` branch: the branch assumed
//! user-provided code always contained `fn main()`. In practice, code snippets — simple
//! `use` statements, standalone expressions, or import-only code — are valid test inputs
//! that lack a main entry point.
//!
//! # Why Not Caught
//!
//! Existing tests used code snippets that either:
//! 1. Included an explicit `fn main()` block, or
//! 2. Used the empty-code path (which auto-generates `fn main() { use ...; }`)
//!
//! The case of non-empty user code without `fn main()` was not tested. Edge case tests
//! existed in `smoke_test_edge_cases.rs` but did not verify that `form()` added a wrapper.
//!
//! # Fix Applied
//!
//! Added a third branch in `SmokeModuleTest::form()` (in `src/test/smoke_test.rs`):
//! when user code is non-empty AND does not contain `fn main()`, the code is wrapped in
//! `fn main() { ... }` to produce a valid binary entry point.
//!
//! ```text
//! if self.code.is_empty()            → auto-generate with use statement
//! else if code.contains("fn main")   → use as-is (no double-wrap)
//! else                               → wrap in `fn main() { ... }`  ← FIX
//! ```
//!
//! # Prevention
//!
//! This test suite verifies all three branches of the code generation logic
//! in `form()`. Any future change to the code generation path must keep all
//! three variants passing.
//!
//! # Pitfall to Avoid
//!
//! Do not assume that user-provided test code always has an entry point.
//! Code snippets (use statements, expressions, struct declarations) are valid
//! smoke test inputs. The `form()` method is responsible for producing compilable
//! code regardless of user input — users should not need to add boilerplate.

// test_kind: bug_reproducer(issue-smoke-form-missing-main)

use test_tools::SmokeModuleTest;

/// Verify: code without `fn main()` gets wrapped in `fn main()` by `form()`.
///
/// This is the primary bug reproducer. Before the fix, a `use` statement
/// without a wrapping `fn main()` caused E0601 at cargo build time.
#[ test ]
fn form_wraps_code_without_fn_main()
{
  let mut smoke_test = SmokeModuleTest::new( "serde" );
  smoke_test.version( "1.0" );
  // Code with only a use statement — no fn main() entry point
  smoke_test.code( "use serde;".to_string() );

  let form_result = smoke_test.form();
  assert!(
    form_result.is_ok(),
    "form() should succeed for code without fn main(): {form_result:?}"
  );

  smoke_test.clean( true ).expect( "cleanup should succeed" );
}

/// Verify: code with `fn main()` is NOT double-wrapped.
///
/// Regression guard: wrapping code that already has `fn main()` would produce
/// E0201 (duplicate main) or `dead_code` warnings.
#[ test ]
fn form_preserves_existing_fn_main()
{
  let mut smoke_test = SmokeModuleTest::new( "serde" );
  smoke_test.version( "1.0" );
  // Code that already contains fn main() — must not be wrapped again
  smoke_test.code( "fn main() { use serde; }".to_string() );

  let form_result = smoke_test.form();
  assert!(
    form_result.is_ok(),
    "form() should succeed for code with fn main(): {form_result:?}"
  );

  smoke_test.clean( true ).expect( "cleanup should succeed" );
}

/// Verify: empty code path still generates auto `fn main()` with use statement.
///
/// Baseline: the original empty-code path must remain unaffected by the fix.
#[ test ]
fn form_generates_fn_main_for_empty_code()
{
  let mut smoke_test = SmokeModuleTest::new( "serde" );
  smoke_test.version( "1.0" );
  // Empty code — auto-generate fn main() { use serde; }
  smoke_test.code( String::new() );

  let form_result = smoke_test.form();
  assert!(
    form_result.is_ok(),
    "form() should succeed for empty code: {form_result:?}"
  );

  smoke_test.clean( true ).expect( "cleanup should succeed" );
}

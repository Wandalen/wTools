//!
//! Bug reproducer test for missing Seed import in non-deterministic mode.
//!
//! ## Root Cause
//!
//! The `hrng_non_deterministic` module's `private` namespace (line 9-12) did not import the
//! `Seed` type from the crate root, but the `master_with_seed()` function at line 114 used
//! `Seed` in its signature. When the `determinism` feature was disabled (making
//! `hrng_non_deterministic` the active implementation), compilation failed with:
//!
//! ```text
//! error[E0412]: cannot find type `Seed` in this scope
//!    --> src/hrng_non_deterministic.rs:114:30
//!     |
//! 114 |   pub fn master_with_seed(_: Seed) -> Self
//!     |                              ^^^^
//! ```
//!
//! The `hrng_deterministic` module had the correct import pattern (`use crate::*;` at line 12),
//! but this pattern was not replicated in the non-deterministic variant. The `Seed` type is
//! defined in `src/seed.rs` and conditionally exported via `lib.rs` when `not(feature = "no_std")`.
//!
//! ## Why Not Caught
//!
//! 1. **Default feature masking**: The `determinism` feature is enabled by default in `Cargo.toml`,
//!    meaning normal `cargo test` runs used `hrng_deterministic` (which has correct imports) and
//!    never exercised the non-deterministic code path.
//!
//! 2. **Missing feature matrix testing**: No CI/test configuration explicitly tested the
//!    `--no-default-features` build, which would have caught this immediately.
//!
//! 3. **Doc test scope limitation**: The doc test in `hrng_non_deterministic.rs:105-111` includes
//!    a usage example of `master_with_seed()`, but doc tests are compiled in a different context
//!    where the import requirements may differ, failing to catch the missing import in the actual
//!    module implementation.
//!
//! 4. **No integration test for non-deterministic mode**: All existing integration tests in
//!    `tests/basic_test.rs` are feature-gated with `#[cfg(feature = "determinism")]`, leaving
//!    the non-deterministic mode untested.
//!
//! ## Fix Applied
//!
//! Added the missing import to `src/hrng_non_deterministic.rs` at line 13-14:
//!
//! ```rust
//! mod private
//! {
//!   use core :: { ops ::Deref, ops ::DerefMut };
//!   #[ cfg(not(feature = "no_std")) ]
//!   use crate ::Seed;  // Added this line
//! ```
//!
//! The import is feature-gated with `#[cfg(not(feature = "no_std"))]` to match the feature gate
//! on the `master_with_seed()` function, ensuring the import is only present when the `Seed`
//! type is actually available (seed module is conditionally compiled in lib.rs:46 with the same gate).
//!
//! ## Prevention
//!
//! 1. **Feature matrix testing**: Add CI jobs that test with `--no-default-features` and all
//!    feature permutations to catch feature-gated code paths.
//!
//! 2. **Integration tests for both modes**: Create tests that explicitly verify both deterministic
//!    and non-deterministic modes work correctly, not just feature-gated tests for one mode.
//!
//! 3. **Shared test fixtures**: When dual implementations exist (deterministic/non-deterministic),
//!    create shared test fixtures that both implementations must pass, ensuring API parity.
//!
//! 4. **Import consistency linting**: Consider using workspace-level lints or custom tooling to
//!    ensure symmetric modules (like `hrng_deterministic` and `hrng_non_deterministic`) have
//!    consistent import patterns.
//!
//! ## Pitfall
//!
//! **Dual implementation symmetry assumption**: Developers may assume that if one implementation
//! compiles, the mirror implementation will also compile, especially when feature flags swap
//! between them. This is false - each feature-gated code path must be independently verified.
//!
//! **Doc test false confidence**: The presence of a passing doc test for `master_with_seed()`
//! in the non-deterministic module gave false confidence that the code was correct. Doc tests
//! compile in a different module context (`#[doc]`) and may have different import resolution
//! than the actual implementation.
//!
//! **Default feature masking**: Default-enabled features effectively hide code paths from
//! standard development workflows (`cargo test`, `cargo build`). Critical code paths should
//! not be behind default-disabled features unless intentionally opt-in.
//!

#![allow(missing_docs)]

#[ test ]
#[ cfg(not(feature = "no_std")) ]
fn bug_reproducer_seed_import_non_deterministic()
{
  // This test verifies that the Seed type is accessible in non-deterministic mode.
  // It reproduces the original compilation error that occurred when building with
  // --no-default-features (which disables determinism and uses hrng_non_deterministic).

  use deterministic_rand :: { Hrng, Rng };

  // This line would fail to compile before the fix because hrng_non_deterministic
  // didn't import the Seed type, even though master_with_seed() used it.
  let hrng = Hrng ::master_with_seed("test_seed".into());

  let rng_ref = hrng.rng_ref();
  let mut rng = rng_ref.lock().unwrap();
  let _got: u64 = rng.gen();

  // In non-deterministic mode, we can't assert a specific value,
  // but we can verify the code compiles and runs without panic.
  #[ cfg(not(feature = "determinism")) ]
  {
    // Verify we can generate multiple values without panic
    let _v1: u64 = rng.gen();
    let _v2: u64 = rng.gen();
  }

  // In deterministic mode, verify we still get deterministic values
  #[ cfg(feature = "determinism") ]
  {
    // With the same seed, we should get deterministic values
    let hrng2 = Hrng ::master_with_seed("test_seed".into());
    let rng_ref2 = hrng2.rng_ref();
    let mut rng2 = rng_ref2.lock().unwrap();
    let got2: u64 = rng2.gen();

    // Verify determinism by creating another generator with the same seed
    let hrng3 = Hrng ::master_with_seed("test_seed".into());
    let rng_ref3 = hrng3.rng_ref();
    let mut rng3 = rng_ref3.lock().unwrap();
    let got3: u64 = rng3.gen();

    assert_eq!(got2, got3, "Same seed should produce same first value");
  }
}

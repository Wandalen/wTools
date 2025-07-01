# Task Plan: Refactor `pth` for `no_std` compatibility

### Goal
*   Refactor the `pth` crate to be fully compatible with `no_std` environments by replacing `std` types and functionalities with `alloc` or `core` equivalents, and conditionally compiling `std`-dependent code. The crate must compile successfully with `cargo check -p pth --features "no_std"`.

### Ubiquitous Language (Vocabulary)
*   **`pth`:** The crate to be refactored for `no_std` compatibility.
*   **`no_std`:** A Rust compilation mode where the standard library is not available.
*   **`alloc`:** The Rust allocation library, available in `no_std` environments when an allocator is provided.
*   **`core`:** The most fundamental Rust library, always available in `no_std` environments.
*   **`std-only`:** Code that depends on the standard library and must be conditionally compiled.

### Progress
*   **Roadmap Milestone:** M0: Foundational `no_std` compatibility
*   **Primary Editable Crate:** `module/core/pth`
*   **Overall Progress:** 1/4 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Setup `no_std` foundation and dependencies
    *   ⚫ Increment 2: Replace `std` types with `core` and `alloc` equivalents
    *   ⚫ Increment 3: Conditionally compile all `std`-only APIs
    *   ⚫ Increment 4: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:** N/A

### Relevant Context
*   Control Files to Reference:
    *   `module/core/pth/spec.md`
*   Files to Include:
    *   `module/core/pth/Cargo.toml`
    *   `module/core/pth/src/lib.rs`
    *   `module/core/pth/src/as_path.rs`
    *   `module/core/pth/src/try_into_path.rs`
    *   `module/core/pth/src/try_into_cow_path.rs`
    *   `module/core/pth/src/transitive.rs`
    *   `module/core/pth/src/path.rs`
    *   `module/core/pth/src/path/joining.rs`
    *   `module/core/pth/src/path/absolute_path.rs`
    *   `module/core/pth/src/path/canonical_path.rs`
    *   `module/core/pth/src/path/native_path.rs`
    *   `module/core/pth/src/path/current_path.rs`

### Expected Behavior Rules / Specifications
*   The `pth` crate must compile successfully in a `no_std` environment (`cargo check -p pth --features "no_std"`).
*   All `std::` imports must be replaced with `alloc::` or `core::` equivalents, or be conditionally compiled under `#[cfg(not(feature = "no_std"))]`.
*   Functionality dependent on `std::env` or `std::io` that cannot be replicated in `no_std` must be conditionally compiled.
*   Existing functionality under the `default` features must not be broken.

### Crate Conformance Check Procedure
*   **Step 1: Run `no_std` build check.** Execute `timeout 90 cargo check -p pth --features "no_std"`. If this fails, fix the errors before proceeding.
*   **Step 2: Run `std` build check.** Execute `timeout 90 cargo check -p pth`. If this fails, fix the errors before proceeding.
*   **Step 3: Run Tests (Conditional).** Only if Steps 1 and 2 pass, execute `timeout 90 cargo test -p pth --all-targets`. If this fails, fix all test errors before proceeding.
*   **Step 4: Run Linter (Conditional).** Only if Step 3 passes, execute `timeout 120 cargo clippy -p pth --all-features -- -D warnings`.

### Increments
##### Increment 1: Setup `no_std` foundation and dependencies
*   **Goal:** Configure `Cargo.toml` and `lib.rs` to correctly handle the `no_std` feature and its dependencies.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: In `module/core/pth/Cargo.toml`, modify the `regex` dependency to disable its default features, making it `no_std` compatible.
    *   Step 2: In `module/core/pth/src/lib.rs`, add the `#[cfg(feature = "no_std")] #[macro_use] extern crate alloc;` attribute to make the `alloc` crate available for `no_std` builds.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p pth`. This should pass.
    *   Execute `timeout 90 cargo check -p pth --features "no_std"`. This is expected to fail, but we will proceed to the next increment to fix the errors.
*   **Commit Message:** `feat(pth): setup no_std foundation and dependencies`

##### Increment 2: Replace `std` types with `core` and `alloc` equivalents
*   **Goal:** Systematically replace all `std` types that have `core` or `alloc` counterparts across the entire crate.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: In all relevant `.rs` files (`as_path.rs`, `try_into_path.rs`, `try_into_cow_path.rs`, `transitive.rs`, `path.rs`, `path/*.rs`), add `#[cfg(feature = "no_std")] extern crate alloc;` where needed.
    *   Step 2: In the same files, replace `use std::` with `use core::` for modules like `fmt`, `ops`, `hash`, and `cmp`.
    *   Step 3: In the same files, replace `std::string::String` with `alloc::string::String`, `std::vec::Vec` with `alloc::vec::Vec`, and `std::borrow::Cow` with `alloc::borrow::Cow`.
    *   Step 4: Add `allow` attributes for `clippy::std_instead_of_alloc` and `clippy::std_instead_of_core` at the crate level in `lib.rs` to manage warnings during the transition.
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p pth --features "no_std"`. The number of errors should be significantly reduced.
    *   Execute `timeout 90 cargo check -p pth`. This should still pass.
*   **Commit Message:** `refactor(pth): replace std types with core and alloc equivalents`

##### Increment 3: Conditionally compile all `std`-only APIs
*   **Goal:** Isolate and gate all functionality that depends on `std`-only modules like `std::io` and `std::env`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: In `path/current_path.rs`, wrap the entire module content in `#[cfg(not(feature = "no_std"))]`.
    *   Step 2: In `path.rs`, `path/absolute_path.rs`, `path/canonical_path.rs`, and `path/native_path.rs`, find all functions and `impl` blocks that use `std::io`, `std::env`, or `path::canonicalize`.
    *   Step 3: Wrap these identified functions and `impl` blocks with the `#[cfg(not(feature = "no_std"))]` attribute.
    *   Step 4: In `lib.rs` and `path.rs`, update the `mod_interface!` declarations to conditionally export the gated modules and layers (e.g., `#[cfg(not(feature = "no_std"))] layer current_path;`).
    *   Step 5: Perform Increment Verification.
    *   Step 6: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p pth --features "no_std"`. This should now pass.
    *   Execute `timeout 90 cargo check -p pth`. This should also pass.
*   **Commit Message:** `refactor(pth): conditionally compile all std-only APIs`

##### Increment 4: Finalization
*   **Goal:** Perform a final, holistic review, run all checks, and ensure the crate is clean and correct.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Perform a self-critique of all changes against the requirements.
    *   Step 2: Run the full `Crate Conformance Check Procedure`, including `clippy` and `test`.
    *   Step 3: Remove any temporary `allow` attributes or comments added during the refactoring.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p pth --features "no_std"`. Must pass.
    *   Execute `timeout 90 cargo check -p pth`. Must pass.
    *   Execute `timeout 90 cargo test -p pth --all-targets`. Must pass.
    *   Execute `timeout 120 cargo clippy -p pth --all-features -- -D warnings`. Must pass.
*   **Commit Message:** `chore(pth): finalize no_std refactoring`

### Task Requirements
*   The `pth` crate must be fully `no_std` compatible.
*   All `std` dependencies must be removed or conditionally compiled.

### Project Requirements
*   (Inherited from workspace `Cargo.toml`)

### Assumptions
*   `alloc` is available in `no_std` environments.
*   `camino` and `serde` crates are `no_std` compatible or can be conditionally compiled as needed.

### Out of Scope
*   Adding `no_std` specific tests. The focus is on making the code compile.
*   Implementing new features in `pth`.

### External System Dependencies
*   N/A

### Notes & Insights
*   This plan prioritizes broad, sweeping changes by concern, which is more efficient for this type of refactoring.
*   The key challenge is correctly identifying and gating all code that relies on the standard library's IO and environment capabilities.

### Changelog
*   [Initial] Plan created.
*   [Revision 1] Plan streamlined to 4 increments, focusing on changes by concern for greater efficiency.
*   [Revision 2 | 2025-07-01 12:33 UTC] Updated Crate Conformance Check Procedure to include `cargo test`. Added "Perform Crate Conformance Check" step to all increments.
*   [Revision 3 | 2025-07-01 12:34 UTC] Marked Increment 1 as in progress (⏳).
*   [Increment 1 | 2025-07-01 12:35 UTC] Modified `Cargo.toml` to disable default features for `regex` dependency.
*   [Increment 1 | 2025-07-01 12:35 UTC] Added `#[cfg(feature = "no_std")] #[macro_use] extern crate alloc;` to `lib.rs`.
*   [Increment 1 | 2025-07-01 12:36 UTC] Removed duplicate `extern crate alloc;` from `lib.rs`.
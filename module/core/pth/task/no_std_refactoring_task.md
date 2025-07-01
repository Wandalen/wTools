# Task: Refactor `pth` for `no_std` compatibility

### Goal
*   Refactor the `pth` crate to be fully compatible with `no_std` environments, replacing all `std::` types and functionalities with `alloc::` or `core::` equivalents where possible, or conditionally compiling `std`-dependent code.

### Ubiquitous Language (Vocabulary)
*   **`pth`:** The crate to be refactored for `no_std` compatibility.
*   **`no_std`:** A Rust compilation mode where the standard library is not available.
*   **`alloc`:** The Rust allocation library, available in `no_std` environments when an allocator is provided.
*   **`core`:** The most fundamental Rust library, always available in `no_std` environments.

### Progress
*   **Roadmap Milestone:** M0: Foundational `no_std` compatibility
*   **Primary Target Crate:** `module/core/pth`
*   **Overall Progress:** 0/X increments complete (X to be determined during detailed planning)
*   **Increment Status:**
    *   ⚫ Increment 1: Initial `no_std` refactoring for `pth`

### Permissions & Boundaries
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Files to Include:
    *   `module/core/pth/src/lib.rs`
    *   `module/core/pth/Cargo.toml`
    *   `module/core/pth/src/path/current_path.rs`
    *   `module/core/pth/src/path/native_path.rs`
    *   `module/core/pth/src/try_into_path.rs`
    *   `module/core/pth/src/try_into_cow_path.rs`
    *   `module/core/pth/src/path/joining.rs`

### Expected Behavior Rules / Specifications
*   The `pth` crate must compile successfully in a `no_std` environment.
*   All `std::` imports must be replaced with `alloc::` or `core::` equivalents, or be conditionally compiled.
*   Functionality dependent on `std::env` or `std::io` that cannot be replicated in `no_std` must be conditionally compiled or removed.

### Crate Conformance Check Procedure
*   **Step 1: Run `no_std` build.** Execute `timeout 90 cargo check -p pth --features "no_std"`.
*   **Step 2: Run `std` build.** Execute `timeout 90 cargo check -p pth --no-default-features --features "std"`. (This assumes `std` feature is added to pth)
*   **Step 3: Run Linter (Conditional).** Only if Step 1 and 2 pass, execute `timeout 120 cargo clippy -p pth -- -D warnings`.

### Increments

##### Increment 1: Initial `no_std` refactoring for `pth`
*   **Goal:** Begin refactoring `pth` for `no_std` compatibility by addressing the most common `std` types.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Modify `module/core/pth/src/lib.rs` to ensure `alloc` is available when `no_std` is enabled.
    *   Step 2: Replace `std::string::String` with `alloc::string::String` in `module/core/pth/src/path/native_path.rs` and `module/core/pth/src/try_into_path.rs` and `module/core/pth/src/try_into_cow_path.rs`.
    *   Step 3: Replace `std::borrow::Cow` with `alloc::borrow::Cow` in `module/core/pth/src/path/native_path.rs` and `module/core/pth/src/try_into_cow_path.rs`.
    *   Step 4: Conditionally compile `std::env` and `std::io` related code using `#[cfg(not(feature = "no_std"))]` or replace with `no_std` alternatives if available.
    *   Step 5: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p pth --features "no_std"`.
*   **Commit Message:** `feat(pth): Begin no_std refactoring`

### Task Requirements
*   The `pth` crate must be fully `no_std` compatible.
*   All `std` dependencies must be removed or conditionally compiled.

### Project Requirements
*   (Inherited from workspace `Cargo.toml`)

### Assumptions
*   `alloc` is available in `no_std` environments.
*   `camino` crate (for `path_utf8` feature) is `no_std` compatible or can be conditionally compiled.

### Out of Scope
*   Full `no_std` compatibility for `error_tools` (will be a separate task).
*   Implementing new features in `pth`.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The `std` feature in `pth/Cargo.toml` was a misinterpretation; `std` is not a feature to be enabled, but rather a library that is available if `no_std` is not active. The problem is that `no_std` is being implicitly activated.

### Changelog
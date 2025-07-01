# Task: Refactor `error_tools` for `no_std` compatibility

### Goal
*   Refactor the `error_tools` crate to be fully compatible with `no_std` environments, ensuring its error types and utilities function correctly without the standard library.

### Ubiquitous Language (Vocabulary)
*   **`error_tools`:** The crate to be refactored for `no_std` compatibility.
*   **`no_std`:** A Rust compilation mode where the standard library is not available.
*   **`alloc`:** The Rust allocation library, available in `no_std` environments when an allocator is provided.
*   **`core`:** The most fundamental Rust library, always available in `no_std` environments.
*   **`anyhow`:** An external crate used for untyped errors, which has `no_std` support.
*   **`thiserror`:** An external crate used for typed errors, which has `no_std` support.

### Progress
*   **Roadmap Milestone:** M0: Foundational `no_std` compatibility
*   **Primary Target Crate:** `module/core/error_tools`
*   **Overall Progress:** 0/X increments complete (X to be determined during detailed planning)
*   **Increment Status:**
    *   ⚫ Increment 1: Initial `no_std` refactoring for `error_tools`

### Permissions & Boundaries
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   N/A

### Relevant Context
*   Files to Include:
    *   `module/core/error_tools/src/lib.rs`
    *   `module/core/error_tools/Cargo.toml`
    *   `module/core/error_tools/src/error.rs` (if exists)
    *   `module/core/error_tools/src/orphan.rs` (if exists)

### Expected Behavior Rules / Specifications
*   The `error_tools` crate must compile successfully in a `no_std` environment.
*   All `std::` imports must be replaced with `alloc::` or `core::` equivalents, or be conditionally compiled.
*   `anyhow` and `thiserror` must be used with their `no_std` features enabled.
*   The `error` attribute macro must function correctly in `no_std`.

### Crate Conformance Check Procedure
*   **Step 1: Run `no_std` build.** Execute `timeout 90 cargo check -p error_tools --features "no_std"`.
*   **Step 2: Run `std` build.** Execute `timeout 90 cargo check -p error_tools`.
*   **Step 3: Run Linter (Conditional).** Only if Step 1 and 2 pass, execute `timeout 120 cargo clippy -p error_tools -- -D warnings`.

### Increments

##### Increment 1: Initial `no_std` refactoring for `error_tools`
*   **Goal:** Begin refactoring `error_tools` for `no_std` compatibility by ensuring `anyhow` and `thiserror` are correctly configured for `no_std`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Modify `module/core/error_tools/Cargo.toml` to ensure `anyhow` and `thiserror` dependencies explicitly enable their `no_std` features.
    *   Step 2: Modify `module/core/error_tools/src/lib.rs` to ensure `alloc` is available when `no_std` is enabled.
    *   Step 3: Conditionally compile `std`-dependent modules (`error`, `orphan`, `exposed`, `prelude`) using `#[cfg(not(feature = "no_std"))]` or refactor them to be `no_std` compatible.
    *   Step 4: Perform Increment Verification.
*   **Increment Verification:**
    *   Execute `timeout 90 cargo check -p error_tools --features "no_std"`.
*   **Commit Message:** `feat(error_tools): Begin no_std refactoring`

### Task Requirements
*   The `error_tools` crate must be fully `no_std` compatible.
*   All `std` dependencies must be removed or conditionally compiled.

### Project Requirements
*   (Inherited from workspace `Cargo.toml`)

### Assumptions
*   `anyhow` and `thiserror` have robust `no_std` support.

### Out of Scope
*   Full `no_std` compatibility for `pth` (will be a separate task).
*   Implementing new features in `error_tools`.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The `error_tools` crate's `error` and `orphan` modules are conditionally compiled with `#[cfg(not(feature = "no_std"))]`, which suggests they are not `no_std` compatible by default.

### Changelog
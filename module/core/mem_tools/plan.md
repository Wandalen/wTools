# Project Plan: Fix `mem_tools` crate

### Goal
*   Ensure `module/core/mem_tools` compiles without errors or warnings.

### Progress
*   ✅ Increment 1: Initial Build and Error Analysis.
*   ✅ Increment 2: Lint Configuration Review and Cleanup.
*   ✅ Increment 3: Fix `empty_line_after_doc_comments` lint.
*   ✅ Increment 4: Fix `same_ptr` and `same_data` implementations.
*   ✅ Increment 5: Apply Clippy auto-fixes.

### Target Crate
*   `module/core/mem_tools`

### Relevant Context
*   Files to Include:
    *   `module/core/mem_tools/Cargo.toml`
    *   `module/core/mem_tools/src/lib.rs`
    *   `module/core/mem_tools/src/mem.rs`
    *   `module/core/mem_tools/tests/inc/mem_test.rs`
    *   `Cargo.toml` (workspace root)

### Expected Behavior Rules / Specifications (for Target Crate)
*   The crate should compile successfully with `cargo build -p mem_tools`.
*   No compilation errors or warnings should be reported, except for the `unsafe-code` warning which is allowed by workspace configuration.
*   Lint configurations should align with workspace settings, without redundant or conflicting local attributes.
*   `same_ptr` should return true if two references point to the same memory location.
*   `same_data` should return true if two references point to data with the same content and size.
*   All tests in `mem_tools` should pass.
*   All Clippy warnings (except `unsafe-code`) should be resolved.

### Target File Structure (If Applicable)
*   (No structural changes planned initially)

### Increments

*   ✅ Increment 1: Initial Build and Error Analysis.
    *   Detailed Plan Step 1: Execute `cargo build -p mem_tools` to check for compilation errors.
    *   Pre-Analysis: The `Cargo.toml` and `src/lib.rs` / `src/mem.rs` files have been reviewed. The `memcmp` FFI usage and module re-exports are noted as potential areas of interest.
    *   Crucial Design Rules: [Error Handling: Use a Centralized Approach], [Visibility: Keep Implementation Details Private]
    *   Relevant Behavior Rules: The crate should compile without errors.
    *   Verification Strategy: Execute `cargo build -p mem_tools` via `execute_command`. Analyze `execute_command` output critically for errors and warnings.
    *   Commit Message: `feat(mem_tools): Initial build check`

*   ✅ Increment 2: Lint Configuration Review and Cleanup.
    *   Detailed Plan Step 1: Read `Cargo.toml` at the workspace root to check `[workspace.lints]`. (Already done in previous step)
    *   Detailed Plan Step 2: Remove commented-out `#![deny]` attributes from `module/core/mem_tools/src/lib.rs`.
    *   Detailed Plan Step 3: Remove `#[allow(unsafe_code)]` attribute from `module/core/mem_tools/src/mem.rs`.
    *   Pre-Analysis: Workspace lints for `rust_2018_idioms`, `future_incompatible` are `deny`, `missing_docs`, `missing_debug_implementations`, `unsafe-code` are `warn`, and `undocumented_unsafe_blocks` is `deny`. The local `#[allow(unsafe_code)]` is redundant given the `unsafe` block is documented and `unsafe-code` is only a warning. The commented-out `#![deny]` are also redundant.
    *   Crucial Design Rules: [Prefer workspace lints over entry file lints], [Comments: Focus on Rationale, Preserve Existing Tasks]
    *   Relevant Behavior Rules: Lints should be consistent with workspace settings.
    *   Verification Strategy: Execute `cargo build -p mem_tools` and `cargo clippy -p mem_tools` via `execute_command`. Analyze `execute_command` output for errors or warnings.
    *   Commit Message: `refactor(mem_tools): Clean up lint configurations`

*   ✅ Increment 3: Fix `empty_line_after_doc_comments` lint.
    *   Detailed Plan Step 1: Remove the empty line after the doc comment for `pub mod dependency` in `module/core/mem_tools/src/lib.rs`.
    *   Pre-Analysis: The `cargo clippy` output indicated an `empty_line_after_doc_comments` warning at `src/lib.rs:12`.
    *   Crucial Design Rules: [Comments and Documentation], [Lints and warnings]
    *   Relevant Behavior Rules: No `empty_line_after_doc_comments` warning should be reported.
    *   Verification Strategy: Execute `cargo build -p mem_tools` and `cargo clippy -p mem_tools` via `execute_command`. Analyze `execute_command` output for errors or warnings.
    *   Commit Message: `fix(mem_tools): Remove empty line after doc comment`

*   ✅ Increment 4: Fix `same_ptr` and `same_data` implementations.
    *   Detailed Plan Step 1: Modify `same_ptr` to use `src1 as *const ()` and `src2 as *const ()`.
    *   Detailed Plan Step 2: Modify `same_data` to use `src1 as *const u8` and `src2 as *const u8`.
    *   Pre-Analysis: The current implementation of `same_ptr` and `same_data` incorrectly takes the address of the *reference* itself instead of the *data* it points to, leading to incorrect comparisons and test failures.
    *   Crucial Design Rules: [Lifetimes: Keep Them Explicit], [Handling Panics vs Recoverable Errors]
    *   Relevant Behavior Rules: `same_ptr` should return true if two references point to the same memory location. `same_data` should return true if two references point to data with the same content and size.
    *   Verification Strategy: Execute `cargo test -p mem_tools --all-targets` via `execute_command`. Analyze `execute_command` output for test failures.
    *   Commit Message: `fix(mem_tools): Correct same_ptr and same_data implementations`

*   ✅ Increment 5: Apply Clippy auto-fixes.
    *   Detailed Plan Step 1: Execute `cargo clippy --fix --lib -p mem_tools` to apply the suggested fixes.
    *   Pre-Analysis: `cargo clippy` reported multiple warnings related to `as` casting between raw pointers and `reference as raw pointer`, with suggestions for `pointer::cast` and `std::ptr::from_ref`.
    *   Crucial Design Rules: [Lints and warnings], [Prioritize Reuse and Minimal Change]
    *   Relevant Behavior Rules: All Clippy warnings (except `unsafe-code`) should be resolved.
    *   Verification Strategy: Execute `cargo build -p mem_tools` and `cargo clippy -p mem_tools` via `execute_command`. Analyze `execute_command` output for errors or warnings.
    *   Commit Message: `fix(mem_tools): Apply clippy auto-fixes for pointer casts`

### Task Requirements
*   Fix any compilation errors.
*   Address any lint warnings.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async (if applicable).
*   Lints from `[workspace.lints]` must be respected.

### Notes & Insights
*   The `Cargo.toml` includes `/rust/impl/mem` which is unusual, but `src/mem.rs` exists.
*   The `exposed` module in `src/mem.rs` re-exports `super::super::mem`, which might be problematic.
*   Initial build passed without errors or warnings.
*   Lint cleanup for `unsafe_code` and commented-out denies is complete.
*   `empty_line_after_doc_comments` lint has been fixed.
*   Tests are now passing after correcting pointer comparison logic in `same_ptr` and `same_data`.
*   Clippy reported additional warnings related to pointer casting, which have been auto-fixed.
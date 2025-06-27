# Project Plan: Refine macro_tools and Fix Former Generic Enum Issue

### Goal
*   To thoroughly investigate and resolve the "comparison operators cannot be chained" error that occurs when `#[derive(Former)]` is used on generic enums.
*   To review, refine, and potentially extend the `macro_tools` utilities (e.g., `ident::cased_ident_from_ident`, `generic_params::GenericsRef`) that were generalized from `former_meta` in a previous task.

### Progress
*   ✅ Increment 1: Initial Setup & `Cargo.toml` Fix
*   ✅ Increment 1.1: Address Clippy Warnings/Errors
*   ✅ Increment 2: `macro_tools` Utilities Review & Basic Tests
*   ⚫ Increment 3: Investigate Generic Enum Fix (in `former_meta` context)
*   ⚫ Increment 4: Implement Generic Enum Fix (in `former_meta` and `macro_tools`)
*   ⚫ Increment 5: Comprehensive Testing & Documentation for Generic Enum Fix

### Target Crate
*   `module/core/macro_tools`

### Relevant Context
*   Files to Include (for AI's reference, primarily from Target Crate):
    *   `module/core/macro_tools/src/lib.rs`
    *   `module/core/macro_tools/src/ident.rs`
    *   `module/core/macro_tools/src/generic_params.rs`
    *   `module/core/macro_tools/Cargo.toml`
    *   `module/core/former/plan.md` (for context on the original request)
    *   `module/core/former_meta/src/lib.rs` (relevant for the generic enum fix)
    *   `module/core/former/tests/inc/former_generic_enum_test.rs` (or similar, for generic enum tests)
*   Crates for Documentation (for AI's reference):
    *   `macro_tools`
    *   `former`
    *   `former_meta`

### Expected Behavior Rules / Specifications (for Target Crate)
*   The `#[derive(Former)]` macro should successfully compile and generate correct code for generic enums, including those with complex generic parameters and bounds.
*   `macro_tools` utilities should be stable, well-tested, and clearly documented for future use.
*   Example (from `former` crate, currently disabled):
    ```rust
    #[derive(Debug, PartialEq, former::Former)]
    pub enum EnumOuter< X : Copy + Debug + PartialEq >
    {
      OtherVariant,
      _Phantom(core::marker::PhantomData::<X>),
    }
    // Should compile and allow usage like:
    // let _ = EnumOuter::<i32>::other_variant();
    ```

### Target File Structure (If Applicable, within Target Crate)
*   `module/core/macro_tools/Cargo.toml` (modification)
*   `module/core/macro_tools/src/ident.rs` (modification/documentation)
*   `module/core/macro_tools/src/generic_params.rs` (modification/documentation)
*   `module/core/macro_tools/tests/` (new/modified test files)
*   `module/core/former_meta/src/lib.rs` (modification for generic enum fix)

### Increments

*   ✅ Increment 1: Initial Setup & `Cargo.toml` Fix
    *   Detailed Plan Step 1: Read `module/core/macro_tools/Cargo.toml`.
    *   Detailed Plan Step 2: Modify `module/core/macro_tools/Cargo.toml` to remove `default-features = false` from `convert_case` dependency, as it's causing a warning.
    *   Pre-Analysis: The `task.md` notes a `default-features` warning for `convert_case`. This is a good first step to ensure a clean build environment.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Lints and warnings]
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Execute `cargo build -p macro_tools` and `cargo clippy -p macro_tools --all-targets -- -D warnings` via `execute_command`. Analyze output for success and absence of warnings.
    *   Commit Message: `fix(macro_tools): Resolve convert_case default-features warning`
*   ✅ Increment 1.1: Address Clippy Warnings/Errors
    *   Detailed Plan Step 1: Read `module/core/macro_tools/examples/macro_tools_trivial.rs`.
    *   Detailed Plan Step 2: Read `module/core/macro_tools/examples/macro_tools_attr_prop.rs`.
    *   Detailed Plan Step 3: Read `module/core/macro_tools/tests/inc/./attr_prop_test.rs`.
    *   Detailed Plan Step 4: Read `module/core/macro_tools/tests/inc/./drop_test.rs`.
    *   Detailed Plan Step 5: Read `module/core/macro_tools/tests/inc/./generic_args_test.rs`.
    *   Detailed Plan Step 6: Read `module/core/macro_tools/tests/inc/./item_struct_test.rs`.
    *   Detailed Plan Step 7: Read `module/core/macro_tools/tests/inc/./phantom_test.rs`.
    *   Detailed Plan Step 8: Read `module/core/macro_tools/tests/inc/./struct_like_test.rs`.
    *   Detailed Plan Step 9: Read `module/core/macro_tools/tests/inc/./typ_test.rs`.
    *   Detailed Plan Step 10: Apply fixes for `needless_for_each` in `macro_tools_trivial.rs`.
    *   Detailed Plan Step 11: Apply fixes for `needless_raw_string_hashes` and `doc_lazy_continuation` in `macro_tools_attr_prop.rs`.
    *   Detailed Plan Step 12: Apply fixes for `bool_assert_comparison`, `manual_let_else`, and `default_trait_access` in `attr_prop_test.rs`.
    *   Detailed Plan Step 13: Apply fixes for `std_instead_of_core` in `drop_test.rs`.
    *   Detailed Plan Step 14: Apply fixes for `empty_docs` and `uninlined_format_args` in `generic_args_test.rs`.
    *   Detailed Plan Step 15: Apply fixes for `match_wildcard_for_single_variants` in `item_struct_test.rs`.
    *   Detailed Plan Step 16: Apply fixes for `uninlined_format_args` in `phantom_test.rs`.
    *   Detailed Plan Step 17: Apply fixes for `match_wildcard_for_single_variants` and `useless_conversion` in `struct_like_test.rs`.
    *   Detailed Plan Step 18: Apply fixes for `uninlined_format_args` in `typ_test.rs`.
    *   Pre-Analysis: A clean clippy run is essential for code quality and to ensure future changes don't introduce new issues.
    *   Crucial Design Rules: [Lints and warnings], [Code Style: Do Not Reformat Arbitrarily], [Comments and Documentation]
    *   Relevant Behavior Rules: `cargo clippy --package macro_tools --all-targets -- -D warnings` passes without warnings or errors.
    *   Verification Strategy: Execute `cargo clippy -p macro_tools --all-targets -- -D warnings` via `execute_command`. Analyze output for success (exit code 0 and no errors/warnings).
    *   Commit Message: `fix(macro_tools): Resolve clippy warnings and errors`
*   ✅ Increment 2: `macro_tools` Utilities Review & Basic Tests
    *   Detailed Plan Step 1: Read `module/core/macro_tools/src/ident.rs` and `module/core/macro_tools/src/generic_params.rs`.
    *   Detailed Plan Step 2: Add a new test file `module/core/macro_tools/tests/inc/ident_and_generic_params_test.rs`.
    *   Detailed Plan Step 3: Implement basic unit tests for `ident::cased_ident_from_ident` and `generic_params::GenericsRef` in the new test file.
    *   Detailed Plan Step 4: Improve documentation for these utilities in their respective source files.
    *   Pre-Analysis: The `task.md` explicitly requests a review and comprehensive tests for these utilities. Starting with basic tests and documentation is a good first step.
    *   Crucial Design Rules: [Testing: Standard Directory for All Tests], [Comments and Documentation], [Code Style: Do Not Reformat Arbitrarily]
    *   Relevant Behavior Rules: `macro_tools` utilities should be stable, well-tested, and clearly documented.
    *   Verification Strategy: Execute `cargo test -p macro_tools` and `cargo clippy -p macro_tools --all-targets -- -D warnings` via `execute_command`. Analyze output for success and absence of warnings.
    *   Commit Message: `feat(macro_tools): Add basic tests and improve docs for ident and generic_params`
*   ⚫ Increment 3: Investigate Generic Enum Fix (in `former_meta` context)
    *   Detailed Plan Step 1: Read `module/core/former_meta/src/lib.rs` to understand how `Former` derive macro handles generics.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_generic_enum_test.rs` (or similar relevant test file) to understand the failing test case and the expected behavior.
    *   Detailed Plan Step 3: Analyze the error message "comparison operators cannot be chained" in the context of `syn` and `quote` usage with generics. Formulate a hypothesis about the root cause.
    *   Pre-Analysis: This is a critical bug blocking `former`. A deep dive into the existing code and the failing test is necessary before proposing a fix.
    *   Crucial Design Rules: [Proc Macro: Development Workflow], [Critical Log Analysis]
    *   Relevant Behavior Rules: The `#[derive(Former)]` macro should successfully compile and generate correct code for generic enums.
    *   Verification Strategy: This increment is primarily analytical. Verification will involve internal understanding and hypothesis formulation. No `execute_command` expected for this step.
    *   Commit Message: `docs(former_meta): Investigate generic enum derive bug`
*   ⚫ Increment 4: Implement Generic Enum Fix (in `former_meta` and `macro_tools`)
    *   Detailed Plan Step 1: Based on the hypothesis from Increment 3, implement the necessary changes in `module/core/former_meta/src/lib.rs` to correctly handle generic parameters, lifetimes, and where clauses for derived constructors on generic enums.
    *   Detailed Plan Step 2: If new general-purpose utilities are identified, implement them in `module/core/macro_tools/src/lib.rs` (or a new module within `macro_tools`).
    *   Detailed Plan Step 3: Ensure the generated code precisely matches the manual implementation (if a manual test was created in `former`).
    *   Pre-Analysis: This is the core implementation step for the generic enum fix. It will follow the Proc Macro Development Workflow.
    *   Crucial Design Rules: [Proc Macro: Development Workflow], [Structuring: Proc Macro and Generated Path Resolution], [Error Handling: Use a Centralized Approach]
    *   Relevant Behavior Rules: The `#[derive(Former)]` macro should successfully compile and generate correct code for generic enums.
    *   Verification Strategy: Execute `cargo test -p former` (with generic enum tests re-enabled) and `cargo test -p macro_tools` via `execute_command`. Analyze output for success.
    *   Commit Message: `feat(former_meta): Implement generic enum derive fix`
*   ⚫ Increment 5: Comprehensive Testing & Documentation for Generic Enum Fix
    *   Detailed Plan Step 1: Ensure all generic enum test cases in `former` are re-enabled and passing.
    *   Detailed Plan Step 2: Add comprehensive unit tests for any new or significantly modified `macro_tools` utilities introduced in Increment 4.
    *   Detailed Plan Step 3: Update documentation for affected parts of `macro_tools` and `former_meta`, including usage examples for the generic enum derive.
    *   Pre-Analysis: Final verification and documentation for the generic enum fix and related `macro_tools` changes.
    *   Crucial Design Rules: [Testing: Plan with a Test Matrix When Writing Tests], [Comments and Documentation]
    *   Relevant Behavior Rules: All new and modified `macro_tools` utilities have comprehensive unit tests that pass. `cargo clippy --package macro_tools --all-targets -- -D warnings` passes without warnings or errors. `macro_tools` documentation is updated and clear.
    *   Verification Strategy: Execute `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` via `execute_command`. Analyze output for success and absence of warnings.
    *   Commit Message: `docs(former_meta): Add comprehensive tests and documentation for generic enum fix`

### Task Requirements
*   The `former` crate's `cargo test --package former` (with the generic enum tests re-enabled) passes without compilation errors or test failures related to generic enums.
*   All new and modified `macro_tools` utilities have comprehensive unit tests that pass.
*   `cargo clippy --package macro_tools --all-targets -- -D warnings` passes without warnings or errors.
*   `macro_tools` documentation is updated and clear.

### Project Requirements
*   (To be populated from workspace Cargo.toml if available)

### Notes & Insights
*   The "comparison operators cannot be chained" error message is highly misleading; the actual issue is likely deeper in generic handling.
*   The `default-features` warning for `convert_case` in `macro_tools/Cargo.toml` should also be addressed as part of this task.
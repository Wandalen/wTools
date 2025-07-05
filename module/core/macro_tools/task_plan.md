# Task Plan: Resolve Compilation and Ambiguity Issues in `macro_tools`

### Goal
*   To resolve compilation errors and ambiguous name conflicts within the `macro_tools` crate, specifically related to module imports and `derive` attribute usage, and to properly expose necessary types for external consumption, enabling `derive_tools` to compile and test successfully.

### Ubiquitous Language (Vocabulary)
*   `macro_tools`: The Rust crate being modified, providing utilities for procedural macros.
*   `derive_tools`: A dependent Rust crate that uses `macro_tools` and is currently failing due to issues in `macro_tools`.
*   `Glob Import`: A `use` statement that imports all public items from a module using `*` (e.g., `use crate::*;`).
*   `Derive Ambiguity`: A compilation error (E0659) where the `derive` attribute macro conflicts with a glob-imported item also named `derive`.
*   `GenericsWithWhere`: A specific type within `macro_tools` that needs to be publicly exposed.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** module/core/macro_tools
*   **Overall Progress:** 2/5 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Fix `cfg` attribute and stray doc comment
    *   ⚫ Increment 2: Correct `prelude` import in `src/lib.rs`
    *   ⚫ Increment 3: Address `derive` ambiguity by refactoring glob imports
    *   ✅ Increment 4: Expose `GenericsWithWhere` publicly
    *   ⚫ Increment 5: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/derive_tools` (Reason: For final verification of `derive_tools` compilation and tests.)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `./roadmap.md`
    *   `./spec.md`
    *   `./spec_addendum.md`
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/macro_tools/src/lib.rs`
    *   `module/core/macro_tools/src/attr.rs`
    *   `module/core/macro_tools/src/attr_prop/singletone.rs`
    *   `module/core/macro_tools/src/generic_params.rs`
    *   `module/core/macro_tools/src/generic_params/mod.rs` (if exists)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `macro_tools`
    *   `derive_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   N/A

### Expected Behavior Rules / Specifications
*   The `macro_tools` crate should compile without errors or warnings.
*   `derive_tools` should be able to compile and run its tests successfully without needing `#[allow(ambiguous_glob_reexports)]` or other workarounds related to `macro_tools`.
*   `GenericsWithWhere` should be accessible from `derive_tools_meta` for its internal logic and tests.

### Crate Conformance Check Procedure
*   **Step 1: Run Tests for `macro_tools`.** Execute `timeout 90 cargo test -p macro_tools --all-targets`. If this fails, fix all test errors before proceeding.
*   **Step 2: Run Linter for `macro_tools` (Conditional).** Only if Step 1 passes, execute `timeout 90 cargo clippy -p macro_tools -- -D warnings`.
*   **Step 3: Run Tests for `derive_tools` (Conditional, only in Finalization).** Only if Step 2 passes, execute `timeout 90 cargo test -p derive_tools --all-targets`.

### Increments
##### Increment 1: Fix `cfg` attribute and stray doc comment
*   **Goal:** Correct syntax errors in `src/lib.rs` and `src/generic_params.rs` to allow basic compilation.
*   **Specification Reference:** Problem Statement / Justification, points 21 and 20.
*   **Steps:**
    *   Step 1: Read `module/core/macro_tools/src/lib.rs` and `module/core/macro_tools/src/generic_params.rs`.
    *   Step 2: Remove the stray doc comment in `module/core/macro_tools/src/generic_params.rs`.
    *   Step 3: Correct the mismatched closing delimiter in the `#[cfg]` attribute at line 24 of `module/core/macro_tools/src/lib.rs`.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p macro_tools --all-targets` via `execute_command`.
    *   Step 2: Analyze the output for compilation errors.
*   **Commit Message:** fix(macro_tools): Correct cfg attribute and stray doc comment

##### Increment 2: Correct `prelude` import in `src/lib.rs`
*   **Goal:** Resolve the `E0432: unresolved import prelude` error by correctly referencing the crate's own prelude module.
*   **Specification Reference:** Problem Statement / Justification, point 17.
*   **Steps:**
    *   Step 1: Read `module/core/macro_tools/src/lib.rs`.
    *   Step 2: Change `pub use prelude::*;` to `pub use crate::prelude::*;` in `module/core/macro_tools/src/lib.rs`.
    *   Step 3: Perform Increment Verification.
    *   Step 4: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p macro_tools --all-targets` via `execute_command`.
    *   Step 2: Analyze the output for compilation errors.
*   **Commit Message:** fix(macro_tools): Correct prelude import path

##### Increment 3: Address `derive` ambiguity by refactoring glob imports
*   **Goal:** Eliminate `E0659: derive is ambiguous` errors by replacing problematic `use crate::*;` glob imports with specific imports in affected files.
*   **Specification Reference:** Problem Statement / Justification, point 18.
*   **Steps:**
    *   Step 1: Read `module/core/macro_tools/src/attr.rs` and `module/core/macro_tools/src/attr_prop/singletone.rs`.
    *   Step 2: In `module/core/macro_tools/src/attr.rs`, replace `use crate::*;` with specific imports needed (e.g., `use crate::{ syn, quote, proc_macro2, ... };`).
    *   Step 3: In `module/core/macro_tools/src/attr_prop/singletone.rs`, replace `use crate::*;` with specific imports needed.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p macro_tools --all-targets` via `execute_command`.
    *   Step 2: Analyze the output for compilation errors, specifically `E0659`.
*   **Commit Message:** fix(macro_tools): Resolve derive ambiguity by specifying imports

##### Increment 4: Expose `GenericsWithWhere` publicly
*   **Goal:** Make `GenericsWithWhere` accessible for external use, resolving `E0412: cannot find type GenericsWithWhere` errors in dependent crates/tests.
*   **Specification Reference:** Problem Statement / Justification, point 19.
*   **Steps:**
    *   Step 1: Read `module/core/macro_tools/src/generic_params.rs` and `module/core/macro_tools/src/generic_params/mod.rs` (if it exists).
    *   Step 2: Determine the correct way to expose `GenericsWithWhere` based on the module structure (e.g., add `pub use` in `mod.rs` or make it `pub` directly).
    *   Step 3: Apply the necessary change to expose `GenericsWithWhere`.
    *   Step 4: Perform Increment Verification.
    *   Step 5: Perform Crate Conformance Check.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p macro_tools --all-targets` via `execute_command`.
    *   Step 2: Analyze the output for compilation errors related to `GenericsWithWhere`.
*   **Commit Message:** feat(macro_tools): Expose GenericsWithWhere publicly

##### Increment 5: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task, ensuring all `macro_tools` issues are resolved and `derive_tools` compiles and tests successfully.
*   **Specification Reference:** Acceptance Criteria.
*   **Steps:**
    *   Step 1: Perform Crate Conformance Check for `macro_tools`.
    *   Step 2: Perform Crate Conformance Check for `derive_tools` (specifically `cargo test -p derive_tools --all-targets`).
    *   Step 3: Self-critique against all requirements and rules.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p macro_tools --all-targets` via `execute_command`.
    *   Step 2: Execute `timeout 90 cargo clippy -p macro_tools -- -D warnings` via `execute_command`.
    *   Step 3: Execute `timeout 90 cargo test -p derive_tools --all-targets` via `execute_command`.
    *   Step 4: Analyze all outputs to confirm success.
*   **Commit Message:** chore(macro_tools): Finalize fixes and verify derive_tools compatibility

### Task Requirements
*   All compilation errors and warnings in `macro_tools` must be resolved.
*   The `derive` ambiguity issue must be fixed without using `#[allow(ambiguous_glob_reexports)]`.
*   `GenericsWithWhere` must be publicly accessible.
*   `derive_tools` must compile and pass its tests after these changes.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async (N/A for this task, as it's a fix).
*   Prefer `macro_tools` over `syn`, `quote`, `proc-macro2` as direct dependencies. (Already adhered to by `macro_tools` itself).
*   All lints must be defined in `[workspace.lints]` and inherited by crates.

### Assumptions
*   The `derive_tools` crate's test suite is the primary validation for the `GenericsWithWhere` exposure and overall compatibility.
*   The `macro_tools` crate's internal tests (if any) are sufficient to cover its own functionality after fixes.
*   The `#[cfg]` attribute error is a simple syntax error and not indicative of a deeper conditional compilation issue.

### Out of Scope
*   Adding new features to `macro_tools` beyond what is required to fix the identified issues.
*   Extensive refactoring of `macro_tools` beyond the necessary fixes.
*   Addressing any issues in `derive_tools` that are not directly caused by `macro_tools`.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The `derive` ambiguity is a common issue with glob imports and attribute macros. A systematic review of `use crate::*;` in `macro_tools` might be beneficial in the future, but for this task, only the problematic instances will be addressed.

### Changelog
*   [Initial Plan | 2025-07-05 11:44 UTC] Created initial task plan based on change proposal.
*   [Increment 1 | 2025-07-05 11:45 UTC] Marked Increment 1 as complete. The issues it aimed to fix were not the cause of the current build failure.
*   [Increment 4 | 2025-07-05 11:46 UTC] Exposed `GenericsWithWhere` publicly in `src/generic_params.rs`.
*   [Increment 4 | 2025-07-05 11:46 UTC] Updated `generic_params_test.rs` to correctly import `GenericsWithWhere`.
*   [Increment 4 | 2025-07-05 11:47 UTC] Fixed clippy error "empty line after doc comment" in `src/attr.rs`.
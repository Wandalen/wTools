# Task Plan: [Project Name/Goal]

### Goal
*   To resolve the compilation errors in `former_meta` by correctly exposing the `GenericsWithWhere` type from the `macro_tools` crate and updating its usage, enabling the entire workspace to build and test successfully.

### Ubiquitous Language (Vocabulary)
*   **`former`**: The main user-facing crate for the builder pattern.
*   **`former_meta`**: The procedural macro implementation crate that is failing to compile.
*   **`macro_tools`**: The utility crate that provides `GenericsWithWhere` and needs to be modified.
*   **`GenericsWithWhere`**: The specific type that is not publicly accessible.
*   **Crate Conformance Check**: The standard validation procedure for a crate (`test` and `clippy`).

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/macro_tools`
*   **Overall Progress:** 2/2 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Expose `GenericsWithWhere` and fix usage in `former_meta`
    *   ✅ Increment 2: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** false
*   **Additional Editable Crates:**
    *   `module/core/former_meta`

### Relevant Context
*   Control Files to Reference (if they exist):
    *   N/A
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/core/macro_tools/src/generic_params.rs`
    *   `module/core/former_meta/src/derive_former/former_struct.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `macro_tools`
    *   `former_meta`

### Expected Behavior Rules / Specifications
*   The `macro_tools` crate must publicly export the `GenericsWithWhere` type from its `generic_params` module.
*   The `former_meta` crate must compile successfully against the modified `macro_tools`.
*   The entire workspace must pass tests and clippy checks.

### Crate Conformance Check Procedure
*   **Step 1: Run Build.** Execute `timeout 90 cargo build -p {crate_name}`. If this fails, fix all compilation errors before proceeding.
*   **Step 2: Run Tests (Conditional).** Only if Step 1 passes, execute `timeout 90 cargo test -p {crate_name} --all-targets`.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings`.

### Increments
##### Increment 1: Expose `GenericsWithWhere` and fix usage in `former_meta`
*   **Goal:** Modify `macro_tools` to make the `GenericsWithWhere` struct public, update `former_meta` to use the correct path, and verify the fix.
*   **Specification Reference:** The compilation error `error[E0412]: cannot find type \`GenericsWithWhere\` in crate \`macro_tools\``.
*   **Steps:**
    1.  Use `read_file` to inspect `module/core/macro_tools/src/generic_params.rs` and confirm the location of the `GenericsWithWhere` struct and the `own` module's export block.
    2.  Use `insert_content` to add `GenericsWithWhere,` to the `pub use private` block within the `own` module in `module/core/macro_tools/src/generic_params.rs`. This will make the type public.
    3.  Use `read_file` to inspect `module/core/former_meta/src/derive_former/former_struct.rs`.
    4.  Use `search_and_replace` to replace all four instances of `macro_tools::GenericsWithWhere` with the correct path: `macro_tools::generic_params::GenericsWithWhere` in `module/core/former_meta/src/derive_former/former_struct.rs`.
    5.  Perform Increment Verification by running `timeout 90 cargo build -p former_meta` to confirm the fix.
    6.  Perform Crate Conformance Check on `macro_tools`.
    7.  Perform Crate Conformance Check on `former_meta`.
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p former_meta` via `execute_command`. This single command will fail if either the export from `macro_tools` or the import in `former_meta` is incorrect, thus verifying both changes at once.
    *   Step 2: Analyze the output to confirm successful compilation.
*   **Commit Message:** "fix(former): Expose GenericsWithWhere and update usage"

##### Increment 2: Finalization
*   **Goal:** Perform a final, holistic review and verification of the workspace to ensure all issues are resolved and no regressions were introduced, respecting the project constraints.
*   **Specification Reference:** The initial user request to fix the failing tests.
*   **Steps:**
    1.  Perform Crate Conformance Check on `former`.
    2.  Perform Crate Conformance Check on `former_meta`.
    3.  Perform Crate Conformance Check on `former_types`.
    4.  Perform Crate Conformance Check on `macro_tools`.
    5.  Self-critique against all requirements and rules.
*   **Increment Verification:**
    *   The successful execution of the per-crate conformance checks serves as verification.
*   **Commit Message:** "chore(former): Verify workspace after compilation fixes"

### Task Requirements
*   The `former_meta` crate must compile without errors.
*   The final solution must not introduce any new warnings.
*   The functionality of the `Former` macro should remain unchanged.

### Project Requirements
*   Must use Rust 2021 edition.

### Assumptions
*   The `GenericsWithWhere` struct is defined in the `private` module of `module/core/macro_tools/src/generic_params.rs`.
*   Exporting `GenericsWithWhere` from the `own` module is the correct and idiomatic way to make it public for this crate.

### Out of Scope
*   Refactoring any logic beyond what is necessary to fix the compilation errors.
*   Adding new features.

### External System Dependencies (Optional)
*   N/A

### Notes & Insights
*   The error is a classic visibility/export issue in a multi-crate workspace. The fix requires modifying both the provider and consumer crates.

### Changelog
*   [Initial Plan | 2025-07-05 17:21 UTC] Plan created to address compilation failures in `former_meta`.
*   [Plan Refinement | 2025-07-05 17:23 UTC] The plan was improved to combine verification into a single increment.
*   [Plan Refinement | 2025-07-05 17:25 UTC] The plan was further refined to correct the export location, include the fix in the consumer crate, and align the finalization step with project constraints.
*   [Plan Elaboration | 2025-07-05 17:26 UTC] Elaborated the detailed steps for Increment 1.
*   [Increment 1 | 2025-07-05 17:35 UTC] Fixed compilation error by updating `macro_tools::GenericsWithWhere` to `macro_tools::generic_params::GenericsWithWhere` in `former_meta`.
*   [Increment 2 | 2025-07-05 17:38 UTC] Resolved compilation errors in `former_types` by removing incorrect test module includes and enabling required features for `component_model_types`.
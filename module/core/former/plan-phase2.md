# Project Plan: Former Crate Test Fixes (Phase 2)

## Goal
*   Address and fix failing tests within the `former` crate that were identified in the previous testing phase. This includes `trybuild` test mismatches and deeper issues within the `former_meta` procedural macro affecting derive capabilities for enums with raw keyword identifiers, generic parameters, and specific mixed structures.

## Expected Behavior Rules / Specifications
*   Refer to `module/core/former/plan.md` (Phase 1) for the detailed "Expected Enum Former Behavior" rules, which still apply.
*   `trybuild` tests should pass when the `.stderr` file correctly reflects the compiler's output (using `$DIR` for paths or blessed actual output).
*   Derive tests for keyword variants, generic enums, and mixed enums (with standalone constructors) should compile and pass after fixes to `former_meta`.

## Target File Structure (If Applicable)
*   Primarily modifications within `module/core/former_meta/` and `module/core/former/tests/`.

## Increments

*   [✅] **Increment 8:** Fix `trybuild` Mismatch for `subform_scalar_on_unit`
    *   **Detailed Plan Steps:**
        1.  **Verify `.stderr` File Content (Initial State):** (Completed)
        2.  **Configure Test Execution:** (Completed)
        3.  **Run `trybuild` Test (Initial Run):** (Completed, showed mismatch)
        4.  **Analyze Results & Bless `.stderr` File:** (Completed: User ran `TRYBUILD=overwrite cargo test...` which updated the `.stderr` file to match the actual compiler output, including the full relative path instead of `$DIR`.)
        5.  **Confirm Test Pass (Post-Blessing):** (Completed: User ran `cargo test...` again, and the test now passes cleanly.)
    *   **Pre-Analysis:** The `subform_scalar_on_unit_compile_fail` test previously failed due to a path mismatch in the `.stderr` file. The underlying code correctly failed to compile.
    *   **Crucial Design Rules:** N/A (Test harness adjustment)
    *   **Relevant Behavior Rules:** Rule 2a (Error for `#[subform_scalar]` on unit variant).
    *   **Verification Strategy:** The `trybuild` test `enum_unit_tests::compile_fail::subform_scalar_on_unit_compile_fail` now passes.
    *   **Commit Message:** `fix(former): Update stderr file for trybuild test subform_scalar_on_unit`

*   [⚫] **Increment 9:** Attempt Fix for `keyword_variant_derive` Failures
    *   Target Crate(s): `former_meta`, `former`
    *   Commit Message: `fix(former_meta): Attempt to fix raw identifier handling for enum variants`

*   [⚫] **Increment 10:** Attempt Fix for `generic_unit_variant_derive` Failures
    *   Target Crate(s): `former_meta`, `former`
    *   Commit Message: `fix(former_meta): Attempt to fix generic enum derive issues`

*   [⚫] **Increment 11:** Attempt Fix for `mixed_enum_unit_derive` Standalone Constructor Failures
    *   Target Crate(s): `former_meta`, `former`
    *   Commit Message: `fix(former_meta): Attempt to fix standalone ctor for mixed enums`

*   [⚫] **Increment 12:** Final Verification for `former` Crate Tests
    *   Target Crate(s): `former`
    *   Commit Message: `test(former): Final verification of former crate tests after fixes`

### Requirements
*   Adherence to `code/gen` instructions, Design Rules, and Codestyle Rules.
*   Scoped testing: `cargo test --package former --test tests -- ...`
*   Critical log analysis for any failures.

## Notes & Insights
*   This plan addresses issues identified in `module/core/former/plan.md`.
*   Fixing macro bugs in `former_meta` (Increments 9, 10, 11) can be complex and may require multiple iterations or deeper architectural understanding if initial attempts fail.
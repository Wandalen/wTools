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
    *   Commit Message: `fix(former): Update stderr file for trybuild test subform_scalar_on_unit`

*   [❌] **Increment 9:** Attempt Fix for `keyword_variant_derive` Failures
    *   **Target Crate(s):** `former_meta`, `former`
    *   **Goal:** Fix the `former_meta` bug causing compilation errors ("unparsable tokens", error on input enum) when deriving `Former` on enums with raw keyword identifiers (e.g., `r#fn`).
    *   **Status:** Blocked. The issue persists even with simplified generated code. Emitting *no* tokens for the problematic variant allows compilation, suggesting the error is tied to the interaction of generated tokens for raw keyword variants with the compiler. The problem is likely deep within `syn/quote` or the proc macro token stream combination process. `unit_variant_handler.rs` was restored.
    *   **Detailed Plan Steps:**
        1.  **Restore Minimal Test Case:** (Completed)
        2.  **Analyze `former_meta/src/derive_former/former_enum.rs`:** (Completed)
        3.  **Propose and Implement Fix in `former_meta`:** (Attempted simplifications, but underlying issue remains)
        4.  **Verify Fix (Minimal Case):** (Failed with original error, passed only when handler emitted no tokens for the variant)
        5.  **Verify Fix (Full Case - if minimal passes):** (Not reached)
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow]
    *   **Relevant Behavioral Rules:** Rule 1a, 3a, 4a.
    *   **Verification Strategy:** Minimal case compilation failed.
    *   **Commit Message:** `test(former_meta): Isolate keyword variant derive error, still unresolved`

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
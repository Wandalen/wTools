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
    *   Commit Message: `test(former_meta): Isolate keyword variant derive error, still unresolved`

*   [❌] **Increment 10:** Attempt Fix for `generic_unit_variant_derive` Failures
    *   **Target Crate(s):** `former_meta`, `former`
    *   **Goal:** Fix the `former_meta` bug or add necessary trait bounds to allow deriving `Former` on generic enums with unit variants.
    *   **Status:** Blocked. Despite adding `#[scalar]` to the generic `Value(T)` variant and improving generic handling in `unit_variant_handler.rs`, the derive still fails with "missing generics for enum GenericOption". This suggests that other variant handlers (e.g., `tuple_single_field_scalar.rs` for `Value(T)`) might also need corrections for generic parameter handling, or there's a more fundamental issue with generic enum expansion.
    *   **Detailed Plan Steps:**
        1.  **Restore Test Case:** (Completed)
        2.  **Analyze Trait Bound Issue in `former_meta`:** (Analysis performed)
        3.  **Attempt Fix 1: Add Trait Bounds to Test Case:** (Attempted various bounds, then switched to `#[scalar]` attribute on `Value(T)`)
        4.  **Attempt Fix 2 (If Fix 1 Fails): Modify `former_meta`:** (Improved generics in `unit_variant_handler.rs`. Further fixes in other handlers like `tuple_single_field_scalar.rs` would be needed but are too complex for this iteration.)
        5.  **Verify Fix:** (Still fails with "missing generics for enum GenericOption")
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow]
    *   **Relevant Behavioral Rules:** Rules for unit variants (1a, 3a, 4a) should apply to `NoValue`.
    *   **Verification Strategy:** `generic_unit_variant_derive` test suite fails.
    *   **Commit Message:** `test(former_meta): Isolate generic enum derive error, still unresolved`

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
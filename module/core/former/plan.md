# Former Standalone Constructors Feature Plan

This plan outlines the steps to implement and verify the `#[standalone_constructors]` and `#[arg_for_constructor]` features for the `former` crate, adopting **Option 2** logic where `#[arg_for_constructor]` solely determines constructor arguments and return type.

## Progress Summary

*   [✅] **Increment 1:** Verify Zero-Argument Standalone Constructors (Existing Files - Modified)
*   [✅] **Increment 2:** Create New Test Files for Argument Constructors (Enums & Structs)
*   [⬜] **Increment 3 (Rework):** Modify Derive Macro for Option 2 Logic (Enums)
*   [⬜] **Increment 4 (Rework):** Update Manual Implementation for Option 2 (Enums)
*   [⬜] **Increment 5 (Rework):** Update Tests for Option 2 (Enums)
*   [⬜] **Increment 6 (Rework):** Verify Enum Tests (Option 2)
*   [⬜] **Increment 7 (Rework):** Implement Manual Argument Constructor Tests (Structs - Option 2)
*   [⬜] **Increment 8 (Rework):** Implement Derive Argument Constructor Tests (Structs - Option 2)
*   [⬜] **Increment 9 (Rework):** Update Documentation

## Detailed Plan

1.  **Increment 1: Verify Zero-Argument Standalone Constructors (Existing Files - Modified)**
    *   **Status:** ✅ Done
    *   **Goal:** Ensure the basic `#[standalone_constructors]` feature (without `#[arg_for_constructor]`) works correctly for both structs and enums using the *existing* test files, with argument-related tests commented out.
    *   **Files & Actions:**
        *   `standalone_constructor_manual.rs` (structs & enums): Ensured constructors take **zero** arguments.
        *   `standalone_constructor_derive.rs` (structs & enums): Ensured `#[standalone_constructors]` is present, but `#[arg_for_constructor]` is **commented out**.
        *   `standalone_constructor_only_test.rs` (structs & enums): Ensured **only** the zero-argument tests (`no_args_test`, `unit_variant_test`, `tuple_variant_test`, `struct_variant_test`) are **uncommented**. Commented out the `*_with_args_test` functions.
    *   **Verification:** Ran `cargo test`. All uncommented tests passed for both manual and derive targets.

2.  **Increment 2: Create New Test Files for Argument Constructors**
    *   **Status:** ✅ Done
    *   **Goal:** Set up the file structure for testing the `#[arg_for_constructor]` feature separately.
    *   **Action:**
        *   Created `module/core/former/tests/inc/former_struct_tests/standalone_constructor_args_manual.rs`.
        *   Created `module/core/former/tests/inc/former_struct_tests/standalone_constructor_args_derive.rs`.
        *   Created `module/core/former/tests/inc/former_struct_tests/standalone_constructor_args_only_test.rs`.
        *   Created `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`.
        *   Created `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs`.
        *   Created `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`.
        *   Added `mod standalone_constructor_args_manual;` and `mod standalone_constructor_args_derive;` to `module/core/former/tests/inc/former_struct_tests/mod.rs`.
        *   Added `mod standalone_constructor_args_manual;` and `mod standalone_constructor_args_derive;` to `module/core/former/tests/inc/former_enum_tests/mod.rs`.

3.  **Increment 3 (Rework): Modify Derive Macro for Option 2 Logic (Enums)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Update `former_enum.rs` to generate standalone constructors according to Option 2 rules (checking if all fields have `#[arg_for_constructor]` to determine return type and body). Remove dependency on `#[scalar]` for standalone constructor generation.
    *   **File:** `module/core/former_meta/src/derive_former/former_enum.rs`

4.  **Increment 4 (Rework): Update Manual Implementation for Option 2 (Enums)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Align the manual enum implementation (`standalone_constructor_args_manual.rs`) with Option 2 logic. Constructors for variants where all fields are args should return `Self`. Others return the Former.
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`

5.  **Increment 5 (Rework): Update Tests for Option 2 (Enums)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Adjust tests in `standalone_constructor_args_only_test.rs` to match Option 2 expectations (check return type based on whether all fields are args).
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`

6.  **Increment 6 (Rework): Verify Enum Tests (Option 2)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Run tests and ensure they pass for both manual and derive implementations according to Option 2 logic.
    *   **Action:** `cargo test`.

7.  **Increment 7 (Rework): Implement Manual Argument Constructor Tests (Structs - Option 2)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Implement manual struct tests reflecting Option 2 (constructor returns `Self` if all fields have `#[arg_for_constructor]`, otherwise returns `Former`).
    *   **Files:** `standalone_constructor_args_manual.rs` (struct), `standalone_constructor_args_only_test.rs` (struct).

8.  **Increment 8 (Rework): Implement Derive Argument Constructor Tests (Structs - Option 2)**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Implement derive struct tests reflecting Option 2. Ensure derive logic in `former_struct.rs` is updated if necessary.
    *   **Files:** `standalone_constructor_args_derive.rs` (struct), `standalone_constructor_args_only_test.rs` (struct), `module/core/former_meta/src/derive_former/former_struct.rs`.

9.  **Increment 9 (Rework): Update Documentation**
    *   **Status:** ⬜ Not Started
    *   **Goal:** Document Option 2 behavior for the attributes.
    *   **Files:**
        *   `module/core/former/Readme.md`
        *   `module/core/former/advanced.md`
        *   `module/core/former_meta/src/lib.rs`

## Notes / Struggling Points / Insights

*   **Initial Struggle (Enum Tests):** Encountered significant difficulty verifying the `#[arg_for_constructor]` implementation for enums using the initial test setup (`standalone_constructor_manual.rs`, `_derive.rs`, `_only_test.rs`). The shared test file (`_only_test.rs`) contained tests for both zero-argument and argument-taking constructors.
*   **Conflict:** The manual implementation (`_manual.rs`) could only define standalone constructors with a single signature (either zero-args or arg-taking). This created a conflict:
    *   If manual constructors took zero args, the argument-taking tests failed compilation against the manual target.
    *   If manual constructors took arguments, the zero-argument tests failed compilation against the manual target.
*   **Resolution/Insight:** The correct approach was to **separate the test cases**.
    *   The original files (`standalone_constructor_*`) were adjusted to *only* test the zero-argument constructor scenario (where `#[arg_for_constructor]` is absent or commented out).
    *   New files (`standalone_constructor_args_*`) were created specifically to test the argument-taking constructor scenario (where `#[arg_for_constructor]` is active). This resolved the conflict and allowed independent verification of both scenarios for manual and derive implementations.
*   **Derive vs Manual Behavior:** Realized that standalone constructors for non-unit enum variants (even scalar ones) should return a `Former` type, not `Self` directly. The tests were adjusted accordingly. (Note: This insight is now being revised based on the switch to Option 2).
*   **`#[scalar]` vs `#[arg_for_constructor]`:** Clarified that `#[scalar]` on an enum variant implies a direct *associated method* returning `Self`, but the *standalone constructor* still returns a Former. `#[arg_for_constructor]` controls the arguments for the standalone constructor (and potentially initial storage state). Using `#[arg_for_constructor]` within a `#[scalar]` variant is disallowed by the derive macro. (Note: This insight is now being revised based on the switch to Option 2).
*   **Decision Change:** Switched from implementing Option 1 (where `#[scalar]` dictated direct return) to **Option 2** (where `#[arg_for_constructor]` on *all* fields dictates direct return). This requires reworking the derive logic and tests for argument handling.

## General Notes

*   This plan adopts **Option 2**: `#[arg_for_constructor]` on fields solely determines the standalone constructor's arguments. Standalone constructor returns `Self` if *all* fields have `#[arg_for_constructor]`, otherwise returns the implicit `VariantFormer`. `#[scalar]` is irrelevant for standalone constructors.
*   Each increment involving code changes should be followed by running `cargo test` to ensure stability and verify the specific goal of the increment.
*   Warnings should be addressed as they appear.

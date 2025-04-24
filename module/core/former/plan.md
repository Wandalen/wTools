# Former Standalone Constructors Feature Plan

This plan outlines the steps to implement and verify the `#[standalone_constructors]` and `#[arg_for_constructor]` features for the `former` crate, adopting **Option 2** logic where `#[arg_for_constructor]` solely determines constructor arguments and return type.

## Progress Summary

*   [✅] **Increment 1:** Verify Zero-Argument Standalone Constructors (Existing Files - Modified)
*   [✅] **Increment 2:** Create New Test Files for Argument Constructors (Enums & Structs)
*   [✅] **Increment 3a (Rework - Option 2):** Parse `#[arg_for_constructor]` on enum variant fields.
*   [✅] **Increment 3b (Rework - Option 2):** Implement logic to determine constructor args based on attributes (Enums).
*   [✅] **Increment 3c (Rework - Option 2):** Implement logic to determine return type (Self vs Former) based on attributes (Enums).
*   [✅] **Increment 3d (Rework - Option 2):** Generate standalone constructor code for enums.
*   [✅] **Increment 4 (Rework - Option 2):** Update Manual Implementation for Option 2 (Enums)
*   [✅] **Increment 5a (Rework - Option 2):** Add test structure/harness for enum args.
*   [✅] **Increment 5b (Rework - Option 2):** Implement test case for tuple variant with args (Enums).
*   [✅] **Increment 5c (Rework - Option 2):** Implement test case for struct variant with args (Enums).
*   [✅] **Increment 6 (Rework - Option 2):** Verify Enum Tests (Option 2)
*   [✅] **Increment 7 (Rework - Option 2):** Implement Manual Argument Constructor Tests (Structs - Option 2)
*   [✅] **Increment 8 (Rework - Option 2):** Implement Derive Argument Constructor Tests (Structs - Option 2)
*   [✅] **Increment 9a (Rework - Option 2):** Update Readme.md examples.
*   [✅] **Increment 9b (Rework - Option 2):** Update advanced.md attribute reference.
*   [✅] **Increment 9c (Rework - Option 2):** Update lib.rs doc comments.

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

3.  **Increment 3a (Rework - Option 2): Parse `#[arg_for_constructor]` on enum variant fields.**
    *   **Status:** ✅ Done
    *   **Goal:** Update `former_enum.rs` to correctly parse and store the `#[arg_for_constructor]` attribute on fields *within* enum variants.
    *   **File:** `module/core/former_meta/src/derive_former/former_enum.rs`
    *   Detailed Plan Step 1: Locate the field processing logic within the variant loop in `former_enum.rs`.
    *   Detailed Plan Step 2: For both `syn::Fields::Unnamed` (tuple) and `syn::Fields::Named` (struct) variants, iterate through the fields.
    *   Detailed Plan Step 3: Inside the field iteration, use `FieldAttributes::from_attrs( field.attrs.iter() )?` to parse attributes for each field.
    *   Detailed Plan Step 4: Store the boolean result of `field_attrs.arg_for_constructor.value( false )` alongside other necessary field information (e.g., in a temporary struct or tuple used for code generation).
    *   Detailed Plan Step 5: Ensure parsing errors are handled and propagated correctly using `Result`.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Error Handling: Use a Centralized Approach]
    *   Verification Strategy: Compile `former_meta` crate (`cargo check -p former_meta`), Manually review changes in `former_enum.rs` to confirm attribute parsing logic is added for tuple and named variant fields.

4.  **Increment 3b (Rework - Option 2): Implement logic to determine constructor args based on attributes (Enums).**
    *   **Status:** ✅ Done
    *   **Goal:** In `former_enum.rs`, add logic to collect fields marked with `#[arg_for_constructor]` for a given variant and generate the corresponding function parameter list for the standalone constructor.
    *   **File:** `module/core/former_meta/src/derive_former/former_enum.rs`
    *   **(Completed as part of derive logic implementation)**

5.  **Increment 3c (Rework - Option 2): Implement logic to determine return type (Self vs Former) based on attributes (Enums).**
    *   **Status:** ✅ Done
    *   **Goal:** In `former_enum.rs`, implement the Option 2 rule: if *all* fields in a variant have `#[arg_for_constructor]`, the standalone constructor returns `Self`; otherwise, it returns the appropriate `Former` type. Handle unit variants (always return `Self`).
    *   **File:** `module/core/former_meta/src/derive_former/former_enum.rs`
    *   **(Completed as part of derive logic implementation)**

6.  **Increment 3d (Rework - Option 2): Generate standalone constructor code for enums.**
    *   **Status:** ✅ Done
    *   **Goal:** In `former_enum.rs`, generate the final `fn` code for each variant's standalone constructor, incorporating the parameters and return type determined in previous steps. Ensure correct initialization of the `FormerStorage` or direct construction of `Self`.
    *   **File:** `module/core/former_meta/src/derive_former/former_enum.rs`
    *   **(Completed as part of derive logic implementation)**

7.  **Increment 4 (Rework - Option 2): Update Manual Implementation for Option 2 (Enums)**
    *   **Status:** ✅ Done
    *   **Goal:** Align the manual enum implementation (`standalone_constructor_args_manual.rs`) with Option 2 logic. Constructors for variants where all fields are args should return `Self`. Others return the Former.
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`
    *   **(Completed)**

8.  **Increment 5a (Rework - Option 2): Add test structure/harness for enum args.**
    *   **Status:** ✅ Done
    *   **Goal:** Set up the basic structure and necessary imports in `standalone_constructor_args_only_test.rs` for enum tests.
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   **(Completed via test updates)**

9.  **Increment 5b (Rework - Option 2): Implement test case for tuple variant with args (Enums).**
    *   **Status:** ✅ Done
    *   **Goal:** Add specific `#[test]` function(s) in `standalone_constructor_args_only_test.rs` to verify the standalone constructor for enum tuple variants, checking both argument handling and return type (Self vs Former based on Option 2).
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   **(Completed via test updates)**

10. **Increment 5c (Rework - Option 2): Implement test case for struct variant with args (Enums).**
    *   **Status:** ✅ Done
    *   **Goal:** Add specific `#[test]` function(s) in `standalone_constructor_args_only_test.rs` to verify the standalone constructor for enum struct variants, checking argument handling and return type (Self vs Former based on Option 2).
    *   **File:** `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   **(Completed via test updates)**

11. **Increment 6 (Rework - Option 2): Verify Enum Tests (Option 2)**
    *   **Status:** ✅ Done
    *   **Goal:** Run tests (`cargo test --test standalone_constructor_args_*`) and ensure they pass for both manual and derive implementations according to Option 2 logic for enums. Debug and fix any failures.
    *   **Action:** `cargo test`
    *   **(Completed)**

12. **Increment 7 (Rework - Option 2): Implement Manual Argument Constructor Tests (Structs - Option 2)**
    *   **Status:** ✅ Done
    *   **Goal:** Implement manual struct tests reflecting Option 2 (constructor returns `Self` if all fields have `#[arg_for_constructor]`, otherwise returns `Former`). Update manual constructor functions and tests accordingly.
    *   **Files:** `standalone_constructor_manual.rs` (struct), `standalone_constructor_only_test.rs` (struct).
    *   **(Completed - No changes needed to manual file, tests already aligned)**

13. **Increment 8 (Rework - Option 2): Implement Derive Argument Constructor Tests (Structs - Option 2)**
    *   **Status:** ✅ Done
    *   **Goal:** Implement derive struct tests reflecting Option 2. Ensure derive logic in `former_struct.rs` is updated if necessary (likely needs similar logic as enums for return type). Verify tests pass.
    *   **Files:** `standalone_constructor_derive.rs` (struct), `standalone_constructor_only_test.rs` (struct), `module/core/former_meta/src/derive_former/former_struct.rs`.
    *   **(Completed - Derive logic updated, tests already aligned)**

14. **Increment 9a (Rework - Option 2): Update Readme.md examples.**
    *   **Status:** ✅ Done
    *   **Goal:** Update examples in `Readme.md` to reflect the new standalone constructor usage (Option 2).
    *   **File:** `module/core/former/Readme.md`
    *   **(Completed)**

15. **Increment 9b (Rework - Option 2): Update advanced.md attribute reference.**
    *   **Status:** ✅ Done
    *   **Goal:** Update the attribute reference in `advanced.md` for `#[standalone_constructors]` and `#[arg_for_constructor]` based on Option 2 behavior.
    *   **File:** `module/core/former/advanced.md`
    *   **(Completed)**

16. **Increment 9c (Rework - Option 2): Update lib.rs doc comments.**
    *   **Status:** ✅ Done
    *   **Goal:** Update the main derive macro documentation in `former_meta/src/lib.rs` to accurately describe the new attributes and their behavior.
    *   **File:** `module/core/former_meta/src/lib.rs`
    *   **(Completed)**

## Notes & Insights

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
*   **Compilation Errors:** Fixed several compilation errors in `former_meta` related to missing `Clone` derives and incorrect type usage/parsing in `former_enum.rs`.
*   **Refactoring:** Refactored `former_enum.rs` to correctly handle Option 2 logic for standalone constructors in named/struct variants, resolving duplicate definition errors.
*   **Doc Test Fixes:** Corrected doc tests in `Readme.md` (included via `lib.rs`) to use correct types and function names.

## General Notes

*   This plan adopts **Option 2**: `#[arg_for_constructor]` on fields solely determines the standalone constructor's arguments. Standalone constructor returns `Self` if *all* fields have `#[arg_for_constructor]`, otherwise returns the appropriate `Former`. `#[scalar]` is irrelevant for standalone constructors.
*   Each increment involving code changes should be followed by running `cargo test` to ensure stability and verify the specific goal of the increment.
*   Warnings should be addressed as they appear.
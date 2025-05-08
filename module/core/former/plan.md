
# Project Plan: Finalizing `former` Enum Tests (Usecases and Known Issues)

## Goal
*   Integrate and verify the remaining specific enum test files in `module/core/former/tests/inc/former_enum_tests/` (`usecase1.rs`, `subform_collection_test.rs`) after the completion of unit, tuple, and named variant test plans.
*   Ensure the behavior observed in these tests aligns with the established "Expected Enum Former Behavior Rules".
*   Resolve the known compilation issue in `subform_collection_test.rs` based on a user-defined strategy.
*   Perform a final verification run of the entire `former` test suite.
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.

## Relevant Context

*   **Primary Test Directory:** `module/core/former/tests/inc/former_enum_tests/`
    *   `usecase1.rs`
    *   `subform_collection_test.rs`
*   **Enum Test Module File:** `module/core/former/tests/inc/former_enum_tests/mod.rs` (Assumed created and populated).
*   **Main Test Module File (Parent):** `module/core/former/tests/inc/mod.rs`.
*   **Macro Implementation:** `module/core/former_meta/src/derive_former/former_enum/` (and its submodules).
*   **Core Types & Traits:** `module/core/former_types/src/lib.rs`.
*   **Documentation:**
    *   `module/core/former/advanced.md`
    *   `module/core/former/Readme.md`
    *   Test Matrices documented in `former_enum_tests/mod.rs`.

### Expected Enum Former Behavior Rules

These rules define the expected code generation behavior for `#[derive(Former)]` on enums, based on variant structure and attributes.

1.  **`#[scalar]` Attribute (on variant):**
    *   **Unit Variant (`V`):** Generates `Enum::variant() -> Enum`. (Rule 1a)
    *   **Zero-Field Tuple Variant (`V()`):** Generates `Enum::variant() -> Enum`. (Rule 1b)
    *   **Zero-Field Struct Variant (`V {}`):** Generates `Enum::variant() -> Enum`. (Rule 1c)
    *   **Single-Field Tuple Variant (`V(T1)`):** Generates `Enum::variant(T1) -> Enum`. (Rule 1d)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant { f1: T1 } -> Enum`. (Rule 1e)
    *   **Multi-Field Tuple Variant (`V(T1, T2, ...)`):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Rule 1f)
    *   **Multi-Field Struct Variant (`V { f1: T1, ... }`):** Generates `Enum::variant { f1: T1, ... } -> Enum`. (Rule 1g)
    *   *Error Cases:* Cannot be combined with `#[subform_scalar]` on the same variant.

2.  **`#[subform_scalar]` Attribute (on variant):**
    *   **Unit Variant:** Error. (Rule 2a)
    *   **Zero-Field Variant (Tuple or Struct):** Error. (Rule 2b, 2c)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`):** Generates `Enum::variant() -> T1Former<...>` (former for the field's type). (Rule 2d)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`):** Error. (Rule 2d)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Rule 2e)
    *   **Multi-Field Tuple Variant:** Error. (Rule 2f)
    *   **Multi-Field Struct Variant (`V { f1: T1, ... }`):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Rule 2g)

3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):**
    *   **Unit Variant (`V`):** Generates `Enum::variant() -> Enum`. (Rule 3a)
    *   **Zero-Field Tuple Variant (`V()`):** Generates `Enum::variant() -> Enum`. (Rule 3b)
    *   **Zero-Field Struct Variant (`V {}`):** Error (requires `#[scalar]` or fields). (Rule 3c)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` derives `Former`):** Generates `Enum::variant() -> T1Former<...>`. (Rule 3d.i)
    *   **Single-Field Tuple Variant (`V(T1)` where `T1` does NOT derive `Former`):** Generates `Enum::variant(T1) -> Enum`. (Rule 3d.ii)
    *   **Single-Field Struct Variant (`V { f1: T1 }`):** Generates `Enum::variant() -> VariantFormer<...>`. (Rule 3e)
    *   **Multi-Field Tuple Variant (`V(T1, T2, ...)`):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Rule 3f)
    *   **Multi-Field Struct Variant (`V { f1: T1, ... }`):** Generates `Enum::variant() -> VariantFormer<...>`. (Rule 3g)

4.  **`#[standalone_constructors]` Attribute (on enum):**
    *   Generates top-level constructor functions for each variant (e.g., `fn my_variant(...)`).
    *   **Return Type & Arguments (Option 2 Logic):**
        *   If **all** fields of a variant are marked `#[arg_for_constructor]`: `fn my_variant(arg1: T1, ...) -> Enum`.
        *   If **zero or some** fields are marked `#[arg_for_constructor]`:
            *   If the variant's default/scalar behavior yields `Enum::variant(all_args) -> Enum`: `fn my_variant(marked_args...) -> EnumVariantFormerForRemainingArgs`. (Requires implicit variant former).
            *   If the variant's default/scalar behavior yields `Enum::variant() -> Enum` (Unit/Zero-Tuple/Scalar-Zero-Struct): `fn my_variant() -> Enum`.
            *   If the variant's default/subform behavior yields `Enum::variant() -> SpecificFormer`: `fn my_variant(marked_args...) -> SpecificFormer` (with args pre-set).

### Test Matrix Coverage
*   `usecase1.rs` primarily tests **Rule 3d.i**.
*   `subform_collection_test.rs` attempts to test `#[subform_entry]` on `Vec<Enum>`, which is currently not defined by the rules above.

### Target File Structure
*   Utilizes the structure established in the previous plan, with test modules declared in `module/core/former/tests/inc/former_enum_tests/mod.rs`.

### Failure Diagnosis Algorithm
*   (Standard algorithm as previously defined)

## Increments

*   [⚫] **Increment 1: Activate and Verify `usecase1.rs`**
    *   **Goal:** Uncomment, fix (if necessary), and verify the `usecase1.rs` test.
    *   **Files:** `usecase1.rs`, `former_enum_tests/mod.rs`.
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs` to uncomment `mod usecase1;`.
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/usecase1.rs`: Uncomment all the code within the file. Address any `xxx`/`qqq` comments if present.
    *   **Pre-Analysis:** The enum `FunctionStep` has variants like `Prompt(Prompt)`, `Break(Break)`, etc., where the inner types derive `Former`. The default behavior (Rule 3d.i) should generate static methods returning subformers (e.g., `FunctionStep::prompt() -> PromptFormer<...>`). The test `enum_variant_subformer_construction` calls these expected methods.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 3d.i.
    *   **Verification Strategy:**
        1.  Run `cargo check --tests --package former`. Fix any compilation errors, ensuring generated code matches Rule 3d.i.
        2.  Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::usecase1`. Analyze failures using the diagnosis algorithm.

*   [⚫] **Increment 2: Resolve `subform_collection_test.rs` (Known Issue)**
    *   **Goal:** Address the known compilation failure related to using `#[subform_entry]` on a `Vec<Enum>`.
    *   **Files:** `subform_collection_test.rs`, `former_enum_tests/mod.rs`. Potentially `former_meta` files.
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs` to uncomment `mod subform_collection_test;`.
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`: Uncomment the code within the file. Address any `xxx`/`qqq` comments.
    *   **Detailed Plan Step 3 (Decision Point):**
        *   Run `cargo check --tests --package former`. Confirm it fails compilation as expected (no current rule defines behavior for `#[subform_entry]` on `Vec<Enum>`).
        *   **Present the failure to the user and ask for a decision:**
            *   **Option A: Implement Feature:** Define behavior and implement in `former_meta`. *Estimate: High effort.*
            *   **Option B: Change Test Expectation:** Modify test to use `trybuild` and assert compilation failure. *Estimate: Medium effort.*
            *   **Option C: Remove/Comment Test:** Remove/comment out the test. *Estimate: Low effort.*
    *   **Detailed Plan Step 4 (Execution based on decision):** Implement the chosen option (A, B, or C).
    *   **Pre-Analysis:** The feature `#[subform_entry]` on `Vec<Enum>` is not covered by the current Expected Behavior Rules.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow) (if implementing), [Testing: Avoid Writing Automated Tests Unless Asked](#testing-avoid-writing-tests-unless-asked) (if removing).
    *   **Verification Strategy:** Depends on the chosen option. Requires user confirmation of the strategy before Step 4.

*   [⚫] **Increment 3: Final Full Suite Verification**
    *   **Goal:** Ensure all activated enum tests and the entire `former` crate test suite pass.
    *   **Detailed Plan Step 1:** Verify all relevant `mod` declarations in `former_enum_tests/mod.rs` are uncommented (reflecting the outcome of Increment 2).
    *   **Detailed Plan Step 2:** Run `cargo check --all-targets --package former`. Address any remaining errors or warnings.
    *   **Detailed Plan Step 3:** Run `cargo clippy --all-targets --package former --features full -- -D warnings`. Address any lints.
    *   **Detailed Plan Step 4:** Run `cargo test --package former --all-targets`. Ensure all tests pass.
    *   **Pre-Analysis:** Assumes previous increments were successful.
    *   **Verification Strategy:** Zero errors/warnings from `check` and `clippy`. All tests pass in the final `cargo test` run.

### Requirements
*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications.
*   **Detailed Increment Plan:** Before starting implementation of an increment, a detailed plan for *that increment only* must be generated and approved.
*   **Paired Testing:** Follow the [Proc Macro: Development Workflow](#proc-macro-development-workflow) rule where applicable.
*   **Incremental Verification:** Verify after each increment.
*   **Failure Analysis:** Follow the "Failure Diagnosis Algorithm".
*   **Minimal Changes:** Prioritize minimal changes.
*   **Approval Gates:** Obtain user approval before and after each increment, and **critically, before executing Step 4 of Increment 2**.

## Notes & Insights
*   This plan addresses the final specific enum test files identified.
*   The full "Expected Enum Former Behavior Rules" are included for context.
*   The resolution of `subform_collection_test.rs` requires explicit user direction.
*   A final full test run is included to catch any integration issues.

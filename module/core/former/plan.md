# Project Plan: Comprehensive Testing of `former` Crate for Enum Unit Variants

## Goal
*   Systematically test the `#[derive(Former)]` macro for Rust enum **unit variants**.
*   Cover combinations of relevant `former` attributes (`#[scalar]`, default behavior, `#[standalone_constructors]`) for unit variants, as defined in the "Test Matrix for Unit Variants".
*   Incrementally uncomment, pre-analyze, fix, and verify existing test files related to unit variants within `module/core/former/tests/inc/former_enum_tests/`.
*   **Embed the "Test Matrix for Unit Variants" (or a clear reference to it) as documentation within `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs` (or a central point in `inc/mod.rs` for `former_enum_tests`).**
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.

## Relevant Context

*   **Primary Test Directory:** `module/core/former/tests/inc/former_enum_tests/`
    *   `unit_variant_derive.rs`, `unit_variant_manual.rs`, `unit_variant_only_test.rs`.
*   **Main Test Module File:** `module/core/former/tests/inc/mod.rs` (declares `former_enum_tests` and its submodules).
*   **Macro Implementation:** `module/core/former_meta/src/derive_former/former_enum/`
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (main dispatch)
*   **Core Types & Traits:** `module/core/former_types/src/lib.rs`
*   **Documentation:**
    *   `module/core/former/advanced.md`
    *   `module/core/former/Readme.md`

### Test Matrix for Unit Variants

Factors to consider for unit variants (`enum MyEnum { MyUnitVariant }`):

1.  **Variant-Level Attribute:**
    *   None (Default behavior)
    *   `#[scalar]`
    *   `#[subform_scalar]` (Expected: Error, as per rules)
2.  **Enum-Level Attribute:**
    *   None
    *   `#[standalone_constructors]`
3.  **Field-Level Attribute `#[arg_for_constructor]`:** Not applicable to unit variants as they have no fields.

**Combinations to Test (Focusing on Valid/Expected Behaviors):**

| # | Variant Attribute | Enum Attribute              | Expected Constructor Signature (Static Method on Enum) | Expected Standalone Constructor (if `#[standalone_constructors]`) | Relevant Rule(s) | Handler File (Meta)        |
|---|-------------------|-----------------------------|------------------------------------------------------|--------------------------------------------------------------------|------------------|----------------------------|
| 1 | Default           | None                        | `MyEnum::my_unit_variant() -> MyEnum`                | N/A                                                                | 3a               | `unit_variant_handler.rs`  |
| 2 | `#[scalar]`       | None                        | `MyEnum::my_unit_variant() -> MyEnum`                | N/A                                                                | 1a               | `unit_variant_handler.rs`  |
| 3 | Default           | `#[standalone_constructors]` | `MyEnum::my_unit_variant() -> MyEnum`                | `fn my_unit_variant() -> MyEnum`                                   | 3a, 4            | `unit_variant_handler.rs`  |
| 4 | `#[scalar]`       | `#[standalone_constructors]` | `MyEnum::my_unit_variant() -> MyEnum`                | `fn my_unit_variant() -> MyEnum`                                   | 1a, 4            | `unit_variant_handler.rs`  |
| 5 | `#[subform_scalar]`| (Any)                       | *Compile Error*                                      | *Compile Error*                                                    | 2a               | (Dispatch logic in `former_enum.rs` should error) |

*(Note: "Default" for unit variants behaves like `#[scalar]`)*

### Target File Structure for Unit Variant Tests

The relevant files are within `module/core/former/tests/inc/former_enum_tests/`. Module declarations are in `module/core/former/tests/inc/mod.rs`.

```
module/core/former/tests/inc/
├── mod.rs                      // Declares `former_enum_tests` and its test files.
│                               // Potentially a place for high-level enum test matrix docs.
└── former_enum_tests/
    ├── unit_variant_derive.rs
    ├── unit_variant_manual.rs
    └── unit_variant_only_test.rs // Will contain the Test Matrix for Unit Variants documentation.
    // ... other enum test files ...
```

### Expected Enum Former Behavior Rules (Unit Variants Only)
(Same as before)

### Failure Diagnosis Algorithm (Abbreviated for this plan)
(Standard algorithm as previously defined)

## Increments

*   [✅] **Increment 1: Activate `former_enum_tests` Module & Document Unit Test Matrix**
    *   **Goal:** Ensure the `former_enum_tests` module is active and document the "Test Matrix for Unit Variants".
    *   **Detailed Plan Step 1:** Check `module/core/former/tests/inc/mod.rs`. If `mod former_enum_tests;` (or the block `mod former_enum_tests { ... }`) is commented or missing, add/uncomment it.
    *   **Detailed Plan Step 2:** Modify `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs`. Add a file-level documentation comment (`//!`) at the top, containing the "Test Matrix for Unit Variants" table and a brief explanation of its purpose for unit variant testing.
        *   Alternatively, if a more centralized approach for all enum test matrices is preferred later, this documentation could be moved to `module/core/former/tests/inc/mod.rs` within the `former_enum_tests` module block. For now, `unit_variant_only_test.rs` is suitable.
    *   **Pre-Analysis:** This step primarily involves documentation and module activation.
    *   **Verification Strategy:**
        *   Run `cargo check --tests --package former`. Expect compilation success.
        *   Manually review `unit_variant_only_test.rs` to ensure the matrix is correctly embedded and formatted.
    *   **Crucial Design Rules:** [Comments and Documentation](#comments-and-documentation).

*   [⏳] **Increment 2: Test Unit Variants - Default and `#[scalar]` Behavior (Combinations 1 & 2)**
    *   **Goal:** Uncomment and verify tests for unit variants with default behavior and with the `#[scalar]` attribute.
    *   **Files:** `unit_variant_derive.rs`, `unit_variant_manual.rs`, `unit_variant_only_test.rs`.
    *   **Matrix Coverage:** Combinations #1 and #2.
    *   **Pre-Analysis:**
        *   `unit_variant_derive.rs`: Enum `Status { Pending, Complete }`. Expects `Status::pending() -> Status` and `Status::complete() -> Status`.
        *   `unit_variant_manual.rs`: Should manually implement `Status::pending() -> Status` and `Status::complete() -> Status`.
        *   `unit_variant_only_test.rs`: Contains tests calling these static methods.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1a, 3a.
    *   **Verification Strategy:**
        1.  In `module/core/former/tests/inc/mod.rs` (within `mod former_enum_tests { ... }` if it's a block, or directly if flat), uncomment `mod unit_variant_manual;`.
        2.  Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::unit_variant_manual`. Analyze and fix if needed.
        3.  In `module/core/former/tests/inc/mod.rs`, uncomment `mod unit_variant_derive;`.
        4.  Run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::unit_variant_derive`. Analyze and fix if needed.
    *   **Detailed Plan Step 5:** Modify `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs` to generate static constructor methods for unit variants.
    *   **Detailed Plan Step 6:** Re-run `cargo test --package former --test tests -- --test-threads=1 --nocapture former_enum_tests::unit_variant_derive` to verify the fix.

*   [⚫] **Increment 3: Test Unit Variants - `#[standalone_constructors]` (Combinations 3 & 4)**
    *   **Goal:** Verify `#[standalone_constructors]` attribute on enums containing unit variants.
    *   **Files:** `unit_variant_derive.rs`, `unit_variant_manual.rs`, `unit_variant_only_test.rs`.
    *   **Matrix Coverage:** Combinations #3 and #4.
    *   **Pre-Analysis:**
        *   Modify/check `unit_variant_derive.rs`: Add `#[standalone_constructors]` to `Status` enum. Expect top-level `fn pending() -> Status` and `fn complete() -> Status`.
        *   Modify/check `unit_variant_manual.rs`: Manually implement equivalent top-level `fn pending() -> Status` and `fn complete() -> Status`.
        *   Modify/check `unit_variant_only_test.rs`: Add tests that call these top-level standalone constructors.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1a, 3a, 4.
    *   **Verification Strategy:** Staged testing as in Increment 2.

*   [⚫] **Increment 4: Test Unit Variants - `#[subform_scalar]` (Error Case - Combination 5)**
    *   **Goal:** Verify that using `#[subform_scalar]` on a unit variant results in a compile-time error.
    *   **Files:** Create `module/core/former/tests/inc/former_enum_tests/compile_fail/unit_subform_scalar_error.rs`.
    *   **Matrix Coverage:** Combination #5.
    *   **Pre-Analysis:** Define an enum with a unit variant annotated with `#[subform_scalar]`. Expect `former_meta` to produce a `syn::Error`.
    *   **Crucial Design Rules:** Expected Behavior Rule 2a.
    *   **Verification Strategy:** Add a `trybuild` test case. Ensure `former_meta` is a dev-dependency of `former` if `trybuild` tests are in the `former` crate, or adjust paths if `trybuild` tests are in `former_meta`.

### Requirements
*   (Same as previous plan)

## Notes & Insights
*   This plan focuses specifically on unit variants.
*   The "Test Matrix for Unit Variants" will be embedded in `unit_variant_only_test.rs` (or `inc/mod.rs`).
*   The "Expected Enum Former Behavior Rules" are simplified for unit variants.
*   **[5/7/2025] Increment 1 Complete:** Activated `former_enum_tests` module (it was already active) and documented the unit test matrix in `unit_variant_only_test.rs`. Verified with `cargo check`.
*   **[5/7/2025] Increment 2 Failed:** The derive test `former_enum_tests::unit_variant_derive` failed because the `#[derive(Former)]` macro is not generating the expected static constructor methods for unit variants. The next step is to fix the macro implementation in `former_meta`.

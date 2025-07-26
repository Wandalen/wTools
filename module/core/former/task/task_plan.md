# Task Plan: Complete Implementation for Unnamed Enum Variants

### Goal
*   To complete the implementation of the `#[derive(Former)]` procedural macro for enums with unnamed (tuple-style) variants within the `former_meta` crate. This will be achieved by methodically implementing the logic for each case defined in the specification and enabling the corresponding disabled tests in the `former` crate to verify the implementation.

### Ubiquitous Language (Vocabulary)
*   **Unnamed Variant:** An enum variant with tuple-style fields, e.g., `MyVariant(i2)`, `MyVariant()`, or `MyVariant(MyType)`.
*   **Scalar Constructor:** A generated method that takes all of the variant's fields as arguments and directly returns an instance of the enum (e.g., `Enum::my_variant(10, "hello") -> Enum`).
*   **Subform Constructor:** A generated method that takes no arguments and returns a `Former` for either the variant's inner type (if it has a single field that derives `Former`) or an implicit `Former` for the variant itself.
*   **Implicit Variant Former:** A `Former` struct that is generated automatically by the macro for a specific multi-field or struct-like enum variant, allowing its fields to be set individually.
*   **Standalone Constructor:** A top-level function (e.g., `my_variant()`) generated when `#[standalone_constructors]` is present on the enum.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/former_meta`
*   **Overall Progress:** 2/13 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Initial Analysis and Handler File Setup
    *   ✅ Increment 2: Implement Zero-Field Tuple Variant - Scalar Constructor (Rules 1b, 3b)
    *   ✅ Increment 3: Implement Zero-Field Tuple Variant - `#[subform_scalar]` Compile-Fail (Rule 2b)
    *   ⏳ Increment 4: Implement Single-Field Tuple Variant - Scalar Constructor (Rule 1d)
    *   ⚫ Increment 5: Implement Single-Field Tuple Variant - Subform Constructor (Rules 2d, 3d)
    *   ⚫ Increment 6: Implement Multi-Field Tuple Variant - Scalar Constructor (Rule 1f)
    *   ⚫ Increment 7: Implement Multi-Field Tuple Variant - Implicit Variant Former (Rule 3f)
    *   ⚫ Increment 8: Implement Multi-Field Tuple Variant - `#[subform_scalar]` Compile-Fail (Rule 2f)
    *   ⚫ Increment 9: Implement Standalone Constructors - Zero-Field Variants
    *   ⚫ Increment 10: Implement Standalone Constructors - Single-Field Variants
    *   ⚫ Increment 11: Implement Standalone Constructors - Multi-Field Variants
    *   ⚫ Increment 12: Update Documentation
    *   ⚫ Increment 13: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/former` (Reason: To enable and potentially fix tests)

### Relevant Context
*   **`macro_tools` API Signatures:** The implementation in `former_meta` must prefer utilities from `macro_tools`.
    *   `ident::cased_ident_from_ident(original: &syn::Ident, case: convert_case::Case) -> syn::Ident`: For converting variant `PascalCase` names to `snake_case` method names, correctly handling raw identifiers.
    *   `generic_params::GenericsRef`: A wrapper around `syn::Generics` with these methods:
        *   `.impl_generics_tokens_if_any() -> TokenStream`: Returns `<T: Trait>`.
        *   `.ty_generics_tokens_if_any() -> TokenStream`: Returns `<T>`.
        *   `.where_clause_tokens_if_any() -> TokenStream`: Returns `where T: Trait`.
        *   `.type_path_tokens_if_any(base_ident: &syn::Ident) -> TokenStream`: Returns `MyType<T>`.
    *   `syn_err!(span, "message")` and `return_syn_err!(span, "message")`: For generating clear, spanned compile-time errors.
    *   `qt!{...}`: As a replacement for `quote::quote!`.

### Expected Behavior Rules / Specifications
*   The implementation must adhere to the rules for unnamed (tuple) variants as defined in `spec.md`.

| Rule | Variant Structure | Attribute(s) | Generated Constructor Behavior |
| :--- | :--- | :--- | :--- |
| **1b** | Tuple: `V()` | `#[scalar]` or Default | Direct constructor: `Enum::v() -> Enum` |
| **1d** | Tuple: `V(T1)` | `#[scalar]` | Scalar constructor: `Enum::v(T1) -> Enum` |
| **1f** | Tuple: `V(T1, T2)` | `#[scalar]` | Scalar constructor: `Enum::v(T1, T2) -> Enum` |
| **2b** | Tuple: `V()` | `#[subform_scalar]` | **Compile Error** |
| **2d** | Tuple: `V(T1)` | `#[subform_scalar]` or Default | Subformer for inner type: `Enum::v() -> T1::Former` |
| **2f** | Tuple: `V(T1, T2)` | `#[subform_scalar]` | **Compile Error** |
| **3b** | Tuple: `V()` | Default | Direct constructor: `Enum::v() -> Enum` |
| **3d** | Tuple: `V(T1)` | Default | Subformer for inner type: `Enum::v() -> T1::Former` |
| **3f** | Tuple: `V(T1, T2)` | Default | **Implicit variant former: `Enum::v() -> VFormer`** |

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `tuple_zero_fields_*.rs` | Fixed (Monitored) | `test_zero_field_default_static_constructor` passed unexpectedly. |
| `compile_fail/tuple_zero_subform_scalar_error.rs` | Fixed (Monitored) | Test failed with expected compile error. |
| `scalar_generic_tuple_*.rs` | Not Started | |
| `basic_*.rs` | Not Started | |
| `generics_shared_tuple_*.rs` | Not Started | |
| `usecase1_*.rs` | Not Started | |
| `tuple_multi_scalar_*.rs` | Not Started | |
| `tuple_multi_default_*.rs` | Not Started | |
| `compile_fail/tuple_multi_subform_scalar_error.rs` | Not Started | |
| `standalone_constructor_tuple_*.rs` | Not Started | |
| `standalone_constructor_args_tuple_*.rs` | Not Started | |
| `tuple_multi_standalone_*.rs` | Not Started | |
| `tuple_multi_standalone_args_*.rs` | Not Started | |

### Crate Conformance Check Procedure
*   **Step 1: Run Build.** Execute `timeout 300 cargo build --workspace`. If this fails, fix all compilation errors before proceeding.
*   **Step 2: Run Tests (Conditional).** Only if Step 1 passes, execute `timeout 300 cargo test --workspace`.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 300 cargo clippy --workspace --all-targets -- -D warnings`.

### Increments
(Note: The status of each increment is tracked in the `### Progress` section.)
##### Increment 1: Initial Analysis and Handler File Setup
*   **Goal:** Understand the current state of the `enum_unnamed_tests` module and create the necessary handler files in `former_meta`.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `list_files` to recursively list all files in `module/core/former/tests/inc/enum_unnamed_tests/`.
    2.  Use `read_file` to inspect `module/core/former/tests/inc/enum_unnamed_tests/mod.rs` to identify which test modules are currently commented out.
    3.  Use `read_file` to inspect `module/core/former_meta/src/derive_former/former_enum.rs` to understand the current dispatch logic.
    4.  Create the necessary handler files in `module/core/former_meta/src/derive_former/former_enum/` as placeholders: `tuple_zero_fields_handler.rs`, `tuple_single_field_scalar.rs`, `tuple_single_field_subform.rs`, `tuple_multi_fields_scalar.rs`.
    5.  Use `insert_content` to add the new `mod` declarations for the created files into `module/core/former_meta/src/derive_former/former_enum.rs`.
*   **Increment Verification:**
    *   Confirm that the new handler files have been created and declared as modules.
*   **Commit Message:** "chore(former_meta): Setup handler files for unnamed enum variants"

##### Increment 2: Implement Zero-Field Tuple Variant - Scalar Constructor (Rules 1b, 3b)
*   **Goal:** Implement the direct scalar constructor for zero-field tuple variants like `MyVariant()`.
*   **Specification Reference:** Rules 1b, 3b.
*   **Steps:**
    1.  In `module/core/former/tests/inc/enum_unnamed_tests/mod.rs`, uncomment the `tuple_zero_fields_derive` and `tuple_zero_fields_manual` modules.
    2.  Execute `cargo test --package former --test tests -- --nocapture test_zero_field_default_static_constructor`. Expect failure.
    3.  Implement the logic in `module/core/former_meta/src/derive_former/former_enum/tuple_zero_fields_handler.rs` to generate a direct constructor.
    4.  Update the dispatch logic in `former_enum.rs` to call this handler for zero-field tuple variants.
    5.  Execute `cargo test --package former --test tests -- --nocapture tuple_zero_fields`. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
    7.  Perform Crate Conformance Check.
*   **Increment Verification:**
    *   The `tuple_zero_fields` test passes.
*   **Commit Message:** "feat(former): Implement scalar constructor for zero-field tuple variants"

##### Increment 3: Implement Zero-Field Tuple Variant - `#[subform_scalar]` Compile-Fail (Rule 2b)
*   **Goal:** Ensure using `#[subform_scalar]` on a zero-field tuple variant results in a compile-time error.
*   **Specification Reference:** Rule 2b.
*   **Steps:**
    1.  In `module/core/former/tests/inc/enum_unnamed_tests/compile_fail/mod.rs`, uncomment the test for `tuple_zero_subform_scalar_error.rs`.
    2.  Execute `cargo test --package former --test tests -- --nocapture former_trybuild`. Expect the test to fail if the check is missing.
    3.  In `tuple_zero_fields_handler.rs`, add a check to detect `#[subform_scalar]` and return a `syn::Error`.
    4.  Execute `cargo test --package former --test tests -- --nocapture former_trybuild` again. Expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `tuple_zero_subform_scalar_error` compile-fail test passes.
*   **Commit Message:** "fix(former): Add compile error for subform_scalar on zero-field tuple variant"

##### Increment 4: Implement Single-Field Tuple Variant - Scalar Constructor (Rule 1d)
*   **Goal:** Implement the scalar constructor for single-field tuple variants like `MyVariant(i32)` when `#[scalar]` is used.
*   **Specification Reference:** Rule 1d.
*   **Steps:**
    1.  Uncomment the `scalar_generic_tuple_derive` and `scalar_generic_tuple_manual` modules in `enum_unnamed_tests/mod.rs`.
    2.  Run `cargo test --package former --test tests -- --nocapture scalar_on_single_generic_tuple_variant`. Expect failure.
    3.  Implement the logic in `tuple_single_field_scalar.rs` to generate a constructor that takes the inner type as an argument.
    4.  Update dispatch logic in `former_enum.rs`.
    5.  Run the test again. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
    7.  Perform Crate Conformance Check.
*   **Increment Verification:**
    *   The `scalar_on_single_generic_tuple_variant` test passes.
*   **Commit Message:** "feat(former): Implement scalar constructor for single-field tuple variants"

##### Increment 5: Implement Single-Field Tuple Variant - Subform Constructor (Rules 2d, 3d)
*   **Goal:** Implement the subform constructor for single-field tuple variants, which returns a former for the inner type.
*   **Specification Reference:** Rules 2d, 3d.
*   **Steps:**
    1.  Uncomment `basic_derive`, `basic_manual`, `generics_shared_tuple_derive`, `generics_shared_tuple_manual`, and `usecase1_derive` modules in `enum_unnamed_tests/mod.rs`.
    2.  Run `cargo test --package former --test tests -- --nocapture build_break_variant_static`. Expect failure.
    3.  Implement logic in `tuple_single_field_subform.rs` to generate a method that returns `T1::Former`. This involves generating the appropriate `End` condition struct and `FormingEnd` implementation.
    4.  Update dispatch logic.
    5.  Run all newly enabled tests. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
    7.  Perform Crate Conformance Check.
*   **Increment Verification:**
    *   All subform single-field tuple tests pass.
*   **Commit Message:** "feat(former): Implement subform constructor for single-field tuple variants"

##### Increment 6: Implement Multi-Field Tuple Variant - Scalar Constructor (Rule 1f)
*   **Goal:** Implement the scalar constructor for multi-field tuple variants like `MyVariant(i32, bool)` when `#[scalar]` is used.
*   **Specification Reference:** Rule 1f.
*   **Steps:**
    1.  Uncomment `tuple_multi_scalar_derive` and `tuple_multi_scalar_manual` modules.
    2.  Run `cargo test --package former --test tests -- --nocapture tuple_multi_scalar_only_test`. Expect failure.
    3.  Implement logic in `tuple_multi_fields_scalar.rs` to generate a constructor taking all fields as arguments.
    4.  Update dispatch logic.
    5.  Run the test again. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `tuple_multi_scalar` tests pass.
*   **Commit Message:** "feat(former): Implement scalar constructor for multi-field tuple variants"

##### Increment 7: Implement Multi-Field Tuple Variant - Implicit Variant Former (Rule 3f)
*   **Goal:** Implement the default behavior for multi-field tuple variants, which generates an implicit former for the variant itself.
*   **Specification Reference:** Rule 3f.
*   **Steps:**
    1.  **Analysis:** Read `tuple_multi_default_only_test.rs`. Note that it currently tests for a scalar constructor, which contradicts Rule 3f.
    2.  **Test Refactoring:** Modify `tuple_multi_default_manual.rs` and `tuple_multi_default_only_test.rs` to reflect the expected "implicit variant former" behavior. The test should now expect a `variant()` method that returns a former, which has setters like `._0()` and `._1()`.
    3.  Uncomment `tuple_multi_default_derive` and `tuple_multi_default_manual` modules.
    4.  Run the refactored test. Expect failure.
    5.  Implement logic in a new `tuple_multi_fields_subform.rs` handler to generate a full `Former` ecosystem (Storage, Definition, Former struct with setters) for the variant.
    6.  Update dispatch logic in `former_enum.rs` to use this new handler for the default multi-field tuple case.
    7.  Run the test again. Expect success.
    8.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The refactored `tuple_multi_default` tests pass.
*   **Commit Message:** "feat(former): Implement implicit variant former for multi-field tuple variants"

##### Increment 8: Implement Multi-Field Tuple Variant - `#[subform_scalar]` Compile-Fail (Rule 2f)
*   **Goal:** Ensure using `#[subform_scalar]` on a multi-field tuple variant results in a compile-time error.
*   **Specification Reference:** Rule 2f.
*   **Steps:**
    1.  Uncomment the `trybuild` test for `tuple_multi_subform_scalar_error.rs`.
    2.  Run the `trybuild` test and expect failure if the check is missing.
    3.  Add a check in the `former_enum.rs` dispatch logic to error on this combination.
    4.  Run the `trybuild` test again and expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `tuple_multi_subform_scalar_error` compile-fail test passes.
*   **Commit Message:** "fix(former): Add compile error for subform_scalar on multi-field tuple variant"

##### Increment 9: Implement Standalone Constructors - Zero-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for zero-field tuple variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  In `tuple_zero_fields_only_test.rs`, enable the standalone constructor tests.
    2.  Run tests; expect failure.
    3.  Modify `tuple_zero_fields_handler.rs` to check for `ctx.struct_attrs.standalone_constructors` and generate the top-level function.
    4.  Run tests; expect success.
*   **Increment Verification:**
    *   Standalone constructor tests in `tuple_zero_fields_only_test.rs` pass.
*   **Commit Message:** "feat(former): Add standalone constructors for zero-field tuple variants"

##### Increment 10: Implement Standalone Constructors - Single-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for single-field tuple variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  Uncomment `standalone_constructor_tuple_derive` and `standalone_constructor_args_tuple_*` modules.
    2.  Run tests; expect failure.
    3.  Modify `tuple_single_field_scalar.rs` and `tuple_single_field_subform.rs` to generate standalone constructors, respecting `#[arg_for_constructor]` and Option 2 Logic for the return type.
    4.  Run tests; expect success.
*   **Increment Verification:**
    *   All `standalone_constructor_*` tests for single-field tuple variants pass.
*   **Commit Message:** "feat(former): Add standalone constructors for single-field tuple variants"

##### Increment 11: Implement Standalone Constructors - Multi-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for multi-field tuple variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  Uncomment `tuple_multi_standalone_derive` and `tuple_multi_standalone_args_derive` modules.
    2.  Run tests; expect failure.
    3.  Modify `tuple_multi_fields_scalar.rs` and the subform handler to generate standalone constructors, respecting `#[arg_for_constructor]` and Option 2 Logic.
    4.  Run tests; expect success.
*   **Increment Verification:**
    *   All `standalone_constructor_*` tests for multi-field tuple variants pass.
*   **Commit Message:** "feat(former): Add standalone constructors for multi-field tuple variants"

##### Increment 12: Update Documentation
*   **Goal:** Update user-facing documentation to reflect the completed enum support.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Read `module/core/former/Readme.md`.
    2.  Locate the `<!-- qqq : xxx : fix it -->` comment in the "Enum Standalone Constructors" section.
    3.  Replace the commented-out code block with a correct, working example of standalone constructors for an enum with unnamed (tuple) variants.
    4.  Read `module/core/former/advanced.md` and ensure the attribute reference is consistent with the implementation for tuple variants.
*   **Increment Verification:**
    *   The `Readme.md` file is updated with a correct example.
*   **Commit Message:** "docs(former): Update documentation for unnamed enum variant support"

##### Increment 13: Finalization
*   **Goal:** Perform a final verification of the entire workspace.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Ensure all test modules in `module/core/former/tests/inc/enum_unnamed_tests/mod.rs` are uncommented.
    2.  Perform a final Crate Conformance Check on the entire workspace.
    3.  Self-critique against all requirements and rules.
*   **Increment Verification:**
    *   All workspace checks pass.
*   **Commit Message:** "chore(former): Finalize unnamed enum variant implementation"

### Out of Scope
*   Implementing features for named (struct-like) or true unit enum variants.
*   Refactoring any code outside of the `former_meta` and `former` crates.
*   Adding new features not specified in the `spec.md` for unnamed variants.

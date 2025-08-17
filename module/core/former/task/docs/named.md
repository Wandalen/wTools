# Task Plan: Complete Implementation for Named Enum Variants

### Goal
*   To complete the implementation of the `#[derive(Former)]` procedural macro for enums with **named (struct-like) variants** within the `former_meta` crate. This will be achieved by methodically implementing the logic for each case defined in the specification and enabling the corresponding disabled tests in the `former` crate to verify the implementation.

### Ubiquitous Language (Vocabulary)
*   **Named Variant:** An enum variant with struct-like fields, e.g., `MyVariant { field: i32 }` or `MyVariant {}`.
*   **Scalar Constructor:** A generated method that takes all of the variant's fields as arguments and directly returns an instance of the enum.
*   **Implicit Variant Former:** A `Former` struct that is generated automatically by the macro for a specific multi-field or struct-like enum variant, allowing its fields to be set individually.
*   **Standalone Constructor:** A top-level function (e.g., `my_variant()`) generated when `#[standalone_constructors]` is present on the enum.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/former_meta`
*   **Overall Progress:** 0/12 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Initial Analysis and Handler File Setup
    *   ⚫ Increment 2: Implement Zero-Field Struct Variant - Scalar Constructor (Rule 1c)
    *   ⚫ Increment 3: Implement Zero-Field Struct Variant - Compile-Fail Rules (2c, 3c)
    *   ⚫ Increment 4: Implement Single-Field Struct Variant - Scalar Constructor (Rule 1e)
    *   ⚫ Increment 5: Implement Single-Field Struct Variant - Implicit Variant Former (Rules 2e, 3e)
    *   ⚫ Increment 6: Implement Multi-Field Struct Variant - Scalar Constructor (Rule 1g)
    *   ⚫ Increment 7: Implement Multi-Field Struct Variant - Implicit Variant Former (Rules 2g, 3g)
    *   ⚫ Increment 8: Implement Standalone Constructors - Zero-Field Variants
    *   ⚫ Increment 9: Implement Standalone Constructors - Single-Field Variants
    *   ⚫ Increment 10: Implement Standalone Constructors - Multi-Field Variants
    *   ⚫ Increment 11: Update Documentation
    *   ⚫ Increment 12: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/former` (Reason: To enable and potentially fix tests)

### Relevant Context
*   **`macro_tools` API Signatures:** The implementation in `former_meta` must prefer utilities from `macro_tools`.
    *   `ident::cased_ident_from_ident(original: &syn::Ident, case: convert_case::Case) -> syn::Ident`: For converting variant `PascalCase` names to `snake_case` method names.
    *   `generic_params::GenericsRef`: A wrapper around `syn::Generics` with these methods:
        *   `.impl_generics_tokens_if_any() -> TokenStream`: Returns `<T: Trait>`.
        *   `.ty_generics_tokens_if_any() -> TokenStream`: Returns `<T>`.
        *   `.where_clause_tokens_if_any() -> TokenStream`: Returns `where T: Trait`.
        *   `.type_path_tokens_if_any(base_ident: &syn::Ident) -> TokenStream`: Returns `MyType<T>`.
    *   `syn_err!(span, "message")` and `return_syn_err!(span, "message")`: For generating clear, spanned compile-time errors.
    *   `qt!{...}`: As a replacement for `quote::quote!`.

### Expected Behavior Rules / Specifications
*   The implementation must adhere to the rules for named (struct-like) variants as defined in `spec.md`.

| Rule | Variant Structure | Attribute(s) | Generated Constructor Behavior |
| :--- | :--- | :--- | :--- |
| **1c** | Struct: `V {}` | `#[scalar]` | Direct constructor: `Enum::v() -> Enum` |
| **1e** | Struct: `V {f1:T1}` | `#[scalar]` | Scalar constructor: `Enum::v{f1:T1} -> Enum` |
| **1g** | Struct: `V {f1:T1, f2:T2}` | `#[scalar]` | Scalar constructor: `Enum::v{f1:T1, f2:T2} -> Enum` |
| **2c** | Struct: `V {}` | `#[subform_scalar]` | **Compile Error** |
| **2e** | Struct: `V {f1:T1}` | `#[subform_scalar]` or Default | Implicit variant former: `Enum::v() -> VFormer` |
| **2g** | Struct: `V {f1:T1, f2:T2}` | `#[subform_scalar]` or Default | Implicit variant former: `Enum::v() -> VFormer` |
| **3c** | Struct: `V {}` | Default | **Compile Error** |
| **3e** | Struct: `V {f1:T1}` | Default | Implicit variant former: `Enum::v() -> VFormer` |
| **3g** | Struct: `V {f1:T1, f2:T2}` | Default | Implicit variant former: `Enum::v() -> VFormer` |

### Tests
| Test File | Status | Notes |
|---|---|---|
| `enum_named_fields_named_*.rs` | Not Started | |
| `compile_fail/struct_zero_*.rs` | Not Started | |
| `generics_independent_struct_*.rs` | Not Started | |
| `generics_shared_struct_*.rs` | Not Started | |
| `standalone_constructor_named_*.rs` | Not Started | |
| `standalone_constructor_args_named_*.rs` | Not Started | |

### Crate Conformance Check Procedure
*   **Step 1: Run Build.** Execute `timeout 300 cargo build --workspace`. If this fails, fix all compilation errors before proceeding.
*   **Step 2: Run Tests (Conditional).** Only if Step 1 passes, execute `timeout 300 cargo test --workspace`.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 300 cargo clippy --workspace --all-targets -- -D warnings`.

### Increments
##### Increment 1: Initial Analysis and Handler File Setup
*   **Goal:** Understand the current state of the `enum_named_tests` module and create the necessary handler files in `former_meta`.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Use `list_files` to recursively list all files in `module/core/former/tests/inc/enum_named_tests/`.
    2.  Use `read_file` to inspect `module/core/former/tests/inc/enum_named_tests/mod.rs` to identify which test modules are currently commented out.
    3.  Create the necessary handler files in `module/core/former_meta/src/derive_former/former_enum/` as placeholders: `struct_zero_fields_handler.rs`, `struct_single_field_scalar.rs`, `struct_single_field_subform.rs`, `struct_multi_fields_scalar.rs`, `struct_multi_fields_subform.rs`.
    4.  Use `insert_content` to add the new `mod` declarations for the created files into `module/core/former_meta/src/derive_former/former_enum.rs`.
*   **Increment Verification:**
    *   Confirm that the new handler files have been created and declared as modules.
*   **Commit Message:** "chore(former_meta): Setup handler files for named enum variants"

##### Increment 2: Implement Zero-Field Struct Variant - Scalar Constructor (Rule 1c)
*   **Goal:** Implement the direct scalar constructor for zero-field struct variants like `MyVariant {}`.
*   **Specification Reference:** Rule 1c.
*   **Context:** The target test file `enum_named_fields_named_only_test.rs` contains `variant_zero_scalar_test`, which tests this variant from `enum_named_fields_named_derive.rs`:
    ```rust
    // in enum EnumWithNamedFields
    VariantZeroScalar {}, // Expect: variant_zero_scalar() -> Enum
    ```
*   **Steps:**
    1.  In `module/core/former/tests/inc/enum_named_tests/mod.rs`, uncomment the `enum_named_fields_named_derive`, `_manual`, and `_only_test` modules.
    2.  Execute `cargo test --package former --test tests -- --nocapture variant_zero_scalar_test`. Expect failure.
    3.  Implement the logic in `module/core/former_meta/src/derive_former/former_enum/struct_zero_fields_handler.rs` to generate a direct constructor (e.g., `pub fn variant_zero_scalar() -> Self { Self::VariantZeroScalar {} }`).
    4.  Update the dispatch logic in `former_enum.rs` to call this handler for zero-field struct variants with `#[scalar]`.
    5.  Execute `cargo test --package former --test tests -- --nocapture variant_zero_scalar_test`. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `variant_zero_scalar_test` test passes.
*   **Commit Message:** "feat(former): Implement scalar constructor for zero-field struct variants"

##### Increment 3: Implement Zero-Field Struct Variant - Compile-Fail Rules (2c, 3c)
*   **Goal:** Ensure using `#[subform_scalar]` or no attribute on a zero-field struct variant results in a compile-time error.
*   **Specification Reference:** Rules 2c, 3c.
*   **Steps:**
    1.  In `module/core/former/tests/inc/enum_named_tests/compile_fail/mod.rs`, uncomment the tests for `struct_zero_default_error.rs` and `struct_zero_subform_scalar_error.rs`.
    2.  Execute `cargo test --package former --test tests -- --nocapture former_trybuild`. Expect failures.
    3.  In `former_enum.rs` dispatch logic, add checks to detect these invalid combinations and return a `syn::Error`.
    4.  Execute `cargo test --package former --test tests -- --nocapture former_trybuild` again. Expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `struct_zero_*_error` compile-fail tests pass.
*   **Commit Message:** "fix(former): Add compile errors for invalid zero-field struct variants"

##### Increment 4: Implement Single-Field Struct Variant - Scalar Constructor (Rule 1e)
*   **Goal:** Implement the scalar constructor for single-field struct variants like `MyVariant { field: T }` when `#[scalar]` is used.
*   **Specification Reference:** Rule 1e.
*   **Context:** The target test is `variant_one_scalar_test` for the variant:
    ```rust
    // in enum EnumWithNamedFields
    VariantOneScalar { field_a : String }, // Expect: variant_one_scalar(String) -> Enum
    ```
*   **Steps:**
    1.  Execute `cargo test --package former --test tests -- --nocapture variant_one_scalar_test`. Expect failure.
    2.  Implement the logic in `struct_single_field_scalar.rs` to generate a constructor that takes the field as an argument.
    3.  Update dispatch logic in `former_enum.rs`.
    4.  Run the test again. Expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `variant_one_scalar_test` test passes.
*   **Commit Message:** "feat(former): Implement scalar constructor for single-field struct variants"

##### Increment 5: Implement Single-Field Struct Variant - Implicit Variant Former (Rules 2e, 3e)
*   **Goal:** Implement the default/subform behavior for single-field struct variants, which generates an implicit former for the variant itself.
*   **Specification Reference:** Rules 2e, 3e.
*   **Context:** The target test is `variant_one_subform_test` for the variant:
    ```rust
    // in enum EnumWithNamedFields
    VariantOneSubform { field_b : InnerForSubform }, // Expect: variant_one_subform() -> InnerForSubformFormer
    ```
*   **Steps:**
    1.  Run `cargo test --package former --test tests -- --nocapture variant_one_subform_test`. Expect failure.
    2.  Implement logic in `struct_single_field_subform.rs` to generate a full `Former` ecosystem (Storage, Definition, Former struct with setters) for the variant.
    3.  Update dispatch logic in `former_enum.rs`.
    4.  Run `variant_one_subform_test` and `variant_one_default_test`. Expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `variant_one_subform_test` and `variant_one_default_test` tests pass.
*   **Commit Message:** "feat(former): Implement implicit variant former for single-field struct variants"

##### Increment 6: Implement Multi-Field Struct Variant - Scalar Constructor (Rule 1g)
*   **Goal:** Implement the scalar constructor for multi-field struct variants like `MyVariant { a: T1, b: T2 }` when `#[scalar]` is used.
*   **Specification Reference:** Rule 1g.
*   **Context:** The target test is `variant_two_scalar_test` for the variant:
    ```rust
    // in enum EnumWithNamedFields
    VariantTwoScalar { field_d : i32, field_e : bool }, // Expect: variant_two_scalar(i32, bool) -> Enum
    ```
*   **Steps:**
    1.  Run `cargo test --package former --test tests -- --nocapture variant_two_scalar_test`. Expect failure.
    2.  Implement logic in `struct_multi_fields_scalar.rs` to generate a constructor taking all fields as arguments.
    3.  Update dispatch logic.
    4.  Run the test again. Expect success.
    5.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   The `variant_two_scalar_test` test passes.
*   **Commit Message:** "feat(former): Implement scalar constructor for multi-field struct variants"

##### Increment 7: Implement Multi-Field Struct Variant - Implicit Variant Former (Rules 2g, 3g)
*   **Goal:** Implement the default/subform behavior for multi-field struct variants.
*   **Specification Reference:** Rules 2g, 3g.
*   **Context:** The target tests are `generics_shared_struct_variant` and `generics_independent_struct_variant`.
*   **Steps:**
    1.  Uncomment the `generics_independent_struct_*` and `generics_shared_struct_*` test modules.
    2.  Run `cargo test --package former --test tests -- --nocapture shared_generics_struct_variant`. Expect failure.
    3.  Implement logic in `struct_multi_fields_subform.rs` to generate a full `Former` ecosystem for the variant.
    4.  Update dispatch logic.
    5.  Run all newly enabled tests. Expect success.
    6.  Update the `### Tests` table with the status `Passed`.
*   **Increment Verification:**
    *   All `generics_*_struct_*` tests pass.
*   **Commit Message:** "feat(former): Implement implicit variant former for multi-field struct variants"

##### Increment 8: Implement Standalone Constructors - Zero-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for zero-field struct variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  Enable the `standalone_variant_zero_scalar_test` in `enum_named_fields_named_only_test.rs`.
    2.  Run test; expect failure.
    3.  Modify `struct_zero_fields_handler.rs` to generate the top-level function.
    4.  Run test; expect success.
*   **Increment Verification:**
    *   The `standalone_variant_zero_scalar_test` passes.
*   **Commit Message:** "feat(former): Add standalone constructors for zero-field struct variants"

##### Increment 9: Implement Standalone Constructors - Single-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for single-field struct variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  Uncomment `standalone_constructor_named_derive` and `standalone_constructor_args_named_derive` (and related `_manual` and `_only_test` files).
    2.  Run tests; expect failure.
    3.  Modify `struct_single_field_scalar.rs` and `struct_single_field_subform.rs` to generate standalone constructors, respecting `#[arg_for_constructor]` and Option 2 Logic.
    4.  Run tests; expect success.
*   **Increment Verification:**
    *   All `standalone_constructor_*` tests for single-field named variants pass.
*   **Commit Message:** "feat(former): Add standalone constructors for single-field struct variants"

##### Increment 10: Implement Standalone Constructors - Multi-Field Variants
*   **Goal:** Add `#[standalone_constructors]` support for multi-field struct variants.
*   **Specification Reference:** Option 2 Logic.
*   **Steps:**
    1.  Enable relevant tests in `standalone_constructor_args_named_only_test.rs` for multi-field variants.
    2.  Run tests; expect failure.
    3.  Modify `struct_multi_fields_scalar.rs` and `struct_multi_fields_subform.rs` to generate standalone constructors, respecting `#[arg_for_constructor]` and Option 2 Logic.
    4.  Run tests; expect success.
*   **Increment Verification:**
    *   All `standalone_constructor_*` tests for multi-field named variants pass.
*   **Commit Message:** "feat(former): Add standalone constructors for multi-field struct variants"

##### Increment 11: Update Documentation
*   **Goal:** Update user-facing documentation to reflect the completed enum support for named variants.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Read `module/core/former/Readme.md`.
    2.  Ensure the "Enum Standalone Constructors" section has a clear and correct example that includes a named (struct-like) variant.
    3.  Read `module/core/former/advanced.md` and `module/core/former/spec.md` to ensure the attribute references and behavior tables are consistent with the final implementation for named variants.
*   **Increment Verification:**
    *   The documentation is updated and accurate.
*   **Commit Message:** "docs(former): Update documentation for named enum variant support"

##### Increment 12: Finalization
*   **Goal:** Perform a final verification of the entire workspace.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Ensure all test modules in `module/core/former/tests/inc/enum_named_tests/mod.rs` are uncommented.
    2.  Perform a final Crate Conformance Check on the entire workspace.
    3.  Self-critique against all requirements and rules.
*   **Increment Verification:**
    *   All workspace checks pass.
*   **Commit Message:** "chore(former): Finalize named enum variant implementation"

### Out of Scope
*   Implementing features for unnamed (tuple-style) or true unit enum variants.
*   Refactoring any code outside of the `former_meta` and `former` crates.
*   Adding new features not specified in the `spec.md` for named variants.
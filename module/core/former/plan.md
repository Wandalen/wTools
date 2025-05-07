# Project Plan: Comprehensive Testing of `former` Crate for Enum Named (Struct-like) Variants

## Goal
*   Systematically test the `#[derive(Former)]` macro for Rust enum **named (struct-like) variants**.
*   Cover combinations of relevant `former` attributes (`#[scalar]`, `#[subform_scalar]`, default behavior, `#[standalone_constructors]`, `#[arg_for_constructor]`) for struct-like variants with 0, 1, and multiple fields.
*   Incrementally uncomment, pre-analyze, fix, and verify existing test files related to struct-like variants within `module/core/former/tests/inc/former_enum_tests/`.
*   **Embed the "Test Matrix for Named (Struct-like) Variants" as documentation within `module/core/former/tests/inc/former_enum_tests/mod.rs` (or a dedicated shared test file for named fields).**
*   Ensure all code modifications adhere strictly to `code/gen` instructions, Design Rules, and Codestyle Rules.

## Relevant Context

*   **Primary Test Directory:** `module/core/former/tests/inc/former_enum_tests/`
    *   `enum_named_fields_derive.rs`, `enum_named_fields_manual.rs`, `enum_named_fields_only_test.rs` (primary target for these tests).
    *   Files like `generics_independent_struct_*.rs` (for struct variants with generic inner types).
    *   Files like `generics_shared_struct_*.rs` (for struct variants with shared generic inner types).
    *   Files like `standalone_constructor_*.rs` and `standalone_constructor_args_*.rs` (for struct variants with these enum-level attributes).
*   **Enum Test Module File:** `module/core/former/tests/inc/former_enum_tests/mod.rs`
*   **Main Test Module File (Parent):** `module/core/former/tests/inc/mod.rs`
*   **Macro Implementation:** `module/core/former_meta/src/derive_former/former_enum/`
    *   `struct_zero_fields_handler.rs`
    *   `struct_single_field_scalar.rs`
    *   `struct_single_field_subform.rs`
    *   `struct_multi_fields_scalar.rs`
    *   `struct_multi_fields_subform.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs` (main dispatch)
*   **Core Types & Traits:** `module/core/former_types/src/lib.rs`
*   **Documentation:**
    *   `module/core/former/advanced.md`
    *   `module/core/former/Readme.md`

### Test Matrix for Named (Struct-like) Variants

**Factors:**

1.  **Number of Fields:**
    *   Zero (`V {}`)
    *   One (`V { f1: T1 }`)
    *   Multiple (`V { f1: T1, f2: T2, ... }`)
2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
    *   Derives `Former`
    *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
3.  **Variant-Level Attribute:**
    *   None (Default behavior)
    *   `#[scalar]`
    *   `#[subform_scalar]`
4.  **Enum-Level Attribute:**
    *   None
    *   `#[standalone_constructors]`
5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
    *   Not applicable (for zero-field)
    *   On the single field (for one-field)
    *   On all fields / some fields / no fields (for multi-field)

---

**Combinations for Zero-Field Struct Variants (`V {}`):**

| #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
|----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
| S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
| S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
| S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
| S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
| S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |

---

**Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**

| #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
|----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
| S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
| S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
| S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
| S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
| S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
| S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
| S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |

---

**Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**

| #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
|----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
| SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
| SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
| SN.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
| SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
| SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
| SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
| SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_multi_fields_subform.rs` (for static method), standalone logic |

---

### Target File Structure for Named (Struct-like) Variant Tests

Within `module/core/former/tests/inc/former_enum_tests/`:
The primary files are `enum_named_fields_derive.rs`, `enum_named_fields_manual.rs`, and `enum_named_fields_only_test.rs`. These will be the main focus. Documentation for this matrix will be added to `former_enum_tests/mod.rs`.

```module/core/former/tests/inc/
├── mod.rs                      // Declares `mod former_enum_tests;`
└── former_enum_tests/
    ├── mod.rs                  // Declares all specific enum test files.
    │                           // Will contain the Test Matrix documentation for named variants.
    ├── enum_named_fields_derive.rs
    ├── enum_named_fields_manual.rs
    └── enum_named_fields_only_test.rs
    // ... other existing files like generics_*, standalone_constructor_* ...
    └── compile_fail/
        ├── struct_zero_default_error.rs             // For S0.1
        ├── struct_zero_subform_scalar_error.rs      // For S0.5
```

### Expected Enum Former Behavior Rules (Named/Struct-like Variants Only)
(Extracted and focused from the general rules)

1.  **`#[scalar]` Attribute (on variant):**
    *   Zero-Field Struct Variant (`V {}`): `Enum::variant() -> Enum`. (Rule 1c)
    *   Single-Field Struct Variant (`V { f1: T1 }`): `Enum::variant { f1: T1 } -> Enum`. (Rule 1e)
    *   Multi-Field Struct Variant (`V { f1: T1, ... }`): `Enum::variant { f1: T1, ... } -> Enum`. (Rule 1g)
2.  **`#[subform_scalar]` Attribute (on variant):**
    *   Zero-Field Struct Variant: Error. (Rule 2c)
    *   Single-Field Struct Variant (`V { f1: T1 }`): `Enum::variant() -> VariantFormer<...>`. (Rule 2e)
    *   Multi-Field Struct Variant (`V { f1: T1, ... }`): `Enum::variant() -> VariantFormer<...>`. (Rule 2g)
3.  **Default Behavior (No `#[scalar]` or `#[subform_scalar]` on variant):**
    *   Zero-Field Struct Variant (`V {}`): Error. (Rule 3c)
    *   Single-Field Struct Variant (`V { f1: T1 }`): `Enum::variant() -> VariantFormer<...>`. (Rule 3e)
    *   Multi-Field Struct Variant (`V { f1: T1, ... }`): `Enum::variant() -> VariantFormer<...>`. (Rule 3g)
4.  **`#[standalone_constructors]` Attribute (on enum):**
    *   (As per general Rule 4, applied to the outcomes of Rules 1-3 above for struct-like variants).

### Failure Diagnosis Algorithm
(Standard algorithm as previously defined)

## Increments

*   [⚫] **Increment 1: Document Test Matrix for Named (Struct-like) Variants**
    *   **Goal:** Embed the "Test Matrix for Named (Struct-like) Variants" into the documentation within `module/core/former/tests/inc/former_enum_tests/mod.rs`.
    *   **Detailed Plan Step 1:** Modify `module/core/former/tests/inc/former_enum_tests/mod.rs`.
        *   Append to the existing module-level documentation comment (`//!`):
            *   A clear title, e.g., "## Test Matrix for Enum Named (Struct-like) Variants".
            *   The full "Test Matrix for Named (Struct-like) Variants" tables (Zero-Field, Single-Field, Multi-Field).
            *   A brief explanation.
    *   **Pre-Analysis:** Documentation-only change.
    *   **Crucial Design Rules:** [Comments and Documentation](#comments-and-documentation).
    *   **Verification Strategy:**
        1.  Request user to apply the changes.
        2.  Request user to run `cargo check --tests --package former`.
        3.  Request user to run `cargo doc --package former --no-deps --open` and verify the matrix documentation in the `former_enum_tests` module.

*   [⚫] **Increment 2: Zero-Field Struct Variants (Combinations S0.2, S0.4)**
    *   **Goal:** Test `V {}` variants with `#[scalar]`.
    *   **Files:** `enum_named_fields_*`.
    *   **Matrix Coverage:** S0.2, S0.4.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1c, 4.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 3: Single-Field Struct Variants (Combinations S1.1-S1.3 without standalone)**
    *   **Goal:** Test `V { f1: T1 }` with Default, `#[scalar]`, and `#[subform_scalar]`.
    *   **Files:** `enum_named_fields_*`.
    *   **Matrix Coverage:** S1.1, S1.2, S1.3.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1e, 2e, 3e.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 4: Multi-Field Struct Variants (Combinations SN.1-SN.3 without standalone)**
    *   **Goal:** Test `V { f1: T1, ... }` with Default, `#[scalar]`, and `#[subform_scalar]`.
    *   **Files:** `enum_named_fields_*`.
    *   **Matrix Coverage:** SN.1, SN.2, SN.3.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rules 1g, 2g, 3g.
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 5: Struct Variants with `#[standalone_constructors]` (Combinations S0.4, S1.4-S1.7, SN.4-SN.7)**
    *   **Goal:** Test `#[standalone_constructors]` with zero, single, and multi-field struct variants, including `#[arg_for_constructor]` interactions.
    *   **Files:** Adapt `enum_named_fields_*` and `standalone_constructor_args_*`.
    *   **Matrix Coverage:** S0.4, S1.4, S1.5, S1.6, S1.7, SN.4, SN.5, SN.6, SN.7.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow), Expected Behavior Rule 4 in conjunction with 1c/e/g, 2e/g, 3e/g.
    *   **Verification Strategy:** Staged testing. This is a large increment; may need to be broken down further during detailed planning.

*   [⚫] **Increment 6: Error Cases for Struct Variants (S0.1, S0.5)**
    *   **Goal:** Verify compile errors for invalid attribute usage on struct variants.
    *   **Files:** Create new `trybuild` tests in `module/core/former/tests/inc/former_enum_tests/compile_fail/`:
        *   `struct_zero_default_error.rs` (for S0.1)
        *   `struct_zero_subform_scalar_error.rs` (for S0.5)
    *   **Crucial Design Rules:** Expected Behavior Rules 2c, 3c.
    *   **Verification Strategy:** Add `trybuild` test cases.

*   [⚫] **Increment 7: Generics with Struct Variants**
    *   **Goal:** Integrate and verify tests from `generics_independent_struct_*` and `generics_shared_struct_*`.
    *   **Files:** `generics_independent_struct_*`, `generics_shared_struct_*`.
    *   **Matrix Coverage:** Implicitly covers S1.1/SN.1 type behavior but with generics.
    *   **Crucial Design Rules:** [Proc Macro: Development Workflow](#proc-macro-development-workflow).
    *   **Verification Strategy:** Staged testing.

*   [⚫] **Increment 8: Final Review and Full Test Suite for Named (Struct-like) Variants**
    *   **Goal:** Ensure all named (struct-like) variant tests are active and passing.
    *   **Verification Strategy:** `cargo check --all-targets --package former`, `cargo clippy ...`, `cargo test ... former_enum_tests`.

### Requirements
*   (Same as initial plan)

## Notes & Insights
*   This plan focuses on named (struct-like) variants.
*   The "Test Matrix for Named (Struct-like) Variants" will be appended to the documentation in `module/core/former/tests/inc/former_enum_tests/mod.rs`.
*   The `enum_named_fields_*` files are central to many of these tests.
*   Increment 5 is large and might be subdivided during its detailed planning phase.

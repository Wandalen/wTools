//! ## Test Matrix Coverage (Unit Variants)
//!
//! This plan focuses on verifying the behavior for **Unit Variants**. The relevant factors and combinations tested by the `unit_variant_*` files are:
//!
//! *   **Factors:**
//!     1.  Variant Type: Unit (Implicitly selected)
//!     2.  Variant-Level Attribute: None (Default), `#[scalar]`
//!     3.  Enum-Level Attribute: None, `#[standalone_constructors]`
//!
//! *   **Combinations Covered by `unit_variant_only_test.rs`:**
//!     *   Unit + Default + None (Rule 3a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test.
//!     *   Unit + `#[scalar]` + None (Rule 1a) -> Tested via `Status::pending()` / `Status::complete()` in `unit_variant_constructors()` test (as default is scalar).
//!     *   Unit + Default + `#[standalone_constructors]` (Rule 3a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.
//!     *   Unit + `#[scalar]` + `#[standalone_constructors]` (Rule 1a, 4) -> Tested via `pending()` / `complete()` in `unit_variant_standalone_constructors()` test.
//!
//! ---
//!
//! ## Test Matrix for Enum Unnamed (Tuple) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum unnamed (tuple) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V()`)
//!     *   One (`V(T1)`)
//!     *   Multiple (`V(T1, T2, ...)`)
//! 2.  **Field Type `T1` (for Single-Field Variants):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former`
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Tuple Variants (`V()`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | T0.1| Default      | None                        | `Enum::v() -> Enum`           | N/A                             | 3b      | `tuple_zero_fields_handler.rs` |
//! | T0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1b      | `tuple_zero_fields_handler.rs` |
//! | T0.3| Default      | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 3b, 4   | `tuple_zero_fields_handler.rs` |
//! | T0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1b, 4   | `tuple_zero_fields_handler.rs` |
//! | T0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2b      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Tuple Variants (`V(T1)`):**
//!
//! | #   | Variant Attr      | T1 Derives Former | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s)     | Handler (Meta)                 |
//! |-----|-------------------|-------------------|-----------------------------|-------------------------------|---------------------------------|-------------|--------------------------------|
//! | T1.1| Default           | Yes               | None                        | `Enum::variant() -> T1Former` | N/A                             | 3d.i        | `tuple_single_field_subform.rs`|
//! | T1.2| Default           | No                | None                        | `Enum::variant(T1) -> Enum`   | N/A                             | 3d.ii       | `tuple_single_field_scalar.rs` |
//! | T1.3| `#[scalar]`       | Any               | None                        | `Enum::variant(T1) -> Enum`   | N/A                             | 1d          | `tuple_single_field_scalar.rs` |
//! | T1.4| `#[subform_scalar]`| Yes               | None                        | `Enum::variant() -> T1Former` | N/A                             | 2d          | `tuple_single_field_subform.rs`|
//! | T1.5| `#[subform_scalar]`| No                | None                        | *Compile Error*               | *Compile Error*                 | 2d          | (Dispatch)                     |
//! | T1.6| Default           | Yes               | `#[standalone_constructors]`| `Enum::variant() -> T1Former` | `fn variant() -> T1Former`      | 3d.i, 4     | `tuple_single_field_subform.rs`|
//! | T1.7| Default           | No                | `#[standalone_constructors]`| `Enum::variant(T1) -> Enum`   | `fn variant(T1) -> Enum`        | 3d.ii, 4    | `tuple_single_field_scalar.rs` |
//! | T1.8| `#[scalar]`       | Any               | `#[standalone_constructors]`| `Enum::variant(T1) -> Enum`   | `fn variant(T1) -> Enum`        | 1d, 4       | `tuple_single_field_scalar.rs` |
//! | T1.9| `#[subform_scalar]`| Yes               | `#[standalone_constructors]`| `Enum::variant() -> T1Former` | `fn variant() -> T1Former`      | 2d, 4       | `tuple_single_field_subform.rs`|
//! | T1.10| `#[subform_scalar]`| No                | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 2d          | (Dispatch)                     |
//!
//! Note: The effect of `#[arg_for_constructor]` is covered by Rule 4 in conjunction with the base behavior.
//!
//! ---
//!
//! **Combinations for Multi-Field Tuple Variants (`V(T1, T2, ...)`):**
//!
//! | #   | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |-----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | TN.1| Default      | None                        | `Enum::variant(T1, T2,...) -> Enum` | N/A                             | 3f      | `tuple_multi_fields_scalar.rs` |
//! | TN.2| `#[scalar]`  | None                        | `Enum::variant(T1, T2,...) -> Enum` | N/A                             | 1f      | `tuple_multi_fields_scalar.rs` |
//! | TN.3| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2f      | (Dispatch)                     |
//! | TN.4| Default      | `#[standalone_constructors]`| `Enum::variant(T1, T2,...) -> Enum` | `fn variant(T1, T2,...) -> Enum` | 3f, 4   | `tuple_multi_fields_scalar.rs` |
//! | TN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::variant(T1, T2,...) -> Enum` | `fn variant(T1, T2,...) -> Enum` | 1f, 4   | `tuple_multi_fields_scalar.rs` |
//!
//! Note: The effect of `#[arg_for_constructor]` is covered by Rule 4 in conjunction with the base behavior.
//!
//! ---
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! **Combinations for Multi-Field Struct Variants (`V { f1: T1, f2: T2, ... }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method             | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|------------------------------------|---------------------------------|---------|--------------------------------|
//! | SN.1| Default      | None                        | `Enum::v() -> VariantFormer<...>`  | N/A                             | 3g      | `struct_multi_fields_subform.rs`|
//! | SN.2| `#[scalar]`  | None                        | `Enum::v {f1:T1,...} -> Enum`      | N/A                             | 1g      | `struct_multi_fields_scalar.rs` |
//! | SN.3| `#[subform_scalar]` | (Any)                | `Enum::v() -> VariantFormer<...>`  | N/A                             | 2g      | `struct_multi_fields_subform.rs`|
//! | SN.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>`  | `fn v() -> VariantFormer<...>` (no args) | 3g,4 | `struct_multi_fields_subform.rs`|
//! | SN.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v {f1:T1,...} -> Enum`      | `fn v(f1:T1,...) -> Enum` (all args) | 1g,4 | `struct_multi_fields_scalar.rs` |
//! | SN.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2g,4 | `struct_multi_fields_subform.rs`|
//! | SN.7| Default      | `#[standalone_constructors]` + some/all `#[arg_for_constructor]` | `Enum::v() -> VariantFormer<...>` (args pre-set) | `fn v(marked_args...) -> VariantFormer_or_Enum` (logic per Rule 4) | 3g,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! ## Test Matrix for Enum Named (Struct-like) Variants
//!
//! This matrix guides the testing of `#[derive(Former)]` for enum named (struct-like) variants,
//! linking combinations of attributes and variant structures to expected behaviors and
//! relevant internal rule numbers.
//!
//! ---
//!
//! **Factors:**
//!
//! 1.  **Number of Fields:**
//!     *   Zero (`V {}`)
//!     *   One (`V { f1: T1 }`)
//!     *   Multiple (`V { f1: T1, f2: T2, ... }`)
//! 2.  **Field Type `T1` (for Single-Field Variants, relevant for `#[subform_scalar]`):**
//!     *   Derives `Former`
//!     *   Does NOT derive `Former` (Note: `#[subform_scalar]` on a single-field struct variant *always* creates an implicit variant former, so this distinction is less critical than for tuples, but good to keep in mind for consistency if `T1` itself is used in a subform-like way *within* the implicit former).
//! 3.  **Variant-Level Attribute:**
//!     *   None (Default behavior)
//!     *   `#[scalar]`
//!     *   `#[subform_scalar]`
//! 4.  **Enum-Level Attribute:**
//!     *   None
//!     *   `#[standalone_constructors]`
//! 5.  **Field-Level Attribute `#[arg_for_constructor]` (within `#[standalone_constructors]` context):**
//!     *   Not applicable (for zero-field)
//!     *   On the single field (for one-field)
//!     *   On all fields / some fields / no fields (for multi-field)
//!
//! ---
//!
//! **Combinations for Zero-Field Struct Variants (`V {}`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S0.1| Default      | None                        | *Compile Error*               | N/A                             | 3c      | (Dispatch)                     |
//! | S0.2| `#[scalar]`  | None                        | `Enum::v() -> Enum`           | N/A                             | 1c      | `struct_zero_fields_handler.rs`|
//! | S0.3| Default      | `#[standalone_constructors]`| *Compile Error*               | *Compile Error*                 | 3c, 4   | (Dispatch)                     |
//! | S0.4| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v() -> Enum`           | `fn v() -> Enum`                | 1c, 4   | `struct_zero_fields_handler.rs`|
//! | S0.5| `#[subform_scalar]` | (Any)                | *Compile Error*               | *Compile Error*                 | 2c      | (Dispatch)                     |
//!
//! ---
//!
//! **Combinations for Single-Field Struct Variants (`V { f1: T1 }`):**
//!
//! | #  | Variant Attr | Enum Attr                   | Expected Static Method        | Expected Standalone Constructor | Rule(s) | Handler (Meta)                 |
//! |----|--------------|-----------------------------|-------------------------------|---------------------------------|---------|--------------------------------|
//! | S1.1| Default      | None                        | `Enum::v() -> VariantFormer<...>` | N/A                           | 3e      | `struct_single_field_subform.rs`|
//! | S1.2| `#[scalar]`  | None                        | `Enum::v { f1: T1 } -> Enum`  | N/A                             | 1e      | `struct_single_field_scalar.rs` |
//! | S1.3| `#[subform_scalar]` | None                 | `Enum::v() -> VariantFormer<...>` | N/A                           | 2e      | `struct_single_field_subform.rs`|
//! | S1.4| Default      | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 3e,4 | `struct_single_field_subform.rs`|
//! | S1.5| `#[scalar]`  | `#[standalone_constructors]`| `Enum::v { f1: T1 } -> Enum`  | `fn v(f1: T1) -> Enum` (f1 is arg) | 1e,4 | `struct_single_field_scalar.rs` |
//! | S1.6| `#[subform_scalar]` | `#[standalone_constructors]`| `Enum::v() -> VariantFormer<...>` | `fn v() -> VariantFormer<...>` (no args) | 2e,4 | `struct_single_field_subform.rs`|
//! | S1.7| Default      | `#[standalone_constructors]` + `#[arg_for_constructor]` on `f1` | `Enum::v() -> VariantFormer<...>` (f1 pre-set) | `fn v(f1: T1) -> Enum` (f1 is arg, returns Self) | 3e,4 | `struct_single_field_subform.rs` (for static method), standalone logic |
//!
//! ---
//!
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!
//! use super::*;
//! use test_tools::exposed::*;
//!
//! // Uncomment modules as they are addressed in increments.
//!
//! // Increment 1: Unit Variant Tests
//! mod unit_variant_derive;
//! mod unit_variant_manual;
//!
//! // Increment 2: Zero-Field Tuple Variants
//! // mod enum_named_fields_derive;
//! // mod enum_named_fields_manual;
//!
//! // // Increment 3: Single-Field Tuple Variants - T1 derives Former
//! // mod basic_derive;
//! // mod basic_manual;
//! // mod generics_in_tuple_variant_derive;
//! // mod generics_in_tuple_variant_manual;
//! // mod generics_shared_tuple_derive;
//! // mod generics_shared_tuple_manual;
//! //
//! // // Increment 4: Single-Field Tuple Variants - T1 does NOT derive Former
//! // // mod tuple_single_non_former_derive; // May need to create
//! // // mod tuple_single_non_former_manual; // May need to create
//! //
//! // // Increment 5: Single-Field Tuple Variants - #[scalar]
//! // // mod generics_independent_tuple_derive;
//! // // mod generics_independent_tuple_manual;
//! // mod scalar_generic_tuple_derive; // May need adaptation
//! // mod scalar_generic_tuple_manual; // May need adaptation
//! //
//! // // Increment 6: Single-Field Tuple Variants - #[standalone_constructors]
//! // mod standalone_constructor_derive; // May need adaptation
//! // mod standalone_constructor_manual; // May need adaptation
//! // mod standalone_constructor_args_derive; // May need adaptation
//! // mod standalone_constructor_args_manual; // May need adaptation
//! //
//! // // Increment 7: Multi-Field Tuple Variants (Default & #[scalar])
//! // mod tuple_multi_default_derive; // May need to create
//! // mod tuple_multi_default_manual; // May need to create
//! // mod tuple_multi_scalar_derive; // May need to create
//! // mod tuple_multi_scalar_manual; // May need to create
//! //
//! // // Increment 8: Multi-Field Tuple Variants - #[standalone_constructors]
//! // mod tuple_multi_standalone_manual; // New for Increment 8
//! // mod tuple_multi_standalone_derive; // New for Increment 8
//! // mod tuple_multi_standalone_args_manual; // New for Increment 8
//! // mod tuple_multi_standalone_args_derive; // New for Increment 8
//!
//! // Increment 9: Error Cases for Tuple Variants
//! // mod compile_fail; // This is a directory, needs a mod declaration
//!
//! // mod usecase1_manual;
//! // mod usecase1_derive;

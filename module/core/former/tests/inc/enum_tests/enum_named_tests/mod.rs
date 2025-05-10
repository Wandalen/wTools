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
//! This documentation will be expanded as testing for other variant types (struct, unit) is planned.
//!

// Uncomment modules as they are addressed in increments.

// mod generics_independent_struct_derive;
// mod generics_independent_struct_manual;
// mod generics_independent_struct_only_test;
// mod generics_shared_struct_derive;
// mod generics_shared_struct_manual;
// mod generics_shared_struct_only_test;
// mod enum_named_fields_named_derive;
// mod enum_named_fields_named_manual;
// mod enum_named_fields_named_only_test;
// mod standalone_constructor_named_derive;
// mod standalone_constructor_named_only_test;
// mod standalone_constructor_args_named_derive;
// mod standalone_constructor_args_named_manual;
// mod standalone_constructor_args_named_only_test;

// pub mod compile_fail;
# Technical Specification: `derive_tools`

### Project Goal

To create a comprehensive, standalone, and idiomatic procedural macro library, `derive_tools`, that provides a suite of essential derive macros for common Rust traits. This library will be self-contained, with no external dependencies on other macro-providing crates, establishing its own clear design principles and implementation patterns.

### Problem Solved

Rust developers frequently wrap primitive types or compose structs that require boilerplate implementations for common traits (e.g., `From`, `Deref`, `AsRef`). By creating a first-party, full-scale `derive_tools` library, we can:

1.  **Eliminate External Dependencies:** Gives us full control over the implementation, features, and error handling.
2.  **Establish a Canonical Toolset:** Provide a single, consistent, and well-documented set of derive macros that follow a unified design philosophy.
3.  **Improve Developer Ergonomics:** Reduce boilerplate code for common patterns in a way that is predictable, robust, and easy to debug.
4.  **Eliminate External Dependencies**: Remove the reliance on derive_more, strum, parse-display, and other similar crates, giving us full control over the implementation, features, and error handling.

### Ubiquitous Language (Vocabulary)

*   **`derive_tools`**: The user-facing facade crate. It provides the derive macros (e.g., `#[derive(From)]`) and is the only crate a user should list as a dependency.
*   **`derive_tools_meta`**: The procedural macro implementation crate. It contains all the `#[proc_macro_derive]` logic and is a private dependency of `derive_tools`.
*   **`macro_tools`**: The foundational utility crate providing abstractions over `syn`, `quote`, and `proc_macro2`. It is a private dependency of `derive_tools_meta`.
*   **Master Attribute**: The primary control attribute `#[derive_tools(...)]` used to configure behavior for multiple macros at once.
*   **Macro Attribute**: An attribute specific to a single macro, like `#[from(...)]` or `#[display(...)]`.
*   **Container**: The struct or enum to which a derive macro is applied.
*   **Newtype Pattern**: A common Rust pattern of wrapping a single type in a struct to create a new, distinct type (e.g., `struct MyId(u64);`).

### Architectural Principles

1.  **Two-Crate Structure**: The framework will always maintain a two-crate structure: a user-facing facade crate (`derive_tools`) and a procedural macro implementation crate (`derive_tools_meta`).
2.  **Abstraction over `syn`/`quote`**: All procedural macro logic within `derive_tools_meta` **must** exclusively use the `macro_tools` crate for AST parsing, manipulation, and code generation. Direct usage of `syn`, `quote`, or `proc_macro2` is forbidden.
3.  **Convention over Configuration**: Macros should work out-of-the-box for the most common use cases (especially the newtype pattern) with zero configuration. Attributes should only be required to handle ambiguity or to enable non-default behavior.
4.  **Clear and Actionable Error Messages**: Compilation errors originating from the macros must be clear, point to the exact location of the issue in the user's code, and suggest a correct alternative whenever possible.
5.  **Orthogonality**: Each macro should be independent and address a single concern. Deriving one trait should not implicitly alter the behavior of another, with the noted exception of `Phantom`.

### Macro Design & Implementation Rules

#### Design Rules
1.  **Consistency**: All macros must use a consistent attribute syntax.
2.  **Explicitness over Magic**: Prefer explicit user configuration (e.g., `#[error(source)]`) over implicit "magical" behaviors (e.g., auto-detecting a source field). Auto-detection should be a documented fallback, not the primary mechanism.
3.  **Scoped Attributes**: Field-level attributes always take precedence over container-level attributes.

#### Codestyle Rules
1.  **Repository as Single Source of Truth**: The project's version control repository is the single source of truth for all artifacts.
2.  **Naming Conventions**: All asset names (files, variables, etc.) **must** use `snake_case`.
3.  **Modular Implementation**: Each derive macro implementation in `derive_tools_meta` must reside in its own module.
4.  **Testing**: Every public-facing feature of a macro must have at least one corresponding test case, including `trybuild` tests for all limitations.

### Core Macro Attribute Syntax

The framework uses a master attribute `#[derive_tools(...)]` for global configuration, alongside macro-specific attributes.

*   **Master Attribute**: `#[derive_tools( skip( <Trait1>, <Trait2>, ... ) )]`
    *   Used on fields to exclude them from specific derive macro implementations. This is the preferred way to handle fields that do not implement a given trait.
*   **Macro-Specific Attributes**: `#[<macro_name>( ... )]`
    *   Used for configurations that only apply to a single macro (e.g., `#[display("...")]` or `#[add(Rhs = i32)]`).

---
### Macro-Specific Specifications

#### `From` Macro
*   **Purpose**: To automatically implement the `core::convert::From` trait. The `Into` macro is intentionally not provided; users should rely on the blanket `Into` implementation provided by the standard library when `From` is implemented.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, generates a `From<InnerType>` implementation for the container.
    *   **Multi-Field Structs**: By default, generates a `From` implementation from a tuple of all field types, in the order they are defined.
    *   **Enums**: The macro can be used on enum variants to generate `From` implementations that construct a specific variant.
*   **Attribute Syntax**:
    *   `#[from(forward)]`: (Container-level, single-field structs only) Generates a generic `impl<T> From<T> for Container where InnerType: From<T>`. This allows the container to be constructed from anything the inner type can be constructed from.
    *   `#[from((Type1, Type2, ...))]`: (Container-level, multi-field structs only) Specifies an explicit tuple type to convert from. The number of types in the tuple must match the number of fields in the struct.
    *   `#[from]`: (Enum-variant-level) Marks a variant as the target for a `From` implementation. The implementation will be `From<FieldType>` for single-field variants, or `From<(Field1Type, ...)>` for multi-field variants.
*   **Interaction with `Phantom` Macro**: The `_phantom` field added by `derive(Phantom)` is automatically ignored and is not included in the tuple for multi-field struct implementations.
*   **Limitations**: Cannot be applied to unions. For enums, only one variant can be the target for a given source type to avoid ambiguity.

#### `AsRef` Macro
*   **Purpose**: To implement `core::convert::AsRef<T>`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, implements `AsRef<InnerType>`.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[as_ref]`: (Field-level) Marks the target field in a multi-field struct. Implements `AsRef<FieldType>`. This is mandatory for this case.
    *   `#[as_ref(forward)]`: (Container or Field-level) Forwards the `AsRef` implementation from the inner field. Generates `impl<T: ?Sized> AsRef<T> for Container where FieldType: AsRef<T>`.
    *   `#[as_ref(Type1, Type2, ...)]`: (Container or Field-level) Generates specific `AsRef` implementations for the listed types, assuming the inner field also implements them.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions.

#### `AsMut` Macro
*   **Purpose**: To implement `core::convert::AsMut<T>`.
*   **Prerequisites**: The container must also implement `AsRef<T>` for the same type `T`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, implements `AsMut<InnerType>`.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[as_mut]`: (Field-level) Marks the target field in a multi-field struct. Implements `AsMut<FieldType>`.
    *   `#[as_mut(forward)]`: (Container or Field-level) Forwards the `AsMut` implementation from the inner field.
    *   `#[as_mut(Type1, ...)]`: (Container or Field-level) Generates implementations for specific types.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions.

#### `Deref` Macro
*   **Purpose**: To implement `core::ops::Deref`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, dereferences to the inner type.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[deref]`: (Field-level) Marks the target field in a multi-field struct.
    *   `#[deref(forward)]`: (Container or Field-level) Forwards the `Deref` implementation, setting `Target` to the inner field's `Target`.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions.

#### `DerefMut` Macro
*   **Purpose**: To implement `core::ops::DerefMut`.
*   **Prerequisites**: The container must also implement `Deref`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, mutably dereferences to the inner type.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[deref_mut]`: (Field-level) Marks the target field in a multi-field struct.
    *   `#[deref_mut(forward)]`: (Container or Field-level) Forwards the `DerefMut` implementation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions.

#### `Index` Macro
*   **Purpose**: To implement `core::ops::Index<Idx>`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, forwards the `Index` implementation to the inner field.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[index]`: (Field-level) Marks the target field in a multi-field struct.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions. The target field must implement `Index`.

#### `IndexMut` Macro
*   **Purpose**: To implement `core::ops::IndexMut<Idx>`.
*   **Prerequisites**: The container must also implement `Index<Idx>`.
*   **Behavior and Rules**:
    *   **Single-Field Structs**: By default, forwards the `IndexMut` implementation.
    *   **Multi-Field Structs**: By default, does nothing. An explicit field-level attribute is required.
*   **Attribute Syntax**:
    *   `#[index_mut]`: (Field-level) Marks the target field in a multi-field struct.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: Cannot be applied to enums or unions. The target field must implement `IndexMut`.

#### `Not` Macro
*   **Purpose**: To implement `core::ops::Not`.
*   **Default Behavior**: Performs element-wise negation on all fields.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( Not ) )]`: (Field-level) Excludes a field from the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `Not`.

#### `Add` Macro
*   **Purpose**: To implement `core::ops::Add`.
*   **Default Behavior**: Performs element-wise addition on all fields against a `rhs` of type `Self`.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( Add ) )]`: (Field-level) Excludes a field from the operation.
    *   `#[add( Rhs = i32 )]`: (Container-level) Specifies a right-hand-side type for the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `Add`.

#### `Sub` Macro
*   **Purpose**: To implement `core::ops::Sub`.
*   **Default Behavior**: Performs element-wise subtraction on all fields against a `rhs` of type `Self`.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( Sub ) )]`: (Field-level) Excludes a field from the operation.
    *   `#[sub( Rhs = i32 )]`: (Container-level) Specifies a right-hand-side type for the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `Sub`.

#### `Mul` Macro
*   **Purpose**: To implement `core::ops::Mul`.
*   **Default Behavior**: Performs element-wise multiplication on all fields against a `rhs` of type `Self`.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( Mul ) )]`: (Field-level) Excludes a field from the operation.
    *   `#[mul( Rhs = i32 )]`: (Container-level) Specifies a right-hand-side type for the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `Mul`.

#### `Div` Macro
*   **Purpose**: To implement `core::ops::Div`.
*   **Default Behavior**: Performs element-wise division on all fields against a `rhs` of type `Self`.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( Div ) )]`: (Field-level) Excludes a field from the operation.
    *   `#[div( Rhs = i32 )]`: (Container-level) Specifies a right-hand-side type for the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `Div`.

#### `AddAssign` Macro
*   **Purpose**: To implement `core::ops::AddAssign`.
*   **Default Behavior**: Performs in-place element-wise addition on all fields.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( AddAssign ) )]`: (Field-level) Excludes a field from the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `AddAssign`.

#### `SubAssign` Macro
*   **Purpose**: To implement `core::ops::SubAssign`.
*   **Default Behavior**: Performs in-place element-wise subtraction on all fields.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( SubAssign ) )]`: (Field-level) Excludes a field from the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `SubAssign`.

#### `MulAssign` Macro
*   **Purpose**: To implement `core::ops::MulAssign`.
*   **Default Behavior**: Performs in-place element-wise multiplication on all fields.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( MulAssign ) )]`: (Field-level) Excludes a field from the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `MulAssign`.

#### `DivAssign` Macro
*   **Purpose**: To implement `core::ops::DivAssign`.
*   **Default Behavior**: Performs in-place element-wise division on all fields.
*   **Attribute Syntax**:
    *   `#[derive_tools( skip( DivAssign ) )]`: (Field-level) Excludes a field from the operation.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions. All non-skipped fields must implement `DivAssign`.

#### `InnerFrom` Macro
*   **Purpose**: To implement `core::convert::From<Container>` for the inner type(s) of a struct.
*   **Default Behavior**:
    *   **Single-Field Structs**: Implements `From<Container>` for the inner field's type.
    *   **Multi-Field Structs**: Implements `From<Container>` for a tuple containing all field types.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums or unions.

#### `VariadicFrom` Macro
*   **Purpose**: To generate a generic `From` implementation from a tuple of convertible types.
*   **Default Behavior**: Generates `impl<T1, ...> From<(T1, ...)> for Container` where each `Tn` can be converted into the corresponding field's type.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically ignored.
*   **Limitations**: Cannot be applied to enums, unions, or unit structs.

#### `Display` Macro
*   **Purpose**: To implement `core::fmt::Display`.
*   **Behavior**: Uses a format string to define the implementation.
*   **Attribute**: `#[display("...")]` is required for all but the simplest cases.

#### `FromStr` Macro
*   **Purpose**: To implement `core::str::FromStr`.
*   **Behavior**: Uses a `#[display("...")]` attribute to define the parsing format, relying on a dependency like `parse-display`.
*   **Attribute**: `#[display( ... )]` is used to define the parsing format.

#### `IntoIterator` Macro
*   **Purpose**: To implement `core::iter::IntoIterator`.
*   **Default Behavior**: For a single-field struct, it forwards the implementation. For multi-field structs, a field must be explicitly marked.
*   **Attribute Syntax**:
    *   `#[into_iterator]`: (Field-level) Marks the target field for iteration.
    *   `#[into_iterator( owned, ref, ref_mut )]`: (Container or Field-level) Specifies which iterator types to generate.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is ignored and cannot be selected as the target.
*   **Limitations**: The target field must implement the corresponding `IntoIterator` traits. Cannot be applied to enums or unions.

#### `IsVariant` Macro
*   **Purpose**: For enums, to generate `is_variant()` predicate methods.
*   **Behavior**: Generates methods for each variant unless skipped with `#[is_variant(skip)]`.
*   **Limitations**: Can only be applied to enums.

#### `Unwrap` Macro
*   **Purpose**: For enums, to generate panicking `unwrap_variant()` methods.
*   **Behavior**: Generates `unwrap_variant_name`, `..._ref`, and `..._mut` methods for each variant unless skipped with `#[unwrap(skip)]`.
*   **Limitations**: Can only be applied to enums.

#### `New` Macro
*   **Purpose**: To generate a flexible `new()` constructor for a struct.
*   **Default Behavior**: Generates a public function `pub fn new(...) -> Self` that takes all struct fields as arguments in their defined order.
*   **Attribute Syntax**:
    *   `#[new(default)]`: (Field-level) Excludes the field from the `new()` constructor's arguments. The field will be initialized using `Default::default()` in the function body.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically handled. It is not included as an argument in the `new()` constructor and is initialized with `core::marker::PhantomData` in the function body.
*   **Generated Code Logic**:
    *   For `struct MyType<T> { field: T, #[new(default)] id: u32 }` that also derives `Phantom`, the generated code will be:
        ```rust
        impl< T > MyType< T >
        {
          pub fn new( field : T ) -> Self
          {
            Self
            {
              field,
              id: core::default::Default::default(),
              _phantom: core::marker::PhantomData,
            }
          }
        }
        ```
*   **Limitations**: Cannot be applied to enums or unions. Any field not marked `#[new(default)]` must have its type specified as an argument.

#### `Default` Macro
*   **Purpose**: To implement the standard `core::default::Default` trait.
*   **Default Behavior**: Implements `default()` by calling `Default::default()` on every field.
*   **Interaction with `Phantom` Macro**: The `_phantom` field is automatically handled and initialized with `core::marker::PhantomData`.
*   **Limitations**: Cannot be applied to enums or unions. All fields must implement `Default`.

#### `Error` Macro
*   **Purpose**: To implement `std::error::Error`.
*   **Prerequisites**: The container must implement `Debug` and `Display`.
*   **Recommended Usage**: Explicitly mark the source of an error using `#[error(source)]` on a field.
*   **Fallback Behavior**: If no field is marked, the macro will attempt to find a source by looking for a field named `source`, then for the first field that implements `Error`.
*   **Attribute**: `#[error(source)]` is the primary attribute.

#### `Phantom` Macro
*   **Purpose**: To add a `_phantom: PhantomData<...>` field to a struct to handle unused generic parameters.
*   **Design Note**: This macro modifies the struct definition directly.
*   **Interaction with Other Macros**:
    *   **Core Issue**: This macro adds a `_phantom` field *before* other derive macros are expanded. Other macros must be implemented to gracefully handle this modification.
    *   **`New` Macro**: The generated `new()` constructor **must not** include `_phantom` in its arguments. It **must** initialize the field with `core::marker::PhantomData`.
    *   **`Default` Macro**: The generated `default()` method **must** initialize `_phantom` with `core::marker::PhantomData`.
    *   **`From` / `InnerFrom` Macros**: These macros **must** ignore any field named `_phantom` when constructing the tuple representation of the struct.
*   **Limitations**: Can only be applied to structs.

### Meta-Requirements

This specification document must be maintained according to the following rules:

1.  **Deliverables**: Any change to this specification must ensure that both `specification.md` and `spec_addendum.md` are correctly defined as project deliverables.
2.  **Ubiquitous Language**: All terms defined in the `Ubiquitous Language (Vocabulary)` section must be used consistently throughout this document.
3.  **Single Source of Truth**: The version control repository is the single source of truth for this document.
4.  **Naming Conventions**: All examples and definitions within this document must adhere to the project's naming conventions.
5.  **Structure**: The overall structure of this document must be maintained.

### Conformance Check Procedure

To verify that the final implementation of `derive_tools` conforms to this specification, the following checks must be performed and must all pass:

1.  **Static Analysis & Code Review**:
    *   Run `cargo clippy --workspace -- -D warnings` and confirm there are no warnings.
    *   Manually review the `derive_tools_meta` crate to ensure no direct `use` of `syn`, `quote`, or `proc_macro2` exists.
    *   Confirm that the project structure adheres to the two-crate architecture.
    *   Confirm that all code adheres to the rules defined in `codestyle.md`.

2.  **Testing**:
    *   Run `cargo test --workspace --all-features` and confirm that all tests pass.
    *   For each macro, create a dedicated test file (`tests/inc/<macro_name>_test.rs`) that includes:
        *   Positive use cases for all major behaviors (e.g., single-field, multi-field, forwarding).
        *   Edge cases (e.g., generics, lifetimes).
        *   At least one `trybuild` test case for each limitation listed in the specification to ensure it produces a clear compile-time error.
        *   A dedicated test case to verify the interaction with the `Phantom` macro, where applicable.

3.  **Documentation & Deliverables**:
    *   Ensure all public-facing macros and types in the `derive_tools` crate are documented with examples.
    *   Confirm that this `specification.md` document is up-to-date with the final implementation.
    *   Confirm that the `spec_addendum.md` template is available as a deliverable.

# Technical Specification: `variadic_from` Crate (v1.1)

**Note:** This specification governs the behavior of both the `variadic_from` crate, which provides the user-facing traits and macros, and the `variadic_from_meta` crate, which implements the procedural derive macro. Together, they form a single functional unit.

### 1. Introduction & Core Concepts

#### 1.1. Problem Solved
In Rust, creating struct instances often requires boilerplate, especially for structs with multiple fields or for those that need to be constructed from different sets of inputs. This crate aims to significantly reduce this boilerplate and improve developer ergonomics by providing a flexible, "variadic" constructor macro (`from!`). This allows for intuitive struct instantiation from a variable number of arguments, tuples, or single values, reducing cognitive load and making the code cleaner and more readable.

#### 1.2. Goals & Philosophy
The framework is guided by these principles:
*   **Convention over Configuration:** The `#[derive(VariadicFrom)]` macro should automatically generate the most common and intuitive `From`-like implementations without requiring extra attributes or configuration. The structure of the type itself is the configuration.
*   **Minimal Syntactic Noise:** The user-facing `from!` macro provides a clean, concise, and unified interface for constructing objects, abstracting away the underlying implementation details of which `FromN` trait is being called.
*   **Seamless Integration:** The crate should feel like a natural extension of the Rust language. It achieves this by automatically implementing the standard `From<T>` trait for single fields and `From<(T1, T2, ...)>` for multiple fields, enabling idiomatic conversions using `.into()`.
*   **Non-Intrusive Extensibility:** While the derive macro handles the common cases, the system is built on a foundation of public traits (`From1`, `From2`, `From3`) that developers can implement manually for custom behavior or to support types not covered by the macro.

#### 1.3. Key Terminology (Ubiquitous Language)
*   **Variadic Constructor:** A constructor that can accept a variable number of arguments. In the context of this crate, this is achieved through the `from!` macro.
*   **`FromN` Traits:** A set of custom traits (`From1`, `From2`, `From3`) that define a contract for constructing a type from a specific number (`N`) of arguments. They are the low-level mechanism enabling the `from!` macro.
*   **`VariadicFrom` Trait:** A marker trait implemented via a derive macro (`#[derive(VariadicFrom)]`). Its presence on a struct signals that the derive macro should automatically implement the appropriate `FromN` and `From<T>`/`From<tuple>` traits based on the number of fields in the struct.
*   **`from!` Macro:** A declarative, user-facing macro that provides the primary interface for variadic construction. It resolves to a call to `Default::default()`, `From1::from1`, `From2::from2`, or `From3::from3` based on the number of arguments provided.
*   **Named Struct:** A struct where fields are defined with explicit names, e.g., `struct MyStruct { a: i32 }`.
*   **Unnamed Struct (Tuple Struct):** A struct where fields are defined by their type only, e.g., `struct MyStruct(i32)`.

#### 1.4. Versioning Strategy
The `variadic_from` crate adheres to the Semantic Versioning 2.0.0 (SemVer) standard.
*   **MAJOR** version changes indicate incompatible API changes.
*   **MINOR** version changes introduce new, backward-compatible functionality (e.g., increasing the maximum number of supported arguments).
*   **PATCH** version changes are for backward-compatible bug fixes.

This specification document is versioned in lockstep with the crate itself.

### 2. Core Object Definitions

#### 2.1. The `FromN` Traits
The `FromN` traits provide a standardized, type-safe interface for constructing a type from a specific number (`N`) of arguments. They form the low-level contract that the high-level `from!` macro and `VariadicFrom` derive macro use.

*   **`From1<Arg>`**
    ```rust
    pub trait From1<Arg>
    where
        Self: Sized,
    {
        fn from1(arg: Arg) -> Self;
    }
    ```
*   **`From2<Arg1, Arg2>`**
    ```rust
    pub trait From2<Arg1, Arg2>
    where
        Self: Sized,
    {
        fn from2(arg1: Arg1, arg2: Arg2) -> Self;
    }
    ```
*   **`From3<Arg1, Arg2, Arg3>`**
    ```rust
    pub trait From3<Arg1, Arg2, Arg3>
    where
        Self: Sized,
    {
        fn from3(arg1: Arg1, arg2: Arg2, arg3: Arg3) -> Self;
    }
    ```

#### 2.2. Blanket Implementations
To improve ergonomics, the framework provides blanket implementations that allow `From1` to be the single entry point for tuple-based conversions. This enables `from!((a, b))` to work seamlessly.

*   `impl<T, All> From1<(T,)> for All where All: From1<T>`
*   `impl<T1, T2, All> From1<(T1, T2)> for All where All: From2<T1, T2>`
*   `impl<T1, T2, T3, All> From1<(T1, T2, T3)> for All where All: From3<T1, T2, T3>`
*   `impl<All> From1<()> for All where All: Default`

#### 2.3. The `VariadicFrom` Trait
This is a marker trait that enables the `#[derive(VariadicFrom)]` macro. It contains no methods. Its sole purpose is to be attached to a struct to signal that the derive macro should perform code generation for it.

### 3. Processing & Execution Model

#### 3.1. The `VariadicFrom` Derive Macro (`variadic_from_meta`)

The derive macro is the core of the crate's code generation capabilities.

*   **Activation:** The macro is activated when a struct is annotated with `#[derive(VariadicFrom)]`.
*   **Processing Steps:**
    1.  The macro receives the Abstract Syntax Tree (AST) of the struct.
    2.  It inspects the struct's body to determine if it has named or unnamed (tuple) fields.
    3.  It counts the number of fields.
    4.  It extracts the types and generics of the struct.
*   **Code Generation Logic:**
    *   **Generics Handling:** All generated `impl` blocks **must** correctly propagate the struct's generic parameters, including lifetimes, types, consts, and `where` clauses.
    *   **If field count is 1:**
        *   Generates `impl<...> From1<FieldType> for StructName<...>`
        *   Generates `impl<...> From<FieldType> for StructName<...>` which delegates to `From1::from1`.
        *   *Example for `struct S(i32)`:* `impl From<i32> for S { fn from(val: i32) -> Self { Self::from1(val) } }`
    *   **If field count is 2:**
        *   Generates `impl<...> From2<T1, T2> for StructName<...>`
        *   Generates `impl<...> From<(T1, T2)> for StructName<...>` which delegates to `From2::from2`.
        *   **Convenience `From1`:** Generates `impl<...> From1<T1> for StructName<...>` **if and only if** the types of both fields (`T1` and `T2`) are identical. The implementation assigns the single argument to both fields.
        *   *Example for `struct S { a: i32, b: i32 }`:* `impl From1<i32> for S { fn from1(val: i32) -> Self { Self { a: val, b: val } } }`
    *   **If field count is 3:**
        *   Generates `impl<...> From3<T1, T2, T3> for StructName<...>`
        *   Generates `impl<...> From<(T1, T2, T3)> for StructName<...>` which delegates to `From3::from3`.
        *   **Convenience `From1` and `From2`:**
            *   Generates `impl<...> From1<T1> for StructName<...>` **if and only if** all three field types (`T1`, `T2`, `T3`) are identical.
            *   Generates `impl<...> From2<T1, T2> for StructName<...>` **if and only if** the second and third field types (`T2`, `T3`) are identical. The implementation assigns `arg1` to the first field and `arg2` to the second and third fields.
    *   **If field count is 0 or greater than 3:** The derive macro generates **no code**.

#### 3.2. The `from!` Macro (`variadic_from`)

The `from!` macro provides a convenient, unified syntax for variadic construction. It is a standard `macro_rules!` macro that dispatches to the correct implementation based on the number of arguments provided at the call site.

*   **Resolution Rules:**
    *   `from!()` expands to `::core::default::Default::default()`. This requires the target type to implement the `Default` trait.
    *   `from!(arg1)` expands to `$crate::variadic::From1::from1(arg1)`.
    *   `from!(arg1, arg2)` expands to `$crate::variadic::From2::from2(arg1, arg2)`.
    *   `from!(arg1, arg2, arg3)` expands to `$crate::variadic::From3::from3(arg1, arg2, arg3)`.
    *   `from!(arg1, ..., argN)` where `N > 3` results in a `compile_error!`, providing a clear message that the maximum number of arguments has been exceeded.

### 4. Interaction Modalities

#### 4.1. Direct Instantiation via `from!`
This is the primary and most expressive way to use the crate.

*   **Example:**
    ```rust
    # use variadic_from::exposed::*;
    #[derive(Debug, PartialEq, Default, VariadicFrom)]
    struct Point 
{
        x: i32,
        y: i32,
    }

    // Zero arguments (requires `Default`)
    let p0: Point = from!(); // Point { x: 0, y: 0 }

    // One argument (uses generated convenience `From1`)
    let p1: Point = from!(10); // Point { x: 10, y: 10 }

    // Two arguments (uses generated `From2`)
    let p2: Point = from!(10, 20); // Point { x: 10, y: 20 }
    ```

#### 4.2. Standard Conversion via `From` and `Into`
By generating `From<T>` and `From<tuple>` implementations, the derive macro enables seamless integration with the standard library's conversion traits.

*   **Example:**
    ```rust
    # use variadic_from::exposed::*;
    #[derive(Debug, PartialEq, Default, VariadicFrom)]
    struct Point(i32, i32);

    // Using From::from
    let p1: Point = Point::from((10, 20)); // Point(10, 20)

    // Using .into()
    let p2: Point = (30, 40).into(); // Point(30, 40)

    // Using from! with a tuple (leverages the From1 blanket impl)
    let p3: Point = from!((50, 60)); // Point(50, 60)
    ```

### 5. Cross-Cutting Concerns

#### 5.1. Error Handling Strategy
All error handling is designed to occur at **compile time**, providing immediate feedback to the developer.
*   **Invalid Argument Count:** Calling the `from!` macro with more than 3 arguments results in a clear, explicit `compile_error!`.
*   **Unsupported Struct Size:** The `VariadicFrom` derive macro will not generate code for structs with 0 or more than 3 fields. This will result in a standard "method not found" or "trait not implemented" compile error if code attempts to use a non-existent `FromN` implementation.
*   **Type Mismatches:** Standard Rust type-checking rules apply. If the arguments passed to `from!` do not match the types expected by the corresponding `FromN` implementation, a compile error will occur.

#### 5.2. Extensibility Model
The framework is designed to be extensible through manual trait implementation.
*   **Custom Logic:** Developers can implement any of the `FromN` traits manually to provide custom construction logic that overrides the derived behavior or adds new conversion paths.
*   **Supporting Larger Structs:** For structs with more than 3 fields, developers can manually implement the standard `From<tuple>` trait to provide similar ergonomics, though they will not be able to use the `from!` macro for more than 3 arguments.

### 6. Architectural Principles & Design Rules

*   **Modular Design with Traits:** The crate's functionality is built upon a set of public `FromN` traits. This allows for clear contracts and enables developers to extend the functionality with their own custom implementations.
*   **Private Implementation:** Internal logic is kept in private modules (e.g., `variadic`). The public API is exposed through a controlled interface (`exposed`, `prelude`) to hide implementation details and allow for internal refactoring without breaking changes.
*   **Compile-Time Safety:** All error handling must occur at **compile time**. The `from!` macro uses `compile_error!` for invalid argument counts, and the derive macro relies on the compiler to report type mismatches or missing trait implementations.
*   **Generated Path Resolution:**
    *   The `from!` declarative macro **must** use `$crate::...` paths (e.g., `$crate::variadic::From1`) to ensure it works correctly regardless of how the `variadic_from` crate is imported.
    *   The `VariadicFrom` derive macro **must** use absolute paths (e.g., `::variadic_from::exposed::From1`) to ensure the generated code is robust against crate renaming and aliasing in the consumer's `Cargo.toml`.
*   **Dependency Management:** The `variadic_from_meta` crate must prefer using the `macro_tools` crate over direct dependencies on `syn`, `quote`, or `proc-macro2` to leverage its higher-level abstractions.
*   **Test Organization:** All automated tests must reside in the `tests/` directory, separate from the `src/` directory, to maintain a clear distinction between production and test code.

### 7. Appendices

#### A.1. Code Examples

##### Named Struct Example
```rust
use variadic_from::exposed::*;

#[derive(Debug, PartialEq, Default, VariadicFrom)]
struct UserProfile 
{
    id: u32,
    username: String,
}

// Manual implementation for a single argument for convenience
impl From1<&str> for UserProfile 
{
    fn from1(name: &str) -> Self 
{
        Self { id: 0, username: name.to_string() }
    }
}

// Generated implementations allow these conversions:
let _user1: UserProfile = from!(101, "admin".to_string());
let _user2: UserProfile = (102, "editor".to_string()).into();

// Manual implementation allows this:
let _user3: UserProfile = from!("guest");
```

##### Unnamed (Tuple) Struct Example
```rust
use variadic_from::exposed::*;

#[derive(Debug, PartialEq, Default, VariadicFrom)]
struct Point(i32, i32, i32);

// Generated implementations allow these conversions:
let _p1: Point = from!();
let _p2: Point = from!(1, 2, 3);
let _p3: Point = (4, 5, 6).into();
```

### 8. Meta-Requirements

This specification document must adhere to the following rules to ensure its clarity, consistency, and maintainability.
*   **Ubiquitous Language:** All terms defined in the `Key Terminology` section must be used consistently throughout this document and all related project artifacts.
*   **Repository as Single Source of Truth:** The version control repository is the single source of truth for all project artifacts, including this specification.
*   **Naming Conventions:** All asset names (files, variables, etc.) must use `snake_case`.
*   **Mandatory Structure:** This document must follow the agreed-upon section structure. Additions must be justified and placed appropriately.

### 9. Deliverables

*   The `variadic_from` crate, containing the public traits, `from!` macro, and blanket implementations.
*   The `variadic_from_meta` crate, containing the `#[derive(VariadicFrom)]` procedural macro.
*   `specification.md`: This document.
*   `spec_addendum.md`: A template for developers to fill in implementation-specific details.

### 10. Conformance Check Procedure

The following checks must be performed to verify that an implementation of the `variadic_from` crate conforms to this specification.

1.  **Derive on 1-Field Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a struct with 1 field.
    *   **Expected:** The code compiles. `impl From1` and `impl From<T>` are generated and work as expected.
2.  **Derive on 2-Field Named Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a named struct with 2 fields of different types (e.g., `i32`, `String`).
    *   **Expected:** The code compiles. `impl From2` and `impl From<(i32, String)>` are generated. The convenience `impl From1<i32>` is **not** generated.
3.  **Derive on 3-Field Unnamed Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to an unnamed (tuple) struct with 3 fields of the same type (e.g., `i32, i32, i32`).
    *   **Expected:** The code compiles. `impl From3`, `impl From<(i32, i32, i32)>`, and convenience `impl From1<i32>` and `impl From2<i32, i32>` are generated.
4.  **`from!` Macro Correctness:**
    *   **Action:** Call `from!()`, `from!(a)`, `from!(a, b)`, and `from!(a, b, c)` on conforming types.
    *   **Expected:** All calls compile and produce the correct struct instances.
5.  **`from!` Macro Error Handling:**
    *   **Action:** Call `from!(a, b, c, d)`.
    *   **Expected:** The code fails to compile with an error message explicitly stating the argument limit has been exceeded.
6.  **Tuple Conversion Correctness:**
    *   **Action:** Use `(a, b).into()` and `MyStruct::from((a, b))` on a derived 2-field struct.
    *   **Expected:** Both conversions compile and produce the correct struct instance.
7.  **Derive on 4-Field Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a struct with 4 fields and attempt to call `from!(a, b)`.
    *   **Expected:** The code fails to compile with an error indicating that `From2` is not implemented, confirming the derive macro generated no code.
8.  **Manual `From1` Implementation:**
    *   **Action:** Create a struct with `#[derive(VariadicFrom)]` and also provide a manual `impl From1<T> for MyStruct`.
    *   **Expected:** Calling `from!(t)` uses the manual implementation, demonstrating that the compiler selects the more specific, user-defined logic.
9.  **Generics Handling:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a struct with generic parameters and a `where` clause.
    *   **Expected:** The generated `impl` blocks correctly include the generics and `where` clause, and the code compiles.

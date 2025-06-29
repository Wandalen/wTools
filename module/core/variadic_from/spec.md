# Technical Specification: `variadic_from` Crate

### 1. Introduction & Core Concepts

#### 1.1. Goals & Philosophy

The primary goal of the `variadic_from` crate is to enhance developer ergonomics and reduce boilerplate code in Rust by providing flexible, "variadic" constructors for structs. The core philosophy is to offer a single, intuitive, and consistent interface for struct instantiation, regardless of the number of initial arguments (within defined limits).

The framework is guided by these principles:
*   **Convention over Configuration:** The system should work out-of-the-box with sensible defaults. The `VariadicFrom` derive macro should automatically generate the necessary implementations for the most common use cases without requiring manual configuration.
*   **Minimal Syntactic Noise:** The user-facing `from!` macro provides a clean, concise way to construct objects, abstracting away the underlying implementation details of which `FromN` trait is being called.
*   **Seamless Integration:** The crate should feel like a natural extension of the Rust language. It achieves this by automatically implementing the standard `From<T>` trait for single fields and `From<tuple>` for multiple fields, enabling idiomatic conversions like `.into()`.
*   **Non-Intrusive Extensibility:** While the derive macro handles the common cases, the system is built on a foundation of public traits (`From1`, `From2`, etc.) that developers can implement manually for custom behavior or to support types not covered by the macro.

#### 1.2. Key Terminology (Ubiquitous Language)

*   **Variadic Constructor:** A constructor that can accept a variable number of arguments. In the context of this crate, this is achieved through the `from!` macro.
*   **`FromN` Traits:** A set of custom traits (`From1`, `From2`, `From3`) that define a contract for constructing a type from a specific number (`N`) of arguments.
*   **`VariadicFrom` Trait:** A marker trait implemented via a derive macro (`#[derive(VariadicFrom)]`). Its presence on a struct signals that the derive macro should automatically implement the appropriate `FromN` and `From<T>`/`From<tuple>` traits based on the number of fields in the struct.
*   **`from!` Macro:** A declarative, user-facing macro that provides the primary interface for variadic construction. It resolves to a call to `Default::default()`, `From1::from1`, `From2::from2`, or `From3::from3` based on the number of arguments provided.
*   **Named Struct:** A struct where fields are defined with explicit names, e.g., `struct MyStruct { a: i32 }`.
*   **Unnamed Struct (Tuple Struct):** A struct where fields are defined by their type only, e.g., `struct MyStruct(i32)`.

#### 1.3. Versioning Strategy

The `variadic_from` crate adheres to the Semantic Versioning 2.0.0 (SemVer) standard.
*   **MAJOR** version changes indicate incompatible API changes.
*   **MINOR** version changes introduce new, backward-compatible functionality (e.g., increasing the maximum number of supported arguments).
*   **PATCH** version changes are for backward-compatible bug fixes.

This specification document is versioned in lockstep with the crate itself.

### 2. Core Object Definitions

This section provides the formal definitions for the traits that constitute the `variadic_from` framework. These traits define the contracts that are either implemented automatically by the derive macro or manually by the user.

#### 2.1. The `FromN` Traits

The `FromN` traits provide a standardized interface for constructing a type from a specific number (`N`) of arguments.

##### 2.1.1. `From1<Arg>`
*   **Purpose:** Defines a contract for constructing an object from a single argument. It also serves as a unified interface for converting from tuples of varying lengths, which are treated as a single argument.
*   **Signature:**
    ```rust
    pub trait From1<Arg>
    where
        Self: Sized,
    {
        fn from1(arg: Arg) -> Self;
    }
    ```
*   **Blanket Implementations:** The framework provides blanket implementations to unify tuple-based construction under `From1`:
    *   `impl<T, All> From1<(T,)> for All where All: From1<T>`
    *   `impl<T1, T2, All> From1<(T1, T2)> for All where All: From2<T1, T2>`
    *   `impl<T1, T2, T3, All> From1<(T1, T2, T3)> for All where All: From3<T1, T2, T3>`
    *   `impl<All> From1<()> for All where All: Default`

##### 2.1.2. `From2<Arg1, Arg2>`
*   **Purpose:** Defines a contract for constructing an object from exactly two arguments.
*   **Signature:**
    ```rust
    pub trait From2<Arg1, Arg2>
    where
        Self: Sized,
    {
        fn from2(arg1: Arg1, arg2: Arg2) -> Self;
    }
    ```

##### 2.1.3. `From3<Arg1, Arg2, Arg3>`
*   **Purpose:** Defines a contract for constructing an object from exactly three arguments.
*   **Signature:**
    ```rust
    pub trait From3<Arg1, Arg2, Arg3>
    where
        Self: Sized,
    {
        fn from3(arg1: Arg1, arg2: Arg2, arg3: Arg3) -> Self;
    }
    ```

#### 2.2. The `VariadicFrom` Trait

*   **Purpose:** This is a marker trait that enables the `#[derive(VariadicFrom)]` macro. It does not contain any methods. Its sole purpose is to be attached to a struct to signal that the derive macro should perform code generation for it.
*   **Definition:** The trait is defined externally (in `derive_tools_meta`) but is exposed through the `variadic_from` crate.
*   **Behavior:** When a struct is decorated with `#[derive(VariadicFrom)]`, the derive macro is responsible for:
    1.  Implementing the `VariadicFrom` trait for that struct.
    2.  Generating implementations for the appropriate `FromN` trait(s).
    3.  Generating an implementation for the standard `From<T>` trait (for single-field structs) or `From<tuple>` trait (for multi-field structs).

### 3. Processing & Execution Model

This section details the internal logic of the crate's two primary components: the `VariadicFrom` derive macro and the `from!` macro.

#### 3.1. The `VariadicFrom` Derive Macro

The derive macro is the core of the crate's code generation capabilities.

*   **Activation:** The macro is activated when a struct is annotated with `#[derive(VariadicFrom)]`.
*   **Processing Steps:**
    1.  The macro receives the Abstract Syntax Tree (AST) of the struct it is attached to.
    2.  It inspects the struct's body to determine its kind (Named or Unnamed/Tuple) and counts the number of fields.
    3.  It extracts the types of each field in their declared order.
*   **Code Generation Logic:**
    *   **If field count is 1, 2, or 3:**
        *   It generates an implementation of the corresponding `FromN` trait. For a struct with `N` fields, it generates `impl FromN<T1, ..., TN> for MyStruct`, where `T1..TN` are the field types. The body of the generated function constructs an instance of the struct, mapping the arguments to the fields in order.
        *   For structs with 2 or 3 fields, it generates an implementation of the standard `From<(T1, ..., TN)>` trait. The body of this implementation delegates directly to the newly implemented `FromN` trait, calling `Self::fromN(...)`.
        *   For structs with 1 field, it generates an implementation of the standard `From<T>` trait (where `T` is the type of the single field). The body of this implementation delegates directly to the newly implemented `From1` trait, calling `Self::from1(...)`.
    *   **If field count is 0 or greater than 3:** The derive macro generates no code. This is a deliberate design choice to prevent unexpected behavior for unsupported struct sizes.

#### 3.2. The `from!` Macro

The `from!` macro provides a convenient, unified syntax for variadic construction. It is a standard `macro_rules!` macro that dispatches to the correct implementation based on the number of arguments provided at the call site.

*   **Resolution Rules:**
    *   `from!()` expands to `::core::default::Default::default()`. This requires the target type to implement the `Default` trait.
    *   `from!(arg1)` expands to `$crate::From1::from1(arg1)`.
    *   `from!(arg1, arg2)` expands to `$crate::From2::from2(arg1, arg2)`.
    *   `from!(arg1, arg2, arg3)` expands to `$crate::From3::from3(arg1, arg2, arg3)`.
    *   `from!(arg1, ..., argN)` where `N > 3` results in a `compile_error!`, providing a clear message that the maximum number of arguments has been exceeded.

### 4. Interaction Modalities

Users can leverage the `variadic_from` crate in two primary ways, both designed to be idiomatic Rust.

#### 4.1. Direct Instantiation via `from!`

This is the most direct and expressive way to use the crate. It allows for the creation of struct instances with a variable number of arguments.

*   **Example:**
    ```rust
    // Assumes MyStruct has two fields: i32, i32
    // and also implements Default and From1<i32>

    // Zero arguments (requires `Default`)
    let s0: MyStruct = from!();

    // One argument (requires manual `From1<i32>`)
    let s1: MyStruct = from!(10);

    // Two arguments (uses generated `From2`)
    let s2: MyStruct = from!(10, 20);
    ```

#### 4.2. Tuple Conversion via `From` and `Into`

By generating `From<tuple>` implementations, the derive macro enables seamless integration with the standard library's conversion traits.

*   **Example:**
    ```rust
    // Assumes MyStruct has two fields: i32, i32

    // Using From::from
    let s1: MyStruct = MyStruct::from((10, 20));

    // Using .into()
    let s2: MyStruct = (10, 20).into();

    // Using from! with a tuple (leverages the From1 blanket impl)
    let s3: MyStruct = from!((10, 20));
    ```

### 5. Cross-Cutting Concerns

#### 5.1. Error Handling Strategy

All error handling occurs at **compile time**, which is ideal for a developer utility crate.
*   **Invalid Argument Count:** Calling the `from!` macro with more than 3 arguments results in a clear, explicit `compile_error!`.
*   **Unsupported Struct Size:** The `VariadicFrom` derive macro will simply not generate code for structs with 0 or more than 3 fields. This will result in a subsequent compile error if code attempts to use a non-existent `FromN` implementation (e.g., "no method named `from2` found").
*   **Type Mismatches:** Standard Rust type-checking rules apply. If the arguments passed to `from!` do not match the types expected by the corresponding `FromN` implementation, a compile error will occur.

#### 5.2. Extensibility Model

The framework is designed to be extensible through manual trait implementation.
*   **Custom Logic:** Users can (and are encouraged to) implement `From1` manually to provide custom construction logic from a single value, as shown in the `variadic_from_trivial.rs` example.
*   **Overriding Behavior:** A manual implementation of a `FromN` trait will always take precedence over a generated one if both were somehow present.
*   **Supporting Larger Structs:** For structs with more than 3 fields, users can manually implement the `From<tuple>` trait to provide similar ergonomics, though they will not be able to use the `from!` macro for more than 3 arguments.

### 6. Known Limitations

*   **Argument Count Limit:** The `VariadicFrom` derive macro and the `from!` macro are hard-coded to support a maximum of **three** arguments/fields. There is no support for variadic generics beyond this limit.
*   **Type Inference:** In highly complex generic contexts, the compiler may require explicit type annotations (turbofish syntax) to resolve the correct `FromN` implementation. This is a general characteristic of Rust's type system rather than a specific flaw of the crate.

### 7. Appendices

#### A.1. Code Examples

##### Named Struct Example
```rust
use variadic_from::exposed::*;

#[derive(Debug, PartialEq, Default, VariadicFrom)]
struct UserProfile {
    id: u32,
    username: String,
}

// Manual implementation for a single argument
impl From1<&str> for UserProfile {
    fn from1(name: &str) -> Self {
        Self { id: 0, username: name.to_string() }
    }
}

// Usage:
let u1: UserProfile = from!(); // -> UserProfile { id: 0, username: "" }
let u2: UserProfile = from!("guest"); // -> UserProfile { id: 0, username: "guest" }
let u3: UserProfile = from!(101, "admin".to_string()); // -> UserProfile { id: 101, username: "admin" }
let u4: UserProfile = (102, "editor".to_string()).into(); // -> UserProfile { id: 102, username: "editor" }
```

##### Unnamed (Tuple) Struct Example
```rust
use variadic_from::exposed::*;

#[derive(Debug, PartialEq, Default, VariadicFrom)]
struct Point(i32, i32, i32);

// Usage:
let p1: Point = from!(); // -> Point(0, 0, 0)
let p2: Point = from!(1, 2, 3); // -> Point(1, 2, 3)
let p3: Point = (4, 5, 6).into(); // -> Point(4, 5, 6)
```

### 8. Meta-Requirements

This specification document must adhere to the following rules to ensure its clarity, consistency, and maintainability.
*   **Ubiquitous Language:** All terms defined in the `Key Terminology` section must be used consistently throughout this document and all related project artifacts.
*   **Naming Conventions:** All asset names (files, variables, etc.) must use `snake_case`.
*   **Mandatory Structure:** This document must follow the agreed-upon section structure. Additions must be justified and placed appropriately.

### 9. Deliverables

Working solution.

### 10. Conformance Check Procedure

The following checks must be performed to verify that an implementation of the `variadic_from` crate conforms to this specification.

1.  **Derive on 2-Field Named Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a named struct with 2 fields.
    *   **Expected:** The code compiles. `impl From2` and `impl From<(T1, T2)>` are generated.
2.  **Derive on 3-Field Unnamed Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to an unnamed (tuple) struct with 3 fields.
    *   **Expected:** The code compiles. `impl From3` and `impl From<(T1, T2, T3)>` are generated.
3.  **`from!` Macro Correctness:**
    *   **Action:** Call `from!()`, `from!(a)`, `from!(a, b)`, and `from!(a, b, c)` on conforming types.
    *   **Expected:** All calls compile and produce the correct struct instances as defined by the `Default`, `From1`, `From2`, and `From3` traits respectively.
4.  **`from!` Macro Error Handling:**
    *   **Action:** Call `from!(a, b, c, d)`.
    *   **Expected:** The code fails to compile with an error message explicitly stating the argument limit has been exceeded.
5.  **Tuple Conversion Correctness (2-3 fields):**
    *   **Action:** Use `(a, b).into()` and `MyStruct::from((a, b))` on a derived 2-field struct.
    *   **Expected:** Both conversions compile and produce the correct struct instance.
6.  **Single-Field Conversion Correctness:**
    *   **Action:** Use `a.into()` and `MyStruct::from(a)` on a derived 1-field struct.
    *   **Expected:** Both conversions compile and produce the correct struct instance.
7.  **Derive on 4-Field Struct:**
    *   **Action:** Apply `#[derive(VariadicFrom)]` to a struct with 4 fields and attempt to call `from!(a, b, c, d)`.
    *   **Expected:** The code fails to compile with an error indicating that no `From4` trait or method exists, confirming the derive macro did not generate code.
8.  **Manual `From1` Implementation:**
    *   **Action:** Create a struct with `#[derive(VariadicFrom)]` and also provide a manual `impl From1<T> for MyStruct`.
    *   **Expected:** Calling `from!(t)` uses the manual implementation, demonstrating that user-defined logic can coexist with the derived logic.
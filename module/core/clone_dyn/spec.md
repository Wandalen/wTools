# clone_dyn

Enable cloning of trait objects (`dyn Trait`) via procedural macro.

## Overview

The `clone_dyn` ecosystem provides a simple and ergonomic solution for cloning trait objects in Rust. By applying the `#[clone_dyn]` procedural macro to a trait definition, the necessary boilerplate code is automatically generated, overcoming the standard library's limitation where the `Clone` trait is not object-safe.

### Scope

#### Responsibility

clone_dyn is responsible for enabling cloning of trait objects (`Box<dyn Trait>`) through a procedural macro. It generates the required `impl Clone` blocks and manages the underlying type-erased cloning mechanism via the `CloneDyn` trait.

#### In-Scope

- **`#[clone_dyn]` macro**: Procedural attribute macro for trait definitions
- **`CloneDyn` trait**: Object-safe marker trait enabling type-erased cloning
- **`clone_into_box()` function**: Core unsafe function performing trait object cloning
- **Auto-trait combinations**: Generated impls for `dyn Trait`, `dyn Trait + Send`, `dyn Trait + Sync`, `dyn Trait + Send + Sync`
- **Three-crate ecosystem**: `clone_dyn` (facade), `clone_dyn_meta` (proc-macro), `clone_dyn_types` (core traits)

#### Out-of-Scope

- **Non-Box smart pointers**: Only `Box<dyn Trait>` is supported, not `Rc`/`Arc`
- **Custom clone behavior**: Always uses standard Clone, no customization
- **Runtime type inspection**: No downcasting or type identification
- **Serialization**: No serde integration

#### Boundaries

- **Upstream**: Depends on `macro_tools` for proc-macro implementation
- **Downstream**: Used by any code needing clonable trait objects
- **Crate boundary**: Facade (`clone_dyn`) re-exports from `clone_dyn_meta` and `clone_dyn_types`

## Technical Specification

### Project Goal

To provide Rust developers with a simple and ergonomic solution for cloning trait objects (`dyn Trait`). This is achieved by offering a procedural macro (`#[clone_dyn]`) that automatically generates the necessary boilerplate code, overcoming the standard library's limitation where the `Clone` trait is not object-safe. The ecosystem is designed to be a "one-liner" solution that is both easy to use and maintain.

### Problem Statement

In Rust, the standard `Clone` trait cannot be used for trait objects. This is because `Clone::clone()` returns `Self`, a concrete type whose size must be known at compile time. For a trait object like `Box<dyn MyTrait>`, the concrete type is erased, and its size is unknown. This "object safety" rule prevents developers from easily duplicating objects that are managed via trait objects. This becomes particularly acute when working with heterogeneous collections, such as `Vec<Box<dyn Drawable>>`, making the `clone_dyn` ecosystem essential for such use cases.

### Ubiquitous Language (Vocabulary)

| Term | Definition |
| :--- | :--- |
| **`clone_dyn` Ecosystem** | The set of three related crates: `clone_dyn` (facade), `clone_dyn_meta` (proc-macro), and `clone_dyn_types` (core traits/logic). |
| **Trait Object** | A reference to a type that implements a specific trait (e.g., `Box<dyn MyTrait>`). The concrete type is erased at compile time. |
| **Object Safety** | A set of rules in Rust that determine if a trait can be made into a trait object. The standard `Clone` trait is not object-safe. |
| **`CloneDyn`** | The central, object-safe trait provided by this ecosystem. Any type that implements `CloneDyn` can be cloned even when it is a trait object. |
| **`#[clone_dyn]`** | The procedural macro attribute that serves as the primary developer interface. Applying this to a trait definition automatically implements `Clone` for its corresponding trait objects. |
| **`clone_into_box()`** | The low-level, `unsafe` function that performs the actual cloning of a trait object, returning a new `Box<T>`. |
| **Feature Combination** | A specific set of Cargo features enabled during a build or test run (e.g., `--no-default-features --features clone_dyn_types`). |

### Non-Functional Requirements (NFRs)

1.  **Code Quality:** All crates in the ecosystem **must** compile without any warnings when checked with `cargo clippy -- -D warnings`.
2.  **Test Coverage:** Tests **must** provide comprehensive coverage for all public APIs and logic. This includes dedicated tests for all meaningful **Feature Combinations** to prevent regressions.
3.  **Documentation:** All public APIs **must** be fully documented with clear examples. The `Readme.md` for each crate must be comprehensive and accurate. Test files should include a Test Matrix in their documentation to justify their coverage.
4.  **Ergonomics:** The primary method for using the library (`#[clone_dyn]`) must be a simple, "one-liner" application of a procedural macro.

### System Architecture

The `clone_dyn` ecosystem follows a layered architectural model based on the **Separation of Concerns** principle. The project is divided into three distinct crates, each with a single, well-defined responsibility.

*   #### Architectural Principles
    *   **Standardize on `macro_tools`:** The `clone_dyn_meta` crate **must** standardize on the `macro_tools` crate for all its implementation. It uses `macro_tools`'s high-level abstractions for parsing, code generation, and error handling, and **must not** depend directly on `proc-macro2`, `quote`, or `syn`. This ensures consistency and reduces boilerplate.

*   #### Crate Breakdown
    *   **`clone_dyn_types` (Foundation Layer):** Provides the core `CloneDyn` trait and the `unsafe` `clone_into_box()` cloning logic.
    *   **`clone_dyn_meta` (Code Generation Layer):** Implements the `#[clone_dyn]` procedural macro, adhering to the `macro_tools` standardization principle.
    *   **`clone_dyn` (Facade Layer):** The primary user-facing crate, re-exporting components from the other two crates to provide a simple, unified API.

*   #### Data & Control Flow Diagram
    ```mermaid
    sequenceDiagram
        actor Developer
        participant Rust Compiler
        participant clone_dyn_meta (Macro)
        participant clone_dyn_types (Logic)

        Developer->>+Rust Compiler: Writes `#[clone_dyn]` on a trait and runs `cargo build`
        Rust Compiler->>+clone_dyn_meta (Macro): Invokes the procedural macro on the trait's code
        clone_dyn_meta (Macro)->>clone_dyn_meta (Macro): Parses trait using `macro_tools` abstractions
        clone_dyn_meta (Macro)-->>-Rust Compiler: Generates `impl Clone for Box<dyn ...>` code
        Note right of Rust Compiler: Generated code contains calls to `clone_into_box()`
        Rust Compiler->>clone_dyn_types (Logic): Compiles generated code, linking to `clone_into_box()`
        Rust Compiler-->>-Developer: Produces final compiled binary
    ```

### Core Trait & Function Definitions

*   #### The `CloneDyn` Trait
    *   **Purpose:** A marker trait that provides the underlying mechanism for cloning a type in a type-erased (dynamic) context.
    *   **Internal Method:** Contains a hidden method `__clone_dyn(&self) -> *mut ()` which returns a raw, heap-allocated pointer to a clone of the object.

*   #### The `clone_into_box()` Function
    *   **Purpose:** The core `unsafe` function that performs the cloning of a trait object.
    *   **Signature:** `pub fn clone_into_box<T>(ref_dyn: &T) -> Box<T> where T: ?Sized + CloneDyn`
    *   **Behavior:** It calls the `__clone_dyn` method on the trait object to get a raw pointer to a new, cloned instance on the heap, and then safely converts that raw pointer back into a `Box<T>`.

### Developer Interaction Models

*   #### High-Level (Recommended): The `#[clone_dyn]` Macro
    *   **Usage:** The developer applies the `#[clone_dyn]` attribute directly to a trait definition.
    *   **Behavior:** The macro automatically adds a `where Self: CloneDyn` supertrait bound and generates four `impl Clone for Box<dyn ...>` blocks (base case and combinations with `Send`/`Sync`).

*   #### Low-Level (Manual): Direct Usage
    *   **Usage:** A developer can depend only on `clone_dyn_types` for full manual control.
    *   **Behavior:** The developer is responsible for manually adding the `where Self: CloneDyn` supertrait and writing all `impl Clone` blocks.

### Cross-Cutting Concerns

*   **Security Model (Unsafe Code):** The use of `unsafe` code in `clone_into_box` is necessary to bridge Rust's compile-time type system with the runtime nature of trait objects. Its safety relies on the contract that `CloneDyn`'s internal method always returns a valid, heap-allocated pointer to a new instance of the same type.
*   **Error Handling:** All error handling occurs at compile time. Incorrect macro usage results in a standard compilation error.
*   **Versioning Strategy:** The ecosystem adheres to Semantic Versioning (SemVer 2.0.0). The three crates are tightly coupled and must be released with synchronized version numbers.

### Meta-Requirements

1.  **Document Authority:** This document is the single source of truth for the design and quality standards of the `clone_dyn` ecosystem.
2.  **Tool Versions:** This specification is based on `rustc >= 1.70` and `macro_tools >= 0.36`.
3.  **Deliverable:** The sole deliverable is this `specification.md` document. The concept of a separate `spec_addendum.md` is deprecated; its essential ideas are incorporated into the appendices of this document.

### Conformance Check Procedure

This procedure must be run for each crate (`clone_dyn`, `clone_dyn_meta`, `clone_dyn_types`) to verify compliance with the specification. The full set of feature combinations to be tested are detailed in **Appendix A**.

1.  **Run Tests:** Execute `timeout 90 cargo test -p {crate_name}` with a specific feature set. If this fails, all test errors must be fixed before proceeding.
2.  **Run Linter:** Only if Step 1 passes, execute `timeout 90 cargo clippy -p {crate_name} -- -D warnings` with the same feature set. The command must pass with zero warnings.

---
### Appendices

#### Appendix A: Feature Combination Matrix

This table lists all meaningful feature combinations that must be tested for each crate in the ecosystem to ensure full compatibility and correctness.

| Crate | Command | Description |
|---|---|---|
| `clone_dyn` | `cargo test -p clone_dyn --no-default-features` | Tests that the crate compiles with no features enabled. |
| `clone_dyn` | `cargo test -p clone_dyn --no-default-features --features clone_dyn_types` | Tests the manual-clone functionality. |
| `clone_dyn` | `cargo test -p clone_dyn --features derive_clone_dyn` | Tests the full functionality with the proc-macro enabled. |
| `clone_dyn_types` | `cargo test -p clone_dyn_types --no-default-features` | Tests that the types crate compiles with no features enabled. |
| `clone_dyn_types` | `cargo test -p clone_dyn_types --features enabled` | Tests the types crate with its core features enabled. |
| `clone_dyn_meta` | `cargo test -p clone_dyn_meta --no-default-features` | Tests that the meta crate compiles with no features enabled. |
| `clone_dyn_meta` | `cargo test -p clone_dyn_meta --features enabled` | Tests the meta crate with its core features enabled. |

#### Appendix B: Detailed Test Matrix

This matrix outlines the test cases required to ensure comprehensive coverage of the `clone_dyn` ecosystem.

| ID | Description | Target Crate(s) | Test File(s) | Key Logic | Feature Combination |
|---|---|---|---|---|---|
| T1.1 | Verify `clone_into_box` for copyable types (`i32`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` |
| T1.2 | Verify `clone_into_box` for clonable types (`String`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` |
| T1.3 | Verify `clone_into_box` for slice types (`&str`, `&[i32]`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` |
| T2.1 | Verify `clone()` helper for various types. | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone` | `clone_dyn_types` |
| T3.1 | Manually implement `Clone` for a `Box<dyn Trait1>`. | `clone_dyn_types` | `inc/basic_manual.rs` | Manual `impl Clone` | `clone_dyn_types` |
| T4.1 | Use `#[clone_dyn]` on a simple trait. | `clone_dyn` | `inc/basic.rs` | `#[clone_dyn]` macro | `derive_clone_dyn` |
| T4.2 | Use `#[clone_dyn]` on a generic trait. | `clone_dyn` | `inc/parametrized.rs` | `#[clone_dyn]` macro | `derive_clone_dyn` |
| T5.1 | Ensure `clone_dyn_meta` uses `macro_tools` abstractions. | `clone_dyn_meta` | `src/clone_dyn.rs` | Macro implementation | `enabled` |
| T6.1 | Verify `clippy::doc_markdown` lint is fixed. | `clone_dyn` | `Readme.md` | Linting | `default` |

#### Appendix C: Release & Deployment Procedure

1.  Ensure all checks from the `Conformance Check Procedure` pass for all crates and all feature combinations listed in Appendix A.
2.  Increment the version number in the `Cargo.toml` of all three crates (`clone_dyn`, `clone_dyn_meta`, `clone_dyn_types`) according to SemVer.
3.  Publish the crates to `crates.io` in the correct dependency order:
    1.  `cargo publish -p clone_dyn_types`
    2.  `cargo publish -p clone_dyn_meta`
    3.  `cargo publish -p clone_dyn`
4.  Create a new git tag for the release version.

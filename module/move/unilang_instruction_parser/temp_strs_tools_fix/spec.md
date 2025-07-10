# Technical Specification: `strs_tools` (Definitive, Reviewed Version)

## Section 1: Global Architecture & Principles

This section defines the high-level architecture, rules, and design philosophies that apply to the entire `strs_tools` library.

### 1.1. Goals & Philosophy

The primary goal of `strs_tools` is to provide a powerful and flexible set of string manipulation utilities that empower developers to parse complex data with confidence and clarity.

*   **Configurability over Hardcoding:** Employ a fluent builder pattern (Formers).
*   **Correctness and Robustness:** Prioritize correct handling of edge cases.
*   **Modularity and Pay-as-you-go:** Utilize a feature-gating system.
*   **Clarity and Ergonomics:** Provide a clear and discoverable API.

### 1.2. Architectural Principles

These are the non-negotiable, crate-wide design laws.

1.  **Consumer Owns Unescaping:** The library **must not** perform any interpretation of escape sequences (e.g., `\"` -> `"`). It yields raw string slices. This is a critical security and correctness principle.
2.  **Panic on Invalid Configuration:** `Former` structures **must** panic if consumed with an invalid configuration. This treats configuration errors as developer errors.
3.  **Composition of Layers:** Higher-level modules **must** be implemented by composing the public APIs of lower-level modules.
4.  **Graceful Handling of Malformed Input:** The library **must not** panic on malformed user input (e.g., unclosed quotes) during iteration.

### 1.3. API Design & Namespace Philosophy

The library's public API is exposed through a deliberate, four-tiered namespace structure to provide flexibility for different import styles.

*   **`private` (Internal):** Contains all implementation details. It is not part of the public API.
*   **`own`:** Contains the primary, owned types of a module (e.g., `SplitIterator`). This is for developers who want to be explicit and avoid name clashes.
    *   *Usage Example:* `use strs_tools::string::split::own::SplitIterator;`
*   **`exposed`:** Re-exports the `own` namespace under the module's name (e.g., `pub use super::own as split`). This is the intended entry point for qualified path usage.
    *   *Usage Example:* `strs_tools::string::split::split()`
*   **`prelude`:** Contains the most essential types and builder functions intended for convenient glob import.
    *   *Usage Example:* `use strs_tools::prelude::*; let iter = split()...;`
*   **`orphan`:** An internal implementation detail used to structure the re-exports between `exposed` and `own`. It should not be used directly.

### 1.4. Component Interaction Model

The `strs_tools` library is designed as a system of composable layers. Higher-level modules delegate their core parsing logic to the `split` tokenizer, ensuring consistent behavior.

#### Static Structure

This diagram shows the static relationships between the main components.

```mermaid
graph TD
    subgraph User Facing API
        A[parse_request::request_parse] --> B{Request String};
        C[split::split] --> D{Source String};
        E[isolate::isolate_left] --> D;
    end

    subgraph Core Logic
        A -- delegates to --> C;
        A -- also delegates to --> E;
        C -- yields --> F[Split Iterator];
    end

    style A fill:#cde4ff,stroke:#333,stroke-width:2px
    style C fill:#cde4ff,stroke:#333,stroke-width:2px
    style E fill:#cde4ff,stroke:#333,stroke-width:2px
```

#### Dynamic Flow (Sequence Diagram)

This diagram illustrates the sequence of calls for a typical `parse_request` operation, demonstrating the "Composition of Layers" principle in action.

```mermaid
sequenceDiagram
    actor User
    participant PR as parse_request
    participant S as split
    participant I as isolate

    User->>PR: Calls .parse() on "cmd k:v"
    activate PR
    PR->>S: Calls .perform() on "cmd k:v" with "" delimiter
    activate S
    S-->>PR: Returns iterator yielding ["cmd k:v"]
    deactivate S
    PR->>I: Calls .isolate() on "cmd k:v" with ":" delimiter
    activate I
    I-->>PR: Returns ("cmd", Some(":"), "k:v")
    deactivate I
    PR->>S: Calls .perform() on "k:v" with ":" delimiter
    activate S
    S-->>PR: Returns iterator yielding ["k", "v"]
    deactivate S
    PR-->>User: Returns Request struct { subject: "cmd", map: {"k": "v"} }
    deactivate PR
```

### 1.5. API Usage & Lifetime Considerations

This section addresses critical design aspects of the API that affect how it must be used, particularly concerning data ownership and lifetimes. Failure to adhere to these patterns will likely result in compiler errors.

#### 1.5.1. Handling Dynamic Delimiters (The `E0716` Pitfall)

A primary design choice of the `split` module is that it **borrows** its delimiters. The `SplitOptionsFormer` holds a lifetime `'a` and expects string slices (`&'a str`) that live at least as long as the `Former` itself. This has a critical implication when working with owned `String` data.

**Problematic Pattern (will not compile):**
```rust,ignore
// This code will fail with E0716: temporary value dropped while borrowed
let my_delims: Vec<String> = vec!["a".to_string(), "b".to_string()];
let iter = split()
    // This creates a temporary Vec<&str> that is dropped at the end of the line,
    // leaving the Former with dangling references.
    .delimeter(my_delims.iter().map(|s| s.as_str()).collect::<Vec<_>>())
    .src("c a d b e")
    .perform();
```

**Correct Pattern:**
The `Vec<&str>` containing the borrowed slices must be bound to a variable with a lifetime that encloses the use of the `Former`.

```rust
let my_delims: Vec<String> = vec!["a".to_string(), "b".to_string()];
// 1. Create the vector of slices and bind it to a variable.
let delims_as_slices: Vec<&str> = my_delims.iter().map(|s| s.as_str()).collect();

// 2. Pass the bound variable to the Former. `delims_as_slices` now lives
//    long enough for the `perform()` call.
let iter = split()
    .delimeter(delims_as_slices)
    .src("c a d b e")
    .perform();
```

#### 1.5.2. The `&mut Self` Builder Pattern

The `Former` structs in this library use a builder pattern where configuration methods (e.g., `.src()`, `.quoting()`) return a mutable reference (`&mut Self`) rather than an owned value (`Self`).

*   **Implication:** This means a configured `Former` cannot be directly returned from a function, as this would involve moving out of a mutable reference.
*   **Rationale:** This design allows a `Former` to be created and then conditionally modified in multiple steps within the same scope before being consumed.

### 1.6. Non-Functional Requirements (NFRs)

| ID | Requirement | Description | Verification |
| :--- | :--- | :--- | :--- |
| **NFR-1** | **Performance** | Iteration over a string **must not** involve unnecessary allocations. The `SplitIterator` should be lazy and only perform work when `.next()` is called. | Benchmarks must show that splitting a large string without collecting has a low, constant memory overhead. |
| **NFR-2** | **Memory** | The library must be usable in `no_std` environments (with `alloc`). | The crate must successfully compile and pass all relevant tests with the `no_std` and `use_alloc` features enabled. |
| **NFR-3** | **Modularity** | Feature gates **must** successfully exclude unused modules from compilation. | Compiling with `--no-default-features --features string_split` must not compile the `parse_request` or `indentation` modules. |

### 1.7. Out of Scope

To clarify the library's boundaries, the following functionalities are explicitly out of scope:

*   **Character Set Conversion:** The library operates on Rust `&str` slices and assumes the input is valid UTF-8. It does not perform any encoding or decoding.
*   **Content Unescaping:** As per the architectural principles, the library does not interpret escape sequences (e.g., `\n`, `\t`, `\"`). This is the responsibility of the consumer.
*   **Network or I/O Operations:** This is a pure string manipulation library and will not include any features for reading from files, sockets, or other I/O sources.

---

## Section 2: Component Specifications

This section provides a detailed specification for each public module.

### 2.1. Module: `string::split`

#### Purpose

The core tokenization engine. It splits a string based on a complex set of rules, including multiple delimiters and quoted sections.

#### Internal Architecture

The module uses a two-iterator wrapper pattern. The user-facing `SplitIterator` provides the rich feature set (quoting, stripping) by managing and interpreting the raw output of a more primitive, internal `SplitFastIterator`.

```mermaid
graph TD
    subgraph Public API
        A[SplitOptionsFormer] -- .perform() --> B(SplitIterator);
    end
    subgraph Internal Logic
        B -- Wraps & Manages --> C(SplitFastIterator);
        C -- Performs basic tokenization --> D{Raw Split Segments};
        B -- Applies quoting/filtering rules to --> D;
        B -- Yields --> E[Final Split Struct];
    end
    style B fill:#cde4ff,stroke:#333,stroke-width:2px
```

#### Core Data Structures & API

*   **`struct Split<'a>`**: Represents a segment with `string`, `typ`, `start`, and `end` fields.
*   **`enum SplitType`**: `Delimited` or `Delimiter`.
*   **`bitflags! struct SplitFlags`**: `PRESERVING_EMPTY`, `PRESERVING_DELIMITERS`, `PRESERVING_QUOTING`, `STRIPPING`, `QUOTING`.
*   **`SplitOptionsFormer<'a>`**: The builder returned by `split()`. Provides methods like `.src()`, `.delimeter()`, `.quoting(bool)`, etc., and is consumed by `.perform()`.

### 2.2. Module: `string::parse_request`

#### Purpose

A higher-level parser for structured commands that have a subject and a map of key-value properties.

#### Core Data Structures & API

*   **`struct Request<'a>`**: Represents a parsed request with `original`, `subject`, `subjects`, `map`, and `maps` fields.
*   **`enum OpType<T>`**: A wrapper for a property value: `Primitive(T)` or `Vector(Vec<T>)`.
*   **`ParseOptions<'a>`**: The builder returned by `request_parse()`. Provides methods like `.src()`, `.key_val_delimeter()`, and is consumed by `.parse()`.

### 2.3. Module: `string::isolate`

#### Purpose

A specialized function to split a string into exactly three parts: left content, the first delimiter, and right content.

#### Core Data Structures & API

*   **`IsolateOptions<'a>`**: A builder returned by `isolate_left()` or `isolate_right()`.
*   `.isolate() -> (&'a str, Option<&'a str>, &'a str)`: Consumes the builder and returns the result tuple.

### 2.4. Module: `string::indentation`

#### Purpose

A stateless function to add a prefix and/or postfix to each line of a string.

#### Core Data Structures & API

*   `indentation(prefix, src, postfix) -> String`: A direct function call.

### 2.5. Module: `string::number`

#### Purpose

A thin wrapper around the `lexical` crate for parsing numbers, managed by the `string_parse_number` feature gate.

#### Core Data Structures & API

*   Re-exports functions like `parse<T>()` and `parse_partial<T>()` from the `lexical` crate.

---

### Section 3: Verification

#### 3.1. Conformance Check Procedure

This procedure verifies that an implementation conforms to this specification.

| Check ID | Module | Description | Rationale |
| :--- | :--- | :--- | :--- |
| **CHK-SPL-01** | `split` | **Default Behavior:** Correctly splits a simple string. | Ensures the most basic functionality is correct. |
| **CHK-SPL-02** | `split` | **Quoting:** Correctly treats a quoted section as a single token. | Verifies the core logic for handling complex, user-provided content. |
| **CHK-SPL-03** | `split` | **Span Indices:** Correctly reports the start/end byte indices. | Ensures that downstream tools can reliably locate tokens in the original source. |
| **CHK-REQ-01** | `parse_request` | **Composition:** Correctly parses a command with a subject and properties. | Verifies the composition of `split` and `isolate` to build a higher-level parser. |
| **CHK-ISO-01** | `isolate` | **Directional Isolate:** Correctly isolates the first delimiter from the specified direction. | Ensures the lightweight wrapper around `splitn`/`rsplitn` is functioning as expected. |
| **CHK-ARC-01** | Crate-wide | **Unescaping Principle:** Verify that escaped quotes are not unescaped by `split`. | Verifies strict adherence to the 'Consumer Owns Unescaping' architectural principle. |
| **CHK-API-01** | Crate-wide | **Dynamic Delimiter Lifetime:** Verify the documented pattern for using `Vec<String>` as delimiters compiles and works correctly. | To ensure the primary API pitfall is explicitly tested and the documented solution remains valid. |
| **CHK-NFR-03** | Crate-wide | **Modularity Principle:** Verify feature gates correctly exclude code. | Verifies adherence to the 'Modularity' NFR and ensures lean builds are possible. |

# Specification Addendum

### Purpose
This document is a companion to the main `specification.md`. It is intended to be completed by the **Developer** during the implementation phase. While the main specification defines the "what" and "why" of the project architecture, this addendum captures the "how" of the final implementation.

### Instructions for the Developer
As you build the system, please fill out the sections below with the relevant details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

### Implementation Notes
*A space for any key decisions, trade-offs, or discoveries made during development that are not captured elsewhere. For example: "Chose library X over Y because of its superior error handling for our specific use case."*

-   [Note 1]
-   [Note 2]

### Environment Variables
*List all environment variables required to run the application. Include the variable name, a brief description of its purpose, and an example value (use placeholders for secrets).*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `API_KEY_SERVICE_X` | The API key for connecting to Service X. | `sk_xxxxxxxxxxxx` |
| `DATABASE_URL` | The connection string for the production database. | `postgres://user:pass@host:port/db` |

### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions (e.g., from `package.json` or `requirements.txt`).*

-   `rustc`: `1.78.0`
-   `lexical`: `7.0.4`
-   `bitflags`: `2.5.0`

### Deployment Checklist
*A step-by-step guide for deploying the application from scratch. Include steps for setting up the environment, running migrations, and starting the services.*

1.  Clone the repository: `git clone ...`
2.  Install dependencies: `cargo build`
3.  Run test suite: `cargo test`
4.  ...
# Task Plan: Unilang - Phase 2

### Goal
*   Implement Phase 2 of the `unilang` roadmap: "Enhanced Type System, Runtime Commands & CLI Maturity". This involves expanding the argument type system, enabling runtime command registration, and maturing the CLI modality with features like advanced output formatting, shell completions, and interactive prompts.

### Ubiquitous Language (Vocabulary)
*   (Refer to `spec.md` Section 0.2 for the full glossary)
*   **`unilang`**: The specification and the Rust crate being developed.
*   **`utility1`**: A generic placeholder for a utility that integrates the `unilang` crate.
*   **`CommandDefinition`**: The complete specification of a command.
*   **`ArgumentDefinition`**: The specification of an argument for a command.
*   **`kind`**: The data type of an argument.
*   **`VerifiedCommand`**: A fully typed and validated representation of a command ready for execution.
*   **`ExecutionContext`**: An object passed to command routines providing access to global settings and services.
*   **`Test Matrix`**: A structured table to ensure comprehensive test coverage for new features.

### Progress
*   🚀 Phase 1 Complete (as per `roadmap.md`)
*   🚧 Phase 2 In Progress (Increment 2/8)

### Target Crate/Library
*   `module/move/unilang`

### Relevant Context
*   Files to Include:
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang/roadmap.md`
    *   `module/move/unilang/testing.md`
    *   `module/move/unilang/src/lib.rs`
    *   `module/move/unilang/src/parsing.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/interpreter.rs`
*   Crates for Documentation:
    *   `unilang`
    *   `unilang_instruction_parser`
    *   `unilang_meta`

### Expected Behavior Rules / Specifications (for Target Crate)
*   All new functionality must adhere strictly to the definitions in `spec.md`.
*   The implementation of argument types must correctly parse valid inputs and produce specific `ErrorData` for invalid inputs, as per `spec.md` Section 4.2.
*   Runtime command registration APIs must be thread-safe.

### Crate Conformance Check Procedure
*   Step 1: Run `timeout 90 cargo test -p unilang --all-targets` and verify no failures.
*   Step 2: Run `timeout 90 cargo clippy -p unilang -- -D warnings` and verify no errors or warnings.

### Increments
*   ✅ Increment 1: Implement Advanced Scalar and Path-like Argument Types
    *   **Goal:** Implement parsing and validation for `Path`, `File`, `Directory`, `Enum`, `URL`, `DateTime`, and `Pattern` argument kinds as defined in `spec.md` Section 2.2.2. This involves creating a new `types.rs` module, extending the core data structures to recognize these new kinds, implementing the parsing logic, and integrating it into the semantic analysis phase.
    *   **Steps:**
        *   Step 1: Create a new module file `src/types.rs` to house the type parsing and validation logic.
        *   Step 2: Add `pub mod types;` to `src/lib.rs`.
        *   Step 3: In `src/data.rs` (or equivalent core types file), extend the `Kind` enum to include variants for `Path`, `File`, `Directory`, `Enum(Vec<String>)`, `Url`, `DateTime`, and `Pattern`.
        *   Step 4: In `src/types.rs`, implement the parsing and validation functions for the new kinds. This will require adding dependencies like `url` and `chrono`.
        *   Step 5: In `src/semantic.rs`, integrate the new type parsing logic. The argument binding logic should call the appropriate parsing/validation function from `types.rs` based on the `ArgumentDefinition`'s `kind`.
        *   Step 6: Create a new integration test file `tests/inc/phase2/argument_types_test.rs`.
        *   Step 7: Implement the tests defined in the Test Matrix below within the new test file.
        *   Step 8: Perform Increment Verification.
        *   Step 9: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang -- --test-threads=1 argument_types_test` and verify all tests pass.
    *   **Test Matrix for Advanced Argument Types:**
| ID | Argument `kind` | Input Value | Expected Outcome | Notes |
|---|---|---|---|---|
| T1.1 | `Path` | `"./some/relative/path"` | Success, parsed as a relative path | |
| T1.2 | `Path` | `"/an/absolute/path"` | Success, parsed as an absolute path | (Unix-style) |
| T1.3 | `Path` | `"C:\\windows\\path"` | Success, parsed as an absolute path | (Windows-style) |
| T1.4 | `Path` | `""` | Error (`UNILANG_TYPE_MISMATCH`) | Empty string is not a valid path |
| T1.5 | `File` | `(path to an existing file)` | Success | Test setup must create the file |
| T1.6 | `File` | `(path to a directory)` | Error (`UNILANG_VALIDATION_RULE_FAILED`) | `File` kind requires a file, not a directory |
| T1.7 | `File` | `(path to non-existent file)` | Error (`UNILANG_VALIDATION_RULE_FAILED`) | Assuming a validation rule `exists:true` is implicit or added |
| T1.8 | `Directory` | `(path to an existing directory)` | Success | Test setup must create the directory |
| T1.9 | `Directory` | `(path to a file)` | Error (`UNILANG_VALIDATION_RULE_FAILED`) | `Directory` kind requires a directory |
| T1.10 | `Enum("A"|"B"|"C")` | `"A"` | Success, parsed as "A" | |
| T1.11 | `Enum("A"|"B"|"C")` | `"C"` | Success, parsed as "C" | |
| T1.12 | `Enum("A"|"B"|"C")` | `"D"` | Error (`UNILANG_TYPE_MISMATCH`) | "D" is not a valid choice |
| T1.13 | `Enum("A"|"B"|"C")` | `"a"` | Error (`UNILANG_TYPE_MISMATCH`) | Enum choices are case-sensitive |
| T1.14 | `URL` | `"https://example.com/path?q=1"` | Success, parsed as a URL object | |
| T1.15 | `URL` | `"ftp://user:pass@host:21"` | Success, parsed as a URL object | |
| T1.16 | `URL` | `"not a url"` | Error (`UNILANG_TYPE_MISMATCH`) | Invalid URL format |
| T1.17 | `URL` | `"/just/a/path"` | Error (`UNILANG_TYPE_MISMATCH`) | Relative paths are not valid URLs |
| T1.18 | `DateTime` | `"2025-06-28T12:00:00Z"` | Success, parsed as a DateTime object | ISO 8601 UTC |
| T1.19 | `DateTime` | `"2025-06-28T14:00:00+02:00"` | Success, parsed as a DateTime object | ISO 8601 with offset |
| T1.20 | `DateTime` | `"2025-06-28"` | Error (`UNILANG_TYPE_MISMATCH`) | Incomplete format |
| T1.21 | `DateTime` | `"invalid-date"` | Error (`UNILANG_TYPE_MISMATCH`) | |
| T1.22 | `Pattern` | `"^[a-z]+$"` | Success, parsed as a valid regex pattern | |
| T1.23 | `Pattern` | `"[a-z"` | Error (`UNILANG_TYPE_MISMATCH`) | Invalid regex syntax (unterminated character class) |
    *   **Commit Message:** `feat(unilang): Implement advanced scalar and path argument types`

*   ⚫ Increment 2: Implement Collection Argument Types (`List`, `Map`)
    *   **Goal:** Implement parsing and validation for `List<Type>` and `Map<KeyType, ValueType>` argument kinds, including support for custom delimiters.
*   ⚫ Increment 3: Implement Complex Argument Types and Attributes (`JsonString`, `multiple`, `validation_rules`)
    *   **Goal:** Implement the `JsonString`/`Object` kind, the `multiple: true` attribute, and a framework for basic `validation_rules`.
*   ⚫ Increment 4: Implement Runtime Command Registration API
    *   **Goal:** Expose a thread-safe public API (`command_add_runtime`, `command_remove_runtime`) to allow an integrator to add and remove `CommandDefinition`s at runtime.
*   ⚫ Increment 5: Implement Loading Command Definitions from External Files
    *   **Goal:** Provide parsers for YAML/JSON `CommandDefinition` files and a mechanism to resolve `routine_link` attributes to function pointers.
*   ⚫ Increment 6: Enhance CLI with Advanced Output Formatting and Error Handling
    *   **Goal:** Implement `output_format` global argument support for JSON and YAML, and the `on_error::continue` policy.
*   ⚫ Increment 7: Implement Shell Completion and Interactive Prompting Hooks
    *   **Goal:** Implement the logic for a `.system.completion.generate` command and the framework hooks to support interactive argument prompting.
*   ⚫ Increment 8: Enhance `ExecutionContext`
    *   **Goal:** Standardize the fields and access methods within `ExecutionContext` for effective global argument values and a logger instance.

### Changelog
*   **feat(unilang): Implement advanced scalar and path argument types**
    *   Implemented `Path`, `File`, `Directory`, `Enum`, `URL`, `DateTime`, and `Pattern` argument kinds in `src/types.rs`.
    *   Updated `src/data.rs` to use the new `Kind` enum.
    *   Integrated new type parsing logic into `src/semantic.rs`.
    *   Added `url`, `chrono`, and `regex` dependencies to `Cargo.toml`.
    *   Created `tests/inc/phase2/argument_types_test.rs` with comprehensive tests for new types.
    *   Fixed compilation errors and clippy warnings across `src/lib.rs`, `src/parsing.rs`, `src/semantic.rs`, `src/registry.rs`, `src/interpreter.rs`, `src/help.rs`, `src/ca/mod.rs`, `src/ca/parsing/input.rs`, and `src/ca/parsing/engine.rs`.

### Task Requirements
*   The plan must be broken down into small, verifiable increments.
*   Each feature implementation must include a detailed testing strategy, documented via a Test Matrix in the corresponding increment.
*   All code must adhere to the project's existing style and quality standards.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be thoroughly documented.
*   The project must remain compilable and pass all existing tests at the end of each increment.

### Assumptions
*   Phase 1 of the roadmap is fully implemented and stable.
*   The existing test framework is sufficient for the new tests.
*   The `spec.md` is the single source of truth for feature behavior.

### Out of Scope
*   Implementation of Phases 3, 4, and 5 from the roadmap.
*   Implementation of the actual TUI, GUI, or AUI modalities (this plan only covers the framework hooks).
*   Adding new commands beyond what is necessary for testing the framework's new features.

### External System Dependencies (Optional)
*   None.

### Notes & Insights
*   Phase 2 is substantial. Breaking it down into these increments allows for focused development and testing, reducing the risk of regressions and ensuring each new part of the type system and runtime is solid before building upon it.
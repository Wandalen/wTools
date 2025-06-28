# Task Plan: Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity

### Goal
*   Implement advanced type handling for arguments (scalar, path-like, collections, complex types), a robust runtime command registration API, and the ability to load command definitions from external files. This phase aims to significantly enhance the flexibility and extensibility of the `unilang` module, moving towards a more mature and capable CLI.

### Ubiquitous Language (Vocabulary)
*   **Kind:** The type of an argument (e.g., `String`, `Integer`, `Path`, `List(String)`).
*   **Value:** A parsed and validated instance of a `Kind` (e.g., `Value::String("hello")`, `Value::Integer(123)`).
*   **CommandDefinition:** Metadata describing a command, including its name, description, and arguments.
*   **ArgumentDefinition:** Metadata describing a single argument, including its name, kind, optionality, multiplicity, and validation rules.
*   **CommandRegistry:** A central repository for `CommandDefinition`s and their associated `CommandRoutine`s.
*   **CommandRoutine:** A function pointer or closure that represents the executable logic of a command.
*   **Lexer:** The component responsible for breaking raw input strings into a sequence of `Token`s.
*   **Parser:** The component responsible for taking `Token`s from the `Lexer` and building an Abstract Syntax Tree (AST) in the form of a `Program`.
*   **SemanticAnalyzer:** The component responsible for validating the AST against the `CommandRegistry`, binding arguments, and applying validation rules, producing `VerifiedCommand`s.
*   **Interpreter:** The component responsible for executing `VerifiedCommand`s by invoking their associated `CommandRoutine`s.
*   **Program:** The Abstract Syntax Tree (AST) representing the parsed command line input.
*   **Statement:** A single command invocation within a `Program`, consisting of a command identifier and its raw arguments.
*   **VerifiedCommand:** A command that has passed semantic analysis, with its arguments parsed and validated into `Value`s.
*   **ErrorData:** A structured error type containing a code and a message.
*   **TypeError:** A specific error type for issues during type parsing or validation.
*   **Validation Rule:** A string-based rule applied to arguments (e.g., `min:X`, `max:X`, `regex:PATTERN`, `min_length:X`).
*   **Multiple Argument:** An argument that can accept multiple values, which are collected into a `Value::List`.
*   **JsonString:** A `Kind` that expects a string containing valid JSON, stored as a `Value::JsonString`.
*   **Object:** A `Kind` that expects a string containing a valid JSON object, parsed and stored as a `Value::Object(serde_json::Value)`.

### Progress
*   🚀 Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity - In Progress
*   Key Milestones Achieved:
    *   ✅ Increment 1: Implement Advanced Scalar and Path-like Argument Types.
    *   ✅ Increment 2: Implement Collection Argument Types (`List`, `Map`).
    *   ✅ Increment 3: Implement Complex Argument Types and Attributes (`JsonString`, `multiple`, `validation_rules`).
    *   ✅ Increment 4: Implement Runtime Command Registration API.
    *   ✅ Increment 5: Implement Loading Command Definitions from External Files.

### Target Crate/Library
*   `module/move/unilang`

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/move/unilang/src/lib.rs`
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/types.rs`
    *   `module/move/unilang/src/parsing.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/src/registry.rs`
    *   `module/move/unilang/src/error.rs`
    *   `module/move/unilang/src/interpreter.rs`
    *   `module/move/unilang/Cargo.toml`
    *   `module/move/unilang/tests/inc/phase2/argument_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/collection_types_test.rs`
    *   `module/move/unilang/tests/inc/phase2/complex_types_and_attributes_test.rs`
    *   `module/move/unilang/tests/inc/phase2/runtime_command_registration_test.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `unilang`
    *   `url`
    *   `chrono`
    *   `regex`
    *   `serde_json`
    *   `serde`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None

### Expected Behavior Rules / Specifications (for Target Crate)
*   **Argument Type Parsing:**
    *   Scalar types (String, Integer, Float, Boolean) should parse correctly from their string representations.
    *   Path-like types (Path, File, Directory) should parse into `PathBuf` and validate existence/type if specified.
    *   Enum types should validate against a predefined list of choices.
    *   URL, DateTime, and Pattern types should parse and validate according to their respective library rules.
    *   List types should parse comma-separated (or custom-delimited) strings into `Vec<Value>` of the specified item kind. Empty input string for a list should result in an empty list.
    *   Map types should parse comma-separated (or custom-delimited) key-value pairs into `HashMap<String, Value>` with specified key/value kinds. Empty input string for a map should result in an empty map.
    *   JsonString should validate that the input string is valid JSON, but store it as a raw string.
    *   Object should parse the input string into `serde_json::Value`.
*   **Argument Attributes:**
    *   `multiple: true` should collect all subsequent positional arguments into a `Value::List`.
    *   `validation_rules` (`min:X`, `max:X`, `regex:PATTERN`, `min_length:X`) should be applied after type parsing, and trigger an `Error::Execution` with code `VALIDATION_RULE_FAILED` if violated.
*   **Runtime Command Registration:**
    *   Commands can be registered with associated routine (function pointer/closure).
    *   Attempting to register a command with an already existing name should result in an error.
    *   The `Interpreter` should be able to retrieve and execute registered routines.

### Crate Conformance Check Procedure
*   Step 1: Run `timeout 90 cargo test -p unilang --all-targets` and verify no failures.
*   Step 2: Run `timeout 90 cargo clippy -p unilang -- -D warnings` and verify no errors or warnings.

### Increments
*   ✅ Increment 1: Implement Advanced Scalar and Path-like Argument Types.
    *   **Goal:** Introduce `Path`, `File`, `Directory`, `Enum`, `URL`, `DateTime`, and `Pattern` as new `Kind` variants and implement their parsing into `Value` variants.
    *   **Steps:**
        *   Step 1: Modify `src/data.rs` to extend the `Kind` enum with `Path`, `File`, `Directory`, `Enum(Vec<String>)`, `Url`, `DateTime`, and `Pattern`.
        *   Step 2: Modify `src/types.rs` to extend the `Value` enum with corresponding variants (`Path(PathBuf)`, `File(PathBuf)`, `Directory(PathBuf)`, `Enum(String)`, `Url(Url)`, `DateTime(DateTime<FixedOffset>)`, `Pattern(Regex)`).
        *   Step 3: Add `url`, `chrono`, and `regex` as dependencies in `module/move/unilang/Cargo.toml`.
        *   Step 4: Implement `parse_value` function in `src/types.rs` to handle parsing for these new `Kind`s into their respective `Value`s, including basic validation (e.g., for `File` and `Directory` existence/type). Refactor `parse_value` into smaller helper functions (`parse_primitive_value`, `parse_path_value`, `parse_url_datetime_pattern_value`) for clarity.
        *   Step 5: Update `impl PartialEq for Value` and `impl fmt::Display for Value` in `src/types.rs` to include the new variants.
        *   Step 6: Modify `src/semantic.rs` to update `VerifiedCommand` to store `types::Value` instead of `String` for arguments. Adjust `bind_arguments` to use `types::parse_value`.
        *   Step 7: Create `tests/inc/phase2/argument_types_test.rs` with a detailed test matrix covering successful parsing and expected errors for each new type.
        *   Step 8: Perform Increment Verification.
        *   Step 9: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test argument_types_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement advanced scalar and path-like argument types`

*   ✅ Increment 2: Implement Collection Argument Types (`List`, `Map`).
    *   **Goal:** Extend `Kind` and `Value` to support `List` and `Map` types, including nested types and custom delimiters, and implement their parsing logic.
    *   **Steps:**
        *   Step 1: Modify `src/data.rs` to extend `Kind` enum with `List(Box<Kind>, Option<char>)` and `Map(Box<Kind>, Box<Kind>, Option<char>, Option<char>)` variants.
        *   Step 2: Modify `src/types.rs` to extend `Value` enum with `List(Vec<Value>)` and `Map(std::collections::HashMap<String, Value>)` variants. Add `use std::collections::HashMap;`.
        *   Step 3: Implement `parse_list_value` and `parse_map_value` helper functions in `src/types.rs` to handle parsing for `Kind::List` and `Kind::Map`, including delimiter handling and recursive parsing of inner types. Ensure empty input strings result in empty collections.
        *   Step 4: Integrate `parse_list_value` and `parse_map_value` into the main `parse_value` function in `src/types.rs`.
        *   Step 5: Update `impl PartialEq for Value` and `impl fmt::Display for Value` in `src/types.rs` to include the new collection variants.
        *   Step 6: Create `tests/inc/phase2/collection_types_test.rs` with a detailed test matrix covering successful parsing and expected errors for `List` and `Map` types, including nested types and custom delimiters.
        *   Step 7: Perform Increment Verification.
        *   Step 8: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test collection_types_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement collection argument types (List, Map)`

*   ✅ Increment 3: Implement Complex Argument Types and Attributes (`JsonString`, `multiple`, `validation_rules`).
    *   **Goal:** Introduce `JsonString` and `Object` types, and implement `multiple` and `validation_rules` attributes for `ArgumentDefinition`.
    *   **Steps:**
        *   Step 1: Modify `src/data.rs` to extend `Kind` enum with `JsonString` and `Object` variants. Add `multiple: bool` and `validation_rules: Vec<String>` fields to `ArgumentDefinition`.
        *   Step 2: Add `serde_json` as a dependency in `module/move/unilang/Cargo.toml`.
        *   Step 3: Modify `src/types.rs` to extend `Value` enum with `JsonString(String)` and `Object(serde_json::Value)` variants. Add `use serde_json;`. Implement `parse_json_value` helper function and integrate it into `parse_value`. Update `PartialEq` and `Display` for `Value`.
        *   Step 4: Modify `src/semantic.rs`:
            *   Update `bind_arguments` to handle the `multiple` attribute: if `multiple` is true, collect all subsequent raw arguments into a `Value::List`.
            *   Implement `apply_validation_rule` function to apply rules like `min:X`, `max:X`, `regex:PATTERN`, `min_length:X` to `Value`s.
            *   Integrate `apply_validation_rule` into `bind_arguments` to apply rules after parsing.
            *   Add `use regex::Regex;` to `src/semantic.rs`.
        *   Step 5: Create `tests/inc/phase2/complex_types_and_attributes_test.rs` with a detailed test matrix covering `JsonString`, `Object`, `multiple` arguments, and various `validation_rules`.
        *   Step 6: Perform Increment Verification.
        *   Step 7: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test complex_types_and_attributes_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement complex argument types and attributes`

*   ✅ Increment 4: Implement Runtime Command Registration API.
    *   **Goal:** Provide a mechanism to register and retrieve executable routines (function pointers/closures) for commands at runtime.
    *   **Steps:**
        *   Step 1: Define `CommandRoutine` type alias (`Box<dyn Fn(...) + Send + Sync + 'static>`) in `src/registry.rs`.
        *   Step 2: Modify `src/registry.rs` to add a `routines: HashMap<String, CommandRoutine>` field to `CommandRegistry`.
        *   Step 3: Implement `command_add_runtime` method in `CommandRegistry` to register a command definition along with its routine. Handle duplicate registration errors.
        *   Step 4: Implement `get_routine` method in `CommandRegistry` to retrieve a `CommandRoutine` by command name.
        *   Step 5: Extend the `Error` enum in `src/error.rs` with a `Registration(String)` variant for registration-related errors.
        *   Step 6: Modify `src/interpreter.rs`:
            *   Update `Interpreter::new` to take a `&CommandRegistry` instead of `&HashMap<String, CommandDefinition>`.
            *   Update the `run` method to retrieve and execute the `CommandRoutine` from the `CommandRegistry` for each `VerifiedCommand`.
            *   Add `Clone` derive to `ExecutionContext`.
            *   Remove `Debug` derive from `Interpreter` and `CommandRegistry` (and `CommandRegistryBuilder`, `SemanticAnalyzer`) as `CommandRoutine` does not implement `Debug`. Add `#[allow(missing_debug_implementations)]` to these structs.
            *   Remove unused import `crate::registry::CommandRoutine` from `src/interpreter.rs`.
        *   Step 7: Update `tests/inc/phase1/full_pipeline_test.rs` to align with the new `Interpreter::new` signature and `ArgumentDefinition` fields. Add dummy routines for interpreter tests.
        *   Step 8: Create `tests/inc/phase2/runtime_command_registration_test.rs` with a detailed test matrix covering successful registration, duplicate registration errors, and execution of registered commands with arguments.
        *   Step 9: Perform Increment Verification.
        *   Step 10: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test runtime_command_registration_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement runtime command registration API`

*   ✅ Increment 5: Implement Loading Command Definitions from External Files
    *   **Goal:** Provide parsers for YAML/JSON `CommandDefinition` files and a mechanism to resolve `routine_link` attributes to function pointers.
    *   **Steps:**
        *   Step 1: Add `serde`, `serde_yaml`, and `serde_json` as dependencies in `module/move/unilang/Cargo.toml` with `derive` feature for `serde`.
        *   Step 2: Modify `src/data.rs`:
            *   Add `#[derive(Serialize, Deserialize)]` to `CommandDefinition` and `ArgumentDefinition`.
            *   Add `routine_link: Option<String>` field to `CommandDefinition` to specify a path to a routine.
            *   Implement `FromStr` for `Kind` to allow parsing `Kind` from string in YAML/JSON.
        *   Step 3: Create a new module `src/loader.rs` to handle loading command definitions.
        *   Step 4: In `src/loader.rs`, implement `load_command_definitions_from_yaml_str(yaml_str: &str) -> Result<Vec<CommandDefinition>, Error>` and `load_command_definitions_from_json_str(json_str: &str) -> Result<Vec<CommandDefinition>, Error>` functions.
        *   Step 5: In `src/loader.rs`, implement `resolve_routine_link(link: &str) -> Result<CommandRoutine, Error>` function. This will be a placeholder for now, returning a dummy routine or an error if the link is not recognized. The actual resolution mechanism will be implemented in a later increment.
        *   Step 6: Modify `CommandRegistryBuilder` in `src/registry.rs` to add methods like `load_from_yaml_str` and `load_from_json_str` that use the `loader` module to parse definitions and register them.
        *   Step 7: Create `tests/inc/phase2/command_loader_test.rs` with a detailed test matrix covering:
            *   Successful loading of command definitions from valid YAML/JSON strings.
            *   Error handling for invalid YAML/JSON.
            *   Basic testing of `routine_link` resolution (e.g., ensuring it doesn't panic, or returns a placeholder error).
        *   Step 8: Perform Increment Verification.
        *   Step 9: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test command_loader_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement loading command definitions from external files`

*   ⚫ Increment 6: Implement CLI Argument Parsing and Execution.
    *   **Goal:** Integrate the `unilang` core into a basic CLI application, allowing users to execute commands defined in the registry via command-line arguments.
    *   **Steps:**
        *   Step 1: Create a new binary target `src/bin/unilang_cli.rs` in `module/move/unilang/Cargo.toml`.
        *   Step 2: In `src/bin/unilang_cli.rs`, implement a basic `main` function that:
            *   Initializes a `CommandRegistry`.
            *   Registers a few sample commands (using both hardcoded definitions and potentially loading from a dummy file if Increment 5 is complete).
            *   Parses command-line arguments (e.g., using `std::env::args`).
            *   Uses `Lexer`, `Parser`, `SemanticAnalyzer`, and `Interpreter` to process and execute the command.
            *   Handles and prints errors gracefully.
        *   Step 3: Create `tests/inc/phase2/cli_integration_test.rs` with integration tests that invoke the `unilang_cli` binary with various arguments and assert on its output (stdout/stderr) and exit code.
        *   Step 4: Perform Increment Verification.
        *   Step 5: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test cli_integration_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement basic CLI argument parsing and execution`

*   ⚫ Increment 7: Implement Advanced Routine Resolution and Dynamic Loading.
    *   **Goal:** Enhance `routine_link` resolution to support dynamic loading of routines from specified paths (e.g., shared libraries or Rust modules).
    *   **Steps:**
        *   Step 1: Research and select a suitable Rust crate for dynamic library loading (e.g., `libloading` or `dlopen`). Add it as a dependency.
        *   Step 2: Refine `resolve_routine_link` in `src/loader.rs` to:
            *   Parse `routine_link` strings (e.g., `path/to/lib.so::function_name` or `module::path::function_name`).
            *   Dynamically load shared libraries or resolve Rust functions based on the link.
            *   Return a `CommandRoutine` (a `Box<dyn Fn(...)>`) that wraps the dynamically loaded function.
        *   Step 3: Update `CommandRegistryBuilder` to use the enhanced `resolve_routine_link`.
        *   Step 4: Create `tests/inc/phase2/dynamic_routine_loading_test.rs` with tests for:
            *   Successful dynamic loading and execution of routines from dummy shared libraries.
            *   Error handling for invalid paths, missing functions, or incorrect signatures.
        *   Step 5: Perform Increment Verification.
        *   Step 6: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test dynamic_routine_loading_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement advanced routine resolution and dynamic loading`

*   ⚫ Increment 8: Implement Command Help Generation and Discovery.
    *   **Goal:** Develop a comprehensive help system that can generate detailed documentation for commands, including their arguments, types, and validation rules.
    *   **Steps:**
        *   Step 1: Enhance `HelpGenerator` in `src/help.rs` to:
            *   Access `CommandDefinition`s from the `CommandRegistry`.
            *   Generate detailed help messages for individual commands, including argument names, descriptions, kinds, optionality, multiplicity, and validation rules.
            *   Generate a summary list of all available commands.
        *   Step 2: Integrate the enhanced `HelpGenerator` into the `unilang_cli` binary (from Increment 6) to provide `--help` or `help <command>` functionality.
        *   Step 3: Create `tests/inc/phase2/help_generation_test.rs` with tests that:
            *   Invoke the `unilang_cli` with help flags/commands.
            *   Assert on the content and format of the generated help output.
        *   Step 4: Perform Increment Verification.
        *   Step 5: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Execute `timeout 90 cargo test -p unilang --test help_generation_test` and verify no failures.
    *   **Commit Message:** `feat(unilang): Implement command help generation and discovery`

### Changelog
*   **2025-06-28 - Increment 5: Implement Loading Command Definitions from External Files**
    *   **Description:** Implemented parsers for YAML/JSON `CommandDefinition` files and a placeholder mechanism to resolve `routine_link` attributes to function pointers. Added `thiserror` as a dependency. Modified `src/data.rs` to add `#[serde(try_from = "String", into = "String")]` to `Kind` and implemented `From<Kind> for String` and `TryFrom<String> for Kind`. Implemented `Display` for `ErrorData`. Modified `src/loader.rs` to implement `load_command_definitions_from_yaml_str`, `load_command_definitions_from_json_str`, and `resolve_routine_link` (placeholder). Updated `CommandRegistryBuilder` in `src/registry.rs` with `load_from_yaml_str` and `load_from_json_str` methods. Created `tests/inc/phase2/command_loader_test.rs` with a detailed test matrix. Addressed Clippy lints: `single-char-pattern`, `uninlined-format-args`, `std-instead-of-core`, `missing-errors-doc`, `manual-string-new`, and `needless-pass-by-value`.
    *   **Verification:** All tests passed, including `command_loader_test.rs`, and `cargo clippy -p unilang -- -D warnings` passed.
*   **2025-06-28 - Increment 4: Implement Runtime Command Registration API**
    *   **Description:** Implemented the core functionality for registering and retrieving executable command routines at runtime. This involved defining `CommandRoutine` as a `Box<dyn Fn(...)>`, adding a `routines` map to `CommandRegistry`, and implementing `command_add_runtime` and `get_routine` methods. The `Interpreter` was updated to use this registry for command execution. `Clone` was added to `ExecutionContext`. `Debug` derive was removed from `CommandRegistry`, `CommandRegistryBuilder`, `SemanticAnalyzer`, and `Interpreter` due to `CommandRoutine` not implementing `Debug`, and `#[allow(missing_debug_implementations)]` was added. An unused import in `src/interpreter.rs` was removed.
    *   **Verification:** All tests passed, including `runtime_command_registration_test.rs`.
*   **2025-06-28 - Increment 3: Implement Complex Argument Types and Attributes (`JsonString`, `multiple`, `validation_rules`)**
    *   **Description:** Introduced `JsonString` and `Object` kinds, along with `multiple` and `validation_rules` attributes for `ArgumentDefinition`. `serde_json` was added as a dependency. Parsing logic for `JsonString` and `Object` was implemented in `src/types.rs`. The `semantic` analyzer was updated to handle `multiple` arguments (collecting them into a `Value::List`) and to apply `validation_rules` (`min:X`, `max:X`, `regex:PATTERN`, `min_length:X`). Fixed an issue where validation rules were not applied to individual elements of a `Value::List` when `multiple: true`. Corrected test inputs for `JsonString` and `Object` in `complex_types_and_attributes_test.rs` to ensure proper lexing of quoted JSON strings.
    *   **Verification:** All tests passed, including `complex_types_and_attributes_test.rs`.
*   **2025-06-28 - Increment 2: Implement Collection Argument Types (`List`, `Map`)**
    *   **Description:** Extended `Kind` and `Value` enums to support `List` and `Map` types, including nested types and custom delimiters. Implemented parsing logic for these collection types in `src/types.rs`, ensuring empty input strings correctly result in empty collections.
    *   **Verification:** All tests passed, including `collection_types_test.rs`.
*   **2025-06-28 - Increment 1: Implement Advanced Scalar and Path-like Argument Types**
    *   **Description:** Introduced `Path`, `File`, `Directory`, `Enum`, `URL`, `DateTime`, and `Pattern` as new argument `Kind`s and their corresponding `Value` representations. Integrated `url`, `chrono`, and `regex` dependencies. Implemented parsing and basic validation for these types in `src/types.rs`, refactoring `parse_value` into smaller helper functions. Updated `semantic` analysis to use the new `Value` types.
    *   **Verification:** All tests passed, including `argument_types_test.rs`.

### Task Requirements
*   All new code must adhere to Rust 2021 edition.
*   All new APIs must be async where appropriate (though current task is mostly sync parsing/semantic analysis).
*   Error handling should use the centralized `Error` enum.
*   All new public items must have documentation comments.
*   All tests must be placed in the `tests` directory.
*   New features should be covered by comprehensive test matrices.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async.
*   All code must pass `cargo clippy -- -D warnings`.
*   All code must pass `cargo test --workspace`.
*   Code should be modular and extensible.
*   Prefer `mod_interface!` for module structuring.
*   Centralize dependencies in workspace `Cargo.toml`.
*   Prefer workspace lints over entry file lints.

### Assumptions
*   The `unilang` module is part of a larger workspace.
*   The `CommandRoutine` type will eventually be compatible with dynamically loaded functions or closures.
*   The `routine_link` string format will be defined and consistently used for dynamic loading.

### Out of Scope
*   Full implementation of a CLI application (only basic integration in Increment 6).
*   Advanced error recovery during parsing (focus on reporting errors).
*   Complex type inference (types are explicitly defined by `Kind`).
*   Full security validation for dynamically loaded routines (basic error handling only).

### External System Dependencies (Optional)
*   None directly for the core `unilang` module, but `url`, `chrono`, `regex`, `serde_json`, `serde`, `serde_yaml` are used for specific argument kinds and file loading.

### Notes & Insights
*   The `Lexer`'s handling of quoted strings is crucial for `JsonString` and `Object` types.
*   The `multiple` attribute effectively transforms a single argument definition into a list of values.
*   Validation rules provide a powerful mechanism for enforcing constraints on argument values.
*   The `CommandRoutine` type alias and runtime registration are key for extensibility.
# Unilang Crate - Testing Plan

This document details the development and testing strategy for features within the Unilang Crate, starting with Phase 1.

**Legend for Test Status (within this document, if used for tracking):**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   ❌ : Blocked / Needs Revisit

---

## Phase 1: Core `unilang` Language Engine & CLI Foundations

### 1. Foundational Setup

#### Feature 1.1: Establish Testing Strategy & Framework
*   **Description:** Define the overall testing approach, select testing libraries/frameworks, and set up the basic infrastructure for unit and integration tests within the `unilang` crate.
*   **Key Testing Factors:**
    *   Ability to write and run unit tests for individual modules/functions.
    *   Ability to write and run integration tests that use the crate's public API.
    *   Setup of a Continuous Integration (CI) pipeline to automatically run tests.
    *   Decision on code coverage metrics and tools.
    *   Basic test harness utility design for eventual E2E-like testing of the crate's core loop.
*   **Test Relevance/Acceptance Criteria:**
    *   Unit tests can be successfully executed for a sample module.
    *   An integration test can successfully call a public API of the crate.
    *   CI pipeline runs tests on commits/pull requests.
    *   Code coverage reporting is functional (even if initial coverage is low).
*   **Key Code Modules/Areas to Cover:**
    *   `Cargo.toml` (dev-dependencies for testing frameworks).
    *   CI configuration files (e.g., GitHub Actions workflow).
    *   Sample test files in `src/` (for unit tests) and `tests/` (for integration tests).

---

### 2. CLI Input Processing - Phase 1: Lexical and Syntactic Analysis (Spec 1.1.1)

#### Feature 2.1: Implement Lexer
*   **Description:** Tokenizes the raw `unilang` CLI input string into a sequence of fundamental symbols.
*   **Key Testing Factors:**
    *   **Token Recognition:**
        *   Correctly tokenizes identifiers (command/namespace segments, argument names).
        *   Correctly tokenizes `::` (KeyValueSeparator).
        *   Correctly tokenizes `;;` (CommandSeparator).
        *   Correctly tokenizes `?` (HelpOperator).
        *   Correctly tokenizes argument values:
            *   Unquoted values (simple strings, numbers).
            *   Single-quoted values (preserving internal spaces/symbols).
            *   Double-quoted values (preserving internal spaces/symbols).
            *   Values with escaped quotes within quoted strings.
    *   **Whitespace Handling:**
        *   Whitespace between tokens is correctly ignored.
        *   Whitespace within unquoted argument values (should typically delimit them or be part of a single token depending on rules).
        *   Leading/trailing whitespace in the input string.
    *   **Edge Cases & Errors:**
        *   Empty input string.
        *   Input string with only whitespace.
        *   Unrecognized characters/symbols (generates an error token or specific error).
        *   Unterminated quoted strings (generates an error).
*   **Test Relevance/Acceptance Criteria:**
    *   All specified token types are correctly identified and produced for valid inputs.
    *   Whitespace is handled according to defined rules.
    *   Specific and informative errors are generated for lexical errors.
    *   The lexer handles a comprehensive set of valid and invalid input snippets.
*   **Key Code Modules/Areas to Cover:**
    *   Lexer/Tokenizer module (`src/parser/lexer.rs` or similar).
    *   Token enum/struct definitions.
    *   Error types related to lexing.

#### Feature 2.2: Implement Parser
*   **Description:** Builds an Abstract Syntax Tree (AST) or a sequence of "Generic Instructions" from the token stream provided by the Lexer.
*   **Key Testing Factors:**
    *   **AST/Generic Instruction Structure:**
        *   Correct structure for a single command with no arguments.
        *   Correct structure for a command with only positional/default arguments.
        *   Correct structure for a command with only named arguments.
        *   Correct structure for a command with mixed positional and named arguments.
        *   Correct structure for a command with the help operator (`?`).
        *   Correct structure for a namespace help request (e.g., `.files. ?`).
        *   Correct structure for a root namespace help request (`. ?` or `.`).
    *   **Command Path Parsing:**
        *   Correctly parses dot-separated command `FullName`s (e.g., `.namespace.sub.command`).
        *   Handles root namespace commands (e.g., `.command`).
    *   **Argument Parsing (Syntactic):**
        *   Correctly associates `arg_name` with `arg_value` for named arguments.
        *   Correctly identifies sequence of `arg_value`s as potential positional arguments.
    *   **Command Sequence Parsing:**
        *   Correctly parses multiple command expressions separated by `;;` into a sequence of AST nodes/Generic Instructions.
    *   **Error Handling:**
        *   Unexpected token errors (e.g., `::` without a preceding argument name).
        *   Missing components (e.g., argument value after `::`).
        *   Misplaced `;;` or `?`.
    *   **Boundary Conditions:**
        *   Empty token stream (after lexing empty input).
        *   Very long sequences of commands.
*   **Test Relevance/Acceptance Criteria:**
    *   Valid token streams produce a correct and complete AST/Generic Instruction sequence.
    *   Syntactic errors in the token stream result in specific and actionable parse errors.
    *   All `unilang` grammar rules (as per Appendix A.2) are correctly implemented.
*   **Key Code Modules/Areas to Cover:**
    *   Parser module (`src/parser/parser.rs` or similar).
    *   AST node definitions or Generic Instruction struct definitions.
    *   Parser error types.
    *   Integration with the Lexer module.

#### Feature 2.3: Global Argument Identification & Extraction Logic
*   **Description:** Framework logic for integrators to define and extract their global arguments from the initial part of the CLI string, before command expression parsing.
*   **Key Testing Factors:**
    *   Correctly identifies and extracts `key::value` pairs as global arguments if they appear before the first command path.
    *   Stops consuming tokens as global arguments once a token that cannot be part of a global argument (e.g., a command path segment starting with `.`, or `?`, or `;;` if no command preceded) is encountered.
    *   Handles multiple global arguments.
    *   Handles cases with no global arguments (passes entire input to command parser).
    *   Provides a mechanism for the integrator to:
        *   Specify which keys are recognized as global arguments.
        *   Receive the extracted raw string key-value pairs.
    *   Behavior with malformed global arguments (e.g., `global_key_only::`).
    *   Behavior with unrecognized global argument keys (e.g., error if strict, or pass-through to command parsing if lenient – to be defined by `unilang`'s strictness here).
*   **Test Relevance/Acceptance Criteria:**
    *   Integrator-defined global arguments are correctly identified and their raw string values are made available.
    *   The remaining token stream (for command expressions) is correctly passed to the main parser.
    *   Errors are handled appropriately for malformed or (if strict) unrecognized global arguments.
*   **Key Code Modules/Areas to Cover:**
    *   The initial parsing stage that handles global arguments (could be part of the main parser or a pre-processing step).
    *   API/interface for integrators to define their global arguments.

---

### 3. Core Data Structures & Command Registry (Spec 0.2, 2, 2.4)

#### Feature 3.1: Define Core Data Structures
*   **Description:** Implementation of `CommandDefinition`, `ArgumentDefinition`, `Namespace`, `OutputData`, `ErrorData` Rust structs/enums.
*   **Key Testing Factors:**
    *   Correct instantiation with all mandatory and optional fields.
    *   Getters/setters (if applicable) or direct field access works as expected.
    *   Default values for fields (e.g., `ArgumentDefinition.optional` defaults to `false`) are correctly initialized.
    *   Enum variants for fields like `Status` are correctly defined and usable.
    *   `OutputData` and `ErrorData` can hold various payload/details types as specified.
*   **Test Relevance/Acceptance Criteria:**
    *   Instances of these data structures can be created and accurately represent the Unilang specification.
    *   All attributes can be correctly stored and retrieved.
    *   Compile-time type safety is ensured by the Rust type system.
*   **Key Code Modules/Areas to Cover:**
    *   Modules defining these core structs/enums (e.g., `src/core_types.rs`, `src/command.rs`).

#### Feature 3.2: Implement Unified Command Registry
*   **Description:** The central data structure for storing `CommandDefinition`s and logic for compile-time registration.
*   **Key Testing Factors:**
    *   **Basic Operations:**
        *   Successfully add a valid `CommandDefinition`.
        *   Retrieve a `CommandDefinition` by its exact `FullName`.
        *   Attempting to retrieve a non-existent command results in an appropriate error/None.
    *   **Duplicate Handling:**
        *   Behavior when adding a command with a `FullName` that already exists (e.g., returns error, or overwrites based on defined policy – spec says "error or overwrite based on policy").
    *   **Scalability (Conceptual):**
        *   Ensure the chosen data structure (e.g., HashMap) performs adequately with a small and a moderately large number of commands.
    *   **Compile-Time Registration Mechanisms:**
        *   Test the builder API provided by the `unilang` crate for defining commands programmatically (intended for integrator's compile-time setup or `Extension Module`s).
        *   If helper macros are provided (e.g., `#[define_command(...)]`), test their code generation and registration into the registry.
*   **Test Relevance/Acceptance Criteria:**
    *   Commands can be reliably added and retrieved from the registry.
    *   The defined policy for handling duplicate command names is correctly enforced.
    *   Compile-time registration mechanisms successfully populate the registry.
*   **Key Code Modules/Areas to Cover:**
    *   Command Registry module (`src/registry.rs` or similar).
    *   Any macros or builder pattern implementations for command definition.

#### Feature 3.3: Basic Namespace Handling Logic
*   **Description:** Logic within the Command Registry to support namespace resolution and listing.
*   **Key Testing Factors:**
    *   Resolving a `FullName` that includes namespaces (e.g., `.foo.bar.command`).
    *   Listing commands directly within a specific namespace (e.g., all commands in `.foo.bar` but not `.foo.bar.baz`).
    *   Listing immediate sub-namespaces within a given namespace.
    *   Handling requests for the root namespace (`.`).
    *   Behavior when querying a non-existent namespace.
    *   Correctly distinguishing between a command and a namespace if they share part of a path (e.g., `.foo` as a namespace vs. `.foo` as a command).
*   **Test Relevance/Acceptance Criteria:**
    *   Namespace hierarchy is correctly interpreted for command lookups.
    *   Listing commands and sub-namespaces by a given namespace path functions correctly.
    *   Appropriate responses (e.g., empty list, error) for non-existent namespaces.
*   **Key Code Modules/Areas to Cover:**
    *   Command Registry module, specifically methods related to namespace queries.

---

### 4. CLI Input Processing - Phase 2: Semantic Analysis & Command Binding (Spec 1.1.2)

#### Feature 4.1: Command Resolution Logic
*   **Description:** Resolving the raw command name string from a Generic Instruction to a specific `CommandDefinition` in the registry.
*   **Key Testing Factors:**
    *   Successfully resolves a valid, existing command `FullName`.
    *   Correctly handles commands in the root namespace vs. nested namespaces.
    *   Generates `UNILANG_COMMAND_NOT_FOUND` error (in `ErrorData`) if the command name does not exist in the registry.
    *   Case sensitivity of command names is enforced as per spec.
*   **Test Relevance/Acceptance Criteria:**
    *   Valid command names are mapped to their `CommandDefinition`.
    *   Non-existent command names produce the correct error.
*   **Key Code Modules/Areas to Cover:**
    *   Semantic Analyzer module (`src/analyzer.rs` or similar).
    *   Interaction with the Command Registry.

#### Feature 4.2: Argument Binding Logic
*   **Description:** Mapping raw argument values from a Generic Instruction to the `ArgumentDefinition`s of a resolved command.
*   **Key Testing Factors:**
    *   **Named Arguments:**
        *   Correctly binds `key::value` pairs to `ArgumentDefinition`s by name.
        *   Correctly binds using defined aliases for arguments.
        *   Handles unknown argument names (produces `UNILANG_ARGUMENT_INVALID` or a more specific "unknown argument" error).
    *   **Positional (Default) Arguments:**
        *   Correctly binds leading positional values to the argument marked `is_default_arg: true`.
        *   Handles cases where no positional value is provided for a default argument.
        *   Error if positional values are provided but no argument is `is_default_arg`.
    *   **Argument Order:**
        *   Correctly binds arguments regardless of their order on the CLI (for named args).
        *   Correctly handles positional args appearing before or interspersed with named args (if grammar allows).
    *   **Missing Mandatory Arguments:**
        *   Identifies and reports `UNILANG_ARGUMENT_MISSING` if a non-optional argument is not provided and has no default value.
    *   **Applying Default Values:**
        *   If an optional argument with a `default_value` is not provided, its `default_value` (as a string) is used for subsequent type parsing.
*   **Test Relevance/Acceptance Criteria:**
    *   All provided arguments are correctly bound to their definitions.
    *   Errors are generated for unknown arguments or missing mandatory arguments.
    *   Default values are correctly applied.
*   **Key Code Modules/Areas to Cover:**
    *   Semantic Analyzer module.
    *   Interaction with `CommandDefinition` and `ArgumentDefinition` structures.

#### Feature 4.3: Basic Argument Type System (`kind`)
*   **Description:** Parsing and validation logic for `String`, `Integer`, `Float`, `Boolean` kinds, and support for core attributes `optional`, `default_value`, `is_default_arg`.
*   **Key Testing Factors:**
    *   **Type Parsing/Validation (for each basic type):**
        *   Valid string inputs are correctly parsed/coerced to the target Rust type (e.g., "123" to `i64`/`u64`, "true" to `bool`).
        *   Invalid string inputs result in `UNILANG_TYPE_MISMATCH` error.
        *   Handles various valid string representations (e.g., "TRUE", "1" for `Boolean`; "1.0", "-1.5e-2" for `Float`).
        *   Empty string input for each type (should generally be a type mismatch unless `String`).
    *   **Integration with `optional` attribute:** (Covered by 4.2, but re-verify type parsing isn't attempted if optional and not present).
    *   **Integration with `default_value` attribute:** Ensure the string `default_value` is correctly parsed using the argument's `kind`. Error if `default_value` is incompatible with `kind`.
    *   **Integration with `is_default_arg` attribute:** (Covered by 4.2, ensure type parsing applies to the bound default argument).
*   **Test Relevance/Acceptance Criteria:**
    *   Argument values are correctly parsed to their specified `kind` or appropriate `UNILANG_TYPE_MISMATCH` errors are generated.
    *   Core attributes interact correctly with the type system.
*   **Key Code Modules/Areas to Cover:**
    *   Type parsing/validation module/functions (`src/types.rs` or similar).
    *   Semantic Analyzer module where type parsing is invoked.

#### Feature 4.4: `VerifiedCommand` Object Generation
*   **Description:** Creating the `VerifiedCommand` object once a command is resolved and all its arguments are successfully bound, parsed, and validated.
*   **Key Testing Factors:**
    *   `VerifiedCommand` struct is correctly populated with:
        *   A reference to (or copy of) the resolved `CommandDefinition`.
        *   A collection (e.g., HashMap) mapping argument names (String) to their final, parsed, and typed Rust values (e.g., `Box<dyn Any>`, or specific enum variants if using an enum for typed values).
    *   Ensures all mandatory arguments are present in the final collection.
    *   Ensures default values are correctly represented.
*   **Test Relevance/Acceptance Criteria:**
    *   A syntactically and semantically valid command expression results in a correctly populated `VerifiedCommand` object.
    *   The types of values within `VerifiedCommand` match their `ArgumentDefinition` `kind`.
*   **Key Code Modules/Areas to Cover:**
    *   Semantic Analyzer module.
    *   `VerifiedCommand` struct definition.

#### Feature 4.5: Implement Standard `UNILANG_*` Error Code Usage
*   **Description:** Ensure `ErrorData` generated during parsing and semantic analysis uses the standard error codes defined in Spec 4.2.
*   **Key Testing Factors:**
    *   `UNILANG_COMMAND_NOT_FOUND` used for unresolved commands.
    *   `UNILANG_ARGUMENT_INVALID` (or more specific like "UnknownArgument") used for bad argument names.
    *   `UNILANG_ARGUMENT_MISSING` used for missing mandatory args.
    *   `UNILANG_TYPE_MISMATCH` used for values that can't be parsed to the argument's `kind`.
    *   `ErrorData` includes relevant `message` and `details` (e.g., `argument_name`).
*   **Test Relevance/Acceptance Criteria:**
    *   All parsing and semantic errors produce `ErrorData` with the correct standard `UNILANG_*` code and informative messages/details.
*   **Key Code Modules/Areas to Cover:**
    *   Lexer, Parser, Semantic Analyzer modules (where errors are generated).
    *   `ErrorData` struct and its construction.

---

### 5. Interpreter / Execution Engine - Core (Spec 5)

#### Feature 5.1: Define `ExecutionContext` Structure (basic version)
*   **Description:** Initial, basic definition of the `ExecutionContext` struct that will be passed to routines.
*   **Key Testing Factors:**
    *   Struct can be instantiated by the `unilang` framework.
    *   (Phase 1 content is minimal: perhaps a placeholder for future global args or logger).
*   **Test Relevance/Acceptance Criteria:**
    *   `ExecutionContext` struct is defined and can be passed to routines.
*   **Key Code Modules/Areas to Cover:**
    *   `ExecutionContext` struct definition (`src/execution.rs` or similar).

#### Feature 5.2: Implement Routine Invocation mechanism
*   **Description:** The core logic in the Interpreter to call the `Routine (Handler Function)` associated with a `VerifiedCommand`.
*   **Key Testing Factors:**
    *   Correctly retrieves the `Routine` (e.g., function pointer) from the `CommandDefinition` within `VerifiedCommand`.
    *   Successfully calls the `Routine` with the `VerifiedCommand` and `ExecutionContext` as arguments.
    *   Handles different routine signatures if a trait-based approach is used for routines.
*   **Test Relevance/Acceptance Criteria:**
    *   The Interpreter can dynamically call the correct, registered `Routine` for a command.
    *   Arguments are passed correctly.
*   **Key Code Modules/Areas to Cover:**
    *   Interpreter/Execution Engine module (`src/interpreter.rs` or similar).

#### Feature 5.3: Basic Handling of Routine Results (`OutputData`, `ErrorData`)
*   **Description:** The Interpreter captures the `Result<OutputData, ErrorData>` from a routine and prepares it for modality handling.
*   **Key Testing Factors:**
    *   Correctly captures `Ok(OutputData)`.
    *   Correctly captures `Err(ErrorData)`.
    *   The captured data is passed on (e.g., to a modality handler function or a result processing stage).
*   **Test Relevance/Acceptance Criteria:**
    *   The Interpreter correctly processes both success and error results from routines.
*   **Key Code Modules/Areas to Cover:**
    *   Interpreter/Execution Engine module.

#### Feature 5.4: Command Separator (`;;`) Processing (Interpreter Support)
*   **Description:** The Interpreter executes a sequence of `VerifiedCommand`s.
*   **Key Testing Factors:**
    *   Executes commands in the correct order as they appeared in the `;;` separated sequence.
    *   Default "stop on error": if a routine returns `ErrorData`, subsequent commands in the sequence are not executed.
    *   `ExecutionContext` is correctly passed to each command in the sequence (is it the same instance or re-created/updated?).
*   **Test Relevance/Acceptance Criteria:**
    *   Command sequences are executed correctly according to the "stop on error" policy.
*   **Key Code Modules/Areas to Cover:**
    *   Interpreter/Execution Engine module (main execution loop).

---

### 6. Basic Help Generation & Output (Spec 3.2.6, 4.2.1)

#### Feature 6.1: Logic to generate structured help data (JSON)
*   **Description:** Core logic to transform `CommandDefinition` and `ArgumentDefinition` metadata into a structured JSON format for help.
*   **Key Testing Factors:**
    *   Correct JSON structure produced for a command with no arguments.
    *   Correct JSON structure for a command with various argument types and attributes (name, kind, hint, optional, default_value, aliases).
    *   Includes command `FullName`, `hint`, `examples`, `status`, `version`, `deprecation_message` in the JSON.
    *   Correct JSON structure for namespace help (listing sub-commands/namespaces and their hints).
    *   The output adheres to the fields specified in Spec 3.2.6.
*   **Test Relevance/Acceptance Criteria:**
    *   Accurate and complete structured JSON help data is generated.
*   **Key Code Modules/Areas to Cover:**
    *   Help generation module (`src/help.rs` or similar).
    *   Serialization logic (e.g., using `serde_json`).

#### Feature 6.2: Framework support for `.system.help.globals ?`
*   **Description:** `unilang` crate provides a mechanism for integrators to register metadata about their global arguments, and for the help system to generate structured JSON help for them.
*   **Key Testing Factors:**
    *   Integrator can register global argument metadata (name, hint, type string, default value string).
    *   Invoking help for global arguments (e.g., via a specific system command or flag handled by `utility1` which then calls into `unilang` help logic) produces correct structured JSON.
*   **Test Relevance/Acceptance Criteria:**
    *   Structured help for integrator-defined global arguments can be generated.
*   **Key Code Modules/Areas to Cover:**
    *   Help generation module.
    *   API for registering global argument metadata.

#### Feature 6.3: Provide default text formatters for structured help, `OutputData`, and `ErrorData`
*   **Description:** Basic functions within the `unilang` crate that can take the structured JSON help, `OutputData`, and `ErrorData` and produce a human-readable plain text representation suitable for a simple CLI.
*   **Key Testing Factors:**
    *   Text output for command help is readable and includes all key information.
    *   Text output for `OutputData.payload` (if simple string/number) is direct.
    *   Text output for `ErrorData` is user-friendly (message, code, relevant details).
    *   Handles various combinations of fields in the structured data.
*   **Test Relevance/Acceptance Criteria:**
    *   Default text formatters produce clear, human-readable output for basic CLI scenarios.
    *   Integrators can use these formatters as a starting point or choose to implement their own.
*   **Key Code Modules/Areas to Cover:**
    *   Formatting utilities module (`src/formatters.rs` or similar).

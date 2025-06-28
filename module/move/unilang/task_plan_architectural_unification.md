# Task Plan: Architectural Unification

### Roadmap Milestone
This task plan implements **M3.1: implement_parser_integration** from `roadmap.md`.

### Goal
*   To refactor the `unilang` crate by removing the legacy parser and fully integrating the `unilang_instruction_parser` crate. This will create a single, unified parsing pipeline, resolve architectural debt, and align the codebase with the formal specification.

### Target Crate
*   `module/move/unilang`

### Crate Conformance Check Procedure
*   Step 1: Run `timeout 90 cargo test -p unilang --all-targets` and verify no failures.
*   Step 2: Run `timeout 90 cargo clippy -p unilang -- -D warnings` and verify no errors or warnings.

### Increments

*   **✅ Increment 1: Remove Legacy Components**
    *   **Goal:** To purge the old parser (`unilang::parsing`) and the associated command aggregator (`unilang::ca`) modules from the codebase. This is a clean, atomic first step that creates a clear "point of no return" and forces all dependent components to be updated.
    *   **Specification Reference:** This action directly supports the architectural goal of a single, unified pipeline as described conceptually in `spec.md` (Section 2.2.1) and is the first implementation step of `roadmap.md` (Milestone M3.1).
    *   **Steps:**
        1.  Delete the legacy parser file: `git rm module/move/unilang/src/parsing.rs`.
        2.  Delete the legacy command aggregator module: `git rm -r module/move/unilang/src/ca/`.
        3.  Update the crate root in `module/move/unilang/src/lib.rs` to remove the module declarations: `pub mod parsing;` and `pub mod ca;`.
    *   **Increment Verification:**
        1.  Execute `cargo check -p unilang`.
        2.  **Expected Outcome:** The command **must fail** with compilation errors, specifically "unresolved import" or "module not found" errors. This confirms that the legacy dependencies have been successfully severed at the source level.
    *   **Commit Message:** `refactor(unilang): Remove legacy parser and command aggregator modules`

*   **✅ Increment 2: Refactor `SemanticAnalyzer` to Consume `GenericInstruction`**
    *   **Goal:** To update the `SemanticAnalyzer` to consume `Vec<unilang_instruction_parser::GenericInstruction>` instead of the legacy `Program` AST. This is the core of the refactoring, adapting the semantic logic to the new, correct parser output.
    *   **Specification Reference:** Implements the "Semantic Analysis" stage of the "Unified Processing Pipeline" defined in `spec.md` (Section 2.2.1).
    *   **Steps:**
        1.  **Update Imports:** In `module/move/unilang/src/semantic.rs`, replace `use crate::parsing::Program;` with `use unilang_instruction_parser::{GenericInstruction, Argument as ParserArgument};`.
        2.  **Refactor `SemanticAnalyzer::new`:** Change the constructor's signature from `new(program: &'a Program, ...)` to `new(instructions: &'a [GenericInstruction], ...)`. Update the struct definition to hold `&'a [GenericInstruction]`.
        3.  **Refactor `SemanticAnalyzer::analyze`:**
            *   Rewrite the main loop to iterate over `self.instructions`.
            *   Inside the loop, resolve the command name by joining the `instruction.command_path_slices` with `.` to form the `String` key for `CommandRegistry` lookup.
        4.  **Refactor `bind_arguments` function:**
            *   Change the function signature to `bind_arguments(instruction: &GenericInstruction, command_def: &CommandDefinition) -> Result<HashMap<String, Value>, Error>`.
            *   Implement the new binding logic:
                *   Iterate through the `command_def.arguments`.
                *   For each `arg_def`, first check `instruction.named_arguments` for a match by name or alias.
                *   If not found, check if `arg_def.is_default_arg` is `true` and if there are any available `instruction.positional_arguments`.
                *   If a value is found (either named or positional), use `unilang::types::parse_value` to convert the raw string into a strongly-typed `unilang::types::Value`.
                *   If no value is provided, check if `arg_def.optional` is `true` or if a `default_value` exists.
                *   If a mandatory argument is not found, return a `MISSING_ARGUMENT` error.
    *   **Increment Verification:**
        1.  Execute `cargo build -p unilang`.
        2.  **Expected Outcome:** The `unilang` library crate **must build successfully**. Tests and the CLI binary will still fail to compile, but this step ensures the library's internal logic is now consistent.
    *   **Commit Message:** `refactor(unilang): Adapt SemanticAnalyzer to consume GenericInstruction`

*   **✅ Increment 3: Refactor `unilang_cli` Binary**
    *   **Goal:** To update the main CLI binary to use the new, unified parsing pipeline, making it the first fully functional end-to-end component of the refactored system.
    *   **Specification Reference:** Fulfills the CLI modality's adherence to the `spec.md` (Section 2.2.1) "Unified Processing Pipeline".
    *   **Steps:**
        1.  **Update Imports:** In `src/bin/unilang_cli.rs`, remove `use unilang::parsing::Parser;` and add `use unilang_instruction_parser::{Parser, UnilangParserOptions};`.
        2.  **Instantiate New Parser:** Replace the old parser instantiation with `let parser = Parser::new(UnilangParserOptions::default());`.
        3.  **Update Parsing Logic:** The core change is to stop joining `env::args()` into a single string. Instead, pass the arguments as a slice directly to the new parser: `let instructions = parser.parse_slice(&args[1..])?;`.
        4.  **Update Analyzer Invocation:** Pass the `instructions` vector from the previous step to the `SemanticAnalyzer::new(...)` constructor.
        5.  **Adapt Help Logic:** Review and adapt the pre-parsing help logic (e.g., `if args.len() < 2` or `if command_name == "--help"`) to ensure it still functions correctly before the main parsing pipeline is invoked.
    *   **Increment Verification:**
        1.  Execute `cargo build --bin unilang_cli`. The build must succeed.
        2.  Execute the compiled binary with a simple command via `assert_cmd` or manually: `target/debug/unilang_cli add 5 3`. The command should execute and print the correct result. This provides a basic smoke test before fixing the entire test suite.
    *   **Commit Message:** `refactor(cli): Migrate unilang_cli to use the new parsing pipeline`

*   **⏳ Increment 4: Migrate Integration Tests**
    *   **Goal:** To update all integration tests to use the new parsing pipeline, ensuring the entire framework is correct, robust, and fully verified against its expected behavior.
    *   **Specification Reference:** Verifies the end-to-end conformance of the new pipeline (`spec.md` Section 2.2.1) and the correctness of argument binding (`spec.md` Section 2.3.3).
    *   **Steps:**
        1.  **Identify and Update All Test Files:** Systematically go through all files in `tests/inc/`, including `full_pipeline_test.rs`, `cli_integration_test.rs`, and all tests in `phase2/`.
        2.  **Replace Parser Instantiation:** In each test setup, replace `unilang::parsing::Parser` with `unilang_instruction_parser::Parser`.
        3.  **Adapt Test Input:** Change test inputs from single strings that are parsed into a `Program` to using `parser.parse_single_str(input)` or `parser.parse_slice(input)` to get a `Vec<GenericInstruction>`.
        4.  **Update `SemanticAnalyzer` Usage:** Pass the resulting `Vec<GenericInstruction>` to the `SemanticAnalyzer` in each test.
        5.  **Update Assertions:** This is the most critical part. Assertions must be updated to reflect the new `VerifiedCommand` structure.
            *   For command names, assert on `verified_command.definition.name`.
            *   For arguments, assert on the contents of the `verified_command.arguments` `HashMap`, checking for the correct `unilang::types::Value` variants.
        6.  **Verify Error Tests:** Ensure tests for error conditions (e.g., `COMMAND_NOT_FOUND`, `MISSING_ARGUMENT`) are updated to feed invalid input into the new parser and correctly assert on the `ErrorData` produced by the refactored `SemanticAnalyzer`.
    *   **Increment Verification:**
        1.  Execute `cargo test -p unilang --all-targets`. All tests **must pass**.
        2.  Execute `cargo clippy -p unilang -- -D warnings`. There **must be no warnings**.
    *   **Commit Message:** `fix(tests): Migrate all integration tests to the new parsing pipeline`

### Changelog
*   **Increment 1: Remove Legacy Components**
    *   Removed `module/move/unilang/src/parsing.rs` and `module/move/unilang/src/ca/`.
    *   Updated `module/move/unilang/src/lib.rs` to remove module declarations for `parsing` and `ca`.
*   **Increment 2: Refactor `SemanticAnalyzer` to Consume `GenericInstruction`**
    *   Updated `module/move/unilang/src/semantic.rs` to use `unilang_instruction_parser::GenericInstruction`.
    *   Refactored `SemanticAnalyzer::new` and `SemanticAnalyzer::analyze` to work with `GenericInstruction`.
    *   Refactored `bind_arguments` to correctly handle named and positional arguments from `GenericInstruction` and removed references to non-existent fields in `ArgumentDefinition`.
    *   Added `unilang_instruction_parser` as a dependency in `module/move/unilang/Cargo.toml`.
*   **Increment 3: Refactor `unilang_cli` Binary**
    *   Updated `src/bin/unilang_cli.rs` to use `unilang_instruction_parser::Parser` and `UnilangParserOptions`.
    *   Migrated parsing logic to use `parser.parse_single_str()` with joined arguments.
    *   Adapted `SemanticAnalyzer` invocation to use the new `instructions` vector.
    *   Verified successful build and smoke test execution.

### Task Requirements
*   None

### Project Requirements
*   None

### Assumptions
*   None

### Out of Scope
*   None

### External System Dependencies
*   None

### Notes & Insights
*   None
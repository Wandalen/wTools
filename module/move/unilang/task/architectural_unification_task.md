# Task Plan: Architectural Unification (Elaborated)

### Goal
*   To refactor the `unilang` crate by removing the legacy parser, fully integrating the `unilang_instruction_parser` crate, and updating the core data models to align with the formal specification. This task is the core of the `unilang` framework's current development phase.

### Task Relationships
*   **Prerequisite:** This task is **blocked by** and depends on the successful completion of:
    *   `unilang_instruction_parser/task/fix_command_parsing_task.md`: The parser must be fixed before it can be integrated.
*   **Unblocks:** Successful completion of this task will **unblock**:
    *   `unilang_meta/task/implement_command_macro_task.md`: The macro needs a stable, correctly implemented `unilang` core to target.

### Ubiquitous Language (Vocabulary)
*   **`SemanticAnalyzer`**: The core component of `unilang` that validates instructions.
*   **`GenericInstruction`**: The output of the `unilang_instruction_parser`, which will become the input for the `SemanticAnalyzer`.
*   **`CommandDefinition` / `ArgumentDefinition`**: The core data models in `src/data.rs`.
*   **Legacy Parser**: The old parsing logic located in `unilang/src/parsing.rs` and `unilang/src/ca/`, which will be deleted.

### Progress
*   **Roadmap Milestone:** M3.1 & M3.2
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 0/6 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Remove Legacy Components
    *   ⚫ Increment 2: Refactor Core Data Models
    *   ⚫ Increment 3: Adapt `SemanticAnalyzer` to New Parser & Data Models
    *   ⚫ Increment 4: Refactor `unilang_cli` Binary with Correct Parsing
    *   ⚫ Increment 5: Migrate Integration Tests Incrementally
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** true
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

---

### Dependency API Guides

This section provides the necessary API information for dependencies, as direct access to their source code is unavailable.

#### 1. `unilang_instruction_parser` API Guide

*   **Main Entry Point:** `unilang_instruction_parser::Parser`
    *   `Parser::new(UnilangParserOptions::default()) -> Self`: Creates a new parser with default settings.
    *   `parser.parse_single_str(&str) -> Result<Vec<GenericInstruction>, ParseError>`: Parses a single, complete command string. **This is the primary method to use for the CLI binary after joining arguments.**
    *   `parser.parse_slice(&[&str]) -> Result<Vec<GenericInstruction>, ParseError>`: Parses a slice of strings, treating each element as a separate instruction. **Do not use this for CLI arguments from the shell.**

*   **Output Data Structure:** `unilang_instruction_parser::GenericInstruction`
    ```rust
    // This is the primary input to the SemanticAnalyzer.
    pub struct GenericInstruction {
        // A vector of strings representing the command path.
        // e.g., for ".files.copy", this will be `vec!["files", "copy"]`.
        pub command_path_slices: Vec<String>,

        // A map of named arguments.
        // e.g., for "src::file.txt", the key is "src".
        pub named_arguments: HashMap<String, Argument>,

        // A vector of positional arguments in order of appearance.
        pub positional_arguments: Vec<Argument>,

        // True if a '?' was found after the command path.
        pub help_requested: bool,

        // The location of the instruction in the source string.
        pub overall_location: SourceLocation,
    }
    ```

*   **Argument Structure:** `unilang_instruction_parser::Argument`
    ```rust
    // Represents a single parsed argument.
    pub struct Argument {
        // The name of the argument (e.g., "src"). None for positional args.
        pub name: Option<String>,

        // The raw, unescaped string value of the argument.
        pub value: String,

        // Location information for names and values.
        pub name_location: Option<SourceLocation>,
        pub value_location: SourceLocation,
    }
    ```

#### 2. `former` Crate API Guide

*   **Usage:** The `unilang` data structures use `#[derive(former::Former)]`. This automatically generates a builder struct named `[StructName]Former`.
*   **Builder Pattern:**
    1.  Start the builder with `StructName::former()`.
    2.  Set fields using methods with the same name as the fields (e.g., `.name("...")`, `.description("...")`).
    3.  Finalize the builder and get the struct instance by calling `.form()`.
*   **Example:**
    ```rust
    // This is how you will need to update the code in unilang_cli.rs
    let echo_def = CommandDefinition::former()
      .name("echo")
      .namespace(".system") // Example of a new field
      .hint("Echoes a message.")
      .form();
    ```

#### 3. `thiserror` Crate API Guide

*   **Usage:** Used in `src/error.rs` to simplify error type implementation.
*   `#[derive(Error)]`: Implements the `std::error::Error` trait.
*   `#[error("...")]`: Defines the `Display` implementation for the error enum variant.
*   `#[from]`: Automatically implements `From<SourceError> for MyError`, allowing for easy error conversion with the `?` operator.

---

### Expected Behavior Rules / Specifications
*   The legacy parser must be completely removed.
*   `CommandDefinition` and `ArgumentDefinition` in `src/data.rs` must be updated to include all fields from the latest specification.
*   The `SemanticAnalyzer` must be refactored to accept `&[GenericInstruction]` and use the updated data models.
*   The `unilang_cli` binary must join its command-line arguments into a single string and use `parser.parse_single_str()`.
*   All existing tests must be migrated to the new parsing pipeline and must pass.

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang --all-targets` via `execute_command`.
*   Step 2: Analyze `execute_command` output. If it fails, initiate Critical Log Analysis.
*   Step 3: If tests pass, execute `timeout 90 cargo clippy -p unilang -- -D warnings` via `execute_command`.
*   Step 4: Analyze `execute_command` output. If it fails, initiate Linter Fix & Regression Check Procedure.

### Increments

##### Increment 1: Remove Legacy Components
*   **Goal:** To purge the old parser (`unilang::parsing`) and command aggregator (`unilang::ca`) modules. This is a clean first step that creates a clear "point of no return".
*   **Steps:**
    1.  Delete `module/move/unilang/src/parsing.rs` and `module/move/unilang/src/ca/`.
    2.  Update `module/move/unilang/src/lib.rs` to remove the `mod` declarations for `parsing` and `ca`.
*   **Increment Verification:**
    1.  Execute `cargo check -p unilang` via `execute_command`.
    2.  **Expected Outcome:** The command **must fail** with compilation errors, confirming the legacy dependencies have been severed.
*   **Commit Message:** "refactor(unilang): Remove legacy parser and command aggregator modules"

##### Increment 2: Refactor Core Data Models
*   **Goal:** Update the core `CommandDefinition` and `ArgumentDefinition` structs to match the full specification, and adapt the `HelpGenerator` to use the new fields.
*   **Steps:**
    1.  In `src/data.rs`, add the following fields to `CommandDefinition`: `namespace: String`, `hint: String`, `status: String`, `version: Option<String>`, `tags: Vec<String>`, `aliases: Vec<String>`, `permissions: Vec<String>`, `idempotent: bool`.
    2.  In `src/data.rs`, add the following fields to `ArgumentDefinition`: `hint: String`, `is_default_arg: bool`, `default_value: Option<String>`, `aliases: Vec<String>`, `tags: Vec<String>`, `interactive: bool`, `sensitive: bool`.
    3.  Update the `former` derives and any manual constructors for these structs.
    4.  In `src/help.rs`, update `HelpGenerator::command` to display information from the new fields (e.g., aliases, status).
*   **Increment Verification:**
    1.  Execute `cargo build -p unilang` via `execute_command`. The build must succeed.
*   **Commit Message:** "feat(unilang): Update core data models to align with spec v1.3"

##### Increment 3: Adapt `SemanticAnalyzer` to New Parser & Data Models
*   **Goal:** To update the `SemanticAnalyzer` to consume `Vec<GenericInstruction>` and operate on the newly refactored data models.
*   **Steps:**
    1.  Update `module/move/unilang/src/semantic.rs`: replace legacy imports with `use unilang_instruction_parser::{GenericInstruction, Argument as ParserArgument};`.
    2.  Refactor `SemanticAnalyzer::new` to take `instructions: &'a [GenericInstruction]`.
    3.  Refactor `SemanticAnalyzer::analyze` to loop over `self.instructions` and resolve command names from `instruction.command_path_slices`.
    4.  Refactor `bind_arguments` to work with `GenericInstruction` and the updated `ArgumentDefinition` struct, correctly handling new fields like `aliases` and `is_default_arg`.
*   **Increment Verification:**
    1.  Execute `cargo build -p unilang` via `execute_command`. The library must build successfully.
*   **Commit Message:** "refactor(unilang): Adapt SemanticAnalyzer to new parser and data models"

##### Increment 4: Refactor `unilang_cli` Binary with Correct Parsing
*   **Goal:** To update the main CLI binary to use the new, unified parsing pipeline with the correct argument handling strategy.
*   **Steps:**
    1.  Update `src/bin/unilang_cli.rs` to use `unilang_instruction_parser::Parser`.
    2.  **Crucially, modify the parsing logic:**
        *   Take the arguments from `env::args().skip(1)`.
        *   `join` the arguments with a space to reconstruct the original command string.
        *   Pass this single string to `parser.parse_single_str()`.
    3.  Update the sample command definitions in `main` to use the new `CommandDefinition` fields and the `former` builder pattern.
*   **Increment Verification:**
    1.  Execute `cargo build --bin unilang_cli` via `execute_command`. The build must succeed.
    2.  Execute a simple command: `target/debug/unilang_cli add a::1 b::2`. The command should execute correctly.
*   **Commit Message:** "refactor(cli): Migrate unilang_cli to use correct parsing pipeline"

##### Increment 5: Migrate Integration Tests Incrementally
*   **Goal:** To methodically update all integration tests to use the new parsing pipeline and verify the full system behavior.
*   **Steps:**
    1.  **Fix Core Logic Tests First:**
        *   Start with `tests/inc/phase1/full_pipeline_test.rs` and other tests in `tests/inc/phase2/` that call `SemanticAnalyzer` directly.
        *   Update their test setup to use `unilang_instruction_parser::Parser`.
        *   Update assertions to check the structure of `VerifiedCommand` and `ErrorData`.
        *   Run these specific tests until they pass.
    2.  **Fix End-to-End CLI Tests:**
        *   Once the core logic is verified, fix `tests/inc/phase2/cli_integration_test.rs`.
        *   Update the `assert_cmd` assertions to match the new, correct `stderr` and `stdout` formats.
        *   Run this test file until it passes.
*   **Increment Verification:**
    1.  Execute `timeout 90 cargo test -p unilang --all-targets` via `execute_command`. All tests **must pass**.
*   **Commit Message:** "fix(tests): Migrate all integration tests to the new parsing pipeline"

##### Increment 6: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output.
*   **Steps:**
    1.  Perform a self-critique of all changes against the plan's goal and requirements.
    2.  Run the Crate Conformance Check one last time.
    3.  Execute `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    1.  Execute the full `Crate Conformance Check Procedure`.
    2.  Execute `git status` via `execute_command` and confirm the output shows no uncommitted changes.
*   **Commit Message:** "feat(unilang): Finalize architectural unification and verification"

### Changelog
*   [Initial] Plan created to unify the parsing architecture by removing the legacy parser, integrating `unilang_instruction_parser`, and updating core data models.

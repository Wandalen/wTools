# Task Plan: Implement Phase 5: Core API Enhancements and Modality Support (v4 - Wasm Compatible)

### Goal
*   To implement the remaining mandatory functional requirements from Spec v2.2.0, ensuring the framework fully supports REPL and interactive CLI modalities, resulting in a functionally complete API for building sophisticated command-line applications. **A core goal is to ensure the `unilang` library is `no_std`/Wasm compatible, allowing it to be used in a web browser environment.**

### Ubiquitous Language (Vocabulary)
*   **REPL:** Read-Eval-Print Loop, an interactive command-line session.
*   **Modality:** A specific way of interacting with the application (e.g., CLI, REPL, Web).
*   **Interactive Argument:** An argument that, if missing, should trigger a prompt for user input rather than an immediate error.
*   **Pipeline:** The sequence of components (Parser, SemanticAnalyzer, Interpreter) that process a command from string to execution.
*   **Stateless Component:** A component that does not retain data between invocations, making it reusable in a loop.
*   **Wasm:** WebAssembly, a binary instruction format that allows code to run in web browsers.

### Progress
*   **Roadmap Milestone:** Phase 5: Core API Enhancements and Modality Support
*   **Primary Editable Crate:** `unilang`
*   **Overall Progress:** 0/5 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Ensure `no_std`/Wasm Compatibility for Core Library
    *   ⚫ Increment 2: Implement Abstracted REPL Example
    *   ⚫ Increment 3: Implement Interactive Argument Signaling in SemanticAnalyzer
    *   ⚫ Increment 4: Create Tests and CLI Example for Interactive Prompting
    *   ⚫ Increment 5: Finalization

### Permissions and Boundaries
*   **Mode:** `code`
*   **Run workspace-wise commands:** `false`
*   **Add transient comments:** `true`
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   Control Files to Reference:
    *   `unilang/roadmap.md`
    *   `unilang/spec.md`
*   Files to Include:
    *   `unilang/src/types.rs`
    *   `unilang/src/pipeline.rs`
    *   `unilang/src/semantic.rs`
    *   `unilang/src/interpreter.rs`
    *   `unilang/src/error.rs`
    *   `unilang/src/data.rs`
    *   `unilang/src/bin/unilang_cli.rs`
    *   `unilang/Cargo.toml`

### Expected Behavior Rules / Specifications
*   **FR-REPL-1 (REPL Support):** The framework's core components **must** be structured to support a REPL-style execution loop.
*   **FR-INTERACTIVE-1 (Interactive Argument Prompting):** The Semantic Analyzer **must** return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`) for missing interactive arguments.
*   **New NFR (Wasm Compatibility):** The core `unilang` library crate must be compilable for the `wasm32-unknown-unknown` target.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `interactive_args_test::test_interactive_prompt_signal` | Not Started | |

#### Test Matrix for Interactive Argument Signaling

**Test Factors:**
-   Argument Presence: Whether the interactive argument is provided or omitted.
-   `interactive` Attribute: The value of the `interactive` flag on the argument definition.
-   `optional` Attribute: The value of the `optional` flag on the argument definition.

**Test Combinations:**

| ID | Aspect Tested | `interactive` | `optional` | Argument Provided | Expected Behavior |
|---|---|---|---|---|---|
| T1.1 | Signal on Missing | `true` | `false` | No | `Err(ErrorData { code: "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED", .. })` |
| T1.2 | No Signal on Present | `true` | `false` | Yes | `Ok(VerifiedCommand)` |
| T1.3 | Standard Missing Error | `false` | `false` | No | `Err(ErrorData { code: "UNILANG_ARGUMENT_MISSING", .. })` |
| T1.4 | No Signal on Optional | `true` | `true` | No | `Ok(VerifiedCommand)` (with default or no value) |

### Crate Conformance Check Procedure
*   Step 1: Execute `timeout 90 cargo test -p unilang --all-targets`. Analyze output for failures or warnings.
*   Step 2: If tests pass, execute `timeout 90 cargo clippy -p unilang -- -D warnings`. Analyze output for linter errors.
*   Step 3: **(Wasm Check)** If clippy passes, execute `cargo build -p unilang --no-default-features --target wasm32-unknown-unknown`. Analyze output for compilation errors. This verifies `no_std` compatibility.

### Increments
##### Increment 1: Ensure `no_std`/Wasm Compatibility for Core Library
*   **Goal:** Refactor filesystem-dependent validation logic in `unilang/src/types.rs` to be conditionally compiled, enabling the core library to build for the `wasm32-unknown-unknown` target.
*   **Specification Reference:** New NFR (Wasm Compatibility).
*   **Steps:**
    *   Step 1: **Analyze `unilang/src/types.rs`.** Read the file and identify the `std`-dependent code.
        *   **Context: Problematic Code in `parse_path_value`**
            ```rust
            // in unilang/src/types.rs
            fn parse_path_value( input : &str, kind : &Kind ) -> Result< Value, TypeError >
            {
              // ...
              let path = PathBuf::from(input);
              match kind {
                // ...
                Kind::File => {
                  if path.is_file() { // This uses std::fs
                    Ok(Value::File(path))
                  } else if path.is_dir() { // This uses std::fs
                    // ... error
                  } else {
                    // ... error
                  }
                }
                Kind::Directory => {
                  if path.is_dir() { // This uses std::fs
                    Ok(Value::Directory(path))
                  } // ... etc
                }
                // ...
              }
            }
            ```
    *   Step 2: **Apply Conditional Compilation.** Use `search_and_replace` to wrap the filesystem checks in `unilang/src/types.rs` with a `#[cfg(not(target_arch = "wasm32"))]` attribute. For Wasm targets, the validation will be skipped, and the path will be accepted as valid.
        *   **Action:** Replace the `Kind::File` and `Kind::Directory` match arms in `parse_path_value` with the following:
            ```rust
            // New logic for unilang/src/types.rs
            Kind::File => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if path.is_file() {
                        Ok(Value::File(path))
                    } else if path.is_dir() {
                        Err(TypeError {
                            expected_kind: kind.clone(),
                            reason: "Expected a file, but found a directory".to_string(),
                        })
                    } else {
                        Err(TypeError {
                            expected_kind: kind.clone(),
                            reason: format!("File not found at path: {input}"),
                        })
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // On Wasm, we cannot validate filesystem paths, so we accept any valid string.
                    Ok(Value::File(path))
                }
            }
            Kind::Directory => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if path.is_dir() {
                        Ok(Value::Directory(path))
                    } else if path.is_file() {
                        Err(TypeError {
                            expected_kind: kind.clone(),
                            reason: "Expected a directory, but found a file".to_string(),
                        })
                    } else {
                        Err(TypeError {
                            expected_kind: kind.clone(),
                            reason: format!("Directory not found at path: {input}"),
                        })
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // On Wasm, we cannot validate filesystem paths, so we accept any valid string.
                    Ok(Value::Directory(path))
                }
            }
            ```
*   **Increment Verification:**
    *   Step 1: Perform the full Crate Conformance Check, which now includes the Wasm build step. All three steps must pass.

##### Increment 2: Implement Abstracted REPL Example
*   **Goal:** Fulfill `FR-REPL-1` by creating a REPL example that separates the reusable pipeline logic from the I/O, making it web-compatible.
*   **Specification Reference:** `spec.md` Section 3: `FR-REPL-1`. `roadmap.md` Milestone `M5.1`.
*   **Steps:**
    *   Step 1: **Create New Example File.** Use `write_to_file` to create `unilang/examples/12_repl_loop.rs`.
    *   Step 2: **Implement Abstracted REPL.** Use `write_to_file` to populate `unilang/examples/12_repl_loop.rs` with the full content below. This version uses a generic `run_repl` function to demonstrate the core reusable logic, while `main` provides the `std::io` implementation.
        ```rust
        //! # REPL Loop Example (Wasm Compatible)
        //!
        //! This example demonstrates how to use the `unilang` Pipeline API within a
        //! Read-Eval-Print Loop (REPL), fulfilling the FR-REPL-1 requirement.
        //!
        //! The core logic is in `run_repl`, which is generic over its input and output.
        //! This separation ensures the core pipeline is reusable and can be compiled
        //! for WebAssembly, where I/O is handled differently (e.g., via JS interop).

        use unilang::prelude::*;
        use std::io::{self, Write};

        /// Generic REPL runner, demonstrating the reusability of the Pipeline.
        /// This function is modality-agnostic and Wasm-compatible.
        fn run_repl<I>(pipeline: &Pipeline, input: &mut I, output: &mut Vec<String>)
        where
            I: Iterator<Item = String>,
        {
            for line in input {
                let trimmed = line.trim();
                if trimmed == "exit" {
                    break;
                }
                if trimmed.is_empty() {
                    continue;
                }

                let result = pipeline.process_command_simple(trimmed);
                if result.success {
                    if let Some(out) = result.outputs.get(0) {
                        if !out.content.is_empty() {
                            output.push(out.content.clone());
                        }
                    }
                } else {
                    output.push(format!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string())));
                }
            }
        }

        fn main() -> Result<(), unilang::Error> {
            println!("=== REPL Loop Example ===");
            println!("Type a command or 'exit' to quit.");

            let mut registry = CommandRegistry::new();
            let echo_cmd = CommandDefinition::former()
                .name("echo")
                .arguments(vec![ArgumentDefinition::former()
                    .name("message")
                    .kind(Kind::String)
                    .attributes(ArgumentAttributes { multiple: true, ..Default::default() })
                    .end()])
                .end();
            let echo_routine = Box::new(|cmd: VerifiedCommand, _ctx| {
                let message = cmd.arguments.get("message").unwrap_or(&Value::String("".to_string()));
                Ok(OutputData { content: message.to_string(), format: "text".to_string() })
            });
            registry.command_add_runtime(&echo_cmd, echo_routine)?;
            let pipeline = Pipeline::new(registry);

            // This is the environment-specific part. For a native CLI, we use std::io.
            // For Wasm, this would be replaced with calls to JavaScript to get input.
            let mut stdin_lines = io::stdin().lines().map(|l| l.unwrap_or_default());
            let mut output_buffer = Vec::new();

            // Create a custom iterator to handle the prompt
            let mut prompted_input = std::iter::from_fn(move || {
                print!("> ");
                io::stdout().flush().unwrap();
                stdin_lines.next()
            });

            run_repl(&pipeline, &mut prompted_input, &mut output_buffer);

            // Print the captured output
            for line in output_buffer {
                println!("{}", line);
            }

            Ok(())
        }
        ```
    *   Step 3: **Add Example to `Cargo.toml`.** Use `insert_content` to add the new example target to `unilang/Cargo.toml`.
        ```toml

        [[example]]
        name = "12_repl_loop"
        path = "examples/12_repl_loop.rs"
        ```
*   **Increment Verification:**
    *   Step 1: Execute `timeout 90 cargo build -p unilang --examples`. Analyze output for success.
    *   Step 2: Perform Crate Conformance Check.

##### Increment 3: Implement Interactive Argument Signaling in SemanticAnalyzer
*   **Goal:** Fulfill `FR-INTERACTIVE-1` by modifying `SemanticAnalyzer` to return a specific error for missing interactive arguments.
*   **Specification Reference:** `spec.md` Section 3: `FR-INTERACTIVE-1` and Section 9.1: `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`. `roadmap.md` Milestone `M5.2`.
*   **Steps:**
    *   Step 1: **Modify `bind_arguments` Logic.** Use `search_and_replace` on `unilang/src/semantic.rs` to update the error handling for missing arguments.
        *   **Context: Existing Code Snippet in `bind_arguments`**
            ```rust
            if !value_found
            {
              if !arg_def.attributes.optional
              {
                return Err( Error::Execution( ErrorData::new(
                  "UNILANG_ARGUMENT_MISSING".to_string(),
                  format!( "Argument Error: The required argument '{}' is missing. Please provide a value for this argument.", arg_def.name ),
                )));
              }
              else if let Some( default_value ) = &arg_def.attributes.default
              {
                bound_arguments.insert( arg_def.name.clone(), parse_value( default_value, &arg_def.kind )? );
                value_found = true;
              }
            }
            ```
        *   **Action:** Replace the snippet above with the following improved logic.
            ```rust
            if !value_found
            {
              if !arg_def.attributes.optional
              {
                if arg_def.attributes.interactive
                {
                  return Err( Error::Execution( ErrorData::new(
                    "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED".to_string(),
                    format!( "Interactive input required for argument '{}'.", arg_def.name ),
                  )));
                }
                else
                {
                  return Err( Error::Execution( ErrorData::new(
                    "UNILANG_ARGUMENT_MISSING".to_string(),
                    format!( "Argument Error: The required argument '{}' is missing. Please provide a value for this argument.", arg_def.name ),
                  )));
                }
              }
              else if let Some( default_value ) = &arg_def.attributes.default
              {
                bound_arguments.insert( arg_def.name.clone(), parse_value( default_value, &arg_def.kind )? );
                // value_found = true; // This was a bug in the previous plan, not needed here.
              }
            }
            ```
*   **Increment Verification:**
    *   Step 1: Perform Crate Conformance Check.

##### Increment 4: Create Tests and CLI Example for Interactive Prompting
*   **Goal:** Verify the interactive argument signal with a new unit test and update the main CLI binary to demonstrate how to handle the signal.
*   **Specification Reference:** `spec.md` Section 3: `FR-INTERACTIVE-1`. `roadmap.md` Milestone `M5.3`.
*   **Steps:**
    *   Step 1: **Create New Test File.** Use `write_to_file` to create `unilang/tests/inc/phase5/interactive_args_test.rs`.
    *   Step 2: **Implement Test Case.** Use `write_to_file` to populate the new test file with the full content below.
        ```rust
        //! ## Test Matrix for Interactive Argument Signaling
        //!
        //! | ID   | Aspect Tested      | `interactive` | `optional` | Argument Provided | Expected Behavior                                           |
        //! |------|--------------------|---------------|------------|-------------------|-------------------------------------------------------------|
        //! | T1.1 | Signal on Missing  | `true`        | `false`    | No                | `Err(ErrorData { code: "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED", .. })` |

        use unilang::prelude::*;
        use unilang_parser::{Parser, UnilangParserOptions};
        use unilang::semantic::SemanticAnalyzer;

        /// Tests that the SemanticAnalyzer returns the correct error code
        /// when a mandatory interactive argument is missing.
        /// Test Combination: T1.1
        #[test]
        fn test_interactive_prompt_signal() {
            let mut registry = CommandRegistry::new();
            let cmd_def = CommandDefinition::former()
                .name("interactive_cmd")
                .arguments(vec![
                    ArgumentDefinition::former()
                        .name("prompt_me")
                        .kind(Kind::String)
                        .attributes(ArgumentAttributes {
                            optional: false,
                            interactive: true,
                            ..Default::default()
                        })
                        .end()
                ])
                .end();
            registry.register(cmd_def);

            let parser = Parser::new(UnilangParserOptions::default());
            let instruction = parser.parse_single_instruction("interactive_cmd").unwrap();
            let instructions = &[instruction];
            let analyzer = SemanticAnalyzer::new(instructions, &registry);

            let result = analyzer.analyze();
            assert!(result.is_err());

            if let Err(unilang::error::Error::Execution(err_data)) = result {
                assert_eq!(err_data.code, "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED");
                assert!(err_data.message.contains("prompt_me"));
            } else {
                panic!("Expected Execution Error with interactive signal, but got {:?}", result);
            }
        }
        ```
    *   Step 3: **Add Test to `Cargo.toml`.** Use `insert_content` to add the new test target to `unilang/Cargo.toml`.
        ```toml

        [[test]]
        name = "interactive_args_test"
        path = "tests/inc/phase5/interactive_args_test.rs"
        ```
    *   Step 4: **Update CLI Binary.** Use `search_and_replace` on `unilang/src/bin/unilang_cli.rs` to add a new command for demonstration and update the error handling.
        *   **Action 1: Add new command.** Insert the following command definition into the `run()` function.
            ```rust
            let user_add_def = CommandDefinition::former()
                .name("add")
                .namespace(".user".to_string())
                .description("Adds a new user.".to_string())
                .arguments(vec![
                    ArgumentDefinition::former().name("username").kind(ArgumentKind::String).end(),
                    ArgumentDefinition::former()
                        .name("password")
                        .kind(ArgumentKind::String)
                        .attributes(ArgumentAttributes { interactive: true, sensitive: true, ..Default::default() })
                        .end(),
                ])
                .end();
            let user_add_routine: CommandRoutine = Box::new(|cmd, _ctx| {
                let username = cmd.arguments.get("username").unwrap();
                println!("Adding user: {}", username);
                Ok(OutputData { content: "User added".to_string(), format: "text".to_string() })
            });
            registry.command_add_runtime(&user_add_def, user_add_routine)?;
            ```
        *   **Action 2: Update error handling.** Replace the existing `semantic_analyzer.analyze()` call and its `match` block with the updated version.
            *   **Before:**
                ```rust
                let commands = semantic_analyzer.analyze()?;
                ```
            *   **After:**
                ```rust
                let commands = match semantic_analyzer.analyze()
                {
                  Ok( commands ) => commands,
                  Err( unilang::error::Error::Execution( error_data ) ) if error_data.code == "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" =>
                  {
                    eprintln!( "Input needed: {}", error_data.message );
                    std::process::exit( 2 );
                  },
                  Err( e ) => return Err( e ),
                };
                ```
*   **Increment Verification:**
    *   Step 1: Perform Crate Conformance Check.
    *   Step 2: Manually run `cargo run --bin unilang_cli -- .user.add username::test`. Verify it prints the "Interactive input required for argument 'password'." message to stderr and exits with code 2.

##### Increment 5: Finalization
*   **Goal:** Perform a final review, update project documentation, and ensure all changes for Phase 5 are complete and verified.
*   **Specification Reference:** `roadmap.md` Phase 5 completion.
*   **Steps:**
    *   Step 1: **Update `readme.md`.** Use `insert_content` to add a new section to `unilang/readme.md` under the "Advanced Features" heading.
        ```markdown

        ### REPL and Interactive Use
        The framework's components are stateless and `no_std`/Wasm compatible, making them suitable for REPLs (Read-Eval-Print Loops) and web-based environments. It also supports interactive arguments, allowing CLIs to prompt users for missing information instead of failing. See `examples/12_repl_loop.rs` for a demonstration.
        ```
    *   Step 2: **Update `roadmap.md`.** Use `search_and_replace` on `unilang/roadmap.md` to change the status of Phase 5 milestones.
        *   Replace `[⚫] **M5.1: refactor_pipeline_for_reusability_and_add_repl_example:**` with `[✅] **M5.1: refactor_pipeline_for_reusability_and_add_repl_example:**`
        *   Replace `[⚫] **M5.2: implement_interactive_argument_signaling:**` with `[✅] **M5.2: implement_interactive_argument_signaling:**`
        *   Replace `[⚫] **M5.3: create_interactive_prompting_test:**` with `[✅] **M5.3: create_interactive_prompting_test:**`
    *   Step 3: **Update `changelog.md`.** Use `insert_content` to prepend a new entry to `unilang/changelog.md`.
        ```markdown
        - Ensured core library is `no_std`/Wasm compatible by making filesystem validation conditional.
        - Implemented REPL support by ensuring core components are stateless and added a modality-agnostic REPL example.
        - Implemented interactive argument signaling to allow CLIs to prompt for missing input.
        ```
    *   Step 4: **Perform Final Verification.** Execute the full `Crate Conformance Check Procedure`.
*   **Increment Verification:**
    *   Step 1: All checks must pass.

### Task Requirements
*   All code must adhere to the existing style and pass Clippy lints.
*   The implementation must fully satisfy the functional requirements `FR-REPL-1` and `FR-INTERACTIVE-1` from `spec.md`.
*   The core `unilang` library must be compilable for the `wasm32-unknown-unknown` target.

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook.
*   Must use Rust 2021 edition.

### Assumptions
*   The core components (`Parser`, `SemanticAnalyzer`, `Interpreter`) are already largely stateless and will not require major refactoring.
*   The `textdistance` dependency (used for `on_unknown_suggest`) may not be `no_std` compatible. This feature might need to be conditionally compiled in the future, but that is out of scope for this task.

### Out of Scope
*   Implementing the actual input reading logic for interactive prompts in the CLI. The goal is only to demonstrate catching the signal and printing a prompt.
*   Implementing a full-featured REPL with history, completion, etc. The example will be a simple input loop.
*   Making the `on_unknown_suggest` feature (and its `textdistance` dependency) `no_std` compatible.

### External System Dependencies
*   None.

### Notes and Insights
*   The Wasm compatibility requirement is a significant architectural driver. By addressing it early, we ensure the framework remains flexible for future modalities. The abstraction in the REPL example is a key pattern for maintaining this flexibility.

### Changelog
*   [2025-08-05] Initial plan created.
*   [2025-08-05] Plan critiqued and improved with more specific implementation details, test matrices, and documentation steps.
*   [2025-08-05] Plan elaborated with context-rich details, including embedded code snippets, API definitions, and direct rule references to make it fully self-contained for the executor.
*   [2025-08-05] Plan re-architected to address the new `no_std`/Wasm compatibility requirement. Added a new increment for Wasm-proofing, redesigned the REPL example to be I/O agnostic, and updated the verification procedure to include a Wasm build check.

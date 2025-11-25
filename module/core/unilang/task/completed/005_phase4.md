
# Task Plan: Phase 4 - Zero-Overhead Static Command Registry (Revised & Elaborated)

### Goal
*   To implement Phase 4 of the `unilang` roadmap, focusing on the mandatory performance non-functional requirement for a zero-overhead static command system. This will be achieved by creating a hybrid command registry that uses a Perfect Hash Function (PHF) map for all compile-time commands, ensuring instantaneous startup and sub-millisecond command resolution.

### Ubiquitous Language (Vocabulary)
*   **Static Command:** A command whose definition is known at compile-time.
*   **`StaticCommandDefinition`:** A `const`-compatible representation of a command, using `&'static str` and `&'static [...]` instead of `String` and `Vec`.
*   **Runtime Command:** A command registered dynamically after the application has started.
*   **PHF (Perfect Hash Function):** A hash function that maps a static set of keys to a set of integers with no collisions.
*   **Static Registry:** The part of the `CommandRegistry` that stores static commands in a PHF map, generated at compile-time.
*   **Dynamic Registry:** The part of the `CommandRegistry` that stores runtime commands in a standard `HashMap`.
*   **Hybrid Registry:** The final `CommandRegistry` design that combines the static PHF and the dynamic `HashMap`.

### Progress
*   **Roadmap Milestone:** Phase 4: Zero-Overhead Static Command Registry
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 0/6 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Project Setup and `StaticCommandDefinition`
    *   ⚫ Increment 2: Implement PHF Generation Logic in `build.rs`
    *   ⚫ Increment 3: Refactor `CommandRegistry` to a Hybrid Model
    *   ⚫ Increment 4: Create Performance Stress Test Harness
    *   ⚫ Increment 5: Implement and Run Performance Assertions
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   Control Files to Reference:
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang/roadmap.md`
*   Files to Include (for AI's reference):
    *   `module/move/unilang/src/registry.rs`
    *   `module/move/unilang/src/data.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/Cargo.toml`

### Expected Behavior Rules / Specifications
*   **NFR-Performance:** For an application with 1,000+ static commands, the framework must introduce zero runtime overhead for command registration. Startup time must not be impacted by the number of static commands. The p99 latency for resolving a command `FullName` must be less than 1 millisecond.
*   The `CommandRegistry` must function as a hybrid, seamlessly resolving both compile-time (static) and run-time (dynamic) commands, with static lookups taking precedence.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| | | |

### Crate Conformance Check Procedure
*   Run `timeout 180 cargo test -p unilang -- --nocapture` and verify it passes with no warnings.
*   Run `timeout 180 cargo clippy -p unilang -- -D warnings -A clippy::too-many-lines` and verify it passes with no warnings.

### Increments

##### Increment 1: Project Setup and `StaticCommandDefinition`
*   **Goal:** To prepare the `unilang` crate for build-time code generation by adding dependencies, creating the `build.rs` script, and defining the necessary `const`-compatible static data structures.
*   **Specification Reference:** `roadmap.md` M4.1
*   **Steps:**
    1.  **Read `Cargo.toml`:** Use `read_file` to load `module/move/unilang/Cargo.toml`.
    2.  **Add Dependencies:** Use `insert_content` to add `phf = { version = "0.11", features = ["macros"] }` to the `[dependencies]` section.
    3.  **Add Build Dependencies:** Use `insert_content` to add a `[build-dependencies]` section with `phf_codegen = "0.11"`, `serde = "1.0"`, and `serde_yaml = "0.9"`.
    4.  **Create `build.rs`:** Use `write_to_file` to create `module/move/unilang/build.rs` with the initial content:
        ```rust
        fn main() {
            println!("cargo:rerun-if-changed=build.rs");
        }
        ```
    5.  **Create Static Data Models:** Use `write_to_file` to create a new file `module/move/unilang/src/static_data.rs`. This file will contain `const`-compatible versions of the data models.
        ```rust
        // module/move/unilang/src/static_data.rs
        //! Contains `const`-compatible data structures for static command definitions.

        // Note: These structs will be expanded in the build script and here.
        // For now, we just create the file.
        ```
    6.  **Declare Module:** Use `insert_content` in `module/move/unilang/src/lib.rs` to add `pub mod static_data;`.
    7.  **Perform Increment Verification.**
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo build -p unilang`. The build must complete successfully, confirming the `build.rs` script is recognized and dependencies are resolved.
*   **Commit Message:** "chore(unilang): Set up build script and static data models for PHF generation"

##### Increment 2: Implement PHF Generation Logic in `build.rs`
*   **Goal:** To implement the core logic in `build.rs` that reads a manifest of static commands and generates a Rust source file containing a PHF map and all associated `const` data.
*   **Specification Reference:** `roadmap.md` M4.2
*   **Steps:**
    1.  **Create Manifest:** Use `write_to_file` to create `module/move/unilang/unilang.commands.yaml` with a few static command definitions.
    2.  **Define Static Structs:** In `build.rs`, define the `StaticCommandDefinition` and related structs. These need to be `serde::Deserialize` for parsing the YAML and must be `const`-compatible for code generation. This is a known challenge; the approach will be to deserialize into temporary structs and then generate code for the `const` static structs.
    3.  **Implement Build Logic:** Update `build.rs` to:
        a. Read and parse `unilang.commands.yaml` into `Vec<CommandDefinition>` (the existing, dynamic struct).
        b. Determine the output path: `let path = Path::new(&env::var("OUT_DIR").unwrap()).join("static_commands.rs");`.
        c. Open this path for writing.
        d. Write `use` statements for `phf` and the static data models.
        e. Iterate through the parsed definitions and generate `const` data as a string (e.g., `const CMD_GREET_NAME: &'static str = "greet";`).
        f. Generate `const` instances of the `StaticCommandDefinition` structs.
        g. Generate the `phf_codegen::Map` builder code, mapping full command names to the `const` structs.
        h. Write the final `phf::Map` to the file.
        i. Add `println!("cargo:rerun-if-changed=unilang.commands.yaml");`.
    4.  **Perform Increment Verification.**
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo build -p unilang`.
    2.  Use `read_file` to inspect the generated `target/debug/build/unilang-*/out/static_commands.rs`. It must contain valid Rust code defining `const` data and a `phf::Map`.
*   **Commit Message:** "feat(unilang): Implement build-time generation of PHF for static commands"

##### Increment 3: Refactor `CommandRegistry` to a Hybrid Model
*   **Goal:** To integrate the generated static PHF map into the runtime `CommandRegistry` and adapt all lookup logic to use this new hybrid structure.
*   **Specification Reference:** `roadmap.md` M4.3
*   **Steps:**
    1.  **Update `static_data.rs`:** Populate `module/move/unilang/src/static_data.rs` with the final `StaticCommandDefinition` and related structs, making them public. Add an implementation of `From<&'static StaticCommandDefinition>` for `CommandDefinition` to convert from the static to the dynamic version.
    2.  **Modify `registry.rs`:**
        a. Use `include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));` at the top level.
        b. Change the `CommandRegistry` struct: rename `commands` to `dynamic_commands`.
        c. Create a new public method `command(&self, name: &str) -> Option<CommandDefinition>`.
        d. Implement the hybrid lookup logic in `command()`: check `STATIC_COMMANDS` first, convert the result to `CommandDefinition`, and if not found, fall back to `dynamic_commands`.
    3.  **Update `SemanticAnalyzer`:** In `semantic.rs`, change the lookup logic to use the new `registry.command()` method.
    4.  **Update Tests:** Modify all tests that interact with the registry (e.g., `full_pipeline_test.rs`, `command_loader_test.rs`) to account for the new hybrid lookup. Some tests might need to register commands dynamically to test that part of the registry.
    5.  **Perform Increment Verification.**
*   **Increment Verification:**
    1.  Perform the Crate Conformance Check. All existing tests must pass.
*   **Commit Message:** "refactor(unilang): Integrate static PHF map into a hybrid CommandRegistry"

##### Increment 4: Create Performance Stress Test Harness
*   **Goal:** To create the necessary infrastructure for a performance stress test, including a mechanism to generate a large number of static commands and a dedicated binary to test them.
*   **Specification Reference:** `roadmap.md` M4.4.1, M4.4.2
*   **Steps:**
    1.  **Create Test File:** Use `write_to_file` to create `module/move/unilang/tests/inc/phase4/performance_stress_test.rs`.
    2.  **Create Test Binary:** Use `write_to_file` to create `module/move/unilang/tests/stress_test_bin.rs`.
    3.  **Implement YAML Generator:** In `performance_stress_test.rs`, write a function `generate_stress_yaml(count: usize) -> String` that creates a YAML string with `count` unique command definitions.
    4.  **Implement Test Binary Logic:** In `stress_test_bin.rs`, write a `main` function that initializes the `CommandRegistry`, performs a large number of random lookups against the static commands, measures the p99 latency using a library like `hdrhistogram`, and prints the result to stdout before printing "Ready".
    5.  **Orchestrate the Test:** In `performance_stress_test.rs`, the main test function will:
        a. Set an environment variable `UNILANG_STATIC_COMMANDS_PATH` to a path in the `target` directory.
        b. Call `generate_stress_yaml(1000)` and write the result to that path.
        c. Modify `build.rs` to read from `UNILANG_STATIC_COMMANDS_PATH` if it is set.
    6.  **Perform Increment Verification.**
*   **Increment Verification:**
    1.  The `performance_stress_test.rs` test should successfully generate the large YAML file.
    2.  Execute `cargo test --test stress_test_bin --no-run`. The binary must compile successfully against the large generated PHF.
*   **Commit Message:** "test(unilang): Create harness for performance stress testing"

##### Increment 5: Implement and Run Performance Assertions
*   **Goal:** To execute the performance stress test and assert that the startup time and command resolution latency meet the non-functional requirements.
*   **Specification Reference:** `roadmap.md` M4.4.3, M4.4.4; `spec.md` NFR-Performance
*   **Steps:**
    1.  **Expand Test Logic:** In `performance_stress_test.rs`, use `assert_cmd::Command::cargo_bin("stress_test_bin")` to run the compiled test binary.
    2.  **Measure Startup:** The test will measure the total execution time of the binary as a proxy for startup time + lookup time.
    3.  **Parse Output:** The test will capture the stdout from the binary, parse the p99 latency value.
    4.  **Assert Performance:** Assert that the total time is within a reasonable bound (e.g., < 200ms) and that the parsed p99 latency is below the required threshold (< 1ms).
    5.  **Perform Increment Verification.**
*   **Increment Verification:**
    1.  Execute `timeout 300 cargo test -p unilang --test performance_stress_test -- --nocapture`. The test must pass all performance assertions.
*   **Commit Message:** "test(unilang): Implement and pass performance stress test for static registry"

##### Increment 6: Finalization
*   **Goal:** To perform a final review, remove any temporary test artifacts, and verify the entire task's output.
*   **Steps:**
    1.  Review all changes made during this phase.
    2.  Ensure all new code is documented.
    3.  Clean up the `unilang.commands.yaml` file, leaving only a few representative examples.
    4.  Unset the `UNILANG_STATIC_COMMANDS_PATH` environment variable logic or make it test-only.
    5.  Perform the full Crate Conformance Check procedure one last time.
    6.  Perform the `Finalization Increment Verification` procedure from the design rules.
*   **Increment Verification:**
    1.  All checks must pass.
*   **Commit Message:** "feat(unilang): Complete and finalize zero-overhead static command registry"

### Notes & Insights
*   **`const` Compatibility is Key:** The core of this phase is the `StaticCommandDefinition` struct. It's crucial that this struct and all its nested types are `const`-compatible, which means no heap allocations (`String`, `Vec`).
*   **Routine Registration Compromise:** This plan explicitly acknowledges that `CommandRoutine`s cannot be stored statically. The performance gain comes from offloading the parsing and storage of command *definitions* to compile time. Routines for all commands (static and dynamic) will still need to be registered at runtime into a `HashMap`. This is a pragmatic approach that meets the performance NFR for command *resolution*.

### Changelog
*   [Initial] Created a new development plan for Phase 4.
*   [Critique] Revised the plan to address a critical flaw regarding Rust's `const` rules by introducing `StaticCommandDefinition` and refining the build process. Clarified the hybrid nature of routine handling.
*   [Elaboration] Provided a full, detailed version of the revised plan with explicit steps for each increment.

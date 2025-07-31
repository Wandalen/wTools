# Unilang Crate/Framework Implementation Roadmap

### Current Status (as of 2025-07-31)
The project has successfully completed its foundational phases (1-3), culminating in a critical architectural refactoring that unified the parsing pipeline and data models. The framework is now stable and robust. The next phase will focus on implementing the mandatory performance requirement for a zero-overhead static command registry, which is the cornerstone for building large-scale, high-performance utilities.

**Legend:**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   🏁 : Phase Complete / Major Milestone

---

### Phase 1: Core `unilang` Language Engine & CLI Foundations 🏁
*   **Goal:** Establish the `unilang` parsing pipeline, core data structures, command registration, basic type handling, execution flow, initial help capabilities, and error reporting to enable a functional CLI.
*   **Outcome:** A working, foundational `unilang` crate capable of handling basic CLI commands from parsing to execution.
*   **Status:** All milestones are complete.

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*   **Goal:** Expand the `unilang` crate's type system, provide APIs for runtime command management, and mature CLI support.
*   **Outcome:** A feature-rich framework capable of handling complex data types, dynamic command loading, and advanced CLI interactions.
*   **Status:** All milestones are complete.

### Phase 3: Architectural Unification & Enhancement 🏁
*   **Goal:** Correct the project's architecture by removing legacy components, integrating `unilang_parser` as the single source of truth, and fully aligning data models with the specification.
*   **Outcome:** A stable, maintainable codebase with a unified architecture, ready for the implementation of core functional requirements.
*   **Status:** All milestones are complete.

### Phase 4: Zero-Overhead Static Command Registry ⏳
*   **Goal:** To implement the mandatory performance NFR for a zero-overhead static command system, enabling utilities with thousands of commands to start instantly.
*   **Outcome:** A framework with a hybrid command registry where all compile-time commands are stored in a Perfect Hash Function (PHF), eliminating runtime registration costs and ensuring sub-millisecond command resolution.

*   [⚫] **M4.1: design_hybrid_registry_architecture:**
    *   **Spec Reference:** FR-PERF-1, NFR-Performance
    *   **Deliverable:** A detailed task plan for implementing a zero-overhead static command registry.
    *   **Description:** Design a build-time mechanism (using `build.rs` and the `phf` crate) to generate a Perfect Hash Function (PHF) map for all compile-time command definitions. This plan will outline the steps to refactor the `CommandRegistry` into a hybrid model (static PHF for compile-time commands + dynamic HashMap for runtime commands).
*   [⚫] **M4.2: implement_build_time_phf_generation:**
    *   **Prerequisites:** M4.1
    *   **Deliverable:** A `build.rs` script that generates a `.rs` file containing the static PHF maps for commands and routines.
    *   **Description:** Implement the build script that scans the source code (or a manifest) for static command definitions and uses the `phf_codegen` crate to construct the perfect hash maps.
*   [⚫] **M4.3: refactor_command_registry_to_hybrid_model:**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** An updated `CommandRegistry` that uses the generated PHF for static commands.
    *   **Tasks:**
        *   [⚫] **4.3.1:** Modify the `CommandRegistry` struct to hold both the static PHF (included via `include!`) and the dynamic `HashMap`.
        *   [⚫] **4.3.2:** Refactor all lookup methods (`get_command`, `get_routine`) to query the static PHF first before falling back to the dynamic `HashMap`.
*   [⚫] **M4.4: implement_performance_stress_test:**
    *   **Prerequisites:** M4.3
    *   **Spec Reference:** FR-PERF-1
    *   **Deliverable:** A new integration test that proves the performance non-functional requirement is met.
    *   **Tasks:**
        *   [⚫] **4.4.1:** Create a test that programmatically generates source code for over 1,000 static command definitions.
        *   [⚫] **4.4.2:** Use this generated code in a test binary to trigger the `build.rs` PHF generation.
        *   [⚫] **4.4.3:** Measure and assert that the resulting binary's startup time is negligible and not proportional to the number of commands.
        *   [⚫] **4.4.4:** Measure and assert that the p99 latency for command resolution is under 1ms.

### Phase 5: Core API Enhancements & Modality Support
*   **Goal:** To implement the remaining mandatory functional requirements from Spec v2.2.0, ensuring the framework fully supports REPL and interactive CLI modalities.
*   **Outcome:** A functionally complete API that provides all necessary hooks for building sophisticated, user-friendly command-line applications.

*   [⚫] **M5.1: refactor_pipeline_for_reusability_and_add_repl_example:**
    *   **Spec Reference:** FR-REPL-1
    *   **Deliverable:** A new example file (`repl_example.rs`) demonstrating the reusability of framework components in a loop.
    *   **Description:** Audit the core pipeline components (`Parser`, `SemanticAnalyzer`, `Interpreter`) to ensure they are stateless and can be reused. Create an example that simulates a REPL by repeatedly taking input and invoking the full pipeline using the same long-lived `Pipeline` instance.
*   [⚫] **M5.2: implement_interactive_argument_signaling:**
    *   **Spec Reference:** FR-INTERACTIVE-1
    *   **Deliverable:** The `SemanticAnalyzer` correctly returns a specific error for interactive prompts.
    *   **Tasks:**
        *   [⚫] **5.2.1:** In `semantic.rs`, modify the `bind_arguments` logic to check for missing mandatory arguments that have `interactive: true`.
        *   [⚫] **5.2.2:** When this condition is met, return an `Error::Execution` with the specific `ErrorData` code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`.
*   [⚫] **M5.3: create_interactive_prompting_test:**
    *   **Prerequisites:** M5.2
    *   **Deliverable:** A new unit test for the `SemanticAnalyzer` and an example in the CLI binary.
    *   **Tasks:**
        *   [⚫] **5.3.1:** Write a test that defines a command with a mandatory interactive argument, analyzes an instruction that omits it, and asserts that the returned error has the code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`.
        *   [⚫] **5.3.2:** Update `unilang_cli.rs` to demonstrate how to catch this specific error and print a user-friendly prompt.

### Phase 6: Advanced Features & Web Modality
*   **Goal:** Build on the stable and performant architecture to implement advanced framework features, including a Web API modality and a superior developer experience through procedural macros.
*   **Outcome:** A versatile, multi-modal framework that significantly reduces boilerplate for developers.

*   [⚫] **M6.1: design_web_api_modality:**
    *   **Deliverable:** A plan for mapping `unilang` commands to HTTP endpoints.
*   [⚫] **M6.2: implement_openapi_generator:**
    *   **Prerequisites:** M6.1
    *   **Deliverable:** A function that generates an OpenAPI v3+ specification from the `CommandRegistry`.
*   [⚫] **M6.3: implement_http_to_command_mapper:**
    *   **Prerequisites:** M6.1
    *   **Deliverable:** A utility/adapter that converts an incoming HTTP request into a `unilang` command invocation.
*   [⚫] **M6.4: create_web_api_example:**
    *   **Prerequisites:** M6.3
    *   **Deliverable:** An example application that serves a `unilang` registry as a REST API.
*   [⚫] **M6.5: design_procedural_macros:**
    *   **Deliverable:** An API design for the `#[command]` procedural macro in the `unilang_meta` crate.
*   [⚫] **M6.6: implement_command_macro:**
    *   **Prerequisites:** M6.5
    *   **Deliverable:** A working `#[command]` macro that generates `CommandDefinition` structs from Rust functions.

### Phase 7: Release Candidate Preparation
*   **Goal:** Focus on stability, developer experience, and documentation to prepare for a v1.0 release.
*   **Outcome:** A polished, production-ready v1.0.0-rc.1 release of the `unilang` framework.

*   [⚫] **M7.1: write_core_concepts_guide:**
    *   **Deliverable:** A comprehensive guide in the documentation explaining the core architecture and philosophy of `unilang`.
*   [⚫] **M7.2: write_modality_tutorials:**
    *   **Prerequisites:** M6.4
    *   **Deliverable:** Tutorials for building a CLI, REPL, and a Web API with `unilang`.
*   [⚫] **M7.3: conduct_api_review_and_stabilization:**
    *   **Deliverable:** A final review of the public API, with any necessary breaking changes made before the 1.0 release.
*   [⚫] **M7.4: publish_v1_release_candidate:**
    *   **Prerequisites:** M7.3
    *   **Deliverable:** `unilang` v1.0.0-rc.1 published to crates.io.

### Phase 8: Post-v1.0 Ecosystem & Advanced Features
*   **Goal:** Expand the `unilang` ecosystem with new modalities, improved tooling, and advanced integration capabilities.
*   **Outcome:** A mature and extensible framework that solidifies its position as a universal command-line tool.

*   [⚫] **M8.1: implement_tui_modality_framework:**
    *   **Deliverable:** Utilities and an example for building interactive Textual User Interfaces.
*   [⚫] **M8.2: implement_dynamic_routine_loading:**
    *   **Deliverable:** A robust implementation for `routine_link` that can load routines from dynamic libraries.
*   [⚫] **M8.3: design_plugin_system:**
    *   **Deliverable:** A formal specification for a plugin system, allowing third-party crates to provide `unilang` commands to a host application.
# Unilang Crate/Framework Implementation Roadmap

### Current Status (as of 2025-07-31)
The project has successfully completed its foundational phases (1 and 2), establishing a mature core engine. Phase 3, a critical architectural refactoring, is now substantially complete, having successfully migrated the entire framework to the modern `unilang_parser` and aligned the core data models with the formal specification. The project is now well-positioned to begin work on advanced features and modalities in Phase 4.

**Legend:**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   ❌ : Blocked / Needs Revisit
*   🏁 : Phase Complete / Major Milestone

---

### Phase 1: Core `unilang` Language Engine & CLI Foundations 🏁
*   **Goal:** Establish the `unilang` parsing pipeline, core data structures, command registration, basic type handling, execution flow, initial help capabilities, and error reporting to enable a functional CLI.
*   **Outcome:** A working, foundational `unilang` crate capable of handling basic CLI commands from parsing to execution.

*   [✅] **1. Foundational Setup:**
    *   [✅] **1.1. Establish Testing Strategy & Framework:** (Unit & Integration test setup for the crate).
*   [✅] **2. CLI Input Processing - Phase 1: Lexical and Syntactic Analysis (Spec 1.1.1):**
    *   [✅] **2.1. Implement Lexer:** For `unilang` CLI syntax.
    *   [✅] **2.2. Implement Parser:** To build "Generic Instructions".
    *   [✅] **2.3. Global Argument Identification & Extraction Logic:** (Framework for integrators to define and extract their global arguments).
*   [✅] **3. Core Data Structures & Command Registry (Spec 0.2, 2, 2.4):**
    *   [✅] **3.1. Define Core Data Structures:** `CommandDefinition`, `ArgumentDefinition`, `Namespace`, `OutputData`, `ErrorData`.
    *   [✅] **3.2. Implement Unified Command Registry:**
        *   [✅] Core registry data structure.
        *   [✅] Provide Compile-Time Registration Mechanisms (e.g., builder API).
        *   [✅] Basic Namespace Handling Logic.
*   [✅] **4. CLI Input Processing - Phase 2: Semantic Analysis & Command Binding (Spec 1.1.2):**
    *   [✅] **4.1. Command Resolution Logic.**
    *   [✅] **4.2. Argument Binding Logic.**
    *   [✅] **4.3. Basic Argument Type System (`kind` - Spec 2.2.2):**
        *   [✅] Implement parsing/validation for `String`, `Integer`, `Float`, `Boolean`.
        *   [✅] Support core attributes: `optional`, `default`.
    *   [✅] **4.4. `VerifiedCommand` Object Generation.**
    *   [✅] **4.5. Implement Standard `UNILANG_*` Error Code Usage.**
*   [✅] **5. Interpreter / Execution Engine - Core (Spec 5):**
    *   [✅] **5.1. Define `ExecutionContext` Structure (basic version).**
    *   [✅] **5.2. Implement Routine Invocation mechanism.**
    *   [✅] **5.3. Basic Handling of Routine Results (`OutputData`, `ErrorData`).**
    *   [✅] **5.4. Command Separator (`;;`) Processing.**
*   [✅] **6. Basic Help Generation & Output (Spec 3.2.6, 4.2.1):**
    *   [✅] **6.1. Logic to generate structured help data from `CommandDefinition`s.**
    *   [✅] **6.2. Framework support for `.system.help.globals ?`.**
    *   [✅] **6.3. Provide default text formatters for basic CLI display.**

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*   **Goal:** Expand the `unilang` crate's type system, provide APIs for runtime command management, and mature CLI support.
*   **Outcome:** A feature-rich framework capable of handling complex data types, dynamic command loading, and advanced CLI interactions.

*   [✅] **1. Advanced Built-in Argument Types (`kind` - Spec 2.2.2):**
    *   [✅] **1.1. Implement parsing/validation for:** `Path`, `File`, `Directory`, `Enum`, `URL`, `DateTime`, `Pattern`.
    *   [✅] **1.2. Implement `List<Type>`.**
    *   [✅] **1.3. Implement `Map<KeyType,ValueType>`.**
    *   [✅] **1.4. Implement `JsonString` / `Object` types.**
    *   [✅] **1.5. Implement `multiple: true` attribute logic.**
    *   [✅] **1.6. Implement `validation_rules` attribute processing.**
*   [✅] **2. Runtime Command Registration & Management (Spec 4.5.B):**
    *   [✅] **2.1. Expose Crate API:** For `command_add_runtime`.
    *   [✅] **2.2. Provide Parsers (e.g., for YAML/JSON) for `CommandDefinition`s.**
    *   [✅] **2.3. Framework Support for `routine_link` Resolution (placeholder).**
*   [✅] **3. CLI Modality Enhancements (Integrator Focused):**
    *   [✅] **3.1. Framework support for `output_format` global argument.**
    *   [✅] **3.2. Framework hooks for Interactive Argument Prompting (`interactive: true`).**
    *   [✅] **3.3. Framework support for `on_error::continue` global argument.**
*   [✅] **4. `ExecutionContext` Enhancements (Spec 4.7):**
    *   [✅] **4.1. Standardize fields and access methods.**

### Phase 3: Architectural Unification & Enhancement ⏳
*   **Goal:** Correct the project's architecture by removing legacy components, integrating `unilang_parser` as the single source of truth, and fully aligning data models with the specification.
*   **Outcome:** A stable, maintainable, and consistent codebase with comprehensive test coverage, ready for future feature development.

*   [✅] **M3.1: design_architectural_unification_task:** A detailed `task_plan.md` was created and executed.
*   [✅] **M3.2: implement_parser_integration:**
    *   [✅] **3.2.1:** Refactored `unilang::semantic::SemanticAnalyzer` to consume `unilang_parser::GenericInstruction`.
    *   [✅] **3.2.2:** Refactored the `unilang_cli` binary to use the `unilang_parser`.
    *   [✅] **3.2.3:** Migrated all integration tests to the new unified parsing pipeline.
*   [✅] **M3.3: refactor_data_models_and_help:**
    *   [✅] **3.3.1:** Added all specified fields (`status`, `tags`, `version`, `aliases`, etc.) to `CommandDefinition` and `ArgumentDefinition`.
    *   [✅] **3.3.2:** Updated the `HelpGenerator` to display all new metadata fields.
    *   [✅] **3.3.3:** Implemented command alias resolution in the CLI binary.
*   [✅] **M3.4: update_documentation_and_examples:**
    *   [✅] **3.4.1:** Created a comprehensive example (`full_cli_example.rs`) demonstrating modern framework usage.
    *   [✅] **3.4.2:** Updated `spec.md` to formally document the multi-phase processing pipeline and the complete data models.
*   [⚫] **M3.5: finalize_unification:**
    *   **Deliverable:** A fully unified codebase with no legacy components.
    *   **Tasks:**
        *   [⚫] **3.5.1:** Remove any remaining legacy parsing modules and files (e.g., `src/ca/`).
        *   [⚫] **3.5.2:** Conduct a final code audit to ensure all components adhere to the new architecture.

### Phase 4: Advanced Features & Modalities
*   **Goal:** Build on the stable architecture to implement advanced framework features that enable powerful, multi-modal utilities.
*   **Outcome:** A versatile framework that supports global configuration, Web APIs, and a superior developer experience through procedural macros.

*   [⚫] **M4.1: design_global_argument_framework:**
    *   **Deliverable:** A design document for handling global arguments.
    *   **Description:** Plan how global arguments are defined, parsed, and integrated into the `ExecutionContext`.
*   [⚫] **M4.2: implement_global_argument_parsing:**
    *   **Prerequisites:** M4.1
    *   **Deliverable:** The ability to parse and validate global arguments from the command line and environment variables.
*   [⚫] **M4.3: integrate_globals_into_execution_context:**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** Global argument values are accessible to all command routines via the `ExecutionContext`.
*   [⚫] **M4.4: design_web_api_modality:**
    *   **Deliverable:** A plan for mapping `unilang` commands to HTTP endpoints.
    *   **Description:** Define the strategy for URL structure, HTTP method mapping (`http_method_hint`), and data serialization.
*   [⚫] **M4.5: implement_openapi_generator:**
    *   **Prerequisites:** M4.4
    *   **Deliverable:** A function that generates an OpenAPI v3+ specification from the `CommandRegistry`.
*   [⚫] **M4.6: implement_http_to_command_mapper:**
    *   **Prerequisites:** M4.4
    *   **Deliverable:** A utility/adapter that converts an incoming HTTP request into a `unilang` command invocation.
*   [⚫] **M4.7: create_web_api_example:**
    *   **Prerequisites:** M4.6
    *   **Deliverable:** An example application (e.g., using `axum` or `actix-web`) that serves a `unilang` registry as a REST API.
*   [⚫] **M4.8: design_procedural_macros:**
    *   **Deliverable:** An API design for the `#[command]` procedural macro in the `unilang_meta` crate.
*   [⚫] **M4.9: implement_command_macro:**
    *   **Prerequisites:** M4.8
    *   **Deliverable:** A working `#[command]` macro that generates `CommandDefinition` structs from Rust functions.
*   [⚫] **M4.10: update_documentation_for_macros:**
    *   **Prerequisites:** M4.9
    *   **Deliverable:** Documentation and examples for using the new procedural macros.

### Phase 5: Release Candidate Preparation
*   **Goal:** Focus on stability, performance, developer experience, and documentation to prepare for a v1.0 release.
*   **Outcome:** A polished, production-ready v1.0.0-rc.1 release of the `unilang` framework.

*   [⚫] **M5.1: establish_benchmarks:**
    *   **Deliverable:** A suite of benchmarks for key performance indicators (parsing, semantic analysis, command dispatch).
*   [⚫] **M5.2: optimize_hot_paths:**
    *   **Prerequisites:** M5.1
    *   **Deliverable:** Code optimizations applied to performance-critical sections of the framework.
*   [⚫] **M5.3: write_core_concepts_guide:**
    *   **Deliverable:** A comprehensive guide in the documentation explaining the core architecture and philosophy of `unilang`.
*   [⚫] **M5.4: write_modality_tutorials:**
    *   **Prerequisites:** M4.7
    *   **Deliverable:** Tutorials for building a CLI and a Web API with `unilang`.
*   [⚫] **M5.5: conduct_api_review_and_stabilization:**
    *   **Deliverable:** A final review of the public API, with any necessary breaking changes made before the 1.0 release.
*   [⚫] **M5.6: publish_v1_release_candidate:**
    *   **Prerequisites:** M5.5
    *   **Deliverable:** `unilang` v1.0.0-rc.1 published to crates.io.

### Phase 6: Post-v1.0 Ecosystem & Advanced Features
*   **Goal:** Expand the `unilang` ecosystem with new modalities, improved tooling, and advanced integration capabilities.
*   **Outcome:** A mature and extensible framework that solidifies its position as a universal command-line tool.

*   [⚫] **M6.1: implement_tui_modality_framework:**
    *   **Deliverable:** Utilities and an example for building interactive Textual User Interfaces.
*   [⚫] **M6.2: implement_dynamic_routine_loading:**
    *   **Deliverable:** A robust implementation for `routine_link` that can load routines from dynamic libraries (.so, .dll, .dylib).
*   [⚫] **M6.3: implement_advanced_shell_completions:**
    *   **Deliverable:** Dynamic shell completion logic that provides context-aware suggestions for argument values (e.g., file paths, enum choices).
*   [⚫] **M6.4: design_plugin_system:**
    *   **Deliverable:** A formal specification for a plugin system, allowing third-party crates to provide `unilang` commands to a host application.

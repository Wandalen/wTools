# Unilang Crate/Framework Implementation Roadmap

This roadmap outlines the development plan for the **`unilang` crate/framework**, based on the formal Unilang specification (v1.3). It addresses the current architectural state and provides a clear path toward a robust, feature-complete v1.0 release.

**Legend:**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   ❌ : Blocked / Needs Revisit
*   🏁 : Phase Complete / Major Milestone

---

### Phase 1: Core `unilang` Language Engine & CLI Foundations 🏁
*This phase establishes the `unilang` parsing pipeline, core data structures, command registration, basic type handling, execution flow, initial help capabilities, and error reporting, primarily enabling a functional CLI.*

*   **1. Foundational Setup:**
    *   [✅] **1.1. Establish Testing Strategy & Framework:** (Unit & Integration test setup for the crate).
*   **2. CLI Input Processing - Phase 1: Lexical and Syntactic Analysis (Spec 1.1.1):**
    *   [✅] **2.1. Implement Lexer:** For `unilang` CLI syntax.
    *   [✅] **2.2. Implement Parser:** To build an AST or "Generic Instructions".
    *   [✅] **2.3. Global Argument Identification & Extraction Logic:** (Framework for integrators to define and extract their global arguments).
*   **3. Core Data Structures & Command Registry (Spec 0.2, 2, 2.4):**
    *   [✅] **3.1. Define Core Data Structures:** `CommandDefinition`, `ArgumentDefinition`, `Namespace`, `OutputData`, `ErrorData`.
    *   [✅] **3.2. Implement Unified Command Registry:**
        *   [✅] Core registry data structure.
        *   [✅] Provide Compile-Time Registration Mechanisms (e.g., builder API, helper macros).
        *   [✅] Basic Namespace Handling Logic.
*   **4. CLI Input Processing - Phase 2: Semantic Analysis & Command Binding (Spec 1.1.2):**
    *   [✅] **4.1. Command Resolution Logic.**
    *   [✅] **4.2. Argument Binding Logic.**
    *   [✅] **4.3. Basic Argument Type System (`kind` - Spec 2.2.2):**
        *   [✅] Implement parsing/validation for `String`, `Integer`, `Float`, `Boolean`.
        *   [✅] Support core attributes: `optional`, `default_value`, `is_default_arg`.
    *   [✅] **4.4. `VerifiedCommand` Object Generation.**
    *   [✅] **4.5. Implement Standard `UNILANG_*` Error Code Usage:** Ensure `ErrorData` (from 3.1) utilizes defined codes for parsing/semantic errors (Spec 4.2).
*   **5. Interpreter / Execution Engine - Core (Spec 5):**
    *   [✅] **5.1. Define `ExecutionContext` Structure (basic version, Spec 4.7).**
    *   [✅] **5.2. Implement Routine Invocation mechanism.**
    *   [✅] **5.3. Basic Handling of Routine Results (`OutputData`, `ErrorData`):** Pass through for modality handling.
    *   [✅] **5.4. Command Separator (`;;`) Processing:** Parser support (from 2.2) and Interpreter support for sequential execution.
*   **6. Basic Help Generation & Output (Spec 3.2.6, 4.2.1):**
    *   [✅] **6.1. Logic to generate structured help data (JSON) from `CommandDefinition`s.**
    *   [✅] **6.2. Framework support for `.system.help.globals ?` (or similar) based on integrator-defined globals (structured JSON output).**
    *   [✅] **6.3. Provide default text formatters for structured help, `OutputData`, and `ErrorData` for basic CLI display.**

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*This phase expands the `unilang` crate's type system, provides APIs for runtime command management, and matures CLI support.*

*   **1. Advanced Built-in Argument Types (`kind` - Spec 2.2.2):**
    *   [✅] **1.1. Implement parsing/validation for:** `Path`, `File`, `Directory` (incl. URI utilities, absolute path resolution utilities - Spec 4.1), `Enum`, `URL`, `DateTime`, `Pattern`.
    *   [✅] **1.2. Implement `List<Type>`:** (incl. comma-separated CLI parsing helpers).
    *   [✅] **1.3. Implement `Map<KeyType,ValueType>`:** (incl. `key=value,...` CLI parsing helpers).
    *   [✅] **1.4. Implement `JsonString` / `Object` types.**
    *   [✅] **1.5. Implement `multiple: true` attribute logic for arguments.**
    *   [✅] **1.6. Implement `validation_rules` attribute processing (framework for basic rules like regex, min/max, with clear extension points for integrators).**
*   **2. Runtime Command Registration & Management (Spec 4.5.B, Appendix A.3.2):**
    *   [✅] **2.1. Expose Crate API:** For `command_add_runtime`.
    *   [✅] **2.2. Expose Crate API:** For `command_remove_runtime` (optional).
    *   [✅] **2.3. Provide Parsers (e.g., for YAML/JSON) for `CommandDefinition`s that integrators can use.**
    *   [✅] **2.4. Framework Support for `routine_link` Resolution:** (e.g., helpers for integrators to map these links to their compile-time routines or other dispatch mechanisms).
*   **3. CLI Modality Enhancements (Integrator Focused):**
    *   [✅] **3.1. Framework support for `output_format` global argument (Spec 3.2.4):**
        *   [✅] Provide JSON and YAML serializers for `OutputData`, `ErrorData`, and structured help.
    *   [✅] **3.2. Shell Completion Generation Logic (Spec 3.2.5):**
        *   [✅] Implement logic for a command like `.system.completion.generate shell_type::bash`.
    *   [✅] **3.3. Framework hooks for Interactive Argument Prompting (`interactive: true` - Spec 2.2.1, 5.2):** (e.g., a way for semantic analysis to signal a need for prompting, which the CLI modality would handle).
    *   [✅] **3.4. Framework support for `on_error::continue` global argument in Interpreter (Spec 5.1.3).**
*   **4. `ExecutionContext` Enhancements (Spec 4.7):**
    *   [✅] **4.1. Standardize fields and access methods for effective global args and a logger instance.**

---

### Phase 3: Architectural Unification
*This phase is critical for correcting the project's architecture by removing legacy components and integrating the correct, modern parser as the single source of truth.*

*   [⚫] **M3.0: design_architectural_unification_task**
    *   **Deliverable:** A detailed `task_plan.md` for the parser migration.
    *   **Description:** Analyze the codebase to map out all locations that depend on the legacy `unilang::parsing` module. Create a detailed, step-by-step plan for migrating each component (semantic analyzer, CLI binary, tests) to the `unilang_instruction_parser` crate. Define the verification strategy for each step.
*   [⚫] **M3.1: implement_parser_integration**
    *   **Prerequisites:** M3.0
    *   **Deliverable:** A codebase where `unilang_instruction_parser` is the sole parser.
    *   **Tasks:**
        *   [⚫] **3.1.1:** Remove the legacy `unilang::parsing` module and the redundant `src/ca/` directory.
        *   [⚫] **3.1.2:** Refactor `unilang::semantic::SemanticAnalyzer` to consume `Vec<unilang_instruction_parser::GenericInstruction>` and produce `VerifiedCommand`s.
        *   [⚫] **3.1.3:** Refactor the `unilang_cli` binary (`src/bin/unilang_cli.rs`) to use the `unilang_instruction_parser` directly for its input processing.
        *   [⚫] **3.1.4:** Migrate all existing integration tests (`full_pipeline_test.rs`, `cli_integration_test.rs`, etc.) to use the new unified parsing pipeline and assert on the new behavior.
*   [⚫] **M3.2: refactor_data_models**
    *   **Prerequisites:** M3.1
    *   **Deliverable:** Core data models in `src/data.rs` are fully aligned with the formal specification.
    *   **Tasks:**
        *   [⚫] **3.2.1:** Add `status`, `tags`, `idempotent`, `version` fields to the `CommandDefinition` struct.
        *   [⚫] **3.2.2:** Add `aliases`, `tags`, `interactive`, `sensitive` fields to the `ArgumentDefinition` struct.
        *   [⚫] **3.2.3:** Update the `HelpGenerator` to display information from the new data fields.
        *   [⚫] **3.2.4:** Create new integration tests to verify the behavior and help output of the new fields (e.g., a command with `aliases`).
*   [⚫] **M3.3: update_formal_specification**
    *   **Prerequisites:** M3.2
    *   **Deliverable:** An updated `spec.md` document.
    *   **Tasks:**
        *   [⚫] **3.3.1:** Revise `spec.md` to formally document the multi-phase processing pipeline (Lexical -> Semantic -> Execution).
        *   [⚫] **3.3.2:** Add sections to `spec.md` defining Global Arguments, the Extensibility Model, and Cross-Cutting Concerns like Security and Configuration.
        *   [⚫] **3.3.3:** Update the data model tables in `spec.md` to reflect the complete `CommandDefinition` and `ArgumentDefinition` structs.

### Phase 4: Advanced Features & Modalities
*This phase builds on the stable architecture to implement advanced framework features that enable powerful, multi-modal utilities.*

*   [⚫] **M4.0: implement_global_arguments**
    *   **Prerequisites:** M3.3
    *   **Deliverable:** Framework support for global arguments.
*   [⚫] **M4.1: implement_web_api_modality_framework**
    *   **Prerequisites:** M3.3
    *   **Deliverable:** Utilities and guides for generating a Web API.
    *   **Tasks:**
        *   [⚫] **4.1.1:** Implement OpenAPI v3+ specification generation logic.
        *   [⚫] **4.1.2:** Provide HTTP request-to-command mapping utilities.
*   [⚫] **M4.2: implement_extension_module_macros**
    *   **Prerequisites:** M3.3
    *   **Deliverable:** Procedural macros in `unilang_meta` to simplify command definition.

### Phase 5: Release Candidate Preparation
*This phase focuses on stability, performance, developer experience, and documentation to prepare for a v1.0 release.*

*   [⚫] **M5.0: conduct_performance_tuning**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** Performance benchmarks and identified optimizations.
*   [⚫] **M5.1: write_integrator_documentation**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** Comprehensive guides and tutorials for developers.
*   [⚫] **M5.2: finalize_api_for_v1**
    *   **Prerequisites:** M5.1
    *   **Deliverable:** A stable, well-documented v1.0 API.
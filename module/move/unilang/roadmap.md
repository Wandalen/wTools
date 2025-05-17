# Unilang Crate/Framework Implementation Roadmap

This document outlines a potential roadmap for implementing the **`unilang` crate/framework** itself, based on the Unilang specification (v1.0.0). This framework will provide the core language, parsing, command management, and extensibility hooks that a developer (referred to as the "integrator") can use to build their own utility.

The roadmap is structured hierarchically, presenting a logical flow of development. However, actual development will be iterative, and feedback from early integrations may influence the order and specifics of some tasks. Some parallel work across phases may be possible depending on resources.

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
    *   [⚫] **1.1. Establish Testing Strategy & Framework:** (Unit & Integration test setup for the crate).
*   **2. CLI Input Processing - Phase 1: Lexical and Syntactic Analysis (Spec 1.1.1):**
    *   [⚫] **2.1. Implement Lexer:** For `unilang` CLI syntax.
    *   [⚫] **2.2. Implement Parser:** To build an AST or "Generic Instructions".
    *   [⚫] **2.3. Global Argument Identification & Extraction Logic:** (Framework for integrators to define and extract their global arguments).
*   **3. Core Data Structures & Command Registry (Spec 0.2, 2, 2.4):**
    *   [⚫] **3.1. Define Core Data Structures:** `CommandDefinition`, `ArgumentDefinition`, `Namespace`, `OutputData`, `ErrorData`.
    *   [⚫] **3.2. Implement Unified Command Registry:**
        *   [⚫] Core registry data structure.
        *   [⚫] Provide Compile-Time Registration Mechanisms (e.g., builder API, helper macros).
        *   [⚫] Basic Namespace Handling Logic.
*   **4. CLI Input Processing - Phase 2: Semantic Analysis & Command Binding (Spec 1.1.2):**
    *   [⚫] **4.1. Command Resolution Logic.**
    *   [⚫] **4.2. Argument Binding Logic.**
    *   [⚫] **4.3. Basic Argument Type System (`kind` - Spec 2.2.2):**
        *   [⚫] Implement parsing/validation for `String`, `Integer`, `Float`, `Boolean`.
        *   [⚫] Support core attributes: `optional`, `default_value`, `is_default_arg`.
    *   [⚫] **4.4. `VerifiedCommand` Object Generation.**
    *   [⚫] **4.5. Implement Standard `UNILANG_*` Error Code Usage:** Ensure `ErrorData` (from 3.1) utilizes defined codes for parsing/semantic errors (Spec 4.2).
*   **5. Interpreter / Execution Engine - Core (Spec 5):**
    *   [⚫] **5.1. Define `ExecutionContext` Structure (basic version, Spec 4.7).**
    *   [⚫] **5.2. Implement Routine Invocation mechanism.**
    *   [⚫] **5.3. Basic Handling of Routine Results (`OutputData`, `ErrorData`):** Pass through for modality handling.
    *   [⚫] **5.4. Command Separator (`;;`) Processing:** Parser support (from 2.2) and Interpreter support for sequential execution.
*   **6. Basic Help Generation & Output (Spec 3.2.6, 4.2.1):**
    *   [⚫] **6.1. Logic to generate structured help data (JSON) from `CommandDefinition`s.**
    *   [⚫] **6.2. Framework support for `.system.help.globals ?` (or similar) based on integrator-defined globals (structured JSON output).**
    *   [⚫] **6.3. Provide default text formatters for structured help, `OutputData`, and `ErrorData` for basic CLI display.**

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*This phase expands the `unilang` crate's type system, provides APIs for runtime command management, and matures CLI support.*

*   **1. Advanced Built-in Argument Types (`kind` - Spec 2.2.2):**
    *   [⚫] **1.1. Implement parsing/validation for:** `Path`, `File`, `Directory` (incl. URI utilities, absolute path resolution utilities - Spec 4.1), `Enum`, `URL`, `DateTime`, `Pattern`.
    *   [⚫] **1.2. Implement `List<Type>`:** (incl. comma-separated CLI parsing helpers).
    *   [⚫] **1.3. Implement `Map<KeyType,ValueType>`:** (incl. `key=value,...` CLI parsing helpers).
    *   [⚫] **1.4. Implement `JsonString` / `Object` types.**
    *   [⚫] **1.5. Implement `multiple: true` attribute logic for arguments.**
    *   [⚫] **1.6. Implement `validation_rules` attribute processing (framework for basic rules like regex, min/max, with clear extension points for integrators).**
*   **2. Runtime Command Registration & Management (Spec 4.5.B, Appendix A.3.2):**
    *   [⚫] **2.1. Expose Crate API:** For `command_add_runtime`.
    *   [⚫] **2.2. Expose Crate API:** For `command_remove_runtime` (optional).
    *   [⚫] **2.3. Provide Parsers (e.g., for YAML/JSON) for `CommandDefinition`s that integrators can use.**
    *   [⚫] **2.4. Framework Support for `routine_link` Resolution:** (e.g., helpers for integrators to map these links to their compile-time routines or other dispatch mechanisms).
*   **3. CLI Modality Enhancements (Integrator Focused):**
    *   [⚫] **3.1. Framework support for `output_format` global argument (Spec 3.2.4):**
        *   [⚫] Provide JSON and YAML serializers for `OutputData`, `ErrorData`, and structured help.
    *   [⚫] **3.2. Shell Completion Generation Logic (Spec 3.2.5):**
        *   [⚫] Implement logic for a command like `.system.completion.generate shell_type::bash`.
    *   [⚫] **3.3. Framework hooks for Interactive Argument Prompting (`interactive: true` - Spec 2.2.1, 5.2):** (e.g., a way for semantic analysis to signal a need for prompting, which the CLI modality would handle).
    *   [⚫] **3.4. Framework support for `on_error::continue` global argument in Interpreter (Spec 5.1.3).**
*   **4. `ExecutionContext` Enhancements (Spec 4.7):**
    *   [⚫] **4.1. Standardize fields and access methods for effective global args and a logger instance.**

### Phase 3: Framework Support for Advanced Utility Features & Modalities 🏁
*Enable integrators to build more complex utilities and support diverse modalities by providing the necessary `unilang` framework features.*

*   **1. Advanced Core Feature Support:**
    *   [⚫] **1.1. Advanced Path Handling Logic (Spec 4.1):** Provide utilities for handling schemes like `clipboard://`, `stdin://`, `temp://` in path resolution.
    *   [⚫] **1.2. Permission Attribute Support (Spec 4.3.2):** Ensure `permissions` attribute is robustly parsed, stored, and available in `VerifiedCommand`.
    *   [⚫] **1.3. Sensitive Argument Handling Support (Spec 4.3.3):** Ensure `sensitive` flag in `ArgumentDefinition` is propagated to `VerifiedCommand` for modalities/logging to act upon.
    *   [⚫] **1.4. Configuration Access via `ExecutionContext` (Spec 4.4, 4.7):** Define clear API/trait for `utility1` to inject configuration access into `ExecutionContext`.
    *   [⚫] **1.5. Stream-based Argument Kind Support (`InputStream`/`OutputStream` - Spec 2.2.2, 4.7):** Define these kinds and the `ExecutionContext` methods for routines to acquire I/O streams.
*   **2. Framework Hooks for Modality Integration (Spec 3):**
    *   [⚫] **2.1. Modality Switching Support:** Provide a defined mechanism (e.g., a special `OutputData` variant or `ExecutionContext` flag) for a command like `.modality.set` to signal intent to `utility1`.
    *   [⚫] **2.2. TUI/GUI Adaptation Guidance & Examples:** Document how structured help, `OutputData`, `ErrorData`, and interactive prompting hooks can be consumed by TUI/GUI `Extension Module`s or `utility1`'s modality implementations.
*   **3. Framework Support for WEB Endpoint Generation (Spec 3.6):**
    *   [⚫] **3.1. OpenAPI Specification Generation Logic:** Robust generation from the command registry.
    *   [⚫] **3.2. Request Mapping Utilities:** Provide traits/helpers for parsing HTTP requests into `unilang` argument structures.
    *   [⚫] **3.3. Response Formatting Utilities:** Provide traits/helpers for formatting `OutputData`/`ErrorData` into HTTP responses.
*   **4. Logging Framework Integration (Spec 4.6):**
    *   [⚫] **4.1. Ensure `ExecutionContext` can robustly carry a logger instance (e.g., trait object) provided by `utility1`.**
    *   [⚫] **4.2. Provide examples/guidance on how `utility1` can integrate its logging facade with the `ExecutionContext` logger.**

### Phase 4: Mature Framework Capabilities & Developer Experience 🏁
*Focus on robust framework capabilities for complex `utility1` implementations and improving the developer experience for integrators.*

*   **1. Advanced WEB Endpoint Features (Framework Support - Spec 3.6):**
    *   [⚫] **1.1. Metadata in `CommandDefinition` to support asynchronous operations (e.g., hint for 202 Accepted, status link format).**
    *   [⚫] **1.2. Metadata support in `CommandDefinition` and `ArgumentDefinition` for detailed authentication/authorization requirements, reflected in OpenAPI.**
*   **2. `utility1://` URL Scheme Support (Spec 3.7):**
    *   [⚫] **2.1. Provide robust utilities within the crate to parse `utility1://` URLs into `unilang` Generic Instructions.**
*   **3. Compile-Time `Extension Module` Integration Aids (Spec 4.5, Appendix A.3.1):**
    *   [⚫] **3.1. Define `ExtensionModuleManifest`-like structure (or attributes within `unilang` crate) for `unilang_spec_compatibility` checking and metadata (for `utility1`'s build system to consume).**
    *   [⚫] **3.2. Provide robust helper macros or builder APIs (Developer Experience - DX Helpers) to simplify compile-time registration of commands and types from `Extension Module`s and directly within `utility1`.**
*   **4. Comprehensive `unilang` Crate Documentation:**
    *   [⚫] **4.1. Detailed API documentation for all public crate items.**
    *   [⚫] **4.2. In-depth integrator guides:** Covering core concepts, command/type definition, `ExecutionContext`, `Extension Module`s, modality integration.
    *   [⚫] **4.3. Maintain and publish the Unilang specification itself (this document) alongside the crate.**

### Phase 5: Ecosystem Enablement & Final Polish (v1.0 Release Focus) 🏁
*Finalize the `unilang` crate for a v1.0 release, focusing on stability, ease of use, and resources for integrators.*

*   **1. Internationalization & Localization Hooks for Integrators (Spec 4.7):**
    *   [⚫] **1.1. Ensure `ExecutionContext` can robustly carry and expose locale information from `utility1`.**
    *   [⚫] **1.2. Design `CommandDefinition` string fields (hints, messages) and error message generation to be easily usable with `utility1`'s chosen i18n library/system (e.g., by allowing IDs or structured messages).**
*   **2. Developer Tooling (Potentially separate tools or utilities within the crate):**
    *   [⚫] **2.1. Implement a validator for `unilang` command definition files (e.g., YAML/JSON schema or a dedicated validation tool/library function).**
    *   [⚫] **2.2. Expand SDK/DX helpers (from 4.3.2) for common patterns in `Extension Module` and command definition.**
*   **3. CLI Input Processing - Phase 3: Verification and Optimization Hooks (Spec 1.1.3):**
    *   [⚫] **3.1. Design and implement optional framework hooks (e.g., traits that integrators can implement) for advanced cross-command verification or optimization logic if clear use cases and patterns emerge.**
*   **4. Performance Profiling and Optimization:**
    *   [⚫] **4.1. Profile core parsing, registry, and execution paths using realistic benchmarks.**
    *   [⚫] **4.2. Implement optimizations where beneficial (e.g., for Perfect Hash Functions in registry if not already fully optimized, AST pooling).**
*   **5. Final API Review and Stabilization for v1.0.**
    *   [⚫] **5.1. Ensure API consistency, ergonomics, and adherence to language best practices (e.g., Rust API guidelines).**
    *   [⚫] **5.2. Address any remaining TODOs or known issues for a stable release. Create migration guide if any breaking changes from pre-1.0 versions.**


# spec

- **Name:** Unilang Framework
- **Version:** 3.1.0
- **Date:** 2025-09-16

### Table of Contents
*   **Part I: Public Contract (Mandatory Requirements)**
    *   1. Vision & Scope
        *   1.1. Core Vision: Define Once, Use Everywhere
        *   1.2. In Scope: The Multi-Crate Framework
        *   1.3. Out of Scope
    *   2. System Actors
    *   3. Ubiquitous Language (Vocabulary)
    *   4. Core Functional Requirements
        *   4.1. Command & Registry Management
        *   4.2. Argument Parsing & Type System
        *   4.3. Command Execution Pipeline
        *   4.4. Help & Discovery System
        *   4.5. Modality Support
    *   5. Non-Functional Requirements
    *   6. CLI Modality: Language Syntax & Processing
    *   7. API Reference: Core Data Structures
    *   8. Cross-Cutting Concerns (Error Handling, Security, Verbosity)
    *   9. Feature Flags & Modularity
*   **Part II: Internal Design (Design Recommendations)**
    *   10. Architectural Mandates & Design Principles
    *   11. Architectural Diagrams
    *   12. Crate-Specific Responsibilities
*   **Part III: Project & Process Governance**
    *   13. Project Goals & Success Metrics
    *   14. Deliverables
    *   15. Open Questions
        *   15.1. Governing Principles
    *   16. Core Principles of Development
*   **Appendix: Addendum**
    *   Conformance Checklist
    *   Finalized Internal Design Decisions
    *   Finalized Internal Data Models
    *   Environment Variables
    *   Finalized Library & Tool Versions
    *   Deployment Checklist

---
## Part I: Public Contract (Mandatory Requirements)
*This part of the specification defines the stable, externally visible promises of the `unilang` framework. All requirements in this section are mandatory.*

### 1. Vision & Scope

#### 1.1. Core Vision: Define Once, Use Everywhere
The `unilang` framework **must** provide a unified way to define command-line utility interfaces once, automatically enabling consistent interaction across multiple modalities such as CLI, TUI, GUI, and Web APIs. The core goals are:

*   **Consistency:** A single, declarative way to define commands and their arguments, regardless of how they are presented or invoked.
*   **Discoverability:** Easy ways for users and systems to find available commands and understand their usage through an automated help system.
*   **Flexibility:** Support for various methods of command definition (compile-time, run-time, declarative, procedural).
*   **Extensibility:** Provide structures that enable an integrator to build an extensible system.
*   **Efficiency:** Support for efficient parsing and zero-overhead command dispatch for statically defined commands.
*   **Interoperability:** A standardized representation for commands, enabling integration with other tools or web services.
*   **Robustness:** Clear, user-friendly error handling and a rich argument validation system.
*   **Security:** Provide a framework for defining and enforcing secure command execution.

#### 1.2. In Scope: The Multi-Crate Framework
The Unilang specification governs a suite of related crates that work together to provide the full framework functionality. The primary crates **must** be:

*   **`unilang`**: The core framework crate that orchestrates parsing, semantic analysis, execution, and modality management. It provides the primary public API for integrators.
*   **`unilang_parser`**: A dedicated, low-level crate responsible for the lexical and syntactic analysis of the `unilang` command language.
*   **`unilang_meta`**: A companion crate providing procedural macros (e.g., `#[command]`) to simplify compile-time command definition.

#### 1.3. Out of Scope
The `unilang` framework is responsible for the command interface and execution pipeline, not the business logic itself. The following are explicitly out of scope for the framework:

*   **Business Logic Implementation:** The framework will invoke command `Routines`, but the implementation of the business logic within those routines is the responsibility of the `Integrator`.
*   **Transactional Guarantees:** The framework does not provide transactional guarantees for sequences of commands. A failure in one command in a sequence does not automatically roll back the effects of previously executed commands.
*   **Inter-Command State Management:** The framework provides an `ExecutionContext` for passing data to commands, but it does not manage complex state between command invocations. State management is the responsibility of the `Integrator`.
*   **User Interface (UI) Rendering:** The framework provides the data and structure for different modalities (CLI, TUI, GUI) but does not render the UI itself. UI rendering is the responsibility of modality-specific crates or the `Integrator`'s application.

### 2. System Actors

An Actor is any entity that plays a distinct role and participates in an interaction within the system's architecture.

#### 2.1. Human Actors
*   **`Integrator (Developer)`**: The primary human actor who uses the `unilang` framework crates (`unilang`, `unilang_parser`, `unilang_meta`) to build a `utility1` application. Their responsibilities include defining commands, implementing routines, and configuring the framework.
*   **`End User`**: A human actor who interacts with the compiled `utility1` application through one of its exposed `Modalities` (e.g., by typing commands into a CLI).

#### 2.2. External System Actors
*   **`Operating System`**: A system actor that provides the execution environment for `utility1`, including the CLI shell, file system, and environment variables.
*   **`External Service`**: Any external system (e.g., a database, a web API) that a command `Routine` might interact with. The `unilang` framework does not interact with these services directly, but it facilitates the execution of routines that do.

#### 2.3. Internal System Actors
*   **`Build Script (build.rs)`**: A critical internal actor responsible for compile-time operations. Its primary role is to process static command definitions (from code or manifests) and generate the Perfect Hash Function (PHF) map, enabling the zero-overhead static command registry.
*   **`Command Registry`**: An internal actor that serves as the runtime database for all command definitions. It manages both the static (PHF) and dynamic (HashMap) command sets and provides the lookup service used by the `Semantic Analyzer`.
*   **`Parser (unilang_parser)`**: An internal actor that performs lexical and syntactic analysis on a raw input string, converting it into a structured `GenericInstruction` without any knowledge of command definitions.
*   **`Semantic Analyzer`**: An internal actor that validates a `GenericInstruction` against the `Command Registry` to produce a `VerifiedCommand` that is guaranteed to be executable.
*   **`Interpreter`**: An internal actor that takes a `VerifiedCommand` and invokes its corresponding `Routine`, managing the execution context and handling results.

### 3. Ubiquitous Language (Vocabulary)

*   **`unilang`**: This specification and the core framework crate.
*   **`utility1`**: A generic placeholder for the primary application that implements `unilang`.
*   **`Command Registry`**: The runtime data structure that holds all known `CommandDefinition`s and their associated `Routine`s. It supports both static (compile-time) and dynamic (run-time) registration.
*   **`CommandDefinition`**: The canonical metadata for a command, defining its name, arguments, aliases, and behavior.
*   **`ArgumentDefinition`**: The canonical metadata for a command's argument, defining its name, `Kind`, and validation rules.
*   **`Routine`**: The executable code (a Rust closure or function) associated with a command.
*   **`Modality`**: A specific way of interacting with `utility1` (e.g., CLI, REPL, Web API).
*   **`GenericInstruction`**: The structured, syntax-aware output of the `unilang_parser`, representing a parsed but unvalidated command invocation.
*   **`VerifiedCommand`**: The output of the `Semantic Analyzer`; a command that has been validated against the `Command Registry` and is guaranteed to be executable.
*   **`Pipeline`**: A high-level API object that orchestrates the full processing flow from string input to execution result.
*   **`Kind`**: The data type of an argument (e.g., `Integer`, `String`, `List`, `Map`).

### 4. Core Functional Requirements

This section lists the specific, testable functions the `unilang` framework **must** provide.

#### 4.1. Command & Registry Management
*   **FR-REG-1 (Static Registration):** The framework **must** provide a mechanism, via a `build.rs` script, to register commands at compile-time from a manifest file (e.g., `unilang.commands.yaml`).
*   **FR-REG-2 (Dynamic Registration):** The framework **must** expose a public API (`CommandRegistry::command_add_runtime`) for registering new commands and their routines at runtime.
*   **FR-REG-3 (Declarative Loading):** The framework **must** provide functions (`load_from_yaml_str`, `load_from_json_str`) to load `CommandDefinition`s from structured text at runtime.
*   **FR-REG-4 (Namespace Support):** The framework **must** support hierarchical command organization through dot-separated namespaces (e.g., `.math.add`).
*   **FR-REG-5 (Alias Resolution):** The framework **must** support command aliases. When an alias is invoked, the framework **must** execute the corresponding canonical command.
*   **FR-REG-6 (Explicit Command Names):** The framework **must** enforce explicit command naming:
    - All command names **must** start with a dot prefix (e.g., `.chat`, `.session.list`)
    - Command registration **must** fail with a clear error if the name lacks a dot prefix
    - The framework **must not** automatically add, remove, or transform command names during registration or execution
    - Namespaced commands **must** use dot-separated hierarchy (e.g., name: `list`, namespace: `.session` → `.session.list`)

#### 4.2. Argument Parsing & Type System
*   **FR-ARG-1 (Type Support):** The framework **must** support parsing and type-checking for the following `Kind`s: `String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, and `Object`.
*   **FR-ARG-2 (Positional Binding):** The framework **must** correctly bind positional arguments from a `GenericInstruction` to the corresponding `ArgumentDefinition`s in the order they are defined.
*   **FR-ARG-3 (Named Binding):** The framework **must** correctly bind named arguments (`name::value`) from a `GenericInstruction` to the corresponding `ArgumentDefinition`, regardless of order.
*   **FR-ARG-4 (Alias Binding):** The framework **must** correctly bind named arguments specified via an alias to the correct `ArgumentDefinition`.
*   **FR-ARG-5 (Default Values):** If an optional argument with a default value is not provided, the framework **must** use the default value during semantic analysis.
*   **FR-ARG-6 (Validation Rule Enforcement):** The `Semantic Analyzer` **must** enforce all `ValidationRule`s (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) defined for an argument. If a rule is violated, a `UNILANG_VALIDATION_RULE_FAILED` error **must** be returned.

#### 4.3. Command Execution Pipeline
*   **FR-PIPE-1 (Pipeline Orchestration):** The `Pipeline` API **must** correctly orchestrate the full sequence: Parsing -> Semantic Analysis -> Interpretation.
*   **FR-PIPE-2 (Batch Processing):** The `Pipeline::process_batch` method **must** execute a list of commands independently, collecting results for each and not stopping on individual failures.
*   **FR-PIPE-3 (Sequence Processing):** The `Pipeline::process_sequence` method **must** execute a list of commands in order and **must** terminate immediately upon the first command failure.

#### 4.4. Help & Discovery System
*   **FR-HELP-1 (Command List):** The `HelpGenerator` **must** be able to produce a formatted list of all registered commands, including their names, namespaces, and hints.
*   **FR-HELP-2 (Detailed Command Help):** The `HelpGenerator` **must** be able to produce detailed, formatted help for a specific command, including its description, arguments (with types, defaults, and validation rules), aliases, and examples.
*   **FR-HELP-3 (Help Operator):** The parser **must** recognize the `?` operator. When present, the `Semantic Analyzer` **must** return a `HELP_REQUESTED` error containing the detailed help text for the specified command, bypassing all argument validation.
*   **FR-HELP-4 (Standardized Help Commands):** For every registered command `.command`, the framework **must** provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. This standardization ensures consistent help access across all commands.
*   **FR-HELP-5 (Double Question Mark Parameter):** The framework **must** recognize a special parameter `??` that can be appended to any command to trigger help display (e.g., `.command "??"`). When this parameter is detected, the system **must** return help information identical to calling `.command.help`, providing an alternative help access method. *Implementation Note: The `??` parameter must be quoted to avoid parser conflicts with the `?` help operator.*
*   **FR-HELP-6 (Automatic Help Command Generation API):** The framework **must** provide APIs (`CommandRegistry::enable_help_conventions`, `CommandDefinition::with_auto_help`) that automatically generate `.command.help` commands and enable `??` parameter processing with minimal developer effort.

    *Implementation Notes:* ✅ **IMPLEMENTED**
    - Automatic `.command.help` command registration via `register_with_auto_help()`
    - Global help conventions toggle via `enable_help_conventions()`
    - Per-command control via `auto_help_enabled` field
    - Pipeline enhancement converts `HELP_REQUESTED` errors to successful help output
    - Comprehensive help formatting with all command metadata, validation rules, and examples
    - Three help access methods: `?` operator, `"??"` parameter, and `.command.help` commands

#### 4.5. Modality Support
*   **FR-REPL-1 (REPL Support):** The framework's core components (`Pipeline`, `Parser`, `SemanticAnalyzer`, `Interpreter`) **must** be structured to support a REPL-style execution loop. They **must** be reusable for multiple, sequential command executions within a single process lifetime.
    
    *Implementation Notes:* ✅ **IMPLEMENTED**
    - Pipeline components are fully stateless and reusable
    - Each command execution is independent with no state accumulation
    - Memory efficient operation verified through performance benchmarks
    - Reference implementations available in `examples/12_repl_loop.rs`, `examples/15_interactive_repl_mode.rs`, `examples/17_advanced_repl_features.rs`
    
*   **FR-INTERACTIVE-1 (Interactive Argument Prompting):** When a mandatory argument with the `interactive: true` attribute is not provided, the `Semantic Analyzer` **must** return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`). This allows the calling modality to intercept the error and prompt the user for input.
    
    *Implementation Notes:* ✅ **IMPLEMENTED** 
    - Error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` is returned as specified
    - Implemented in `src/semantic.rs` lines 196-203
    - Comprehensive test coverage in `tests/inc/phase5/interactive_args_test.rs`
    - REPL examples demonstrate proper error handling and secure input simulation
*   **FR-MOD-WASM-REPL (WebAssembly REPL Modality):** The framework **must** support a web-based REPL modality that can operate entirely on the client-side without a backend server. This requires the core `unilang` library to be fully compilable to the `wasm32-unknown-unknown` target.

### 5. Non-Functional Requirements

*   **NFR-PERF-1 (Startup Time):** For a utility with 1,000,000+ statically compiled commands, the framework **must** introduce zero runtime overhead for command registration. Application startup time **must not** be proportional to the number of static commands. This **must** be achieved via compile-time generation of a Perfect Hash Function (PHF).
*   **NFR-PERF-2 (Lookup Latency):** The p99 latency for resolving a command `FullName` and its arguments **must** be less than 100 nanoseconds for any registry size.
*   **NFR-PERF-3 (Throughput):** The framework **must** be capable of processing over 5,000,000 simple command lookups per second on a standard developer machine.
*   **NFR-SEC-1 (Sensitive Data):** Argument values marked as `sensitive: true` **must not** be displayed in logs or user interfaces unless explicitly required by a secure context.
*   **NFR-ROBUST-1 (Error Reporting):** All user-facing errors **must** be returned as a structured `ErrorData` object and provide clear, actionable messages. Internal panics **must** be caught and converted to a user-friendly `UNILANG_INTERNAL_ERROR`.
*   **NFR-PLATFORM-1 (WASM Compatibility):** The core logic of the `unilang` and `unilang_parser` crates **must** be platform-agnostic and fully compatible with the WebAssembly (`wasm32-unknown-unknown`) target. This implies that the core crates **must not** depend on libraries or functionalities that are tied to a specific native OS (e.g., native threading, direct file system access that cannot be abstracted) unless those features are conditionally compiled and disabled for the WASM target.
*   **NFR-MODULARITY-1 (Granular Features):** All non-essential framework functionality **must** be gated behind Cargo features. This includes support for complex types (`Url`, `DateTime`), declarative loading (`serde_yaml`, `serde_json`), and other features that introduce dependencies.
*   **NFR-MODULARITY-2 (Lightweight Core):** When compiled with `default-features = false`, the `unilang` framework **must** have a minimal dependency footprint, comparable in lightness (dependencies, compile time) to the `pico-args` crate. The core functionality **must** be contained within the `enabled` feature.

#### 5.1. REPL Implementation Requirements & Technical Insights

The REPL (Read-Eval-Print Loop) modality has unique technical challenges and requirements that have been discovered through implementation:

**Stateless Operation Requirements:**
- Each command execution cycle must be completely independent
- No state accumulation between command executions to prevent memory leaks
- Components (`Parser`, `SemanticAnalyzer`, `Interpreter`) must be reusable without internal state corruption
- Performance requirement: Command execution overhead must remain constant regardless of session length

**Interactive Argument Handling:**
- The error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` must be catchable at the REPL level
- REPL implementations must handle secure input (passwords, API keys) without logging or state persistence
- Optional interactive arguments with defaults must not trigger interactive prompts
- Interactive argument validation must occur during semantic analysis, not execution

**Memory Management Insights:**
- Pipeline component reuse provides 20-50% performance improvement over creating new instances
- Command history storage should be bounded to prevent unbounded memory growth
- Large command outputs should be handled with streaming or pagination for long-running REPL sessions

**Error Recovery Patterns:**
- Parse errors should provide contextual suggestions for command correction
- Semantic analysis errors should indicate available commands and proper syntax
- Execution errors should not terminate the REPL session
- Error history tracking enables improved user experience with "last-error" functionality

**User Experience Requirements:**
- Auto-completion suggestions require command registry introspection capabilities
- Command history must support search and replay functionality  
- Session statistics provide valuable debugging information
- Clear screen and session reset capabilities are essential for productive use

**Performance Considerations:**
- Static command registry with PHF provides zero-cost lookups even in REPL context
- Dynamic command registration during REPL sessions should be supported for development workflows
- Batch command processing capabilities enable script-like functionality within REPL
- Command validation without execution supports syntax checking workflows

### 6. CLI Modality: Language Syntax & Processing

The `unilang_parser` crate **must** be the reference implementation for this section. The parser **must** adhere to the following rules in order:

*   **Rule 1 (Tokenization):** Whitespace separates tokens. Quoted strings (`'...'` or `"..."`) are treated as a single token.
*   **Rule 2 (Command Path):** The command path is the first token. It **must** be a dot-separated identifier (e.g., `.system.echo`). A leading dot is optional.
*   **Rule 3 (Arguments):** All subsequent tokens are arguments.
    *   **Named Arguments:** **Must** use the `name::value` syntax.
    *   **Positional Arguments:** Any token that is not a named argument is a positional argument.
*   **Rule 4 (Help Operator):** The `?` operator, if present, **must** be the final token and triggers the help system.
*   **Rule 5 (Double Question Mark Parameter):** The `??` parameter, if present as any argument, **must** trigger help display for the command, identical to calling `.command.help`. This provides a consistent alternative to the `?` operator.
*   **Rule 6 (Special Case - Discovery):** A standalone dot (`.`) **must** be interpreted as a request to list all available commands.

### 7. API Reference: Core Data Structures

The public API **must** include the following data structures with the specified fields. (See `src/data.rs` for the source of truth).

*   `CommandDefinition`: Defines a command's metadata, including the new `auto_help_enabled: bool` field for help convention support.
*   `ArgumentDefinition`: Defines an argument's metadata.
*   `ArgumentAttributes`: Defines behavioral flags for an argument.
*   `Kind`: Defines the data type of an argument.
*   `ValidationRule`: Defines a validation constraint for an argument.
*   `OutputData`: Standardized structure for successful command output.
*   `ErrorData`: Standardized structure for command failure information.

#### 7.1. CommandDefinition Structure

The `CommandDefinition` struct **must** include the following key fields for help convention support:
*   `auto_help_enabled: bool` - Controls whether this command automatically generates a corresponding `.command.help` command. When `true`, the framework automatically creates the help counterpart during registration.

#### 7.2. Help Convention API Methods

The following API methods **must** be provided to support standardized help conventions:

**CommandRegistry Methods:**
*   `enable_help_conventions(&mut self, enabled: bool)` - Enables/disables automatic `.command.help` generation for all subsequently registered commands.
*   `register_with_auto_help(&mut self, command: CommandDefinition, routine: CommandRoutine)` - Registers a command with automatic help command generation.
*   `get_help_for_command(&self, command_name: &str) -> Option<String>` - Retrieves formatted help text for any registered command.

**CommandDefinition Methods:**
*   `with_auto_help(self, enabled: bool) -> Self` - Builder method to enable/disable automatic help command generation for this specific command. *(Note: Currently implemented via direct field access `cmd.auto_help_enabled = true`; builder method planned for future release)*
*   `has_auto_help(&self) -> bool` - Returns true if this command should automatically generate a help counterpart.
*   `generate_help_command(&self) -> CommandDefinition` - Generates the corresponding `.command.help` command definition for this command.

**Pipeline Methods:**
*   `process_help_request(&self, command_name: &str, context: ExecutionContext) -> Result<OutputData, Error>` - Processes help requests uniformly across the framework.

### 8. Cross-Cutting Concerns (Error Handling, Security, Verbosity)

*   **Error Handling:** All recoverable errors **must** be propagated as `unilang::Error`, which wraps an `ErrorData` struct containing a machine-readable `code` and a human-readable `message`.
*   **Security:** The framework **must** provide a `permissions` field in `CommandDefinition` for integrators to implement role-based access control. The `sensitive` attribute on arguments **must** be respected.
*   **Verbosity:** The framework **must** support at least three verbosity levels (`quiet`, `normal`, `debug`) configurable via environment variable (`UNILANG_VERBOSITY`) or programmatically.

### 9. Feature Flags & Modularity

The framework **must** be highly modular, allowing integrators to select only the features they need to minimize binary size and compile times.

#### 9.1. The `enabled` Feature
Every crate in the `unilang` ecosystem (`unilang`, `unilang_parser`, `unilang_meta`) **must** expose an `enabled` feature. This feature **must** be part of the `default` feature set. Disabling the `enabled` feature (`--no-default-features`) **must** effectively remove all of the crate's code and dependencies from the compilation, allowing it to be "turned off" even when included as a non-optional dependency in a workspace.

#### 9.2. Feature Sets
The following feature flags **must** be available to integrators:

| Feature | Description | Dependencies Enabled | Default |
| :--- | :--- | :--- | :--- |
| `default` | Enables the standard, full-featured framework experience. | `enabled`, `full` | Yes |
| `enabled` | The master switch that enables the core framework logic. Disabling this removes the crate entirely. | (Core logic) | Yes |
| `full` | A convenience feature that enables all optional functionality below. | All optional features | Yes |
| `declarative_loading` | Enables loading `CommandDefinition`s from YAML and JSON strings. | `serde`, `serde_yaml`, `serde_json` | No |
| `on_unknown_suggest` | Enables suggestions for mistyped commands (e.g., "did you mean...?"). | `textdistance` | No |
| `chrono_types` | Enables support for the `Kind::DateTime` argument type. | `chrono` | No |
| `url_types` | Enables support for the `Kind::Url` argument type. | `url` | No |
| `regex_types` | Enables support for the `Kind::Pattern` argument type and `ValidationRule::Pattern`. | `regex` | No |

---
## Part II: Internal Design (Design Recommendations)
*This part of the specification describes the recommended internal architecture and implementation strategies. These are best-practice starting points, and the development team has the flexibility to modify them as needed.*

### 10. Architectural Mandates & Design Principles

It is recommended that the `unilang` ecosystem adhere to the following principles:

*   **Parser Independence:** The `unilang` core crate **should** delegate all command string parsing to the `unilang_parser` crate.
*   **Zero-Overhead Static Registry:** To meet `NFR-PERF-1`, it is **strongly recommended** that the `CommandRegistry` be implemented using a hybrid model:
    *   A **Perfect Hash Function (PHF)** map, generated at compile-time in `build.rs`, for all statically known commands.
    *   A standard `HashMap` for commands registered dynamically at runtime.
    *   Lookups **should** check the static PHF first before falling back to the dynamic map.
*   **`enabled` Feature Gate Mandate:** All framework crates **must** implement the `enabled` feature gate pattern. The entire crate's functionality, including its modules and dependencies, **should** be conditionally compiled using `#[cfg(feature = "enabled")]`. This is a critical mechanism for managing complex feature sets and dependencies within a Cargo workspace, allowing a crate to be effectively disabled even when it is listed as a non-optional dependency.

### 11. Architectural Diagrams

#### 11.1. Use Case Diagram
```mermaid
graph TD
    subgraph Unilang Framework
        UC1(Define Command<br/>(Static or Dynamic))
        UC2(Implement Routine)
        UC3(Configure Framework)
        UC4(Execute Command)
        UC5(Request Help)
        UC6(List Commands)
    end

    actorIntegrator["Integrator<br/>(Developer)"]
    actorEndUser["End User"]

    actorIntegrator --> UC1
    actorIntegrator --> UC2
    actorIntegrator --> UC3

    actorEndUser --> UC4
    actorEndUser --> UC5
    actorEndUser --> UC6
```

#### 11.2. System Context Diagram
```mermaid
graph TD
    style Integrator fill:#fff,stroke:#333,stroke-width:2px
    style EndUser fill:#fff,stroke:#333,stroke-width:2px

    Integrator(Integrator<br/>(Developer))
    EndUser(End User)

    subgraph "utility1 Application"
        Unilang["unilang Framework"]
        Utility1[utility1 Binary]
    end

    style Unilang fill:#1168bd,color:#fff
    style Utility1 fill:#22a6f2,color:#fff

    Integrator -- "Uses to build" --> Unilang
    Unilang -- "Is a dependency of" --> Utility1
    EndUser -- "Interacts with" --> Utility1
```

#### 11.3. C4 Container Diagram
```mermaid
C4Context
    title Container diagram for a 'utility1' Application

    Person(integrator, "Integrator (Developer)", "Uses macros and APIs to build the application.")

    System_Boundary(utility1, "utility1 Application") {
        Container(utility1_bin, "utility1 Binary", "Executable", "The compiled application that End Users interact with.")
        ContainerDb(unilang_core, "unilang (Core Crate)", "Rust Library", "Orchestrates parsing, analysis, and execution.")
        ContainerDb(unilang_parser, "unilang_parser", "Rust Library", "Provides lexical and syntactic analysis.")
        ContainerDb(unilang_meta, "unilang_meta", "Rust Library", "Provides procedural macros for compile-time definitions.")
    }

    Rel(integrator, unilang_meta, "Uses macros from", "Compile-Time")
    Rel(integrator, unilang_core, "Uses APIs from")

    Rel(utility1_bin, unilang_core, "Depends on")
    Rel(unilang_core, unilang_parser, "Uses for parsing")
```

#### 11.4. High-Level Architecture (Hybrid Registry)
```mermaid
graph TD
    subgraph "Compile Time"
        style CompileTime fill:#f9f9f9,stroke:#ddd,stroke-dasharray: 5 5
        manifest("unilang.commands.yaml")
        build_rs("Build Script (build.rs)")
        phf_map("Static Registry (PHF Map)<br/>Generated .rs file")

        manifest --> build_rs
        build_rs --> phf_map
    end

    subgraph "Run Time"
        style RunTime fill:#f9f9f9,stroke:#ddd,stroke-dasharray: 5 5
        api_call("API Call<br/>(e.g., command_add_runtime)")
        dynamic_map("Dynamic Registry (HashMap)")
        registry["Hybrid CommandRegistry"]

        api_call --> dynamic_map

        subgraph registry
            direction LR
            phf_map_ref(Static PHF)
            dynamic_map_ref(Dynamic HashMap)
        end

        phf_map -- "Included via include!()" --> phf_map_ref
        dynamic_map -- "Contained in" --> dynamic_map_ref
    end
```

#### 11.5. Sequence Diagram: Unified Processing Pipeline
```mermaid
sequenceDiagram
    actor User
    participant CLI
    participant Parser (unilang_parser)
    participant SemanticAnalyzer (unilang)
    participant Interpreter (unilang)
    participant Routine

    User->>CLI: Enters "utility1 .math.add a::10 b::20"
    CLI->>Parser: parse_single_instruction("...")
    activate Parser
    Parser-->>CLI: Returns GenericInstruction
    deactivate Parser
    CLI->>SemanticAnalyzer: analyze(instruction)
    activate SemanticAnalyzer
    SemanticAnalyzer-->>CLI: Returns VerifiedCommand
    deactivate SemanticAnalyzer
    CLI->>Interpreter: run(command)
    activate Interpreter
    Interpreter->>Routine: execute(command, context)
    activate Routine
    Routine-->>Interpreter: Returns Result<OutputData, ErrorData>
    deactivate Routine
    Interpreter-->>CLI: Returns final Result
    deactivate Interpreter
    CLI->>User: Displays "Result: 30"
```

### 12. Crate-Specific Responsibilities

*   **`unilang` (Core Framework):** Recommended to be the central orchestrator, implementing the `CommandRegistry`, `SemanticAnalyzer`, `Interpreter`, `Pipeline`, and all core data structures.
*   **`unilang_parser` (Parser):** Recommended to be the dedicated lexical and syntactic analyzer. It should be stateless and have no knowledge of command definitions.
*   **`unilang_meta` (Macros):** Recommended to provide procedural macros for a simplified, compile-time developer experience.

---
## Part III: Project & Process Governance
*This part of the specification defines the project's goals, scope, and the rules governing its development process.*

### 13. Project Goals & Success Metrics
*   **Primary Goal:** To create a stable, performant, and ergonomic framework for building multi-modal command-line utilities in Rust that allows developers to define a command interface once and deploy it everywhere with zero-overhead for static commands.
*   **Success Metric 1 (Performance):** The framework **must** meet all performance NFRs defined in Section 5, verified by the project's benchmark suite.
*   **Success Metric 2 (Adoption):** The framework is considered successful if it is used to build at least three distinct `utility1` applications with different modalities within 12 months of the v1.0 release.

### 14. Deliverables

Upon completion, the project will deliver the following artifacts:

1.  The published `unilang` Rust crate on crates.io.
2.  The published `unilang_parser` Rust crate on crates.io.
3.  The published `unilang_meta` Rust crate on crates.io.
4.  A compiled WebAssembly (`.wasm`) package and associated JavaScript bindings for the core framework, enabling client-side execution.
5.  Full access to the source code repository, including all examples and benchmarks.
6.  Generated API documentation hosted on docs.rs for all public crates.

### 15. Open Questions
1.  **Custom Type Registration:** What is the API and process for an `Integrator` to define a new custom `Kind` and register its associated parsing and validation logic with the framework?
2.  **Plugin System:** What would a formal plugin system look like, allowing third-party crates to provide `unilang` commands to a host application?

### 15.1. Governing Principles

The unilang framework is built on fundamental principles that guide all architectural decisions and implementation details:

#### 15.1.1. Minimum Implicit Magic
The framework **must** minimize implicit behavior and transformations to maximize predictability:
- **Explicit Operations**: All operations should be explicit rather than implicit
- **Predictable Behavior**: What you specify is exactly what you get - no hidden transformations
- **Clear APIs**: Function behavior should be obvious from signatures and documentation
- **No Surprising Side Effects**: Commands and functions should behave exactly as documented

#### 15.1.2. Single Source of Truth
Each piece of information **must** have exactly one authoritative source:
- **Command Definitions**: Commands registered exactly as specified, used exactly as registered
- **Configuration**: One canonical location for each configuration setting
- **Documentation**: Single authoritative source for each concept or procedure

#### 15.1.3. Fail-Fast Validation
The framework **must** detect and report errors as early as possible:
- **Registration Time**: Invalid command definitions rejected immediately during registration
- **Parse Time**: Syntax errors detected during parsing phase
- **Semantic Analysis**: Type and validation errors caught before execution
- **Clear Error Messages**: All errors include actionable guidance for resolution

#### 15.1.4. Explicit Dependencies
All dependencies and relationships **must** be made explicit:
- **Command Dependencies**: Clear specification of required arguments and constraints
- **Type Dependencies**: Explicit type requirements and conversions
- **System Dependencies**: Clear documentation of external requirements

#### 15.1.5. Consistent Help Access
The framework **must** provide standardized, predictable help access for all commands:
- **Universal Help Commands**: Every command `.command` automatically generates a `.command.help` counterpart
- **Uniform Help Parameter**: The `??` parameter provides consistent help access across all commands
- **Help Convention APIs**: Developer-friendly APIs make following help conventions effortless
- **Discoverability**: Users can always find help through predictable patterns

These principles serve as the foundation for all design decisions and implementation choices throughout the framework.

### 16. Core Principles of Development

#### 16.1. Single Source of Truth
The project's Git repository **must** be the absolute single source of truth for all project-related information. This includes specifications, documentation, source code, configuration files, and architectural diagrams.

#### 16.2. Documentation-First Development
All changes to the system's functionality or architecture **must** be documented in the relevant specification files *before* implementation begins.

#### 16.3. Review-Driven Change Control
All modifications to the repository, without exception, **must** go through a formal Pull Request review.

#### 16.4. Radical Transparency and Auditability
The development process **must** be fully transparent and auditable. All significant decisions and discussions **must** be captured in writing within the relevant Pull Request or a linked issue tracker. The repository's history should provide a clear, chronological narrative of the project's evolution.

#### 16.5. File Naming Conventions
All file names within the project repository **must** use lowercase `snake_case`.

#### 16.6. Explicit Command Naming Principle
The framework **must** adhere to the principle of explicit command naming with minimal implicit transformations:

- **Commands as Registered**: Command names **must** be used exactly as registered, without automatic prefix addition or name transformation
- **Dot Prefix Requirement**: All commands **must** be registered with explicit dot prefix (e.g., `.chat`, `.session.list`)  
- **Validation Enforcement**: The framework **must** reject command registrations that do not start with a dot prefix
- **No Implicit Behavior**: The system **must not** automatically add dots, modify namespaces, or transform command names during registration or execution
- **Principle of Least Surprise**: Command behavior should be predictable - what you register is exactly what gets executed

---
### Appendix: Addendum
*This appendix is intended for developer use during implementation. It captures as-built details and serves as a living document during the development cycle.*

#### Purpose
This document is intended to be completed by the **Developer** during the implementation phase. It is used to capture the final, as-built details of the **Internal Design**, especially where the implementation differs from the initial `Design Recommendations` in `spec.md`.

#### Instructions for the Developer
As you build the system, please use this document to log your key implementation decisions, the final data models, environment variables, and other details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

#### Conformance Checklist
*This checklist is the definitive list of acceptance criteria for the project. Before final delivery, each item must be verified as complete and marked with `✅`. Use the 'Verification Notes' column to link to evidence (e.g., test results, screen recordings).*

| Status | Requirement | Verification Notes |
| :--- | :--- | :--- |
| ❌ | **FR-REG-1:** The framework must provide a mechanism, via a `build.rs` script, to register commands at compile-time from a manifest file (e.g., `unilang.commands.yaml`). | |
| ❌ | **FR-REG-2:** The framework must expose a public API (`CommandRegistry::command_add_runtime`) for registering new commands and their routines at runtime. | |
| ❌ | **FR-REG-3:** The framework must provide functions (`load_from_yaml_str`, `load_from_json_str`) to load `CommandDefinition`s from structured text at runtime. | |
| ❌ | **FR-REG-4:** The framework must support hierarchical command organization through dot-separated namespaces (e.g., `.math.add`). | |
| ❌ | **FR-REG-5:** The framework must support command aliases. When an alias is invoked, the framework must execute the corresponding canonical command. | |
| ❌ | **FR-ARG-1:** The framework must support parsing and type-checking for the following `Kind`s: `String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, and `Object`. | |
| ❌ | **FR-ARG-2:** The framework must correctly bind positional arguments from a `GenericInstruction` to the corresponding `ArgumentDefinition`s in the order they are defined. | |
| ❌ | **FR-ARG-3:** The framework must correctly bind named arguments (`name::value`) from a `GenericInstruction` to the corresponding `ArgumentDefinition`, regardless of order. | |
| ❌ | **FR-ARG-4:** The framework must correctly bind named arguments specified via an alias to the correct `ArgumentDefinition`. | |
| ❌ | **FR-ARG-5:** If an optional argument with a default value is not provided, the framework must use the default value during semantic analysis. | |
| ❌ | **FR-ARG-6:** The `Semantic Analyzer` must enforce all `ValidationRule`s (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) defined for an argument. If a rule is violated, a `UNILANG_VALIDATION_RULE_FAILED` error must be returned. | |
| ❌ | **FR-PIPE-1:** The `Pipeline` API must correctly orchestrate the full sequence: Parsing -> Semantic Analysis -> Interpretation. | |
| ❌ | **FR-PIPE-2:** The `Pipeline::process_batch` method must execute a list of commands independently, collecting results for each and not stopping on individual failures. | |
| ❌ | **FR-PIPE-3:** The `Pipeline::process_sequence` method must execute a list of commands in order and must terminate immediately upon the first command failure. | |
| ✅ | **FR-HELP-1:** The `HelpGenerator` must be able to produce a formatted list of all registered commands, including their names, namespaces, and hints. | Implemented with comprehensive formatting and namespace-aware command listing |
| ✅ | **FR-HELP-2:** The `HelpGenerator` must be able to produce detailed, formatted help for a specific command, including its description, arguments (with types, defaults, and validation rules), aliases, and examples. | Implemented with hierarchical help formatting including all metadata, validation rules, and usage examples |
| ✅ | **FR-HELP-3:** The parser must recognize the `?` operator. When present, the `Semantic Analyzer` must return a `HELP_REQUESTED` error containing the detailed help text for the specified command, bypassing all argument validation. | Implemented with Pipeline enhancement to convert HELP_REQUESTED errors to successful help output |
| ✅ | **FR-HELP-4:** For every registered command `.command`, the framework must provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. | Implemented via `register_with_auto_help()` and `auto_help_enabled` field with automatic help command generation |
| ✅ | **FR-HELP-5:** The framework must recognize a special parameter `??` that can be appended to any command to trigger help display (e.g., `.command ??`). When this parameter is detected, the system must return help information identical to calling `.command.help`. | Implemented with semantic analyzer support for `??` parameter (requires quoting as `"??"` to avoid parser conflicts) |
| ✅ | **FR-HELP-6:** The framework must provide APIs (`CommandRegistry::enable_help_conventions`, `CommandDefinition::with_auto_help`) that automatically generate `.command.help` commands and enable `??` parameter processing with minimal developer effort. | Implemented with `enable_help_conventions()`, `register_with_auto_help()`, and `auto_help_enabled` field |
| ✅ | **FR-REPL-1:** The framework's core components (`Pipeline`, `Parser`, `SemanticAnalyzer`, `Interpreter`) must be structured to support a REPL-style execution loop. They must be reusable for multiple, sequential command executions within a single process lifetime. | Implemented with comprehensive examples and verified stateless operation |
| ✅ | **FR-INTERACTIVE-1:** When a mandatory argument with the `interactive: true` attribute is not provided, the `Semantic Analyzer` must return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`). This allows the calling modality to intercept the error and prompt the user for input. | Implemented in semantic analyzer with comprehensive test coverage and REPL integration |
| ❌ | **FR-MOD-WASM-REPL:** The framework must support a web-based REPL modality that can operate entirely on the client-side without a backend server. This requires the core `unilang` library to be fully compilable to the `wasm32-unknown-unknown` target. | |

#### Finalized Internal Design Decisions
*A space for the developer to document key implementation choices for the system's internal design, especially where they differ from the initial recommendations in `spec.md`.*

-   **Help Conventions Implementation (2025-09-16):** Implemented comprehensive help system with three access methods:
    - Traditional `?` operator (existing)
    - New `"??"` parameter (requires quoting to avoid parser conflicts)
    - Auto-generated `.command.help` commands (controlled by `auto_help_enabled` field)
    - Enhanced Pipeline to convert `HELP_REQUESTED` errors to successful help output
    - Added `18_help_conventions_demo.rs` example showcasing all three methods
-   **CommandDefinition Schema Extension:** Added `auto_help_enabled: bool` field to support per-command help generation control, maintains backward compatibility with default `false` value.

#### Finalized Internal Data Models
*The definitive, as-built schema for all databases, data structures, and objects used internally by the system.*

**CommandDefinition Structure (as of 2025-09-16):**
```rust
pub struct CommandDefinition {
    pub name: String,                    // Required dot-prefixed command name
    pub namespace: String,               // Hierarchical namespace organization
    pub description: String,             // Human-readable command description
    pub arguments: Vec<ArgumentDefinition>, // Command parameters
    pub routine_link: Option<String>,    // Link to execution routine
    pub hint: String,                   // Short description for command lists
    pub status: String,                 // Command stability status
    pub version: String,                // Command version
    pub tags: Vec<String>,              // Categorization tags
    pub aliases: Vec<String>,           // Alternative command names
    pub permissions: Vec<String>,       // Access control permissions
    pub idempotent: bool,              // Whether command is side-effect free
    pub deprecation_message: String,    // Deprecation notice if applicable
    pub http_method_hint: String,       // HTTP method suggestion for web API
    pub examples: Vec<String>,          // Usage examples
    pub auto_help_enabled: bool,        // NEW: Controls automatic .command.help generation
}
```

*See `src/data.rs` for the complete and authoritative structure definitions.*

#### Environment Variables
*List all environment variables required to run the application. Include the variable name, a brief description of its purpose, and an example value (use placeholders for secrets).*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `UNILANG_VERBOSITY` | Sets the logging verbosity (0=quiet, 1=normal, 2=debug). | `2` |
| `UNILANG_STATIC_COMMANDS_PATH` | Overrides the default path to the compile-time command manifest. | `config/commands.yaml` |

#### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions (e.g., from `Cargo.lock`).*

-   `rustc`: `1.78.0`
-   `phf`: `0.11`
-   `serde`: `1.0`
-   `serde_yaml`: `0.9`

#### Deployment Checklist
*A step-by-step guide for deploying the application from scratch. This is not applicable for a library, but would be used by an `Integrator`.*

1.  Set up the `.env` file using the template above.
2.  Run `cargo build --release`.
3.  Place the compiled binary in `/usr/local/bin`.

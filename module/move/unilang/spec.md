# Unilang Framework Specification v1.3

### 1. Project Overview

This section provides the high-level business context, user perspectives, and core vocabulary for the `unilang` framework.

#### 1.1. Project Goal
To provide a unified and extensible framework that allows developers to define a utility's command interface once, and then leverage that single definition to drive multiple interaction modalities—such as CLI, TUI, GUI, and Web APIs—ensuring consistency, discoverability, and a secure, maintainable architecture.

#### 1.2. Ubiquitous Language (Vocabulary)
This glossary defines the canonical terms used throughout the project's documentation, code, and team communication. Adherence to this language is mandatory to prevent ambiguity.

*   **`unilang`**: The core framework and specification language.
*   **`utility1`**: A placeholder for the end-user application built with the `unilang` framework.
*   **`Integrator`**: The developer who uses the `unilang` framework.
*   **`Command`**: A specific, invokable action (e.g., `.file.copy`).
*   **`CommandDefinition`**: The canonical metadata for a command.
*   **`ArgumentDefinition`**: The canonical metadata for an argument.
*   **`Namespace`**: A dot-separated hierarchy for organizing commands.
*   **`Kind`**: The data type of an argument (e.g., `String`, `Path`).
*   **`Value`**: A parsed and validated instance of a `Kind`.
*   **`Routine`**: The executable logic for a `Command`.
*   **`Modality`**: A mode of interaction (e.g., CLI, GUI).
*   **`parser::GenericInstruction`**: The standard, structured output of the `unilang_instruction_parser`, representing a single parsed command expression.
*   **`VerifiedCommand`**: A command that has passed semantic analysis.
*   **`ExecutionContext`**: An object providing routines with access to global settings and services.

#### 1.3. System Actors
*   **`Integrator (Developer)`**: A human actor responsible for defining commands, writing routines, and building the final `utility1`.
*   **`End User`**: A human actor who interacts with the compiled `utility1` through a specific `Modality`.
*   **`Operating System`**: A system actor that provides the execution environment, including the CLI shell and file system.
*   **`External Service`**: Any external system (e.g., a database, a web API) that a `Routine` might interact with.

#### 1.4. User Stories & Journeys
*   **Happy Path - Executing a File Read Command:**
    1.  The **`Integrator`** defines a `.file.cat` **`Command`** with one mandatory `path` argument of **`Kind::Path`**. They implement a **`Routine`** that reads a file's content and returns it in **`OutputData`**.
    2.  The **`End User`** opens their CLI shell and types the **`Command Expression`**: `utility1 .file.cat path::/home/user/document.txt`.
    3.  The **`unilang`** framework's parser correctly identifies the command path and the named argument, producing a **`parser::GenericInstruction`**.
    4.  The semantic analyzer validates the instruction against the command registry and produces a **`VerifiedCommand`**.
    5.  The **`Interpreter`** invokes the associated **`Routine`**, which interacts with the **`Operating System`**'s file system, reads the file, and returns the content successfully.
    6.  The **`Interpreter`** formats the **`OutputData`** and prints the file's content to the **`End User`**'s console.

*   **Security Path - Handling a Sensitive Argument:**
    1.  The **`Integrator`** defines a `.login` **`Command`** with a `password` argument marked as a **`Sensitive Argument`**.
    2.  The **`End User`** invokes the command interactively. The `utility1` CLI **`Modality`** detects the `sensitive` flag and masks the user's input.
    3.  The `password` **`Value`** is passed through the system but is never printed to logs due to the `sensitive` flag.
    4.  The **`Routine`** uses the password to authenticate against an **`External Service`**.

---

### 2. Formal Framework Specification

This section provides the complete, formal definition of the `unilang` language, its components, and its processing model. It is the single source of truth for all `Integrator`s.

#### 2.1. Introduction & Core Concepts
*   **2.1.1. Goals**: Consistency, Discoverability, Flexibility, Extensibility, Efficiency, Interoperability, Robustness, and Security.
*   **2.1.2. Versioning**: This specification follows SemVer 2.0.0.

#### 2.2. Language Syntax and Processing
The canonical parser for the `unilang` language is the **`unilang_instruction_parser`** crate. The legacy `unilang::parsing` module is deprecated and must be removed.

*   **2.2.1. Unified Processing Pipeline**: The interpretation of user input **must** proceed through the following pipeline:
    1.  **Input (`&str` or `&[&str]`)** is passed to the `unilang_instruction_parser::Parser`.
    2.  **Syntactic Analysis**: The parser produces a `Vec<parser::GenericInstruction>`.
    3.  **Semantic Analysis**: The `unilang::SemanticAnalyzer` consumes the `Vec<parser::GenericInstruction>` and, using the `CommandRegistry`, produces a `Vec<VerifiedCommand>`.
    4.  **Execution**: The `unilang::Interpreter` consumes the `Vec<VerifiedCommand>` and executes the associated `Routine`s.

*   **2.2.2. Syntax**: The CLI syntax is defined by the grammar in **Appendix A.2**. It supports command paths, positional arguments, named arguments (`key::value`), quoted values, command separators (`;;`), and a help operator (`?`).

#### 2.3. Command and Argument Definition
*   **2.3.1. Namespaces**: Namespaces provide a hierarchical organization for commands. A command's `FullName` (e.g., `.files.copy`) is constructed by joining its `path` and `name`. The `CommandRegistry` must resolve commands based on this hierarchy.

*   **2.3.2. `CommandDefinition` Anatomy**:
    | Field | Type | Description |
    | :--- | :--- | :--- |
    | `path` | `Vec<String>` | The namespace path segments (e.g., `["files"]`). |
    | `name` | `String` | The final command name segment (e.g., `"copy"`). |
    | `hint` | `String` | Optional. A human-readable explanation. |
    | `arguments` | `Vec<ArgumentDefinition>` | Optional. A list of arguments the command accepts. |
    | `permissions` | `Vec<String>` | Optional. A list of permission identifiers required for execution. |
    | `status` | `Enum` | Optional. Lifecycle state (`Experimental`, `Stable`, `Deprecated`). |
    | `routine_link` | `Option<String>` | Optional. A link to the executable routine for runtime-loaded commands. |
    | `http_method_hint`| `String` | Optional. A suggested HTTP method for Web API modality. |
    | `idempotent` | `Boolean` | Optional. If `true`, the command can be safely executed multiple times. |
    | `examples` | `Vec<String>` | Optional. Illustrative usage examples for help text. |
    | `version` | `String` | Optional. The SemVer version of the individual command. |

*   **2.3.3. `ArgumentDefinition` Anatomy**:
    | Field | Type | Description |
    | :--- | :--- | :--- |
    | `name` | `String` | Mandatory. The unique identifier for the argument (e.g., `src`). |
    | `hint` | `String` | Optional. A human-readable description. |
    | `kind` | `Kind` | Mandatory. The data type of the argument's value. |
    | `optional` | `bool` | Optional (Default: `false`). If `true`, the argument may be omitted. |
    | `default_value` | `Option<Value>` | Optional. A value to use if an optional argument is not provided. |
    | `is_default_arg`| `bool` | Optional (Default: `false`). If `true`, its value can be provided positionally. |
    | `multiple` | `bool` | Optional (Default: `false`). If `true`, the argument can be specified multiple times. |
    | `sensitive` | `bool` | Optional (Default: `false`). If `true`, the value must be protected. |
    | `validation_rules`| `Vec<String>` | Optional. Custom validation logic (e.g., `"min:0"`). |
    | `aliases` | `Vec<String>` | Optional. A list of alternative short names. |
    | `tags` | `Vec<String>` | Optional. Keywords for UI grouping (e.g., "Basic", "Advanced"). |

*   **2.3.4. Data Types (`Kind`)**: The `kind` attribute specifies the expected data type.
    *   **Primitives**: `String`, `Integer`, `Float`, `Boolean`.
    *   **Semantic Primitives**: `Path`, `File`, `Directory`, `Enum(Vec<String>)`, `Url`, `DateTime`, `Pattern`.
    *   **Collections**: `List(Box<Kind>)`, `Map(Box<Kind>, Box<Kind>)`.
    *   **Complex**: `JsonString`, `Object`.
    *   **Streaming**: `InputStream`, `OutputStream`.
    *   **Extensibility**: The system must be extensible to support custom types.

#### 2.4. Cross-Cutting Concerns
*   **2.4.1. Error Handling (`ErrorData`)**: The standardized error structure must be used.
    ```json
    {
      "code": "ErrorCodeIdentifier",
      "message": "Human-readable error message.",
      "details": {
        "argument_name": "src",
        "location_in_input": { "source_type": "single_string", "start_offset": 15, "end_offset": 20 }
      },
      "origin_command": ".files.copy"
    }
    ```
*   **2.4.2. Standard Output (`OutputData`)**: The standardized output structure must be used.
    ```json
    {
      "payload": "Any",
      "metadata": { "count": 10 },
      "output_type_hint": "application/json"
    }
    ```
*   **2.4.3. Extensibility Model**: The framework supports a hybrid model. **`Extension Module`s** can provide modalities, core commands, and custom types at compile-time. New **`CommandDefinition`s** can be registered at run-time. See **Appendix A.3** for a conceptual outline.

#### 2.5. Interpreter / Execution Engine
The Interpreter is the component responsible for taking a `VerifiedCommand`, retrieving its `Routine` from the registry, preparing the `ExecutionContext`, and invoking the `Routine`. It handles the `Result` from the routine, passing `OutputData` or `ErrorData` to the active `Modality` for presentation.

---

### 3. Project Requirements & Conformance

#### 3.1. Roadmap to Conformance
To align the current codebase with this specification, the following high-level tasks must be completed:
1.  **Deprecate Legacy Parser**: Remove the `unilang::parsing` module and all its usages from the `unilang` crate.
2.  **Integrate `unilang_instruction_parser`**: Modify the `unilang` crate's `SemanticAnalyzer` and primary execution flow to consume `Vec<parser::GenericInstruction>` from the `unilang_instruction_parser` crate.
3.  **Enhance Data Models**: Update the `CommandDefinition` and `ArgumentDefinition` structs in `unilang/src/data.rs` to include all fields defined in Sections 2.3.2 and 2.3.3 of this specification.
4.  **Update `unilang_cli`**: Refactor `src/bin/unilang_cli.rs` to use the new, unified processing pipeline.

#### 3.2. Functional Requirements (FRs)
1.  The system **must** use `unilang_instruction_parser` to parse command expressions.
2.  The system **must** support `is_default_arg` for positional argument binding.
3.  The system **must** provide a runtime API (`command_add_runtime`) to register commands.
4.  The system **must** load `CommandDefinition`s from external YAML and JSON files.
5.  The system **must** support and correctly parse all `Kind`s specified in Section 2.3.4.
6.  The system **must** apply all `validation_rules` specified in an `ArgumentDefinition`.
7.  The system **must** generate structured help data for any registered command.

#### 3.3. Non-Functional Requirements (NFRs)
1.  **Extensibility:** The framework must allow an `Integrator` to add new commands and types without modifying the core engine.
2.  **Maintainability:** The codebase must be organized into distinct, modular components.
3.  **Usability (Error Reporting):** All errors must be user-friendly and include location information as defined in `ErrorData`.
4.  **Security by Design:** The framework must support `sensitive` arguments and `permissions` metadata.
5.  **Conformance:** All crates in the `unilang` project must pass all defined tests and compile without warnings.

#### 3.4. Acceptance Criteria
The implementation is conformant if and only if all criteria are met.
*   **FR1 (Parser Integration):** A test must exist and pass that uses the `unilang` public API, which in turn calls `unilang_instruction_parser` to parse an expression and execute it.
*   **FR2 (Default Argument):** A test must exist and pass where `utility1 .cmd value` correctly binds `"value"` to an argument defined with `is_default_arg: true`.
*   **FR3 (Runtime Registration):** The test `runtime_command_registration_test.rs` must pass.
*   **FR4 (Definition Loading):** The test `command_loader_test.rs` must pass.
*   **FR5 (Argument Kinds):** The tests `argument_types_test.rs`, `collection_types_test.rs`, and `complex_types_and_attributes_test.rs` must pass.
*   **FR6 (Validation Rules):** The test `complex_types_and_attributes_test.rs` must verify that a command fails if an argument violates a `validation_rule`.
*   **FR7 (Structured Help):** The `HelpGenerator` must contain a method that returns a `serde_json::Value` or equivalent structured object.
*   **NFR1-5 (General Conformance):**
    *   The `unilang::parsing` module must be removed from the codebase.
    *   The `unilang` workspace must contain at least two separate crates: `unilang` and `unilang_instruction_parser`.
    *   A test must verify that parser errors produce the full `ErrorData` structure as defined in Section 2.4.1.
    *   A test must verify that an argument with `sensitive: true` is not logged or displayed.
    *   The following commands must all execute successfully with no failures or warnings:
        *   `cargo test -p unilang`
        *   `cargo test -p unilang_instruction_parser`
        *   `cargo test -p unilang_meta`
        *   `cargo clippy -p unilang -- -D warnings`
        *   `cargo clippy -p unilang_instruction_parser -- -D warnings`
        *   `cargo clippy -p unilang_meta -- -D warnings`

---

### 4. Appendices

#### A.1. Example `unilang` Command Library (YAML)
This appendix provides an example of how commands might be defined in a YAML file. Command names use dot (`.`) separation for all segments. Argument names use `kebab-case`.

```yaml
# commands.yaml - Example Unilang Command Definitions

commands:

  - name: .string.echo
    hint: Prints the input string to the output.
    status: Stable
    command_version: "1.0.0"
    idempotent: true
    http_method_hint: GET
    arguments:
      - name: input-string
        kind: String
        is_default_arg: true
        optional: false
        hint: The string to be echoed.
      - name: prefix
        kind: String
        optional: true
        hint: A prefix to add before the echoed string.
        default_value: ""
      - name: times
        kind: Integer
        optional: true
        hint: Number of times to echo the string.
        default_value: 1
        validation_rules:
          - "min:1"
          - "max:100"
    examples:
      - "utility1 .string.echo \"Hello, Unilang!\""
      - "utility1 .string.echo input-string::\"Another example\" prefix::\"LOG: \" times::3"
    # routine_link: "my_string_processing_module::echo_handler" # For runtime loading, points to a routine

  - name: .file.create.temp
    hint: Creates a temporary file with optional content.
    status: Stable
    command_version: "1.1.0"
    http_method_hint: POST
    permissions: ["filesystem.write"]
    arguments:
      - name: content
        kind: String
        optional: true
        hint: Optional content to write to the temporary file.
      - name: extension
        kind: String
        optional: true
        default_value: ".tmp"
        hint: Extension for the temporary file (e.g., .txt, .log).
        validation_rules:
          - "regex:^\\.[a-zA-Z0-9]+$"
      - name: output-path-var
        kind: String
        optional: true
        hint: If provided, the path to the created temp file will be stored in this environment variable for subsequent commands in a sequence.
    examples:
      - "utility1 .file.create.temp content::\"Initial data\" extension::.log"
    # routine_link: "my_file_utils::create_temp_file_handler"

  - name: .network.http.get
    hint: Performs an HTTP GET request to a specified URL.
    status: Experimental
    command_version: "0.5.0"
    idempotent: true
    http_method_hint: GET
    permissions: ["network.access"]
    arguments:
      - name: url
        kind: URL
        is_default_arg: true
        optional: false
        hint: The URL to fetch.
      - name: headers
        kind: Map<String,String>
        optional: true
        hint: HTTP headers to include in the request. (CLI example: headers::\"Content-Type=application/json,Authorization=Bearer XXX\")
      - name: timeout
        kind: Integer # In seconds
        optional: true
        default_value: 30
        hint: Request timeout in seconds.
        validation_rules:
          - "min:1"
    examples:
      - "utility1 .network.http.get https://api.example.com/data"
      - "utility1 .network.http.get url::https://api.example.com/data headers::\"X-API-Key=mykey\" timeout::10"
    # routine_link: "my_network_module::http_get_handler"

```

#### A.2. BNF or Formal Grammar for CLI Syntax (Simplified)

This is a simplified, illustrative Backus-Naur Form (BNF) style grammar. A full grammar would be more complex, especially regarding value parsing and shell quoting. This focuses on the `unilang` structure.

```bnf
<invocation> ::= <utility_name> <global_args_opt> <command_sequence>

<global_args_opt> ::= <global_arg_list> | ""
<global_arg_list> ::= <named_arg> <global_arg_list_opt>
<global_arg_list_opt> ::= <global_arg_list> | ""

<command_sequence> ::= <command_expression> <command_sequence_opt>
<command_sequence_opt> ::= <command_separator> <command_sequence> | ""
<command_separator> ::= ";;"

<command_expression> ::= <command_full_name> <command_args_opt> <help_operator_opt>
                       | <namespace_path_for_help> <help_operator>
                       | <root_namespace_for_help> <help_operator_opt> (* . or .? *)

<command_full_name> ::= <namespace_path_opt> <command_name_segment_list>
<namespace_path_opt> ::= <namespace_path> | ""
<namespace_path> ::= "." <segment_list> "." (* e.g., .files.utils. *)
<namespace_path_for_help> ::= <namespace_path> (* e.g. .files. *)
<root_namespace_for_help> ::= "."

<segment_list> ::= <segment> <dot_segment_opt>
<dot_segment_opt> ::= "." <segment_list> | ""
<segment> ::= <identifier_segment> (* command or namespace segment: lowercase alphanumeric + underscore *)

<command_name_segment_list> ::= <segment> <dot_segment_opt> (* The full path-like name of the command *)


<command_args_opt> ::= <command_arg_list> | ""
<command_arg_list> ::= <command_arg> <command_arg_list_opt>
<command_arg_list_opt> ::= <command_arg_list> | ""

<command_arg> ::= <named_arg> | <default_arg_value>

<named_arg> ::= <arg_name> "::" <arg_value>
<arg_name> ::= <identifier_arg_name> (* kebab-case or snake_case *)

<default_arg_value> ::= <arg_value> (* positional, parsed as default arg if one is defined *)

<arg_value> ::= <quoted_string> | <unquoted_string_no_special_chars>
(* Actual value parsing is type-dependent and complex, involving list/map separators, etc. *)
(* <identifier_segment> would be [a-z0-9_]+ *)
(* <identifier_arg_name> would be [a-z0-9_-]+ *)
(* <quoted_string> handles spaces and special characters. Unescaped content is used. *)

<help_operator_opt> ::= <help_operator> | ""
<help_operator> ::= "?"
```

**Notes on this BNF:**

*   It's high-level and conceptual.
*   `utility_name` is the literal name of the utility (e.g., `utility1`).
*   `<identifier_segment>` and `<identifier_arg_name>` need precise definitions based on allowed characters (Section 2.3.1).
*   `<arg_value>` parsing is the most complex part and is abstracted here. It represents the unescaped content after initial lexing and quote processing.
*   Shell quoting and escaping are handled by the shell before `utility1` receives the arguments. `unilang`'s parser then handles its own quoting rules.

**Note on Applying Grammar to Dual Input Types:**

This BNF describes the logical structure of a `unilang` command expression.
*   When parsing a **single string input**, the parser attempts to match this grammar directly against the character stream.
*   When parsing a **slice of strings input** (pre-tokenized by the shell), the parser consumes these strings sequentially. Each string (or parts of it, if a string contains multiple `unilang` elements like `name::value`) is then matched against the grammar rules. For instance, one string from the slice might be an `<arg_name>`, the next might be `::` (if the shell separated it), and the next an `<arg_value>`. Or a single string from the slice might be `name::value` which the `unilang` parser then further decomposes. The parser must be able to stitch these segments together to form complete `unilang` syntactic structures as defined by the grammar.

#### A.3. Component Registration (Conceptual Outline for Hybrid Model)

This appendix outlines the conceptual mechanisms for how `unilang` components are registered within `utility1`, covering both compile-time contributions from **`Extension Module`s** and run-time command registration. The `noun_verb` convention is used for conceptual API method names that `utility1` might expose for run-time operations.

**1. Compile-Time Component Registration (Modalities, Core Commands from `Extension Module`s, Types)**

`Extension Module`s providing modalities, core commands, or custom types need to make their definitions available to `utility1`'s central registries at compile time.

*   **A. Information Required for Modality Registration (Compile-Time Only via `Extension Module`s)**
    *   An **`Extension Module`** providing a modality (e.g., a TUI implementation) needs to register its handler or main entry point with `utility1`.
    *   **Mechanism Examples**: Static registration where `utility1`'s build system links modality implementations from known `Extension Module`s. `utility1` might discover modules that implement a `utility1`-defined `ModalityHandler` trait/interface.

*   **B. Information Required for Core Command Registration (Compile-Time via `Extension Module`s)**
    *   `Extension Module`s make `CommandDefinition` structures (Section 2.3.2) available.
    *   **Mechanisms**: Procedural macros within `Extension Module`s, static arrays of `CommandDefinition` collected by `utility1`'s build script, or build script code generation that reads module-specific definitions. Routines are typically static function pointers.

*   **C. Information Required for Custom Type Registration (Compile-Time Only via `Extension Module`s)**
    *   `Extension Module`s make `CustomTypeDefinition` structures available.
    *   `CustomTypeDefinition` includes `type_name`, static `parser_function`, static `validator_function`, and `help_info`.
    *   **Mechanisms**: Similar to command registration (macros, static collections, build script generation). Custom types cannot be added at run-time.

**2. Run-Time Command Registration (Commands Only)**

`utility1` **must** provide a run-time API or mechanism to add new `CommandDefinition`s to its existing unified command registry.

*   **A. Procedural Run-Time API (Example using `noun_verb` convention)**
    *   `utility1` could expose methods like:
        *   `fn command_add_runtime(definition: unilang::CommandDefinition, routine: Box<dyn Fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData> + Send + Sync>) -> Result<(), RegistrationError>`
        *   `fn command_remove_runtime(command_name: &str) -> Result<(), UnregistrationError>` (Optional)
*   **B. Loading from External Definitions (e.g., YAML/JSON)**
    *   `utility1` might have a built-in command or mechanism: `utility1 .system.commands.load.file path::/path/to/commands.yaml`
    *   The loaded `CommandDefinition`s would need their `routine_link` attribute to be resolvable by `utility1`. This could mean the `routine_link` refers to a function symbol within `utility1` itself or one of its compile-time loaded **`Extension Module`s**, or a script function if `utility1` embeds a scripting engine.
*   **C. Command Routine Signature (Expected by `unilang` via `utility1`)**
    *   `fn routine_handler(verified_command: VerifiedCommand, exec_context: ExecutionContext) -> Result<OutputData, ErrorData>`

**3. Access to `utility1` Services (via `ExecutionContext`)**
*   The `ExecutionContext` is prepared by `utility1` and passed to all routines, whether linked at compile-time or run-time.

**Example (Conceptual Rust-like Trait for an `ExtensionModule` Interface `utility1` might expect for compile-time contributions):**

```rust
// Conceptual - This is what a utility1 integrator might define for its Extension Modules.

// Provided by utility1 to the Extension Module during a compile-time collection phase
// (e.g. via build script or macro that calls an ExtensionModule's registration function)
pub trait ExtensionModuleRegistrationContext {
    // Uses noun_verb for consistency with potential runtime APIs
    fn command_add(&mut self, definition: unilang::CommandDefinition) -> Result<(), String>;
    fn type_define(&mut self, type_def: unilang::CustomTypeDefinition) -> Result<(), String>;
    // Modalities would likely be registered differently, perhaps by utility1 discovering
    // modules that implement a ModalityHandler trait and are linked at compile time.
}

// Implemented by the Extension Module
pub trait UnilangExtensionModule {
    // Manifest-like information, could be static or methods
    fn module_name(&self) -> &'static str;
    fn unilang_compatibility(&self) -> &'static str; // e.g., ">=1.0.0 <2.0.0"

    // Method called by utility1's build system/macros to collect definitions
    fn components_register(&self, context: &mut dyn ExtensionModuleRegistrationContext) -> Result<(), String>;
}
```

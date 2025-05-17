## Unilang Specification

**Version:** 1.0.0
**Project:** (Applicable to any utility, e.g., `utility1`)

---

### 0. Introduction & Core Concepts

#### 0.1. Goals of `unilang`

`unilang` provides a unified way to define command-line utility interfaces once, automatically enabling consistent interaction across multiple modalities such as CLI, GUI, TUI, AUI, and Web APIs.

The core goals of `unilang` are:

1.  **Consistency:** A single way to define commands and their arguments, regardless of how they are presented or invoked.
2.  **Discoverability:** Easy ways for users and systems to find available commands and understand their usage.
3.  **Flexibility:** Support for various methods of command definition (compile-time, run-time, declarative, procedural).
4.  **Extensibility:** Provide structures that enable a `utility1` integrator to build an extensible system with compile-time `Extension Module`s and run-time command registration.
5.  **Efficiency:** Support for efficient parsing and command dispatch, with potential for compile-time optimizations.
6.  **Interoperability:** Standardized representation for commands, enabling integration with other tools or web services, including auto-generation of WEB endpoints.
7.  **Robustness:** Clear error handling and validation mechanisms.
8.  **Security:** Provide a framework for defining and enforcing secure command execution, which `utility1` can leverage.

#### 0.2. Key Terminology (Glossary)

*   **`unilang`**: This specification; the language defining how commands, arguments, and interactions are structured.
*   **`utility1`**: A generic placeholder for the primary utility or application that implements and interprets `unilang`. The actual name will vary depending on the specific tool. The developer of `utility1` is referred to as the "integrator."
*   **Command**: A specific action or operation that can be invoked (e.g., `.files.copy`).
*   **Command Definition**: The complete specification of a command, including its name, arguments, routine, and other attributes, as defined by `unilang`.
*   **Namespace**: A dot-separated hierarchical structure to organize commands (e.g., `.files.`, `.network.`). The root namespace is denoted by `.`.
*   **Argument**: A parameter that a command accepts to modify its behavior or provide data.
*   **Argument Definition**: The specification of an argument, including its name, type (`kind`), optionality, etc., as defined by `unilang`.
*   **Argument Value**: The actual data provided for an argument during command invocation. After parsing, this represents the unescaped content.
*   **Routine (Handler Function)**: The executable code associated with a command that performs its logic. Its signature is defined by `unilang` expectations.
*   **Modality**: A specific way of interacting with `utility1` using `unilang` (e.g., CLI, GUI, WEB Endpoint).
*   **Command Expression (CLI)**: The textual representation of a command invocation in the CLI, as defined by `unilang`.
*   **Generic Instruction**: An intermediate representation of a command parsed from input, before semantic analysis and binding to a `CommandDefinition`.
*   **`VerifiedCommand`**: An internal representation of a command ready for execution, with all arguments parsed, validated, and typed according to `unilang` rules.
*   **Type (`kind`)**: The data type of an argument (e.g., `String`, `Integer`, `Path`), as defined or extended within the `unilang` framework.
*   **`Extension Module`**: A compile-time module or crate that provides `unilang`-compatible components like modalities, core commands, or custom types to `utility1`.
*   **Global Argument**: An argument processed by `utility1` itself to configure its behavior for the current invocation, distinct from command-specific arguments but using the same `unilang` `key::value` syntax.
*   **`ExecutionContext`**: An object, whose content is largely defined by `utility1`, passed to command routines, providing access to global settings, configuration, and `utility1`-level services.
*   **`OutputData`**: A `unilang`-defined structured object representing the successful result of a command.
*   **`ErrorData`**: A `unilang`-defined structured object representing an error that occurred during processing or execution.
*   **Interpreter (Execution Engine)**: The component within `utility1` that executes a `VerifiedCommand`.

#### 0.3. Versioning Strategy (for `unilang` spec)

This `unilang` specification document will follow Semantic Versioning (SemVer 2.0.0).
*   **MAJOR** version when incompatible API changes are made to the core `unilang` structure.
*   **MINOR** version when functionality is added in a backward-compatible manner.
*   **PATCH** version when backward-compatible bug fixes are made to the specification.

Individual commands defined using `unilang` can also have their own versions (see Section 2.1.2).

---

### 1. Language Syntax, Structure, and Processing (CLI)

`unilang` commands are primarily invoked via a `utility1` in a CLI context. The general structure of an invocation is:

`utility1 [global_argument...] [command_expression] [;; command_expression ...]`

This input string might be processed by `utility1` directly, or `utility1` might receive arguments already somewhat tokenized by the invoking shell (e.g., as a list of strings). The `unilang` processing phases described below must be robust to both scenarios, applying `unilang`-specific parsing rules.

The processing of this CLI input occurs in distinct phases:

#### 1.1. CLI Input Processing Phases

The interpretation of a `unilang` CLI string by `utility1` **must** proceed through the following conceptual phases:

1.  **Phase 1: Lexical and Syntactic Analysis (String to Generic Instructions)**
    *   **Input Handling:** The parser must be capable of consuming input either as a single, continuous string or as a sequence of pre-tokenized string segments (e.g., arguments from `std::env::args()`). An internal input abstraction is recommended.
    *   **Lexical Analysis (Lexing):** Whether as a distinct step or integrated into parsing, this stage identifies fundamental `unilang` symbols.
        *   If input is a single string, this involves tokenizing the raw string.
        *   If input is a sequence of strings, lexical analysis applies *within* each string to handle `unilang`-specific quoting, escapes, and to identify `unilang` operators (like `::`, `;;`, `?`) that might be part of or adjacent to these string segments.
    *   **Syntactic Analysis (Parsing):** The (potentially abstracted) token stream is parsed against the `unilang` grammar (see Appendix A.2) to build a sequence of "Generic Instructions."
        *   A **Generic Instruction** at this stage represents a potential command invocation or a help request. It contains:
            *   The raw, unresolved command name string (e.g., `".files.copy"`).
            *   A list of raw argument values, distinguishing between potential positional (default) arguments and named arguments (still as `key_string::value_string` pairs). These values should be stored as string slices (`&str`) referencing the original input if possible, to minimize allocations. The content of these values after parsing represents the unescaped string.
            *   Flags indicating a help request (`?`).
            *   Information about command separators (` ;; `) to delineate multiple Generic Instructions.
        *   This phase **does not** require any knowledge of defined commands, their arguments, or types. It only validates the syntactic structure of the input according to `unilang` rules.
        *   Global arguments (Section 1.2) are also identified and separated at this stage.
        *   The parser should aim to track location information (e.g., byte offset in a single string, or segment index and offset within a segment for pre-tokenized input) to aid in error reporting.
        *   Input that is empty or contains only whitespace (after initial global whitespace skipping) should result in an empty list of Generic Instructions, not an error.

2.  **Phase 2: Semantic Analysis and Command Binding (Generic Instructions to `VerifiedCommand`)**
    *   Each Generic Instruction is processed against `utility1`'s **Unified Command Registry** (Section 2.4).
    *   **Command Resolution:** The raw command name from the Generic Instruction is resolved to a specific `CommandDefinition`. If not found, an error (`UNILANG_COMMAND_NOT_FOUND`) is generated.
    *   **Argument Binding & Typing:**
        *   Raw argument values from the Generic Instruction are mapped to the `ArgumentDefinition`s of the resolved command.
        *   Positional values are assigned to the `is_default_arg`.
        *   Named arguments are matched by name/alias.
        *   Values are parsed and validated against their specified `kind` and `validation_rules`.
        *   `optional` and `default_value` attributes are applied.
    *   This phase transforms a Generic Instruction into a **`VerifiedCommand`** object (Section 0.2), which is a fully typed and validated representation of the command to be executed. If any semantic errors occur (missing mandatory arguments, type mismatches, validation failures), appropriate `ErrorData` is generated.
    *   Help requests (`?`) are typically handled at this stage by generating help output based on the resolved command or namespace definition, often bypassing `VerifiedCommand` creation for execution.

3.  **Phase 3: Verification and Optimization (Optional)**
    *   Before execution, `utility1` **may** perform additional verification or optimization steps on the `VerifiedCommand` or a sequence of them.
    *   This could include:
        *   Cross-command validation for sequences.
        *   Pre-fetching resources.
        *   Instruction reordering or "inlining" for common, performance-sensitive command patterns (an advanced optimization).
    *   This phase is not strictly mandated by `unilang` but is a point where an integrator can add advanced logic.

4.  **Phase 4: Execution**
    *   The `VerifiedCommand` (or a sequence of them) is passed to the **Interpreter / Execution Engine** (Section 5) to be acted upon.

#### 1.2. Global Arguments

*   Global arguments are processed by `utility1` to control its behavior for the current invocation before any specific `command_expression` is processed (typically during or just after Phase 1 of CLI Input Processing).
*   They use the same `key::value` syntax as command arguments (e.g., `output_format::json`, `log_level::debug`).
*   The set of available global arguments is defined by `utility1` itself.
*   These are not part of a specific command's definition but are recognized by the `utility1` parser at the top level.
*   **Discovery of Global Arguments**: `utility1` implementations **must** provide `utility1 .system.globals ?` which outputs a structured description (see Section 3.2.6) of available global arguments, their purpose, types, default values, and status (e.g., Stable, Deprecated). `utility1` should issue warnings when deprecated global arguments are used.
*   **Examples of potential Global Arguments:**
    *   `output_format::format` (e.g., `output_format::json`, `output_format::yaml`, `output_format::table`) - Controls default output format for commands in the invocation.
    *   `log_level::level` (e.g., `log_level::debug`) - Sets the logging verbosity for the current invocation.
    *   `locale::<code>` (e.g., `locale::fr-FR`) - Suggests a localization for `utility1`'s output for the current invocation.
    *   `config_file::path/to/file.toml` - Specifies an alternative configuration file for this invocation.
    *   `on_error::policy` (e.g., `on_error::stop` (default), `on_error::continue`) - Governs behavior for command sequences.

#### 1.3. Command Expression

A `command_expression` (the input to Phase 1 processing after global arguments are handled) can be one of the following:

*   **Full Command Invocation:** `[namespace_path.]command_name [argument_value...] [named_argument...]`
*   **Help/Introspection Request:** `[namespace_path.][command_name] ?` or `[namespace_path.]?`

#### 1.4. Components of a Command Expression

*   **`namespace_path`**: A dot-separated path indicating a module or category of commands (e.g., `.files.`, `.network.`).
    *   A single dot `.` refers to the root namespace.
*   **`command_name`**: The specific action to be performed (e.g., `copy`, `delete`, `list`). This is the final segment of the command's `FullName`.
*   **`argument_value`**: A value provided to the command. After parsing, this represents the unescaped content of the value.
    *   **Default Argument Value**: If a command defines a default argument, its value can be provided without its name. It's typically the first unnamed value after the `command_name`.
    *   **Named Argument**: `argument_name::value` or `argument_name::"value with spaces"`.
        *   `argument_name`: The identifier for the argument.
        *   `::`: The key-value separator.
        *   `value`: The value assigned to the argument.
            *   **Single String Input:** Values with spaces or special `unilang` characters (like `;;`, `::`, `?` if not intended as operators) **must** be quoted using single or double quotes (e.g., `"some path/with space"`, `'value with :: literal'`). Unquoted spaces in a single string input will typically cause the value to be treated as multiple distinct tokens by the initial lexing stage. Standard shell quoting rules might apply first, then `unilang`'s parser re-evaluates quotes for its own syntax.
            *   **Slice of Strings Input:** If `utility1` receives pre-tokenized arguments, each string segment is a potential value. If such a segment itself contains `unilang` quotes (e.g., a segment is literally `"foo bar"` including the quotes), the `unilang` parser must still process these quotes to extract the actual content (`foo bar`). Escaped quotes (`\"`, `\'`) within `unilang`-quoted strings are treated as literal characters.
*   **`;;`**: The command separator, allowing multiple command expressions to be processed sequentially.
*   **`?`**: The introspection/help operator.

#### 1.5. Examples

1.  **Copy files:**
    `utility1 .files.copy src::dir1 dst::../dir2`
2.  **Copy and then delete, with JSON output for all commands in this invocation:**
    `utility1 output_format::json .files.copy src::dir1 dst::../dir2 ;; .files.delete src::dir1`
3.  **Get help for the copy command:**
    `utility1 .files.copy ?`
4.  **List all commands in the root namespace:**
    `utility1 .`
5.  **Switch to TUI modality and then list files:**
    `utility1 .modality.set target::tui ;; .files.list`
6.  **Command with a default argument value and debug logging:**
    `utility1 log_level::debug .log.message "This is a log entry"`

---

### 2. Command Definition (`unilang` Core)

#### 2.1. Command Anatomy

A command is the fundamental unit of action in `unilang`. Each command definition comprises several attributes:

*   **Full Name (String, Mandatory)**: The unique, dot-separated, case-sensitive path identifying the command (e.g., `.files.copy`, `.admin.users.create`, `.file.create.temp`).
    *   **Naming Conventions**:
        *   **Command Paths**: Command paths are formed by segments separated by dots (`.`). Each segment **must** consist of lowercase alphanumeric characters (a-z, 0-9) and underscores (`_`) may be used to separate words within a segment if preferred over shorter, distinct segments (e.g., `.file.create_temp` or `.file.create.temp`). Using only lowercase alphanumeric characters for segments is also common (e.g. `.file.createtemp`). Dots are exclusively for separating these path segments. Names must not start or end with a dot, nor contain consecutive dots. The namespace `.system.` is reserved for core `unilang`/`utility1` functionality.
        *   **Argument Names**: Argument names (e.g., `input-string`, `user_name`, `force`) **should** consist of lowercase alphanumeric characters and **may** use `kebab-case` (e.g., `input-string`) or `snake_case` (e.g., `user_name`) for multi-word names to enhance readability. They must be unique within a command.
*   **Hint/Description (String, Optional)**: A human-readable explanation of the command's purpose. Used in help messages and UI tooltips. `utility1` may implement localization for these strings.
*   **Routine (Mandatory)**: A reference or link to the actual executable code (handler function) that implements the command's logic. This routine receives a `VerifiedCommand` object and an `ExecutionContext` object.
*   **Arguments (List<ArgumentDefinition>, Optional)**: A list defining the arguments the command accepts. See Section 2.2.
*   **HTTP Method Hint (String, Optional)**: For WEB Endpoint modality, a suggested HTTP method (e.g., `GET`, `POST`, `PUT`, `DELETE`). If not provided, it can be inferred.
*   **Tags/Categories (List<String>, Optional)**: Keywords for grouping, filtering, or categorizing commands.
*   **Examples (List<String>, Optional)**: Illustrative usage examples of the command, primarily for CLI help.
*   **Permissions (List<String>, Optional)**: A list of permission identifiers required to execute this command.
*   **Status (Enum, Optional, Default: `Stable`)**: Indicates the maturity or lifecycle state of the command. Values: `Experimental`, `Stable`, `Deprecated`.
*   **Deprecation Message (String, Optional)**: If `status` is `Deprecated`, this message should explain the reason and suggest alternatives.
*   **Command Version (String, Optional)**: Individual commands can have their own SemVer version (e.g., "1.0.2").
*   **Idempotent (Boolean, Optional, Default: `false`)**: If `true`, indicates the command can be safely executed multiple times with the same arguments without unintended side effects.

##### 2.1.1. Namespaces

Namespaces provide a hierarchical organization for commands, preventing naming conflicts and improving discoverability.
*   A namespace is a sequence of identifiers separated by dots (e.g., `.files.utils.`).
*   Commands are typically defined within a namespace.
*   The root namespace `.` can also contain commands.
*   Listing commands in a namespace (e.g., `utility1 .files.`) should show sub-namespaces and commands directly within that namespace.

##### 2.1.2. Command Versioning & Lifecycle

*   **Command Version (String, Optional)**: Individual commands can have their own SemVer version.
    *   **Invocation of Specific Versions**: `unilang` itself doesn't prescribe a syntax like `.command@version`. Version management is typically handled by evolving the command or introducing new versions in different namespaces (e.g., `.v1.command`, `.v2.command`). If a `utility1` implementation supports direct versioned invocation, its parser must handle it before `unilang` command resolution.
*   **Lifecycle:**
    1.  **Experimental:** New commands that are subject to change. Should be used with caution.
    2.  **Stable:** Commands considered reliable and with a stable interface.
    3.  **Deprecated:** Commands planned for removal in a future version. `utility1` should issue a warning when a deprecated command is used. The `deprecation_message` should guide users.
    4.  **Removed:** Commands no longer available.

#### 2.2. Argument Specification

Arguments define the inputs a command accepts.

##### 2.2.1. Argument Attributes

Each argument within a command's `arguments` list is defined by these attributes:

*   **`name` (String, Mandatory)**: The unique (within the command), case-sensitive identifier for the argument (e.g., `src`, `dst`, `force`, `user-name`). Follows naming conventions in Section 2.1.
*   **`hint` (String, Optional)**: A human-readable description of the argument's purpose. `utility1` may localize this.
*   **`kind` (String, Mandatory)**: Specifies the data type of the argument's value. See Section 2.2.2 for defined types. The final value passed to the command routine will be the unescaped content, parsed according to this kind.
*   **`optional` (Boolean, Optional, Default: `false`)**:
    *   `false` (Mandatory): The argument must be provided.
    *   `true` (Optional): The argument may be omitted.
*   **`default_value` (Any, Optional)**: A value to use if an optional argument is not provided. The type of `default_value` must be compatible with `kind`. This value is applied *before* type validation.
*   **`is_default_arg` (Boolean, Optional, Default: `false`)**:
    *   If `true` for *one* argument in a command, its value can be provided in the CLI without specifying its name (positionally). The argument still requires a `name` for other modalities and explicit CLI invocation.
    *   If `is_default_arg` is true for an argument that accepts multiple values (due to `kind: List<Type>` or `multiple: true`), all subsequent positional tokens in the CLI (until a named argument `key::value`, `;;`, or `?` is encountered) are collected into this single default argument.
*   **`interactive` (Boolean, Optional, Default: `false` for CLI, adaptable for other UIs)**:
    *   If `true`, and the argument is mandatory but not provided, and the current UI modality supports it, the system may prompt the user to enter the value.
*   **`multiple` (Boolean, Optional, Default: `false`)**:
    *   If `true`, the argument can be specified multiple times in the CLI (e.g., `arg_name::val1 arg_name::val2`). The collected values are provided to the command routine as a list of the specified `kind`. See Section 2.2.2 for interaction with `List<Type>`.
*   **`aliases` (List<String>, Optional)**: A list of alternative short names for the argument (e.g., `s` as an alias for `source`). Aliases must be unique within the command's arguments and distinct from other argument names and follow naming conventions.
*   **`tags` (List<String>, Optional)**: For grouping arguments within complex commands, potentially for UI layout hints (e.g., "Basic", "Advanced", "Output").
*   **`validation_rules` (List<String> or List<Object>, Optional)**: Custom validation logic or constraints beyond basic type checking.
    *   Examples: Regex pattern for strings (`"regex:^[a-zA-Z0-9_]+$"`), min/max for numbers (`"min:0"`, `"max:100"`), file must exist (`"file_exists:true"`), string length (`"min_length:5"`). The exact format of rules needs definition by `utility1` but should be clearly documented.
*   **`sensitive` (Boolean, Optional, Default: `false`)**:
    *   If `true`, the argument's value should be treated as sensitive (e.g., passwords, API keys). UIs should mask it, and logs should avoid printing it or redact it.

##### 2.2.2. Data Types (`kind`)

The `kind` attribute specifies the expected data type of an argument. `unilang` defines a set of built-in types. The system should attempt to parse/coerce input strings (which are assumed to be unescaped at this stage) into these types.

*   **`String`**: A sequence of characters.
*   **`Integer`**: A whole number. Validation rules can specify range.
*   **`Float`**: A floating-point number.
*   **`Boolean`**: A true or false value. Parsed from "true", "false", "yes", "no", "1", "0" (case-insensitive for strings).
*   **`Path`**: A URI representing a file system path. Defaults to `file://` scheme if not specified. Handled as per Section 4.1.
*   **`File`**: A `Path` that must point to a file. Validation can check for existence.
*   **`Directory`**: A `Path` that must point to a directory. Validation can check for existence.
*   **`Enum(Choice1|Choice2|...)`**: A string that must be one of the predefined, case-sensitive choices. (e.g., `Enum(Read|Write|Execute)`).
*   **`URL`**: A Uniform Resource Locator (e.g., `http://`, `ftp://`, `mailto:`).
*   **`DateTime`**: A date and time. Should support ISO 8601 format by default (e.g., `YYYY-MM-DDTHH:MM:SSZ`).
*   **`Pattern`**: A regular expression pattern string.
*   **`List<Type>`**: A list of elements of a specified `Type` (e.g., `List<String>`, `List<Integer>`).
    *   **CLI Syntax**: If `kind` is `List<SomeType>` (and the argument's `multiple` attribute is `false`): `arg_name::value1,value2,value3`. The list delimiter (default ',') can be specified in the type definition if needed (e.g., `List<String,';'>`). This syntax is for providing multiple values *within a single instance* of the argument.
*   **Interaction with `multiple: true` attribute**:
    *   If `kind` is a non-list type (e.g., `String`) and the argument's `multiple` attribute is `true`:
        *   The argument value passed to the routine will be a `List<String>`.
        *   **CLI Syntax**: Requires repeating the argument: `arg_name::val1 arg_name::val2 arg_name::val3`. Each `value` is parsed as `String`.
    *   If `kind` is `List<SomeType>` and the argument's `multiple` attribute is also `true`: This implies a "list of lists."
        *   **CLI Syntax**: `arg_name::val1,val2 arg_name::val3,val4`. Each `valX,valY` part is parsed as a list, and these lists are collected into an outer list. This should be used sparingly due to CLI complexity; accepting a single JSON string for such complex inputs is often clearer.
*   **`Map<KeyType,ValueType>`**: A key-value map (e.g., `Map<String,String>`).
    *   **CLI Syntax**: `arg_name::key1=val1,key2=val2,key3=val3`. Keys and values follow standard quoting rules if they contain delimiters or spaces. The entry delimiter (default ',') and key-value separator (default '=') can be specified if needed, e.g., `Map<String,String,';',':'>`.
*   **`JsonString` / `Object`**: For arbitrarily complex or nested objects as arguments, the recommended approach for CLI is to accept a JSON string: `complex_arg::'{"name": "item", "details": {"id": 10, "tags": ["a","b"]}}'`. The `kind` could be `JsonString` (parsed and validated as JSON, then passed as string) or `Object` (parsed into an internal map/struct representation).
*   **`InputStream` / `OutputStream`**: Special kinds indicating the argument is not a simple value but a stream provided by `utility1` via `ExecutionContext`.
    *   `InputStream`: For reading data (e.g., from CLI stdin, HTTP request body).
    *   `OutputStream`: For writing data (e.g., to CLI stdout, HTTP response body).
    *   These are typically not specified directly on the CLI as `key::value` but are resolved by `utility1` based on context or special syntax (e.g., a command might define an argument `input_source` of kind `InputStream` which defaults to stdin if not otherwise bound).
*   **`Any`**: Any type, minimal validation. Use sparingly.
*   **Custom Types**: The system should be extensible to support custom types defined by `Extension Module`s, along with their parsing and validation logic.

#### 2.3. Methods of Command Specification

Commands can be defined in `unilang` through several mechanisms:

1.  **Compile-Time Declarative (e.g., Rust Proc Macros)**: Attributes on structures or functions generate command definitions at compile time. Offers performance and type safety.
2.  **Run-Time Procedural (Builder API)**: Code uses a builder pattern to construct and register command definitions at runtime. Offers dynamic command generation.
3.  **Compile-Time External Definition (e.g., YAML, JSON)**: An external file (e.g., `commands.yaml`) is parsed during the build process (e.g., Rust `build.rs`), generating code to include command definitions.
4.  **Run-Time External Definition (e.g., YAML, JSON)**: An external file is loaded and parsed by `utility1` at startup or on-demand to register commands. Requires a mechanism to link routines (e.g., named functions in `Extension Module`s).

#### 2.4. Unified Command Registry

Regardless of the definition method, all commands are registered into a single, unified command registry within `utility1`.
*   This registry is responsible for storing and looking up command definitions.
*   It must ensure the uniqueness of command `FullName`s. Conflicts (e.g., two definitions for the same command name) must be resolved based on a clear precedence rule (e.g., compile-time definitions override runtime, or an error is raised during registration).
*   The registry should support efficient lookup by `FullName` and listing commands by namespace.
*   For compile-time defined commands, Perfect Hash Functions (PHF) can be used for optimal lookup speed. Runtime additions would use standard hash maps.

---

### 3. Interaction Modalities

`unilang` definitions are designed to drive various interaction modalities. `utility1` may start in a default modality (often CLI) or have its modality switched by a specific `unilang` command.

#### 3.1. Common Principles Across Modalities

*   **Command Discovery**: All modalities should provide a way to list available commands and namespaces (e.g., `utility1 .`, `utility1 .files.`).
*   **Help/Introspection**: Access to detailed help for commands and their arguments (e.g., `utility1 .files.copy ?`). The help system should provide structured data (see 3.2.6).
*   **Argument Input**: Modalities provide appropriate mechanisms for users to input argument values based on their `kind` and other attributes.
*   **Error Presentation**: Consistent and clear presentation of errors (validation errors, execution errors). See Section 4.2.
*   **Output Handling**: Displaying command output in a way suitable for the modality, respecting `OutputData` structure (Section 4.2.1).

#### 3.2. Command Line Interface (CLI)

The primary interaction modality.

##### 3.2.1. Syntax and Structure
As defined in Section 1.

##### 3.2.2. Language Processing (Parsing, Validation)
Follows the multi-phase processing defined in Section 1.1.

##### 3.2.3. Request Execution
Handled by the Interpreter / Execution Engine (Section 5).

##### 3.2.4. Output Formatting

The CLI supports various output formats for command results, controllable via a global argument (e.g., `utility1 output_format::json .some.command`).
*   Formats: `text` (default), `json`, `yaml`, `table`.
*   Command routines should return structured `OutputData` (Section 4.2.1) to facilitate this.
*   **Raw Output**: If a command routine's `OutputData` has an `output_type_hint` that is not a common structured type (e.g., `text/plain`, `application/octet-stream`), or if the payload is a raw byte stream, the CLI modality should write this data directly to `stdout`, bypassing structured formatters like JSON/YAML.

##### 3.2.5. Shell Completions

`utility1` should be able to generate shell completion scripts (for Bash, Zsh, Fish, PowerShell, etc.).
*   These scripts would provide completion for command names, namespaces, and argument names.
*   For arguments with `Enum` type or known value sets (e.g., file paths), completions could extend to argument values.
*   A command like `utility1 .system.completion.generate shell_type::bash` could be used.

##### 3.2.6. Help System (`?`) Output

*   Invoking `utility1 .namespace.command.name ?`, `utility1 .namespace. ?`, or `utility1 .system.globals ?` should, by default, produce human-readable text for the CLI.
*   However, the underlying help generation mechanism **must** be capable of producing structured data (e.g., JSON). This can be accessed via the global output format argument: `utility1 .namespace.command.name ? output_format::json`.
*   This structured help output **should** include fields such as:
    *   `name` (full command/global arg name, or namespace path)
    *   `description` (hint)
    *   `arguments` (list of argument definitions, including their name, kind, hint, optionality, default value, aliases, validation rules) - for commands and global args.
    *   `examples` (list of usage examples) - for commands.
    *   `namespace_content` (if querying a namespace: list of sub-commands and sub-namespaces with their hints).
    *   `status`, `version`, `deprecation_message` (if applicable for the command/global arg).

#### 3.3. Textual User Interface (TUI)

*   **Invocation**: May be the default modality for `utility1`, configured globally, or entered via a `unilang` command like `utility1 .modality.set target::tui`.
*   **Presentation**: Uses terminal libraries (e.g., `ratatui`, `ncurses`) for interactive command browsing, argument input forms with validation, and output display. Consumes structured help (3.2.6) and `OutputData`/`ErrorData`.

#### 3.4. Graphical User Interface (GUI)

*   **Invocation**: May be the default, configured, or entered via `utility1 .modality.set target::gui`.
*   **Presentation**: Uses native GUI toolkits (Qt, GTK) or web-based technologies (Tauri, Electron) for menus, rich forms with widgets (file pickers, date selectors), and dedicated output/log views. Consumes structured help and `OutputData`/`ErrorData`.

#### 3.5. Audio User Interface (AUI)

*   **Invocation**: May be the default, configured, or entered via `utility1 .modality.set target::aui`.
*   **Presentation**: Uses speech-to-text for input, text-to-speech for output/prompts. Requires a Natural Language Understanding (NLU) layer to map spoken phrases to `unilang` commands and arguments. Consumes structured help and `OutputData`/`ErrorData` for synthesis.

#### 3.6. WEB Endpoints

Automatically generate a web API from `unilang` command specifications. The HTTP server component is typically initiated by a specific `unilang` command defined within `utility1` (often provided by an `Extension Module`).

*   **Goal**: Automatically generate a web API from `unilang` command specifications.
*   **Invocation**: An HTTP server, potentially started by a user-defined command like `utility1 .server.start port::8080` or `utility1 .api.serve`. This `.server.start` command would itself be defined using `unilang` and its routine would be responsible for initializing and running the web server. The functionality might be provided by a dedicated `Extension Module` that `utility1`'s integrator includes.
*   **Mapping Commands to Endpoints**:
    *   A `unilang` command `.namespace.command.name` maps to an HTTP path (e.g., `/api/v1/namespace/command/name`). The base path (`/api/v1/`) is configurable. Command path segments are typically used directly or converted to `kebab-case` in URLs if that's the API style.
    *   HTTP method determined by `http_method_hint` in command definition, then by inference (e.g., `get*`, `list*` -> `GET`; `create*`, `add*` -> `POST`; `update*` -> `PUT`; `delete*`, `remove*` -> `DELETE`), then defaults (e.g., `POST`).
*   **Argument Passing & Data Serialization**:
    *   `GET`: Arguments as URL query parameters.
        *   `List<Type>` encoding: Repeated parameter names (e.g., `?list-arg=item1&list-arg=item2`).
        *   `Map<KeyType,ValueType>` encoding: Bracketed notation (e.g., `?map-arg[key1]=value1&map-arg[key2]=value2`).
    *   `POST`, `PUT`, `PATCH`: Arguments as a JSON object in the request body. Argument names in `unilang` map to JSON keys (typically `camelCase` or `snake_case` by convention in JSON, conversion from `kebab-case` or `snake_case` argument names may apply).
    *   Binary data (e.g., file uploads for an `InputStream` argument) would use `multipart/form-data`.
    *   Responses are typically JSON, based on `OutputData` (Section 4.2.1) and `ErrorData` (Section 4.2).
*   **Responses & Error Handling (HTTP specific)**:
    *   **Success**: Standard HTTP success codes (200 OK, 201 Created, 204 No Content). Response body (if any) is JSON derived from `OutputData.payload`. `OutputData.metadata` might be in headers or a wrapper object.
    *   **Error**: Standard HTTP error codes (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 500 Internal Server Error). Response body is a JSON object based on `ErrorData`.
*   **API Discoverability (OpenAPI)**:
    *   An endpoint (e.g., `GET /api/v1/openapi.json` or `/api/v1/swagger.json`) automatically generates an OpenAPI (v3+) specification.
    *   This spec is derived from `unilang` command definitions (paths, methods, argument attributes mapping to parameters, `kind` mapping to schema types, hints to descriptions).
*   **Asynchronous Operations**:
    For long-running commands initiated via WEB Endpoints:
    1.  Initial request receives `202 Accepted`.
    2.  Response includes a `Location` header pointing to a status endpoint (e.g., `/api/v1/tasks/{task_id}`).
    3.  Client polls the status endpoint, which returns current status (e.g., `Pending`, `Running`, `Success`, `Failure`) and potentially partial results or logs.
    4.  Upon completion, the status endpoint can provide the final result or a link to it.
    This requires `utility1` to have a task management subsystem.

#### 3.7. `utility1://` URL Scheme (for utility interaction)

*   **Structure**: `utility1://[namespace_path/]command.name[?query_parameters]`
*   Used for inter-application communication or custom protocol handlers invoking `utility1` CLI commands.
*   Distinct from WEB Endpoints. Query parameters should follow standard URL encoding.

---

### 4. Cross-Cutting Concerns

#### 4.1. Path Handling

*   **URI-based Internal Representation**: Path-like arguments are internally converted to and handled as URIs (e.g., `file:///path/to/local`, `clipboard://`, `stdin://`, `temp://filename`). If no scheme is provided, `file://` is assumed.
*   **Absolute Path Conversion**: For `file://` URIs, `utility1` resolves them to absolute paths based on the current working directory before passing them to command routines, unless a command explicitly requires relative paths.
*   **Path Validation**: Can be specified via `validation_rules` (e.g., `exists`, `is_file`, `is_directory`, `is_readable`, `is_writable`).

#### 4.2. Error Handling Strategy

A standardized approach to errors is crucial for predictability.

*   **Command Routine Return**: Routines should return a `Result<OutputData, ErrorData>`.
*   **`ErrorData` Structure**:
    ```json
    {
      "code": "ErrorCodeIdentifier", // e.g., UNILANG_ARGUMENT_INVALID, MYAPP_CUSTOM_ERROR
      "message": "Human-readable error message.", // utility1 may localize this
      "details": { /* Optional: Object for error-specific details */
        // Example for UNILANG_ARGUMENT_INVALID: "argument_name": "src", "reason": "File does not exist"
        // Example for UNILANG_SYNTAX_ERROR: "syntax_issue": "Unterminated quote", "sub_kind": "UnterminatedQuote"
        "location_in_input": { // Describes where in the input the error was detected.
          "source_type": "single_string" /* or "string_slice" */,
          // If "single_string":
          "start_offset": 15, // byte offset from the beginning of the single input string
          "end_offset": 20,   // byte offset for the end of the problematic span
          // If "string_slice":
          "segment_index": 2,         // index of the string in the input slice
          "start_in_segment": 5,  // byte offset from the beginning of that segment string
          "end_in_segment": 10    // byte offset for the end of the span within that segment
        }
      },
      "origin_command": ".files.copy" // Optional: FullName of the command that originated the error (if past parsing)
    }
    ```
*   **Standard Error Codes**: `utility1` implementations **should** use these core `unilang` error codes when applicable, and **may** define more specific codes.
    *   `UNILANG_COMMAND_NOT_FOUND`
    *   `UNILANG_ARGUMENT_INVALID`
    *   `UNILANG_ARGUMENT_MISSING`
    *   `UNILANG_TYPE_MISMATCH`
    *   `UNILANG_VALIDATION_RULE_FAILED`
    *   `UNILANG_PERMISSION_DENIED`
    *   `UNILANG_EXECUTION_ERROR` (Generic for routine failures)
    *   `UNILANG_EXTENSION_MODULE_ERROR` (Error originating from an Extension Module)
    *   `UNILANG_IO_ERROR`
    *   `UNILANG_MODALITY_UNAVAILABLE`
    *   `UNILANG_MODALITY_SWITCH_FAILED`
    *   `UNILANG_INTERNAL_ERROR` (For unexpected framework issues)
    *   `UNILANG_SYNTAX_ERROR` (For errors during Phase 1 lexical or syntactic analysis, e.g., unterminated quote, unexpected token, itemization failure. The `details` field may contain more specific sub-categories of the syntax issue.)
*   **Modality Mapping**: Each modality translates `ErrorData` appropriately (e.g., CLI prints to stderr, WEB Endpoints map to HTTP status codes and JSON bodies).
*   **`ErrorData` `details` Field**:
    *   This field provides context for the error. It **should** include `location_in_input` detailing where the error was detected, structured as shown above to reflect whether the input was a single string or a slice of strings, providing span information (e.g., start/end offsets).
    *   For `UNILANG_ARGUMENT_INVALID`, `UNILANG_ARGUMENT_MISSING`, `UNILANG_TYPE_MISMATCH`, `UNILANG_VALIDATION_RULE_FAILED`: `details` **should** include `argument_name: String`.
    *   For `UNILANG_COMMAND_NOT_FOUND`: `details` **may** include `attempted_command_name: String`.
    *   For `UNILANG_SYNTAX_ERROR`: `details` **should** include a description of the syntax issue and **may** include a more specific `sub_kind` (e.g., "UnterminatedQuote", "InvalidEscapeSequence", "ItemizationFailure").

#### 4.2.1. `OutputData` Structure
When a command routine succeeds, it returns `OutputData`. This structure facilitates consistent handling across modalities and `output_format` settings.
*   **Structure**:
    ```json
    {
      "payload": "Any", // The main command result (e.g., string, number, boolean, list, object).
                       // For commands producing no specific output on success (e.g., a 'set' operation),
                       // this can be null or a success message object like {"status": "success", "message": "Operation complete"}.
      "metadata": { /* Optional: Object for additional information not part of the primary payload.
                       e.g., "count": _integer_, "warnings": [_string_], "pagination_info": _object_ */ },
      "output_type_hint": "String" // Optional: A MIME type like "application/json" (default if payload is object/array),
                                 // "text/plain", "application/octet-stream".
                                 // Helps modalities (especially CLI and WEB) decide on formatting.
                                 // If payload is raw bytes and this is "application/octet-stream",
                                 // formatters like JSON/YAML are bypassed.
    }
    ```

#### 4.3. Security Considerations

##### 4.3.1. Input Sanitization & Validation

*   `unilang`'s type system (`kind`) and `validation_rules` provide a first line of defense.
*   Command routines are ultimately responsible for ensuring inputs are safe before use, especially if interacting with shells, databases, or other sensitive systems. Avoid constructing scripts or queries by direct string concatenation of user inputs.
*   For `Path` arguments, be cautious about path traversal vulnerabilities if not using the resolved absolute paths.

##### 4.3.2. Permissions & Authorization Model

*   The `permissions` attribute in a command definition declares the necessary rights.
*   `utility1`'s execution core or specific modalities (like WEB Endpoints) must implement an authorization mechanism that checks if the invoking user/context possesses these permissions.
*   The permission strings are abstract; their meaning and how they are granted/checked are implementation details of the `utility1` environment.

##### 4.3.3. Sensitive Data Handling

*   Arguments marked `sensitive: true` require special handling:
    *   CLIs should mask input (e.g., password prompts).
    *   GUIs/TUIs should use password fields.
    *   Logs should redact or omit these values (see Section 4.6).
    *   Care should be taken not to inadvertently expose them in error messages or verbose outputs.

##### 4.3.4. WEB Endpoint Security

If `utility1` exposes WEB Endpoints, standard web security practices apply:
*   Use HTTPS.
*   Implement authentication (e.g., API keys, OAuth2/OIDC tokens).
*   Protect against common vulnerabilities (CSRF, XSS, SQLi - if applicable, SSRF).
*   Implement rate limiting and request size limits.
*   The OpenAPI specification should accurately reflect security schemes.

#### 4.4. Configuration of `utility1`

`utility1` itself may require configuration, affecting `unilang` behavior.
*   **Configuration Sources & Precedence**: The listed order of sources **is** the strict precedence order. Items later in the list (higher precedence) override values from earlier items.
    1.  Default built-in values.
    2.  System-wide configuration file (e.g., `/etc/utility1/config.toml`).
    3.  User-specific configuration file (e.g., `~/.config/utility1/config.toml`).
    4.  Project-specific configuration file (e.g., `./.utility1.toml` in the current directory).
    5.  Environment variables (e.g., `UTILITY1_LOG_LEVEL`).
    6.  CLI Global Arguments to `utility1` (e.g., `utility1 log_level::debug ...`).
*   **Configurable Aspects**:
    *   `Extension Module` search paths or integration settings (if applicable beyond build system dependencies).
    *   Default log level (Section 4.6).
    *   Default output format for CLI.
    *   Paths for i18n resource bundles (if `utility1` supports localization).
    *   WEB Endpoint server settings (port, base path, SSL certs).
    *   Authentication provider details for WEB Endpoints.

#### 4.5. Extensibility: Compile-Time Modalities & Hybrid Command Model

`unilang` and `utility1` are designed for extensibility. This is achieved through:
1.  **Compile-Time `Extension Module`s:** For defining core functionalities, representation modalities, and a base set of commands.
2.  **Run-Time Command Registration:** For dynamically adding new commands after `utility1` has been compiled and is running.

*   **A. Compile-Time `Extension Module` Capabilities (Guidance for Integrators)**
    *   `utility1` can be structured such that different internal modules or dependent crates (acting as compile-time **`Extension Module`s**) contribute:
        *   **Representation Modalities**: Implementations for UI modalities (CLI, TUI, GUI, WEB server logic, etc.) and any modifications or themes for them. These are fixed at compile time.
        *   **Core Commands & Types**: A foundational set of `unilang` Command Definitions (as per Section 2) and custom Argument Types (`kind` as per Section 2.2.2). These are registered into `utility1`'s unified registries during the compilation process (e.g., via procedural macros, build scripts).
        *   **Core Routines**: The implementations (handler functions) for these compile-time commands.

*   **B. Run-Time Command Extensibility**
    *   `utility1` **must** provide a mechanism for new **Command Definitions** to be added to its unified command registry *at run-time*.
    *   This allows extending `utility1`'s capabilities without recompilation, for example, by:
        *   Loading command definitions from external files (e.g., YAML, JSON) at startup or on-demand.
        *   Receiving command definitions from other processes or over a network (if `utility1` implements such an interface).
        *   A procedural API within `utility1` (if it's embeddable or has an interactive scripting layer) to define and register commands dynamically.
    *   **Important Note:** Only commands can be added at run-time. Representation modalities and custom argument types (`kind`) are fixed at compile time via **`Extension Module`s**. If a run-time defined command requires a custom argument type not known at compile-time, it must use existing types or more generic ones like `String` or `JsonString` and perform more specific parsing/validation within its routine.

*   **`Extension Module` Integration (Compile-Time Part - Integrator's Responsibility)**:
    *   The mechanism by which `utility1` incorporates compile-time **`Extension Module`s** is a standard part of its build system (e.g., `Cargo.toml` dependencies).

*   **Manifests (For `Extension Module`s & Potentially for Run-Time Definitions)**:
    *   **`Extension Module`s**: May internally use manifest-like structures for organization or to aid code generation (e.g., `module_name`, `module_version`, `unilang_spec_compatibility`, `description`, `author`, `license`, `entry_points` for `unilang` components). The `entry_points` values are strings whose interpretation is specific to `utility1`'s build/integration mechanism.
    *   **Run-Time Command Definition Files**: External files (e.g., YAML/JSON) defining commands for run-time loading act as a form of manifest for those commands. They should adhere to the `unilang` `CommandDefinition` structure.

*   **Component Registration**:
    *   **Compile-Time**: `utility1`'s build process or initialization code collects and registers all `unilang`-compatible components (modalities, core commands, types) from its constituent compile-time **`Extension Module`s** into the relevant `unilang` registries. (Mechanisms: procedural macros, build scripts, static initializers).
    *   **Run-Time (Commands Only)**: `utility1` **must** expose an API or mechanism to add `CommandDefinition` structures to its live, unified command registry. This API would also need a way to associate these commands with their executable routines.
        *   For routines of run-time loaded commands:
            *   If loaded from external files, the `routine_link` (Section A.1) might point to a function in an already loaded (compile-time) **`Extension Module`**, or to a script to be executed by an embedded interpreter (if `utility1` supports this).
            *   If defined procedurally at run-time, the routine is typically provided directly as a closure or function pointer.

*   **Security**:
    *   **Compile-Time `Extension Module`s**: Trust is based on the `utility1` build process and vetting of dependencies.
    *   **Run-Time Commands**: If `utility1` loads command definitions or executes routines from untrusted sources at run-time, the integrator is responsible for implementing robust security measures (sandboxing, permission checks for command registration, validation of definition sources). `unilang`'s permission attributes on commands can be leveraged here.

#### 4.6. Logging (Guidance for `utility1` and Routines)

A consistent logging strategy is essential for debugging and auditing. `utility1` should provide a logging facility accessible to command routines via the `ExecutionContext`.

*   **Logging Facade**: `utility1` should use or provide a common logging facade.
*   **Log Levels**: Standard levels (e.g., `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`).
*   **Configurable Log Level**: The active log level for `utility1` and its routines should be configurable (see Section 4.4, e.g., via global argument `log_level::debug`).
*   **Structured Logging**: It is recommended that `utility1`'s logging output be structured (e.g., JSON format) to include timestamp, level, module/command name, message, and contextual key-value pairs.
*   **Sensitive Data Redaction**: `utility1`'s logging system or conventions for routines should ensure that arguments marked `sensitive: true` are automatically redacted or omitted from logs.
*   **Audit Logging**: For critical operations or WEB Endpoints, `utility1` may implement dedicated audit logs.

#### 4.7. Execution Context

An `ExecutionContext` object **is always** passed to command routines by `utility1` alongside `VerifiedCommand`. Its specific content is defined by `utility1` but **should** provide access to at least:

*   The current effective global argument values (e.g., `output_format`, `log_level`).
*   Access to `utility1`'s configuration settings.
*   A pre-configured logger instance (respecting current log level and command context).
*   (If applicable for streaming kinds like `InputStream`/`OutputStream`) Methods to acquire input/output streams connected to the appropriate source/sink for the current modality.
*   Information about the invoking modality.
*   (If `utility1` supports localization) The current active locale.

#### 4.8. Command Sequences and Atomicity (` ;; `)

*   Commands separated by `;;` are executed sequentially.
*   By default, if a command in the sequence fails, subsequent commands are not executed. This behavior can be controlled by a global argument (e.g., `on_error::continue`).
*   `unilang` itself does not define transactional semantics for command sequences. Each command is typically treated as an independent operation. If a `utility1` implementation or a specific set of commands offers transactional guarantees, this is an extension beyond the core `unilang` specification.

---

### 5. Interpreter / Execution Engine

The Interpreter, also referred to as the Execution Engine, is the component within `utility1` responsible for taking a `VerifiedCommand` (or a sequence thereof) produced by the preceding parsing and semantic analysis phases (Section 1.1), and orchestrating the execution of its associated `Routine (Handler Function)`.

#### 5.1. Core Responsibilities

1.  **Routine Invocation:**
    *   For each `VerifiedCommand`, the Interpreter retrieves the linked `Routine` from the `CommandDefinition`.
    *   It prepares and passes the `VerifiedCommand` object (containing resolved and typed arguments) and the `ExecutionContext` object (Section 4.7) to the `Routine`.

2.  **Handling Routine Results:**
    *   The Interpreter receives the `Result<OutputData, ErrorData>` returned by the `Routine`.
    *   If `Ok(OutputData)`: The `OutputData` is passed to the current `Modality` for presentation to the user (respecting global arguments like `output_format`).
    *   If `Err(ErrorData)`: The `ErrorData` is passed to the current `Modality` for error reporting.

3.  **Sequential Command Execution (` ;; `):**
    *   If the input resulted in a sequence of `VerifiedCommand`s, the Interpreter executes them in the specified order.
    *   It respects the `on_error` global argument policy (e.g., `stop` (default) or `continue`) when a command in the sequence returns `ErrorData`.

4.  **`ExecutionContext` Management:**
    *   The Interpreter is responsible for creating and populating the `ExecutionContext` that is passed to each `Routine`. This context may be updated between commands in a sequence if necessary (though typically, global settings from `ExecutionContext` are established at the start of the entire `utility1` invocation).

5.  **Resource Management (Basic):**
    *   While complex resource management is `utility1`'s broader concern, the Interpreter might handle basic setup/teardown around routine execution if required by the `unilang` framework (e.g., related to `InputStream`/`OutputStream` arguments).

#### 5.2. Interaction with Modalities

*   The Interpreter does not directly handle user input or output rendering. It delegates these tasks to the active `Modality`.
*   The Modality is responsible for:
    *   Providing the initial CLI string (for CLI modality) or equivalent user interaction data.
    *   Displaying `OutputData` in a user-appropriate format.
    *   Presenting `ErrorData` clearly.
    *   Handling interactive prompts if an argument is marked `interactive` and a value is missing (this interaction might loop back through parts of the semantic analysis).

#### 5.3. Extensibility

*   The core Interpreter logic is part of the `unilang` framework provided by the crate.
*   `utility1` integrators influence its behavior by:
    *   Registering commands with their specific `Routine`s.
    *   Defining the content and services available via `ExecutionContext`.
    *   Implementing the presentation logic within their chosen `Modality` handlers.

---

### 6. Appendices

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

---

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
*   `<identifier_segment>` and `<identifier_arg_name>` need precise definitions based on allowed characters (Section 2.1).
*   `<arg_value>` parsing is the most complex part and is abstracted here. It represents the unescaped content after initial lexing and quote processing.
*   Shell quoting and escaping are handled by the shell before `utility1` receives the arguments. `unilang`'s parser then handles its own quoting rules.

**Note on Applying Grammar to Dual Input Types:**

This BNF describes the logical structure of a `unilang` command expression.
*   When parsing a **single string input**, the parser attempts to match this grammar directly against the character stream.
*   When parsing a **slice of strings input** (pre-tokenized by the shell), the parser consumes these strings sequentially. Each string (or parts of it, if a string contains multiple `unilang` elements like `name::value`) is then matched against the grammar rules. For instance, one string from the slice might be an `<arg_name>`, the next might be `::` (if the shell separated it), and the next an `<arg_value>`. Or a single string from the slice might be `name::value` which the `unilang` parser then further decomposes. The parser must be able to stitch these segments together to form complete `unilang` syntactic structures as defined by the grammar.

---

#### A.3. Component Registration (Conceptual Outline for Hybrid Model)

This appendix outlines the conceptual mechanisms for how `unilang` components are registered within `utility1`, covering both compile-time contributions from **`Extension Module`s** and run-time command registration. The `noun_verb` convention is used for conceptual API method names that `utility1` might expose for run-time operations.

**1. Compile-Time Component Registration (Modalities, Core Commands from `Extension Module`s, Types)**

`Extension Module`s providing modalities, core commands, or custom types need to make their definitions available to `utility1`'s central registries at compile time.

*   **A. Information Required for Modality Registration (Compile-Time Only via `Extension Module`s)**
    *   An **`Extension Module`** providing a modality (e.g., a TUI implementation) needs to register its handler or main entry point with `utility1`.
    *   **Mechanism Examples**: Static registration where `utility1`'s build system links modality implementations from known `Extension Module`s. `utility1` might discover modules that implement a `utility1`-defined `ModalityHandler` trait/interface.

*   **B. Information Required for Core Command Registration (Compile-Time via `Extension Module`s)**
    *   `Extension Module`s make `CommandDefinition` structures (Section 2.1) available.
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
*   The `ExecutionContext` (Section 4.7) is prepared by `utility1` and passed to all routines, whether linked at compile-time or run-time.

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

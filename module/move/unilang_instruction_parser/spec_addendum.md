# Specification Addendum

### Purpose
This document is intended to be completed by the **Developer** during the implementation phase. It is used to capture the final, as-built details of the **Internal Design**, especially where the implementation differs from the initial `Design Recommendations` in `specification.md`.

### Instructions for the Developer
As you build the system, please use this document to log your key implementation decisions, the final data models, environment variables, and other details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

### Parser Implementation Notes
*A space for the developer of `unilang_instruction_parser` to document key implementation choices, performance trade-offs, or edge cases discovered while implementing the formal parsing rules from `specification.md` Section 2.5.*

-   **Whitespace Handling:** Implemented by configuring `strs_tools` to treat whitespace as a delimiter but to not preserve the delimiter tokens themselves. This simplifies the token stream that the syntactic analyzer has to process.
-   **Command Path vs. Argument Logic:** The transition from path parsing to argument parsing is handled by a state machine within the parser engine. The parser remains in the `ParsingPath` state until a non-identifier/non-dot token is encountered, at which point it transitions to the `ParsingArguments` state and does not transition back.

### Finalized Internal Design Decisions
*A space for the developer to document key implementation choices for the system's internal design, especially where they differ from the initial recommendations in `specification.md`.*

-   **Decision 1: PHF Crate Selection:** After evaluation, the `phf` crate (version `X.Y.Z`) was chosen for the static registry implementation due to its robust build-time code generation and minimal runtime overhead.
-   **Decision 2: Runtime Routine Linking:** The `routine_link` mechanism will be implemented using a `HashMap<String, Routine>`. `utility1` integrators will be responsible for registering their linkable functions into this map at startup. Dynamic library loading was deemed too complex for v1.0.

### Finalized Internal Data Models
*The definitive, as-built schema for all databases, data structures, and objects used internally by the system.*

-   **`CommandRegistry` Struct:**
    ```rust
    pub struct CommandRegistry {
        static_commands: phf::Map<&'static str, CommandDefinition>,
        static_namespaces: phf::Map<&'static str, NamespaceDefinition>,
        dynamic_commands: HashMap<String, CommandDefinition>,
        dynamic_namespaces: HashMap<String, NamespaceDefinition>,
        routines: HashMap<String, Routine>,
    }
    ```

### Environment Variables
*List all environment variables required to run the application. Include the variable name, a brief description of its purpose, and an example value (use placeholders for secrets).*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `UTILITY1_CONFIG_PATH` | Overrides the default search path for the user-specific configuration file. | `/etc/utility1/main.toml` |
| `UTILITY1_LOG_LEVEL` | Sets the logging verbosity for the current invocation. Overrides config file values. | `debug` |

### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions (e.g., from `Cargo.lock`).*

-   `rustc`: `1.78.0`
-   `serde`: `1.0.203`
-   `serde_yaml`: `0.9.34`
-   `phf`: `0.11.2`
-   `strs_tools`: `0.19.0`
-   `macro_tools`: `0.57.0`

### Deployment Checklist
*A step-by-step guide for deploying the application from scratch. This is not applicable for a library, but would be used by an `Integrator`.*

1.  Set up the `.env` file using the template above.
2.  Run `cargo build --release`.
3.  Place the compiled binary in `/usr/local/bin`.
4.  ...
5
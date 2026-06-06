# Invariant Test: Architectural Boundary

### Scope

- **Purpose**: Verify the architectural boundary invariant from `docs/invariant/001_architectural_boundary.md` holds in the codebase.
- **Responsibility**: Test spec for the one-directional dependency constraint — `cli_fmt` depends on `strs_tools`; `strs_tools` does not depend on `cli_fmt`.
- **In Scope**: Dependency direction correctness (IN-1); absence of CLI-specific output-processing types from `strs_tools` (IN-2); absence of CLI-specific help-rendering types from `strs_tools` (IN-2).
- **Out of Scope**: Runtime performance of either crate; internal implementation details of `strs_tools`.

### IN-1: cli_fmt depends on strs_tools; strs_tools does not depend on cli_fmt

- **Given:** `cli_fmt/Cargo.toml` and the `strs_tools` crate's `Cargo.toml` in the workspace
- **When:** both Cargo.toml files are inspected for dependency declarations
- **Then:** `cli_fmt/Cargo.toml` lists `strs_tools` as a dependency; `strs_tools/Cargo.toml` contains no reference to `cli_fmt`

### IN-2: No CLI-specific types are defined in strs_tools

- **Given:** the `strs_tools` source tree
- **When:** source is searched for CLI-policy symbols from both modules: output-module (`StreamFilter`, `OutputConfig`, `ProcessedOutput`, `process_output`) and help-module (`CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`)
- **Then:** no such symbols are defined in `strs_tools` — they exist exclusively in `cli_fmt`

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/output.rs` | Defines the CLI-specific output types (`StreamFilter`, `OutputConfig`, `ProcessedOutput`) that must not appear in strs_tools |
| `../../../src/help.rs` | Defines the CLI-specific help types (`CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, etc.) that must not appear in strs_tools |
| `../../../Cargo.toml` | Declares the one-directional `strs_tools` dependency |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/output.rs` | Compilation + test success confirms CLI-specific output types (`StreamFilter`, `OutputConfig`, `ProcessedOutput`) remain in `cli_fmt`, not `strs_tools` |
| `../../../tests/help.rs` | Compilation + test success confirms CLI-specific help types (`CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, etc.) remain in `cli_fmt`, not `strs_tools` or `data_fmt` |

### Invariants

| File | Relationship |
|------|-------------|
| `../../../docs/invariant/001_architectural_boundary.md` | Authoritative invariant statement for this spec |

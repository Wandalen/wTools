# API: Builder API

### Scope

- **Purpose**: Provide a fluent builder API for constructing execution plans from source definitions and runtime configuration.
- **Responsibility**: Documents the public interface for Source, Program, and Plan — their builder operations, field semantics, construction entry points, and compatibility guarantees.
- **In Scope**: Builder operations for all three types; field setters and accepted values; nested builder entry and completion semantics; RunOptions configuration fields.
- **Out of Scope**: Runtime execution of a completed plan (→ `api/002`); internal Former derive mechanics; namespace import conventions (→ `feature/001`).

### Abstract

Three composable builder types for constructing execution plans. Plan is the top-level entry point, containing exactly one Program, which holds an ordered collection of Source entries and optional manifest configuration. All three types use the Former builder pattern — construction begins at Plan, descends into Program, then into individual Source entries. Each builder returns focus to its parent on completion, producing the final Plan value when the outermost builder completes.

### Operations

**Source** — represents a single source file:
- `file_path` — the file path for the source within the program (e.g., `"src/main.rs"`, `"src/lib.rs"`). Any string value is accepted; no path validation is performed.
- `data` — the source code content as plain text. Any string value is accepted; no syntax validation is performed.
- Completing a Source appends it to the parent Program's source collection in insertion order and returns builder focus to the Program.

**Program** — an ordered collection of source files:
- `source` — opens a new Source builder entry. Each completed Source is appended to the Program's collection in the order of construction.
- `manifest` — optional inline Cargo manifest content as a string. When absent, the runner generates a minimal default manifest.
- Completing a Program returns builder focus to the Plan.

**Plan** — the top-level execution configuration:
- `former` — the construction entry point; creates a new Plan builder.
- `program` — opens the Program builder for the single embedded program.
- `run_options` — execution parameters: build profile, timeout, environment variables, feature flags, Cargo binary path, artifact cache directory, and cleanup mode.
- Completing a Plan produces the final Plan value, ready for submission to the runner.

**RunOptions** fields (constructed directly and passed via the `.run_options()` setter — `RunOptions` does not have a nested builder, it uses `Default::default()` plus field assignment):
- `build_profile` — Cargo build profile, debug or release (default: debug).
- `target_dir` — optional artifact cache directory path (default: temporary per-run directory).
- `cargo_path` — path to the Cargo binary (default: `"cargo"` resolved via PATH).
- `timeout_ms` — maximum execution duration in milliseconds (default: no limit).
- `features` — additional Cargo features to enable (default: empty).
- `env_vars` — environment variables to set in the child process (default: empty).
- `edition` — Rust edition for generated manifests (default: `"2021"`).
- `package_name` — package name for generated manifests (default: `"script"`).

All fields default to their zero or absent value when not set — string fields default to empty string, collection fields default to empty collection, optional fields default to absent.

### Error Handling

No builder operations return errors. All fields accept any string value without validation. Construction always succeeds. Validation of field contents (path existence, valid Rust edition, valid Cargo manifest) occurs at execution time in the runner, not during plan construction.

### Compatibility Guarantees

Version 0.1.0, marked experimental. The public types and their field names are stable at this version, but builder method signatures may evolve as the workspace `former` crate evolves. Breaking changes are expected before stabilization. Callers should pin to an exact version.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | mod_interface layer declaration exposing public API boundary |
| source | `src/program.rs` | Struct definitions: Source, Program, Plan with Former derives |
| source | `src/run_options.rs` | RunOptions struct definition with Default impl |
| test | `tests/inc/basic.rs` | Complete builder chain: Plan → Program → Source round-trip |
| test | `tests/inc/corner_cases_test.rs` | Edge cases: empty fields, multiple sources, zero sources |
| config | `Cargo.toml` | Feature flags: `enabled` (master switch), `full` |
| doc | `docs/api/002_runner_api.md` | Runner API consuming the completed plan as input |
| doc | `docs/feature/001_script_execution.md` | Script execution feature using the plan produced here |
| doc | `docs/feature/005_configuration_surface.md` | Configuration surface: programmatic defaults layer |
| doc | `docs/pattern/001_builder_hierarchy.md` | Builder hierarchy pattern that this API implements |
| doc | `docs/pattern/002_layered_configuration.md` | Layered configuration: programmatic defaults anchored here |

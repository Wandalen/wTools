# API: Source, Program, and Plan

### Scope

- **Purpose**: Provide a builder API for constructing structured representations of multi-source programs.
- **Responsibility**: Documents the public interface for Source, Program, and Plan — their builder operations, field semantics, construction entry points, and compatibility guarantees.
- **In Scope**: Builder operations for all three types; field setters and their accepted values; nested builder entry and scalar operations.
- **Out of Scope**: Compilation and execution (→ future capability); internal Former derive mechanics; namespace import patterns (→ `feature/001`).

### Abstract

Three composable builder types for constructing program representations. `Plan` is the top-level entry point containing exactly one `Program`, which holds an ordered collection of `Source` entries. All three types use the Former builder pattern — construction begins at `Plan`, descends into `Program`, then into individual `Source` entries. Each builder returns to its parent on completion.

### Operations

**Source** — represents a single source file:
- `file_path` — the file path for the source within the program (e.g., `"main.rs"`, `"src/lib.rs"`). Any string value is accepted; no path validation is performed.
- `data` — the source code content as plain text. Any string value is accepted; no syntax validation is performed.
- Completing a Source appends it to the parent Program's source collection in insertion order and returns builder focus to the Program.

**Program** — an ordered collection of source files:
- `source` — opens a new Source builder entry. Each completed Source is appended to the Program's collection in order of construction.
- Completing a Program returns builder focus to the Plan.

**Plan** — the top-level execution configuration:
- `former` — the construction entry point; creates a new Plan builder.
- `program` — opens the Program builder for the single embedded program.
- Completing a Plan produces the final constructed Plan value.

All fields are initialized to their zero value when not set before completion — string fields default to empty string, collection fields default to empty collection.

### Error Handling

No builder operations return errors. All fields accept any string value without validation. Construction always succeeds — there are no failure modes at the definition layer. Validation of field contents (path existence, code syntax) is the responsibility of the downstream execution layer, not this crate.

### Compatibility Guarantees

Version 0.1.0, marked experimental. The public types (`Source`, `Program`, `Plan`) and their field names are stable at this version, but builder method signatures may evolve as the workspace `former` crate evolves. Breaking changes are expected before stabilization. Callers should pin to an exact version.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | mod_interface layer declaration exposing public API boundary |
| source | `src/program.rs` | Struct definitions: Source, Program, Plan with Former derives |
| test | `tests/inc/basic.rs` | Complete builder chain: Plan → Program → Source round-trip |
| test | `tests/inc/corner_cases_test.rs` | Edge cases: empty fields, multiple sources, zero sources, direct construction |
| config | `Cargo.toml` | Feature flags: `enabled` (master switch activating the public API), `full` |
| doc | `docs/feature/001_program_representation.md` | Design context: three-level hierarchy rationale and boundaries |

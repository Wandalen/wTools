# Feature: Program Representation

### Scope

- **Purpose**: Enable structured construction of multi-source Rust program representations through a fluent builder API.
- **Responsibility**: Documents the program representation feature — its three-level data model, builder design, namespace exports, and architectural boundaries.
- **In Scope**: Source, Program, and Plan builder construction; nested builder chaining; mod_interface namespace exports.
- **Out of Scope**: Program compilation and execution (→ future capability); Cargo.toml generation; process management (→ `process_tools`).

### Design

The crate models a Rust program as a three-level hierarchy:

**Source** — an atomic unit representing a single source file: a path and code content.
**Program** — an ordered collection of source files representing a compilable crate.
**Plan** — the top-level configuration wrapping exactly one Program.

Each level provides a fluent builder through the Former pattern. Construction begins at Plan and descends into Program, then into individual Source entries. Each nested builder returns to its parent on completion, producing the final Plan value when the outermost builder completes.

Source code content is stored as plain text, enabling in-memory construction for test and code-generation use cases where code is generated programmatically before any file I/O.

**Architectural boundary**: `program_tools` handles program definition only. Compilation and execution are delegated to `process_tools`. The two compose at the call site — `program_tools` constructs the Plan; `process_tools` writes files and invokes the build toolchain.

**Design rationale — three-level hierarchy**: Source is the atomic unit; Program groups sources into a compilable crate; Plan wraps the program in an execution configuration layer. Each level can grow independently — Plan may gain working directory and timeout fields; Program may gain manifest configuration; Source may gain encoding or conditional compilation metadata.

**Design rationale — plain text for source content**: String storage allows in-memory generation via format expressions, easy inspection for debugging, and clear ownership semantics. The tradeoff — no lazy loading or copy-on-write — is acceptable because program definitions are small and short-lived.

**Design rationale — Former pattern**: Consistent with the workspace builder convention; enables fluent nested construction with compile-time field tracking; subform-entry and subform-scalar annotations drive the three-level chain without manual builder wiring.

Both `exposed` and `prelude` namespaces export all three types, following the workspace module organization convention.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/program.rs` | Source, Program, Plan struct definitions with Former derives |
| source | `src/lib.rs` | mod_interface layer declaration and namespace exports |
| test | `tests/inc/basic.rs` | Validate complete nested builder chain |
| test | `tests/inc/corner_cases_test.rs` | Edge cases: empty fields, multiple sources, direct struct construction |
| config | `Cargo.toml` | Feature flags: `enabled` (master switch), `full` |
| doc | `docs/api/001_program_api.md` | API surface reference for Source, Program, Plan operations |

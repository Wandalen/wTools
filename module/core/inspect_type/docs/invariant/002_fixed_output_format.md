# Invariant: Fixed Output Format

### Scope

- **Purpose**: Guarantee that inspection output is stable and predictable across all inputs and versions, enabling reliable test assertions and programmatic parsing.
- **Responsibility**: Documents the fixed output format invariant — the exact format string, where it is produced, and what breaks if it changes.
- **In Scope**: The string returned by both inspection macros and emitted to stdout by the print-mode macro.
- **Out of Scope**: Whitespace within the expression text (controlled by the caller's source formatting), locale or character encoding of the process.

### Invariant Statement

For all inputs and all versions: every string produced by inspect_to_str_type_of and inspect_type_of has the form `sizeof( {expression_text} : {type_name} ) = {size_in_bytes}` where expression_text is the verbatim source text of the inspected expression, type_name is the fully qualified runtime type name, and size_in_bytes is a non-negative decimal integer.

### Enforcement Mechanism

The format string is constructed in a single location — the single-expression arm of the inspect_to_str_type_of macro in src/lib.rs. The inspect_type_of macro delegates entirely to inspect_to_str_type_of and cannot diverge in format. Any format change requires modifying that one location, making unintentional drift structurally impossible. Corner case tests (category 11 in tests/corner_cases_test.rs) assert the `sizeof(` prefix appears in every output.

### Violation Consequences

Changing the format is a breaking change for all callers that parse or assert the returned string. Test assertions across the test suite would fail. Documentation examples in readme.md would display incorrect output. Any downstream tooling relying on the fixed prefix for log parsing would break silently.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Single format string construction site for both macros |
| test | `tests/corner_cases_test.rs` | Assertions on format correctness (category 11: Macro Output Format) |
| doc | `docs/api/001_inspect_to_str_type_of.md` | String-mode API that produces this format |
| doc | `docs/api/002_inspect_type_of.md` | Print-mode API that produces and emits this format |
| doc | `docs/feature/001_type_inspection.md` | Feature whose output format this invariant governs |

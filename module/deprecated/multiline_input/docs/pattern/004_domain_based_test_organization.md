# Pattern: Domain-Based Test Organization

### Scope

- **Purpose**: Aligns test file layout with developer mental models to reduce navigation friction.
- **Responsibility**: Documents the rationale for grouping test files by functional domain rather than test methodology.
- **In Scope**: Domain grouping rationale, comparison with methodology-based alternatives, naming guidance.
- **Out of Scope**: Individual test file content (→ test source files), test harness configuration (→ source).

### Problem

Methodology-based test organization (unit/, integration/, e2e/) requires developers to classify a test before they can locate it. This classification is often ambiguous — the same test could reasonably belong in multiple categories. Developers navigate by thinking "I'm working on buffer operations", not "I'm looking for integration tests".

### Solution

Organize test files by the functional domain they cover, mirroring the conceptual structure of the codebase:
- One test file per functional domain (buffer operations, key handling, validation, full workflows, error paths)
- Shared test utilities in a `common/` subdirectory
- File names answer "what is being tested" without requiring methodology classification

### Applicability

- A codebase has two or more distinct functional domains
- Developers navigate tests by domain, not by methodology
- Methodology classification would be arbitrary or contested

### Consequences

- Test file location is predictable from the domain currently being modified
- No classification decisions required when creating new tests
- `common/` for shared utilities is unambiguous
- Tests that span domains should be placed in the primary domain or a dedicated workflows file by convention

### Cross-References

| Type   | File                                      | Responsibility                                              |
|--------|-------------------------------------------|-------------------------------------------------------------|
| test   | `tests/readme.md`                         | Test directory responsibility table and organization intent |
| test   | `tests/buffer_operations_test.rs`         | Domain: buffer state and cursor operations                  |
| test   | `tests/key_handling_test.rs`              | Domain: key event processing and bindings                   |
| test   | `tests/validation_test.rs`               | Domain: input validation logic                              |
| test   | `tests/integration_workflows_test.rs`     | Domain: end-to-end user workflows                           |
| test   | `tests/error_paths_test.rs`              | Domain: error conditions and recovery                       |
| doc    | `docs/feature/001_multiline_input.md`     | Feature that applies this pattern                           |

### Sources

| File                        | Notes                                                                                   |
|-----------------------------|-----------------------------------------------------------------------------------------|
| [../architecture.md](../architecture.md) | Combined source covering four patterns; concepts 1–3 extracted to 001–003 |

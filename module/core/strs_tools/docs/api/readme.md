# API Doc Entity

### Scope

- **Purpose**: Define the public programmatic interface contracts exposed by this crate.
- **Responsibility**: Index of API doc instances; each instance specifies operations, error handling, and compatibility guarantees for one interface group.
- **In Scope**: Public APIs: the split builder, string utility functions, and parser integration extension trait.
- **Out of Scope**: Internal SIMD mechanics (`algorithm/`); user-facing feature descriptions (`feature/`); correctness invariants (`invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Split API](001_split_api.md) | Builder-pattern interface for configuring and executing string splitting | ✅ |
| 002 | [String Utilities API](002_string_utilities_api.md) | Indentation, isolation, number parsing, and command parsing interfaces | ✅ |
| 003 | [Parser Integration API](003_parser_integration_api.md) | Extension trait for single-pass parsing operations on string types | ✅ |

# Invariant Doc Entity

### Scope

- **Purpose**: Capture architectural constraints that must always hold for format_tools, regardless of which features or code paths are used.
- **Responsibility**: Index of invariant doc instances, each defining one non-functional constraint with its enforcement mechanism and violation consequences.
- **In Scope**: Constraints on I/O behavior, output encoding, execution model, and macro implementation approach.
- **Out of Scope**: Behavioral requirements (→ feature/), API contracts (→ api/), data layouts (→ data_structure/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Pure Data Transformation](001_pure_data_transformation.md) | format_tools performs only string-to-string transformation; no I/O or system calls | ✅ |
| 002 | [No Color Styling](002_no_color_styling.md) | All output is plain ASCII text; no ANSI escape sequences | ✅ |
| 003 | [Synchronous Only](003_synchronous_only.md) | All formatting operations complete synchronously before returning | ✅ |
| 004 | [Declarative Macros Only](004_declarative_macros_only.md) | All macros are declarative; no procedural macro infrastructure is used | ✅ |

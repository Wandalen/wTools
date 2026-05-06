# Invariant Doc Entity

### Scope

- **Purpose**: Define the behavioral contracts and non-functional constraints that both include_md macros must uphold unconditionally.
- **Responsibility**: Behavioral contracts and NFR constraints both macros must uphold unconditionally.
- **In Scope**: Path resolution contract, compile-time error guarantee, file size constraint, and section extraction rules.
- **Out of Scope**: Feature design rationale (see docs/feature/), macro API contracts (see docs/api/), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Path Resolution](001_path_resolution.md) | Caller-relative path resolution contract | 🔄 |
| 002 | [Compile-Time Errors](002_compile_time_errors.md) | All failure modes produce compile-time errors | 🔄 |
| 003 | [Size Limit](003_size_limit.md) | 10 MB file size constraint | 🔄 |
| 004 | [Section Extraction Rules](004_section_extraction_rules.md) | Heading match and boundary detection invariants | 🔄 |

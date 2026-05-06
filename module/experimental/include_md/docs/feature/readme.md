# Feature Doc Entity

### Scope

- **Purpose**: Navigational hubs collecting all artifacts for each user-facing include_md capability.
- **Responsibility**: Design rationale for the file inclusion and section extraction capabilities.
- **In Scope**: The two user-facing markdown inclusion capabilities: full-file inclusion and targeted section extraction.
- **Out of Scope**: API contract details (see docs/api/), behavioral invariants (see docs/invariant/), implementation internals.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [File Inclusion](001_file_inclusion.md) | Compile-time embedding of a complete markdown file | 🔄 |
| 002 | [Section Extraction](002_section_extraction.md) | Compile-time embedding of a single named markdown section | 🔄 |

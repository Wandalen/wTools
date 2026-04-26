# API Doc Entity

### Scope

- **Purpose**: Defines the public macro interfaces include_md exposes for compile-time markdown inclusion.
- **Responsibility**: Macro contracts for include_md! and include_md_section! — parameters, output type, and error conditions.
- **In Scope**: Macro contracts for include_md! and include_md_section! — parameters, path semantics, output type, and error behavior.
- **Out of Scope**: Implementation internals, feature-level design rationale (see docs/feature/), behavioral invariants (see docs/invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [include_md Macro](001_include_md.md) | Compile-time full markdown file inclusion | 🔄 |
| 002 | [include_md_section Macro](002_include_md_section.md) | Compile-time targeted section extraction | 🔄 |

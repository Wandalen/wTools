# API Doc Entity

### Scope

- **Purpose**: Document the public interface exposed by `cli_fmt`.
- **Responsibility**: Master index for all API doc instances in this crate.
- **In Scope**: Instance 001 — output processing function, configuration, and result types; Instance 002 — CLI help template types and render method.
- **Out of Scope**: Behavioral rationale — see `feature/` and `invariant/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Output API](001_output_api.md) | Output processing contract — config, result, and processor function | ✅ |
| 002 | [Help Template API](002_help_api.md) | CLI help template public types and render method contract | ✅ |

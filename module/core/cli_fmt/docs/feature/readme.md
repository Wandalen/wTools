# Feature Doc Entity

### Scope

- **Purpose**: Document what `cli_fmt` does and why each capability exists.
- **Responsibility**: Behavioral capabilities — output filtering, truncation, stream merging, and CLI help template rendering.
- **In Scope**: Instance 001 — CLI output filtering, truncation, and stream merging pipeline; Instance 002 — typed CLI help template with style/data separation.
- **Out of Scope**: Public interface contracts — see `api/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Output Processing](001_output_processing.md) | CLI output filtering, truncation, and stream merging | ✅ |
| 002 | [CLI Help Template](002_cli_help_template.md) | Typed, configurable template for CLI help text rendering | ✅ |

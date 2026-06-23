# Feature Doc Entity

### Scope

- **Purpose**: Document what `cli_fmt` does and why each capability exists.
- **Responsibility**: Behavioral capabilities — output filtering, truncation, stream merging, and CLI help template rendering.
- **In Scope**: Instance 001 — CLI output filtering, truncation, and stream merging pipeline; Instance 002 — typed CLI help template with style/data separation.
- **Out of Scope**: Public interface contracts — see `api/` instances.

### Overview Table

| # | File | Name | Status |
|---|------|------|--------|
| 1 | [001_output_processing.md](001_output_processing.md) | Output Processing | ✅ |
| 2 | [002_cli_help_template.md](002_cli_help_template.md) | CLI Help Template | ✅ |

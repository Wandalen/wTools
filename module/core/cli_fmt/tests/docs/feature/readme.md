# Feature Test Specs

### Scope

- **Purpose**: Document test cases verifying behavioral requirements of cli_fmt feature doc instances.
- **Responsibility**: Master index for all feature test spec files in this directory.
- **In Scope**: FT-prefixed specs for Feature 001 (output processing) and Feature 002 (cli_help_template, FT-1..FT-20).
- **Out of Scope**: Test implementation code — see `tests/output.rs` (Feature 001) and `tests/help.rs` (Feature 002).

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `001_output_processing.md` | Feature test spec for Output Processing | ✅ |
| `002_cli_help_template.md` | Feature test spec for CLI Help Template | ✅ |

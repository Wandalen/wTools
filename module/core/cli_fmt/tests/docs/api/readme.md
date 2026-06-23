# API Test Specs

### Scope

- **Purpose**: Document test cases verifying the public API contract of cli_fmt.
- **Responsibility**: Test specifications verifying API contracts for output processing and CLI help rendering.
- **In Scope**: AP-prefixed specs for API 001 (output processing API — infallibility, builder, stream filter, result metadata, width suffix customization, merge_streams filter variants AP-13..AP-15) and API 002 (help template API — render infallibility, default fields, column padding, section omission, desc annotation, OptionGroup construction, CliHelpData::default()).
- **Out of Scope**: Internal processing logic — see `tests/docs/feature/` for feature-level behavioral specs.

### Overview Table

| # | File | Name | Status |
|---|------|------|--------|
| 1 | [001_output_api.md](001_output_api.md) | Output API | ✅ |
| 2 | [002_help_api.md](002_help_api.md) | Help Template API | ✅ |

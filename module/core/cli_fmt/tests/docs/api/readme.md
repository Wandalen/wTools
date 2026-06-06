# API Test Specs

### Scope

- **Purpose**: Document test cases verifying the public API contract of cli_fmt.
- **Responsibility**: Master index for all API test spec files in this directory.
- **In Scope**: AP-prefixed specs for API 001 (output processing API — infallibility, builder, stream filter, result metadata) and API 002 (help template API — render infallibility, default fields, column padding, section omission, desc annotation).
- **Out of Scope**: Internal processing logic — see `tests/docs/feature/` for feature-level behavioral specs.

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `001_output_api.md` | API test spec for Output API | ✅ |
| `002_help_api.md` | API test spec for Help Template API | ✅ |

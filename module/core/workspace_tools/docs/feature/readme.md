# Feature Doc Entity

### Scope

- **Purpose**: Document every user-facing capability of `workspace_tools` so developers understand what the crate does and why.
- **Responsibility**: Define scope, design rationale, and cross-references for each implemented feature; serve as the navigational hub from feature to source, tests, and API.
- **In Scope**: Feature scope definitions, language-agnostic design decisions, and cross-reference tables linking source files, test files, tasks, and related docs for each capability.
- **Out of Scope**: API method signatures (see `api/`), test documentation, implementation algorithms, and speculative future features.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Workspace Root Resolution](001_workspace_root_resolution.md) | Reliable workspace root detection across execution contexts | ✅ |
| 002 | [Configuration Loading](002_configuration_loading.md) | TOML/JSON/YAML typed configuration loading and merging | ✅ |
| 003 | [Secret Management](003_secret_management.md) | Secret file loading with memory-safe access and fallback chain | ✅ |
| 004 | [Resource Discovery](004_resource_discovery.md) | Glob-pattern resource discovery relative to workspace root | ✅ |
| 005 | [Configuration Validation](005_configuration_validation.md) | JSON Schema validation integrated with configuration loading | ✅ |
| 006 | [Testing Support](006_testing_support.md) | Isolated temporary workspace creation for test suites | ✅ |

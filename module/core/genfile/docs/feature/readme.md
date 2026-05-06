# Feature Doc Entity

### Scope

- **Purpose**: Documents functional requirements and user-facing capabilities of the genfile CLI.
- **Responsibility**: Index of all feature doc instances for genfile.
- **In Scope**: Functional requirements FR1-FR10.
- **Out of Scope**: Non-functional constraints (→ `invariant/`), CLI command reference (→ `cli/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Archive Lifecycle Management](001_archive_lifecycle_management.md) | Create, load, save, and build archives | ✅ |
| 002 | [File Content Operations](002_file_content_operations.md) | Add, remove, list, and show archive files | ✅ |
| 003 | [Parameter Definition Management](003_parameter_definition_management.md) | Define, list, and remove template parameters | ✅ |
| 004 | [Parameter Value Management](004_parameter_value_management.md) | Set, list, and clear runtime parameter values | ✅ |
| 005 | [Content Source Management](005_content_source_management.md) | Internalize and externalize external content references | ✅ |
| 006 | [Template Materialization](006_template_materialization.md) | Render templates to generated files with parameter substitution | ✅ |
| 007 | [Archive Serialization](007_archive_serialization.md) | Pack archives to portable self-contained files | ✅ |
| 008 | [Archive Analysis](008_archive_analysis.md) | Analyze archives for status, parameters, and readiness | ✅ |
| 009 | [Help System](009_help_system.md) | Universal and per-command help with auto-generation | ✅ |
| 010 | [REPL Mode](010_repl_mode.md) | Interactive multi-command session with archive state persistence | ✅ |

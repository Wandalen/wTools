# CLI Command Test Specs

### Scope

Test spec files for all CLI command namespaces in `docs/cli/command/`. Category-level specs use prefix `IT-` with namespace-scoped ID ranges. Per-command specs use prefix `cmd_NNN_` with the canonical command number.

### Responsibility Table

| File | Responsibility | Status |
|------|----------------|--------|
| 001_analysis.md | Spec cases for analysis commands (.info, .discover.parameters, .status, .analyze) | 🚧 |
| 002_archive.md | Spec cases for archive namespace (.archive.new/load/save/from_directory) | 🚧 |
| 003_content.md | Spec cases for content namespace (.content.internalize/externalize/list) | 🚧 |
| 004_file.md | Spec cases for file namespace (.file.add/remove/list/show) | 🚧 |
| 005_operations.md | Spec cases for core operations (.materialize, .unpack, .pack) | 🚧 |
| 006_param_mgmt.md | Spec cases for parameter namespace (.parameter.add/list/remove) | 🚧 |
| 007_value.md | Spec cases for value namespace (.value.set/list/clear) | 🚧 |

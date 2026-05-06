# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Catalogs each API interface — its operations, error handling, and compatibility guarantees | [api/readme.md](api/readme.md) | 4 |
| `feature/` | Documents each feature as a navigational hub — scope, design, and pointers to source, test, and API doc instances | [feature/readme.md](feature/readme.md) | 5 |
| `invariant/` | Documents each correctness guarantee — its statement, rationale, enforcement, and violation consequences | [invariant/readme.md](invariant/readme.md) | 4 |
| `pattern/` | Documents each reusable design pattern — context, problem, solution structure, and consequences | [pattern/readme.md](pattern/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Builder API | [api/001_builder_api.md](api/001_builder_api.md) |
| api | 002 | Runner API | [api/002_runner_api.md](api/002_runner_api.md) |
| api | 003 | Output API | [api/003_output_api.md](api/003_output_api.md) |
| api | 004 | CLI Interface | [api/004_cli_interface.md](api/004_cli_interface.md) |
| feature | 001 | Script Execution | [feature/001_script_execution.md](feature/001_script_execution.md) |
| feature | 002 | Output Capture and Comparison | [feature/002_output_capture.md](feature/002_output_capture.md) |
| feature | 003 | Artifact Management | [feature/003_artifact_management.md](feature/003_artifact_management.md) |
| feature | 004 | Programmatic Test Integration | [feature/004_programmatic_test_integration.md](feature/004_programmatic_test_integration.md) |
| feature | 005 | Configuration Surface | [feature/005_configuration_surface.md](feature/005_configuration_surface.md) |
| invariant | 001 | Cleanup Guarantee | [invariant/001_cleanup_guarantee.md](invariant/001_cleanup_guarantee.md) |
| invariant | 002 | Execution Isolation | [invariant/002_execution_isolation.md](invariant/002_execution_isolation.md) |
| invariant | 003 | Output Determinism | [invariant/003_output_determinism.md](invariant/003_output_determinism.md) |
| invariant | 004 | Error Propagation | [invariant/004_error_propagation.md](invariant/004_error_propagation.md) |
| pattern | 001 | Builder Hierarchy | [pattern/001_builder_hierarchy.md](pattern/001_builder_hierarchy.md) |
| pattern | 002 | Layered Configuration | [pattern/002_layered_configuration.md](pattern/002_layered_configuration.md) |

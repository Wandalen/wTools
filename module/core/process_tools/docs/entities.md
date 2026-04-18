# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public API surface: type signatures, function contracts, return type semantics | [api/readme.md](api/readme.md) | 7 |
| `feature/` | Functional capabilities: what each feature does, why it exists | [feature/readme.md](feature/readme.md) | 5 |
| `guide/` | Cross-module usage workflows and integration patterns | [guide/readme.md](guide/readme.md) | 2 |
| `invariant/` | Behavioral contracts and platform guarantees | [invariant/readme.md](invariant/readme.md) | 4 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | Run Builder | [api/001_run_api.md](api/001_run_api.md) |
| api | 002 | Report | [api/002_report_api.md](api/002_report_api.md) |
| api | 003 | Exit Status Synthesis | [api/003_exit_status_api.md](api/003_exit_status_api.md) |
| api | 004 | Signal Lookup | [api/004_signal_api.md](api/004_signal_api.md) |
| api | 005 | Process Liveness Check | [api/005_check_api.md](api/005_check_api.md) |
| api | 006 | Unix Daemonization | [api/006_daemon_api.md](api/006_daemon_api.md) |
| api | 007 | Environment Detection | [api/007_environment_api.md](api/007_environment_api.md) |
| feature | 001 | Process Execution | [feature/001_process_execution.md](feature/001_process_execution.md) |
| feature | 002 | Output Capture | [feature/002_output_capture.md](feature/002_output_capture.md) |
| feature | 003 | CI/CD Environment Detection | [feature/003_environment_detection.md](feature/003_environment_detection.md) |
| feature | 004 | Exit Status Synthesis | [feature/004_exit_status_synthesis.md](feature/004_exit_status_synthesis.md) |
| feature | 005 | Process Lifecycle Management | [feature/005_lifecycle_management.md](feature/005_lifecycle_management.md) |
| guide | 001 | Daemon Monitoring | [guide/001_daemon_monitoring.md](guide/001_daemon_monitoring.md) |
| guide | 002 | Test Exit Status Fixtures | [guide/002_test_exit_status.md](guide/002_test_exit_status.md) |
| invariant | 001 | Result<Report, Report> Contract | [invariant/001_result_contract.md](invariant/001_result_contract.md) |
| invariant | 002 | Cross-Platform Shell Abstraction | [invariant/002_cross_platform_shell.md](invariant/002_cross_platform_shell.md) |
| invariant | 003 | PID File Format | [invariant/003_pidfile_format.md](invariant/003_pidfile_format.md) |
| invariant | 004 | EPERM Means Process Is Alive | [invariant/004_eperm_means_alive.md](invariant/004_eperm_means_alive.md) |

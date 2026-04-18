# Tests Directory

Test suite for process_tools crate covering subprocess execution, output capture, CI/CD environment detection, exit status synthesis, and process lifecycle management.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| tests.rs | Test suite entry point with feature gating |
| smoke_test.rs | Package health verification (local and published) |
| manual_execution_test.rs | Comprehensive process execution tests |
| manual_edge_cases_test.rs | Edge case and corner case tests |
| inc/ | Aggregated integration test modules |
| tool/ | Shared test utility code |
| asset/ | Test fixture source files |
| exit_status_test.rs | Platform-agnostic `ExitStatus` synthesis tests |
| lifecycle_signal_test.rs | POSIX signal name/number bidirectional mapping tests |
| lifecycle_check_test.rs | Process-alive detection and PID file tests |
| lifecycle_daemon_test.rs | PID file round-trip tests for daemon utilities |
| manual/ | Manual testing plan and documentation |

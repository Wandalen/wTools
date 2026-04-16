# Tests Directory

Test suite for process_tools crate providing comprehensive coverage of subprocess execution and CI/CD environment detection.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| tests.rs | Test suite entry point with feature gating |
| smoke_test.rs | Package health verification (local and published) |
| manual_execution_test.rs | Comprehensive process execution tests (14 cases) |
| manual_edge_cases_test.rs | Edge case and corner case tests (16 cases) |
| inc/ | Aggregated integration test modules |
| tool/ | Shared test utility code |
| asset/ | Test fixture source files |
| exit_status_test.rs | Platform-agnostic `ExitStatus` synthesis tests (5 cases) |
| lifecycle_signal_test.rs | POSIX signal name/number bidirectional mapping tests (8 cases) |
| lifecycle_check_test.rs | Process-alive detection and PID file tests (7 cases) |
| lifecycle_daemon_test.rs | PID file round-trip test for daemon utilities (1 case) |
| manual/ | Manual testing plan and documentation |

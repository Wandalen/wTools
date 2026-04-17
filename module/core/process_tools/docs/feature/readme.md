# Feature Documentation

## Purpose

Functional capability descriptions for `process_tools`: what each feature does, why it exists, and when to use it.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_process_execution.md` | Subprocess execution via builder pattern (`Run`, `run()`) |
| `002_output_capture.md` | Stdout/stderr capture and display via `Report` struct |
| `003_environment_detection.md` | CI/CD environment detection (`is_cicd()`) |
| `004_exit_status_synthesis.md` | Platform-agnostic `ExitStatus` construction without spawning |
| `005_lifecycle_management.md` | Process alive checks, signal mapping, Unix daemonization |

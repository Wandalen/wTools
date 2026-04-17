# Invariant Documentation

## Purpose

Behavioral contracts and platform guarantees for `process_tools`: constraints that hold across all invocations.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_result_contract.md` | `Result<Report, Report>` contract: both paths carry full diagnostic context |
| `002_cross_platform_shell.md` | Shell abstraction invariant: `run_with_shell` always invokes the platform-native shell |

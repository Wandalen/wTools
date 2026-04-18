# Invariant Doc Entity

### Scope

- **Purpose:** Document behavioral contracts and platform guarantees of `process_tools` — constraints that hold across all invocations.
- **Responsibility:** Collect one doc instance per invariant; specify enforcement mechanism and violation consequences.
- **In Scope:** Formal invariant statements, how they are enforced in code, and what breaks when violated.
- **Out of Scope:** Function signatures (→ `api/`); feature design rationale (→ `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Result<Report, Report> Contract](001_result_contract.md) | Both `Ok` and `Err` carry fully-populated `Report` | ✅ |
| 002 | [Cross-Platform Shell Abstraction](002_cross_platform_shell.md) | `run_with_shell` always invokes the platform-native shell | ✅ |
| 003 | [PID File Format](003_pidfile_format.md) | Decimal integer format shared between `daemon` and `check` modules | ✅ |
| 004 | [EPERM Means Process Is Alive](004_eperm_means_alive.md) | `EPERM` from `kill(pid,0)` indicates liveness, not error | ✅ |

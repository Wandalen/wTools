# Invariant: Performance

### Scope

- **Purpose**: Bounds command execution and REPL startup latency to ensure acceptable responsiveness.
- **Responsibility**: Documents the performance constraints and their measurement workloads.
- **In Scope**: Per-command latency, REPL startup, pack (100 files), and materialize (10 files) targets.
- **Out of Scope**: Library-level rendering latency (→ genfile_core invariant/001).

### Invariant Statement

Typical command execution (archive load, parameter set, file add) must complete within 100ms. REPL startup must complete within 500ms. Pack operation over 100 files must complete within 2s. Materialize over 10 files must complete within 1s.

### Enforcement Mechanism

Measured via integration tests that time command sequences end-to-end. Standalone binary execution is significantly faster than test-harness invocations, which include compilation overhead. No formal benchmark suite exists yet; integration test timing serves as the practical measurement baseline.

### Violation Consequences

Excessive latency makes the CLI unsuitable for interactive use in REPL mode and for CI/CD automation where many commands are chained together.

### Features

| File | Relationship |
|------|--------------|
| [`feature/010_repl_mode.md`](../feature/010_repl_mode.md) | REPL mode whose startup latency this invariant bounds |

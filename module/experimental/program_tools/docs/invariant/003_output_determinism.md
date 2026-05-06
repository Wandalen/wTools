# Invariant: Output Determinism

### Scope

- **Purpose**: Guarantee that captured output is reproducible for identical inputs and runtime configuration.
- **Responsibility**: Documents what determinism means for captured output, its preconditions, and its boundaries.
- **In Scope**: Standard output and standard error capture completeness; exit status; preconditions for determinism.
- **Out of Scope**: Output of programs that are themselves non-deterministic (random number generators, timestamps, thread-interleaved writes); timing information.

### Statement

For identical source content, identical runtime configuration, and a deterministic target program, the runner produces identical captured output on repeated invocations.

### Rationale

Test assertions against captured output are only meaningful when output is reproducible. Non-deterministic capture — for example, lost lines due to early pipe closure — would cause test failures that are impossible to trace back to actual regressions.

### Enforcement

Standard output and standard error are captured through the subprocess pipe mechanism into separate in-memory buffers. The runner waits for the child process to exit before reading any captured content, ensuring the complete output is available. No line-buffering or stream interleaving is applied by the runner — the raw output is preserved exactly as produced by the child process.

### Violations

Programs that write to stdout and stderr concurrently from multiple threads may produce non-deterministic interleaving between the two streams. This is a property of the target program, not of `program_tools`. The content of each stream individually is always captured completely — only cross-stream byte ordering may vary across runs.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/feature/002_output_capture.md` | Output capture feature that this invariant governs |
| doc | `docs/api/003_output_api.md` | Output assertion API that relies on this guarantee |
| doc | `docs/invariant/002_execution_isolation.md` | Isolation that prevents cross-run output contamination |
| test | `tests/inc/runner_test.rs` | Integration tests verifying identical output on repeated identical runs |

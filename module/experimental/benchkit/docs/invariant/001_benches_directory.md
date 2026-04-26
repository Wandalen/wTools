# Invariant: Benches Directory Mandate

### Scope

- **Purpose**: Ensure benchmarks are placed where Cargo and tooling expect them, preventing integration failures.
- **Responsibility**: States the required placement rule for all benchmark files that use benchkit.
- **In Scope**: Placement of benchmark source files relative to the crate root.
- **Out of Scope**: Internal benchkit module placement (→ src/); test file placement (→ tests/).

### Invariant Statement

All benchmark files that use benchkit MUST reside inside the `benches/` directory of their crate. No benchmark file may exist in `tests/`, `examples/`, `src/`, or any other directory.

### Enforcement Mechanism

At startup in debug builds, benchkit checks the current working directory name. If the directory is not `benches/`, a warning is emitted to stderr identifying the incorrect location and the required location. The check is advisory — it does not abort execution — because build tooling may invoke binaries from parent directories.

### Violation Consequences

A benchmark placed outside `benches/` will not be discovered by `cargo bench`, causing silent omission from CI performance checks. Benchmarks in `tests/` may be incorrectly treated as unit tests and run without benchmark-specific optimizations, producing inflated timing measurements that do not reflect production performance.

### Cross-References

| Type   | File                                       | Responsibility                                       |
|--------|--------------------------------------------|------------------------------------------------------|
| source | `src/lib.rs`                               | check_directory_recommendations() enforcement call   |
| doc    | `docs/feature/001_measurement_timing.md`   | Feature that this invariant constrains               |

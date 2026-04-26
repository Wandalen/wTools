# Invariant: Integration Ease Constraint

### Scope

- **Purpose**: Ensure benchkit remains accessible to developers who want to add benchmarking without restructuring their project.
- **Responsibility**: States the measurable integration effort thresholds and adoption path requirements.
- **In Scope**: Lines of code required for a first working benchmark; sensibility of defaults; compatibility with existing tooling.
- **Out of Scope**: Advanced configuration depth (developers may use as many lines as they choose for complex setups); documentation quality standards (→ feature/003).

### Invariant Statement

Three conditions must hold:
1. A developer must be able to write a working benchmark that produces results using 10 or fewer lines of code (excluding import statements and the function signature).
2. Default configuration — with no explicit options provided — must produce correct, useful output for the most common benchmarking scenario: timing a single closure and printing the result.
3. Adding benchkit to a project must not require removal or modification of any existing code, test, or benchmark.

### Enforcement Mechanism

The core timing path accepts a single closure and returns a result with no required configuration — all parameters have defaults. This is verified by the minimal-integration example in `examples/` which deliberately uses no configuration options.

The 10-line threshold is enforced by design review: any change to the public API that would increase the minimal example beyond 10 lines is rejected. The minimal example is part of the test suite (compiles and runs in CI).

Condition 3 is structural: benchkit is a dev-dependency only. It does not replace or conflict with any existing dependency, including criterion.

### Violation Consequences

Exceeding the 10-line threshold signals an API ergonomics regression — the crate has become more complex to start with, which is the primary barrier to adoption for a benchmarking utility. Developers abandon tools that require extensive setup.

Breaking condition 3 (requiring project restructuring) directly contradicts the toolkit-not-framework design principle (→ pattern/001) and forces adoption to be wholesale rather than incremental.

### Cross-References

| Type   | File                                       | Responsibility                                           |
|--------|--------------------------------------------|----------------------------------------------------------|
| source | `src/lib.rs`                               | Prelude re-exports that define the minimal API surface   |
| source | `src/measurement.rs`                       | Timing primitives with default parameters                |
| test   | `examples/`                                | Minimal integration example (≤10 lines) verified in CI  |
| doc    | `docs/feature/001_measurement_timing.md`   | Feature whose usability this invariant constrains        |
| doc    | `docs/pattern/001_toolkit_not_framework.md` | Architectural principle this invariant operationalises  |

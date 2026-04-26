# Pattern: Test Double Terminal

### Scope

- **Purpose**: Provides a complete, state-programmable terminal implementation for use in tests without mocking frameworks.
- **Responsibility**: Documents the test double design — programmable input, output capture, and explicit state control.
- **In Scope**: Test double semantics, input injection lifecycle, output capture, distinction from mocking.
- **Out of Scope**: Trait abstraction mechanism (→ `001_trait_based_di.md`), production terminal behavior (→ source comments).

### Problem

Tests need deterministic control of terminal behavior — injecting key events, simulating error conditions, capturing output — without a real terminal. External mocking frameworks add dependencies, create code paths separate from production, and are prohibited by codebase hygiene rules.

### Solution

Implement a complete, real implementation of the terminal trait backed by programmable in-memory state: key events are pre-loaded into a queue before each test; output is captured to an in-memory buffer during the test; terminal properties such as TTY presence and dimensions are set explicitly per test scenario. Because this is a full trait implementation with real logic — not intercepted expectations — it is a test double, not a mock.

### Applicability

- External mocking frameworks are prohibited
- Tests need deterministic control of interactive I/O
- Multiple error scenarios must be exercised without environment setup
- The trait-based DI pattern (see `001_trait_based_di.md`) is already in place

### Consequences

- Tests exercise real code paths — same control flow as production
- No external library dependency; test double is maintained in the codebase
- Output assertions are precise — captured exactly what the editor wrote
- Deterministic behavior without any environment configuration
- Test double must be updated when the terminal trait interface evolves

### Cross-References

| Type   | File                                       | Responsibility                                                      |
|--------|--------------------------------------------|---------------------------------------------------------------------|
| source | `tests/common/mock_terminal.rs`            | MockTerminal: the programmable terminal test double                 |
| test   | `tests/integration_workflows_test.rs`      | Integration tests using MockTerminal for end-to-end workflow coverage |
| test   | `tests/error_paths_test.rs`               | Error path tests using MockTerminal to inject failure conditions    |
| doc    | `docs/pattern/001_trait_based_di.md`       | Prerequisite: the trait abstraction this test double implements     |
| doc    | `docs/feature/001_multiline_input.md`      | Feature that applies this pattern                                   |

### Sources

| File                        | Notes                                                                                   |
|-----------------------------|-----------------------------------------------------------------------------------------|
| [../architecture.md](../architecture.md) | Combined source covering four patterns; concepts 1, 3, 4 extracted to 001, 003, 004 |

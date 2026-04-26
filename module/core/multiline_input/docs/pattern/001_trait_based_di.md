# Pattern: Trait-Based Dependency Injection

### Scope

- **Purpose**: Decouples terminal operations from business logic to enable testing without a real TTY.
- **Responsibility**: Documents the trait abstraction approach for injectable terminal dependencies.
- **In Scope**: Trait design rationale, separation of production and test implementations, tradeoffs.
- **Out of Scope**: MockTerminal specifics (→ `002_test_double_terminal.md`), terminal rendering behavior (→ source comments).

### Problem

Terminal-dependent code cannot be tested without a real TTY attachment. Hard dependencies on system calls block error path coverage, prevent integration testing in CI/CD environments where no TTY is available, and make test execution non-deterministic.

Alternative approaches evaluated:
- Conditional compilation: creates separate code paths, cannot test production code paths
- Environment variable checks: fragile, silent failures if variables not set
- External mocking frameworks: adds dependency, separate code path from production, prohibited by codebase hygiene

### Solution

Introduce a trait that abstracts all terminal operations. Provide two implementations: a production implementation that delegates to the real terminal subsystem, and a test implementation that simulates terminal behavior with programmable state. The public API's builder pattern hides the generic type parameter from callers, so users interact with a simple interface while internals remain fully testable.

### Applicability

- Code depends on system-level I/O that cannot be controlled in tests
- Error paths in I/O operations need explicit coverage
- The codebase prohibits external mocking frameworks
- CI/CD execution requires environment-independence (no TTY)

### Consequences

- All error paths become testable by injecting failure conditions into the test implementation
- Tests run deterministically in any environment — no TTY required
- No dependency on external mocking libraries
- Generic type parameters propagate up the call stack (mitigated by the builder pattern at the public API boundary)
- Test implementation must be maintained in sync with the trait contract when the trait evolves

### Cross-References

| Type   | File                                            | Responsibility                                               |
|--------|-------------------------------------------------|--------------------------------------------------------------|
| source | `src/terminal.rs`                               | TerminalOps trait definition and RealTerminal implementation |
| test   | `tests/common/mock_terminal.rs`                 | Test implementation of TerminalOps — see pattern/002        |
| test   | `tests/terminal_basic_test.rs`                  | Tests verifying terminal behavior via the trait              |
| doc    | `docs/pattern/002_test_double_terminal.md`      | Companion pattern: test double implementing this trait       |
| doc    | `docs/feature/001_multiline_input.md`           | Feature that applies this pattern                           |

### Sources

| File                        | Notes                                                                               |
|-----------------------------|-------------------------------------------------------------------------------------|
| [../architecture.md](../architecture.md) | Combined source covering four patterns; concepts 2–4 extracted to 002–004 |

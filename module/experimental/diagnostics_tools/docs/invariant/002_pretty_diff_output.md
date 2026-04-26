# Invariant: Equality Assertions Produce Colored Diff Output

### Scope

- **Purpose**: Guarantee that equality assertion failures always present a colored side-by-side diff rather than a plain value dump.
- **Responsibility**: Documents the behavioral contract for a_id and a_not_id — their failure output format and the upstream delegation that enforces it.
- **In Scope**: a_id and a_not_id failure output; pretty_assertions as the exclusive equality assertion backend.
- **Out of Scope**: a_true, a_false output format; debug-only variant output.

### Invariant Statement

The a_id and a_not_id macros always delegate to pretty_assertions assert_eq and assert_ne respectively. This guarantees that any equality assertion failure presents a colored, line-by-line diff between the left and right values rather than a raw Debug dump.

### Enforcement Mechanism

- Source inspection: a_id and a_not_id expand directly to pretty_assertions assert_eq / assert_ne — no intermediate formatting layer exists.
- The pretty_assertions dependency is required when diagnostics_runtime_assertions is enabled.

### Violation Consequences

Replacing the pretty_assertions delegation with standard assert_eq / assert_ne would silently downgrade failure output to raw Debug formatting, degrading the diagnostic value of equality assertion failures.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_runtime_assertions.md](../feature/001_runtime_assertions.md) | Runtime assertions feature subject to this invariant |
| source | [src/diag/rta.rs](../../src/diag/rta.rs) | a_id and a_not_id macro implementations that enforce delegation to pretty_assertions |
| test | [tests/inc/rta_test.rs](../../tests/inc/rta_test.rs) | Tests verifying colored diff output on equality assertion failure |

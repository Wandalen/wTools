# Invariant Doc Entity

### Scope

- **Purpose**: Documents non-negotiable behavioral constraints that must always hold for correct operation.
- **Responsibility**: Indexes all invariant doc instances in the `invariant/` collection.
- **In Scope**: Invariant statement, enforcement mechanism, and consequences of violation.
- **Out of Scope**: Performance benchmarks (→ `benches/`), algorithmic correctness proofs (→ `algorithm/`).

### Overview Table

| ID  | Name                                                            | Purpose                                               | Status |
|-----|-----------------------------------------------------------------|-------------------------------------------------------|--------|
| 001 | [Terminal Minimum Size](001_terminal_minimum_size.md)           | Minimum terminal dimensions required for operation    | ✅     |

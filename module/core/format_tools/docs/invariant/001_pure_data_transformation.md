# Invariant: Pure Data Transformation

### Scope

- **Purpose**: Ensure format_tools remains a pure string transformation library with no interaction with the system I/O layer.
- **Responsibility**: States the pure transformation constraint, how it is enforced, and what breaks if violated.
- **In Scope**: Absence of file handles, network sockets, terminal state, process spawning, or any system call from formatting operations.
- **Out of Scope**: Allocation behavior (not constrained), caller-level I/O (callers may write outputs anywhere).

### Invariant Statement

All formatting operations in format_tools accept values and produce strings. No formatting operation accepts or returns an I/O handle, writes to a file descriptor, spawns a process, queries terminal state, or makes a system call. Verified by: no I/O interface implementations in the format module; no operating system API imports.

### Enforcement Mechanism

The module boundary enforces this constraint structurally: the format module imports no I/O libraries. Output accumulates in an in-memory buffer (see data_structure/002_context.md). Callers receive the finished string and decide where to write it. Verified via dependency inspection and code review.

### Violation Consequences

Introducing I/O operations into formatting code would couple format_tools to the execution environment, break portability to embedded and bare-metal targets without standard library support, and force callers to manage I/O context alongside formatting configuration — conflating two independent concerns.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format.rs` | Format module root — bounded to pure string operations |
| doc | `docs/data_structure/002_context.md` | In-memory output buffer — satisfies the no-I/O constraint |
| doc | `docs/feature/002_table_formatting.md` | Primary feature constrained by this invariant |
| doc | `docs/feature/004_text_manipulation.md` | Text manipulation constrained by this invariant |

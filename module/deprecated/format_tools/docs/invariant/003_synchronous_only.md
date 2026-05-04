# Invariant: Synchronous Only

### Scope

- **Purpose**: Ensure all format_tools formatting operations are synchronous and complete before returning to the caller.
- **Responsibility**: States the synchronous execution constraint, how it is enforced, and what breaks if violated.
- **In Scope**: Absence of async functions, futures, executors, wakers, and runtime dependencies in format_tools.
- **Out of Scope**: How callers choose to invoke format_tools (callers may call from async contexts without issue).

### Invariant Statement

All formatting operations in format_tools are synchronous: they execute to completion within the calling thread and return a result directly. No function in format_tools is declared asynchronous, returns a future, or registers a waker. Verified by: no asynchronous keywords in source files; no async-runtime dependencies in Cargo.toml.

### Enforcement Mechanism

No async runtime is listed in dependencies. No asynchronous functions appear in source. Absence verified by dependency inspection and source search. Code review enforces at contribution time.

### Violation Consequences

Async formatting would require an executor runtime dependency, propagate async coloring to all callers, and increase compile time significantly — disproportionate overhead for string formatting operations, which are inherently fast and CPU-bound.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Dependency manifest — absence of async runtimes enforces this invariant |
| doc | `docs/feature/001_fallback_string_conversion.md` | Fallback conversion constrained by this invariant |
| doc | `docs/feature/002_table_formatting.md` | Table formatting constrained by this invariant |

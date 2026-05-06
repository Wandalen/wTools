# Pattern: Builder Hierarchy

### Scope

- **Purpose**: Document the three-level fluent builder chain used to construct execution plans.
- **Responsibility**: Describes the structural pattern, its nesting semantics, and how the workspace Former framework drives it without manual wiring.
- **In Scope**: Plan → Program → Source construction chain; subform entry and scalar annotations; builder focus and return semantics.
- **Out of Scope**: Runtime execution of completed plans (→ `feature/001`); Former framework internals; namespace export conventions.

### Context

An execution plan contains three conceptually distinct levels: a top-level configuration holding execution parameters, an intermediate grouping of source files into a compilable crate, and individual source entries each representing a single file. Each level has independent fields and is expected to grow independently as the crate evolves.

### Problem

Without a structured nesting approach, callers must construct deeply nested configuration objects manually — creating inner objects, setting their fields, then inserting them into outer containers. This produces verbose, error-prone construction code with unclear ownership boundaries and no compile-time guarantees about completeness.

### Solution

Use the workspace Former framework with subform annotations to drive a fluent three-level builder. Construction begins at the outermost level (Plan), which opens a nested builder for the intermediate level (Program). The intermediate builder opens builders for individual entries (Source). Each inner builder returns focus to its parent on completion, and the final value is produced when the outermost builder completes.

The three levels are:

- **Plan** — the outermost builder; holds execution configuration and opens exactly one Program builder
- **Program** — the intermediate builder; holds manifest configuration and accumulates Source builders in order
- **Source** — the innermost builder; holds a single file path and its source code content

Two Former annotations drive the nesting: `subform_scalar` on the Program field (exactly one nested instance) and `subform_entry` on the Source collection (any number of instances appended in order).

### Consequences

**Benefits**: Construction code mirrors the configuration hierarchy; compile-time guarantees prevent missing required fields; the Former framework handles all builder wiring without manual implementation; each level can grow independently without affecting the others.

**Tradeoffs**: Callers must understand which builder level they are currently inside, which can be disorienting in long chains; the pattern introduces a dependency on the Former crate; deeply nested chains become horizontally wide in formatted code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/program.rs` | Struct definitions with Former derives and subform annotations |
| doc | `docs/api/001_builder_api.md` | API surface documentation for the three-level chain |
| doc | `docs/feature/001_script_execution.md` | Script execution feature consuming plans built by this pattern |
| doc | `docs/invariant/001_cleanup_guarantee.md` | Cleanup guarantee configurable through the Plan builder |

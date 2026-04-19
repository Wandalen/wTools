# Pattern: Design Principles

### Scope

- **Purpose**: Document the eleven guiding principles that govern library design decisions.
- **Responsibility**: Canonical reference for design principles invoked in architecture and feature decisions.
- **In Scope**: All 11 named principles with descriptions; purpose of the principle set as a whole.
- **Out of Scope**: Layer decomposition (see `001_three_layer_architecture.md`), formatter design (see `003_formatter_design.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../architecture.md` | Original combined architecture document (retained per migration rules) |
| doc | `../invariant/001_data_model.md` | Invariants derived from these principles |

### Problem

Without an explicit set of governing principles, individual design decisions accumulate inconsistently. New formatters, features, or dependencies get added without accounting for their effect on the overall system. Over time, the library loses coherence: some formatters have dependencies while others claim to be zero-cost, some APIs are flexible while others are rigid, and callers face an unpredictable surface.

### Solution

Eleven named principles constrain specific design dimensions. Each principle governs a distinct aspect: data model uniformity, API unification, feature granularity, dependency minimization, genericity, ANSI handling, output flexibility, ergonomic construction, and modular code structure.

| # | Principle | Scope |
|---|-----------|-------|
| 1 | **Single Data Structure** — `TreeNode< T >` for all data (hierarchical and tabular) | Data layer |
| 2 | **Unified Format Interface** — same API for all formatters via Format trait | Formatter layer |
| 3 | **Canonical Data Format** — `TableView` struct for format-agnostic code | Builders layer |
| 4 | **Granular Features** — zero-cost abstractions with optional formatters | Build system |
| 5 | **Mutual Replaceability** — any data can be displayed in any format | Cross-layer |
| 6 | **Minimal Dependencies** — core has zero dependencies, formatters are optional | Build system |
| 7 | **Generic** — works with any data type via `TreeNode< T >` | Data layer |
| 8 | **ANSI-Aware** — proper alignment with color codes | Formatter layer |
| 9 | **Flexible Output** — String return and `io::Write` support | Formatter layer |
| 10 | **Helper Traits** — ergonomic builders and traits for table-shaped trees | Builders layer |
| 11 | **Modular Architecture** — separated concerns across 16 source modules | Code structure |

### Applicability

Invoke these principles when evaluating new feature proposals, choosing between design alternatives, or deciding whether to accept a trade-off. Any proposed change that would violate a principle requires explicit documentation of the trade-off accepted and why the benefit outweighs the cost.

### Consequences

Principles 1 and 5 (Single Data Structure + Mutual Replaceability) together enforce that no formatter receives a data type it cannot handle, keeping the API surface minimal. Principles 4 and 6 (Granular Features + Minimal Dependencies) ensure users pay only for what they enable. Principle 8 (ANSI-Aware) is non-negotiable for terminal-facing output where naive string length breaks column alignment. Decisions that violate any principle must be explicitly documented.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Design Principles" extracted into this instance |

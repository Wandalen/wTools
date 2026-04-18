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

### Description

These principles were established at library inception to ensure the design remains coherent as new formatters and features are added. Each principle constrains a specific design dimension. Decisions that would violate any principle require explicit documentation of the trade-off.

### Structure

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

### Rationale

Principles 1 and 5 (Single Data Structure + Mutual Replaceability) together enforce that no formatter receives a data type it cannot handle, keeping the API surface minimal. Principles 4 and 6 (Granular Features + Minimal Dependencies) ensure users pay only for what they enable. Principle 8 (ANSI-Aware) is non-negotiable for terminal-facing output where naive string length breaks column alignment.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Design Principles" extracted into this instance |

# src/formatters/table

## Purpose
Contains the `TableFormatter` implementation split across focused source files.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | `TableFormatter` struct, core rendering, row/border formatting methods |
| `auto_fit.rs` | Auto-wrap and fold helpers: terminal sizing, budgets, column folding |

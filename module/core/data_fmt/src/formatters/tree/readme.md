# src/formatters/tree/

`TreeFormatter` implementation split by rendering strategy.

| File | Responsibility |
|------|----------------|
| `mod.rs` | `TreeFormatter` struct, `format()`, `format_node()`, `write_to()` |
| `aligned.rs` | Column-aligned two-pass rendering: `format_aligned()` and helpers |
| `aggregated.rs` | Aggregation-aware rendering: `format_with_aggregation()` and helpers |

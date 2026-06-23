# Pattern: Design Principles

### Scope

- **Purpose**: Drive test coverage for the eleven design principles governing library decisions.
- **Responsibility**: Documents test cases for the design principles in `docs/pattern/002_design_principles.md`.
- **In Scope**: Single Data Structure principle, Unified Format Interface principle, Granular Features principle observability.
- **Out of Scope**: Layer decomposition (see `001_three_layer_architecture.md`), formatter design (see `003_formatter_design.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| PT-1 | Single Data Structure principle observable | ⏳ |
| PT-2 | Unified Format Interface principle observable | ⏳ |
| PT-3 | Granular Features principle observable | ⏳ |

---

### PT-1: Single Data Structure principle observable

- **Given:** The `data_fmt` crate data layer.
- **When:** Inspecting the data types used for both hierarchical and tabular input.
- **Then:** `TreeNode` serves both hierarchical and tabular use cases; no separate tabular-only data struct exists at the data layer; `TableView` is a builder output type, not a parallel data structure.

---

### PT-2: Unified Format Interface principle observable

- **Given:** All 10 formatters in the crate.
- **When:** Checking which formatters implement the `Format` trait.
- **Then:** 9 of 10 formatters implement `Format`; `TreeFormatter` is the sole exception (uses direct method dispatch due to generic type parameters); all `Format` implementors accept `&TableView` and return `Result<String, FormatError>`.

---

### PT-3: Granular Features principle observable

- **Given:** The crate `Cargo.toml` feature flags.
- **When:** Building with only `default` features enabled.
- **Then:** Core types (`TreeNode`, `TableView`, `RowBuilder`, `TableFormatter`) are available; serialization formatters (`JsonFormatter`, `YamlFormatter`, `TomlFormatter`) require their respective feature flags (`format_json`, `format_yaml`, `format_toml`); `HtmlFormatter` requires `format_html`; `SqlFormatter` requires `format_sql`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/pattern/002_design_principles.md`](../../../docs/pattern/002_design_principles.md) | Source pattern doc — eleven guiding principles |

# Feature: Unified Format Interface

### Scope

- **Purpose**: Drive test coverage for the unified format interface feature.
- **Responsibility**: Documents test cases for the `Format` trait and multi-formatter dispatch in `docs/feature/003_unified_format_interface.md`.
- **In Scope**: `TableView` production via `build_view()`, nine `Format` trait implementors (TableFormatter, ExpandedFormatter, LogfmtFormatter, HtmlFormatter, SqlFormatter, JsonFormatter, YamlFormatter, TomlFormatter, TextFormatter), feature-flag gating, `enabled` feature defaults, `TreeFormatter` direct dispatch distinction.
- **Out of Scope**: Individual formatter output correctness (covered in formatter-specific docs); tree formatter configuration API.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | TableView produced by build_view() passes to any Format implementor | ✅ |
| FT-2 | nine formatters implement the Format trait | ✅ |
| FT-3 | formatters gated by feature flags are absent when flag is inactive | ✅ |
| FT-4 | enabled feature activates four default formatters | ✅ |
| FT-5 | TreeFormatter dispatches directly (not via Format trait) | ✅ |

---

### FT-1: TableView produced by build_view() passes to any Format implementor

- **Given:** A `RowBuilder` with headers and rows, finalized via `build_view()` into a `TableView`.
- **When:** The same `TableView` is passed to `TableFormatter`, `JsonFormatter`, and `LogfmtFormatter` via `Format::format()`.
- **Then:** Each returns `Ok(non_empty_string)`; the headers and row data appear in the output of each formatter; no formatter panics or returns an error for valid input.

---

### FT-2: nine formatters implement the Format trait

- **Given:** The nine formatters that implement `Format`: `TableFormatter`, `ExpandedFormatter`, `LogfmtFormatter`, `HtmlFormatter`, `SqlFormatter`, `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, `TextFormatter`.
- **When:** Each is called with `Format::format(&table_view)` using a simple 2-column, 2-row `TableView`.
- **Then:** All nine return `Ok(_)`; the result strings are non-empty; each contains recognizable output matching its format.

---

### FT-3: formatters gated by feature flags are absent when flag is inactive

- **Given:** A build configuration without `format_json` enabled.
- **When:** The crate is compiled.
- **Then:** `JsonFormatter` is not accessible; the binary includes no serde or serde_json code; compilation succeeds with zero errors.

---

### FT-4: enabled feature activates four default formatters

- **Given:** A build configuration with only the `enabled` feature.
- **When:** The crate is compiled.
- **Then:** `TableFormatter`, `ExpandedFormatter`, `TreeFormatter`, and `LogfmtFormatter` are available; `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, `HtmlFormatter`, `SqlFormatter`, and `TextFormatter` are not compiled in.

---

### FT-5: TreeFormatter dispatches directly (not via Format trait)

- **Given:** A `TreeNode` tree built with `TreeBuilder`; `TreeFormatter` configured
  with a `TreeConfig`.
- **When:** `TreeFormatter::format_tree(&tree)` is called directly (not via
  `Format::format()`).
- **Then:** The call succeeds and returns a non-empty string; `TreeFormatter` does
  NOT implement the `Format` trait (trait objects of `Format` cannot dispatch to
  `TreeFormatter`); the direct dispatch API is the only way to invoke it.
- **Note:** Distinguishes `TreeFormatter` (direct dispatch only) from the nine
  `Format` implementors listed in FT-2.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/003_unified_format_interface.md`](../../../docs/feature/003_unified_format_interface.md) | Source feature spec — Format trait, nine implementors, feature flags |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/unified_format_trait.rs`](../../unified_format_trait.rs) | Format trait dispatch test cases |
| [`tests/formatters.rs`](../../formatters.rs) | Formatter compilation and feature-flag tests |

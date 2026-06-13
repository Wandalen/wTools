# API: Config Types

### Scope

- **Purpose**: Drive test coverage for the config-types API contracts in `docs/api/003_config_types.md`.
- **Responsibility**: Documents API contract test cases for `TableConfig` builder methods and
  the `TableCaption` type introduced by the plan — specifically `border_color`, `caption()`,
  and `TableCaption` builder methods.
- **In Scope**: `TableCaption::new()` constructor, `TableCaption::field()` chain, `TableConfig::caption()`
  builder and accessor, `TableConfig::border_color()` builder, default-None invariant for both fields
  across all nine presets, `CAPTION_*` public constants.
- **Out of Scope**: Caption rendering behavior (see `feature/007_table_caption.md`); border-color
  rendering (see `feature/004_color_themes.md`); auto-fit terminal-width API (see `feature/005`);
  non-table config types (`TreeConfig`, `ExpandedConfig`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | TableCaption::new stores title and starts with no fields | ✅ |
| AP-2 | TableCaption::field builder appends field and returns Self | ✅ |
| AP-3 | TableConfig::caption attaches caption; caption_ref retrieves it | ✅ |
| AP-4 | TableConfig::border_color stores color string | ✅ |
| AP-5 | all nine preset constructors default caption to None | ✅ |
| AP-6 | CAPTION_FIELD_SEP CAPTION_RULE_CHAR CAPTION_LEAD_WIDTH have expected values | ✅ |

---

### AP-1: TableCaption::new stores title and starts with no fields

- **Given:** `TableCaption::new("Active Sessions")` is called.
- **When:** The resulting caption is inspected.
- **Then:** The title is `"Active Sessions"`; the fields collection is empty (length 0).

---

### AP-2: TableCaption::field builder appends field and returns Self

- **Given:** `TableCaption::new("R").field("10 items").field("3 repos")` is called.
- **When:** The resulting caption is inspected.
- **Then:** The fields collection contains exactly two entries in order: `"10 items"` then
  `"3 repos"`; each call to `.field()` returns a new `Self` (verified by chaining without
  intermediate bindings).

---

### AP-3: TableConfig::caption attaches caption; caption_ref retrieves it

- **Given:** A `TableConfig::plain()` with `.caption(TableCaption::new("T"))` applied.
- **When:** `caption_ref()` is called on the config.
- **Then:** The accessor returns `Some(&caption)` where the caption's title is `"T"`;
  a `TableConfig::plain()` without `.caption()` returns `None` from `caption_ref()`.

---

### AP-4: TableConfig::border_color stores color string

- **Given:** `TableConfig::bordered().border_color("\x1b[2;37m".to_string())` is called.
- **When:** The config's border color accessor is called.
- **Then:** The accessor returns `Some("\x1b[2;37m")`; a `TableConfig::bordered()` without
  any `.border_color()` call returns `None` from the accessor (default is `None`).

---

### AP-5: all nine preset constructors default caption to None

- **Given:** Each of the nine preset constructors: `plain()`, `minimal()`, `bordered()`,
  `markdown()`, `grid()`, `unicode_box()`, `csv()`, `tsv()`, `compact()`.
- **When:** `caption_ref()` is called on each.
- **Then:** All nine return `None`; no preset sets a default caption.

---

### AP-6: CAPTION_FIELD_SEP CAPTION_RULE_CHAR CAPTION_LEAD_WIDTH have expected values

- **Given:** The public constants `CAPTION_FIELD_SEP`, `CAPTION_RULE_CHAR`, `CAPTION_LEAD_WIDTH`
  exported from `data_fmt`.
- **When:** Their values are compared at compile time.
- **Then:** `CAPTION_FIELD_SEP == '·'` (U+00B7 MIDDLE DOT); `CAPTION_RULE_CHAR == '─'`
  (U+2500 BOX DRAWINGS LIGHT HORIZONTAL); `CAPTION_LEAD_WIDTH == 3`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/003_config_types.md`](../../../docs/api/003_config_types.md) | Source API spec — TableConfig builder API, TableCaption type |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | TableCaption rendering tests (see `tests/docs/feature/007_table_caption.md` FT-1..FT-6) |
| [`tests/table_config_validation_test.rs`](../../table_config_validation_test.rs) | TableConfig builder and preset validation (AP-1..AP-6) |

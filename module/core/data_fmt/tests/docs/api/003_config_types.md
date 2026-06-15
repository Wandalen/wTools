# API: Config Types

### Scope

- **Purpose**: Drive test coverage for the config-types API contracts in `docs/api/003_config_types.md`.
- **Responsibility**: Documents API contract test cases for builder methods across all config types
  (`TableConfig`, `ExpandedConfig`, `TreeConfig`, `Heading`) and the `with_` prefix convention.
- **In Scope**: `Heading::new()` constructor, `Heading::with_field()` chain, `TableConfig::with_heading()`
  builder and accessor, `TableConfig::with_border_color()` builder, default-None invariant for heading
  and border_color across all nine presets, `CAPTION_*` public constants, `with_` prefix convention
  on all consuming builder setters across all four config types.
- **Out of Scope**: Caption rendering behavior (see `feature/007_table_caption.md`); border-color
  rendering (see `feature/004_color_themes.md`); auto-fit terminal-width API (see `feature/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | Heading::new stores title and starts with no fields | ✅ |
| AP-2 | Heading::with_field builder appends field and returns Self | ✅ |
| AP-3 | TableConfig::with_heading attaches heading; heading_ref retrieves it | ✅ |
| AP-4 | TableConfig::with_border_color stores color string | ✅ |
| AP-5 | all nine preset constructors default heading to None | ✅ |
| AP-6 | CAPTION_FIELD_SEP CAPTION_RULE_CHAR CAPTION_LEAD_WIDTH have expected values | ✅ |
| AP-7 | all consuming builder setters across four config types use with_ prefix | ✅ |
| AP-8 | Heading type replaces TableCaption in public re-exports | ✅ |

---

### AP-1: Heading::new stores title and starts with no fields

- **Given:** `Heading::new("Active Sessions")` is called.
- **When:** The resulting heading is inspected.
- **Then:** The title is `"Active Sessions"`; the fields collection is empty (length 0).

---

### AP-2: Heading::with_field builder appends field and returns Self

- **Given:** `Heading::new("R").with_field("10 items").with_field("3 repos")` is called.
- **When:** The resulting heading is inspected.
- **Then:** The fields collection contains exactly two entries in order: `"10 items"` then
  `"3 repos"`; each call to `.with_field()` returns a new `Self` (verified by chaining without
  intermediate bindings).

---

### AP-3: TableConfig::with_heading attaches heading; heading_ref retrieves it

- **Given:** A `TableConfig::plain()` with `.with_heading(Heading::new("T"))` applied.
- **When:** `heading_ref()` is called on the config.
- **Then:** The accessor returns `Some(&heading)` where the heading's title is `"T"`;
  a `TableConfig::plain()` without `.with_heading()` returns `None` from `heading_ref()`.

---

### AP-4: TableConfig::with_border_color stores color string

- **Given:** `TableConfig::bordered().with_border_color("\x1b[2;37m".to_string())` is called.
- **When:** The config's border color accessor is called.
- **Then:** The accessor returns `Some("\x1b[2;37m")`; a `TableConfig::bordered()` without
  any `.with_border_color()` call returns `None` from the accessor (default is `None`).

---

### AP-5: all nine preset constructors default heading to None

- **Given:** Each of the nine preset constructors: `plain()`, `minimal()`, `bordered()`,
  `markdown()`, `grid()`, `unicode_box()`, `csv()`, `tsv()`, `compact()`.
- **When:** `heading_ref()` is called on each.
- **Then:** All nine return `None`; no preset sets a default heading.

---

### AP-6: CAPTION_FIELD_SEP CAPTION_RULE_CHAR CAPTION_LEAD_WIDTH have expected values

- **Given:** The public constants `CAPTION_FIELD_SEP`, `CAPTION_RULE_CHAR`, `CAPTION_LEAD_WIDTH`
  exported from `data_fmt`.
- **When:** Their values are compared at compile time.
- **Then:** `CAPTION_FIELD_SEP == '·'` (U+00B7 MIDDLE DOT); `CAPTION_RULE_CHAR == '─'`
  (U+2500 BOX DRAWINGS LIGHT HORIZONTAL); `CAPTION_LEAD_WIDTH == 3`.

---

### AP-7: all consuming builder setters across four config types use with_ prefix

- **Given:** The complete set of consuming builder setters (signature `pub fn name(mut self, ...) -> Self`) across `TableConfig`, `ExpandedConfig`, `TreeConfig`, and `Heading`.
- **When:** Each method name is inspected.
- **Then:** Every consuming builder setter starts with `with_`; specifically: 24 in `TableConfig`, 7 in `ExpandedConfig`, 7 in `TreeConfig`, 1 in `Heading` (`with_field`); no consuming builder setter exists without the `with_` prefix; preset constructors (`plain()`, `postgres_style()`, etc.) and accessors (`heading_ref()`, etc.) are not affected.

---

### AP-8: Heading type replaces TableCaption in public re-exports

- **Given:** The public API surface of `data_fmt::config` module.
- **When:** The re-exports in `src/config/mod.rs` are inspected.
- **Then:** `Heading` is publicly exported; `TableCaption` does not appear anywhere in `src/`; the constants `CAPTION_FIELD_SEP`, `CAPTION_RULE_CHAR`, `CAPTION_LEAD_WIDTH` remain unchanged (they describe formatting, not the type name).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/003_config_types.md`](../../../docs/api/003_config_types.md) | Source API spec — all config types, Heading type, builder setters |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../table_caption_test.rs) | Heading rendering tests (see `tests/docs/feature/007_table_caption.md` FT-1..FT-8) |
| [`tests/table_config_validation_test.rs`](../../table_config_validation_test.rs) | Config builder and preset validation (AP-1..AP-8) |

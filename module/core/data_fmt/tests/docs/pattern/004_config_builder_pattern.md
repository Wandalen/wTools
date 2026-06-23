# Pattern: Config Builder Pattern

### Scope

- **Purpose**: Drive test coverage for the fluent builder API used by all formatter config structs.
- **Responsibility**: Documents test cases for the config builder pattern in `docs/pattern/004_config_builder_pattern.md`.
- **In Scope**: Fluent setter chaining, default values for unchained fields, config consumption by formatter constructors.
- **Out of Scope**: Per-config field details (see `../api/`), builder helper types (see `../builder/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| PT-1 | Config setter returns Self for chaining | ⏳ |
| PT-2 | Default values apply to unchained fields | ⏳ |
| PT-3 | Config passed by value to formatter constructor | ⏳ |

---

### PT-1: Config setter returns Self for chaining

- **Given:** A `TableConfig` instance created via `TableConfig::default()`.
- **When:** Calling a setter method (e.g., `with_column_separator`).
- **Then:** The return type is `Self`; multiple setters can be chained in a single expression; the same pattern holds for `ExpandedConfig` and `TreeConfig`.

---

### PT-2: Default values apply to unchained fields

- **Given:** A `TableConfig::bordered()` preset.
- **When:** Only `with_column_separator(" | ")` is chained, leaving all other fields at their preset defaults.
- **Then:** The column separator is overridden to `" | "`; all other fields (border chars, header separator, padding) retain their `bordered()` preset values; formatting output reflects both the override and the defaults.

---

### PT-3: Config passed by value to formatter constructor

- **Given:** A `TableConfig` built via fluent chaining.
- **When:** Passing it to `TableFormatter::with_config(config)`.
- **Then:** The formatter is constructed successfully; the config is consumed by value (not borrowed); the same pattern applies to `ExpandedFormatter::with_config(ExpandedConfig)` and `TreeFormatter::with_config(TreeConfig)`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/pattern/004_config_builder_pattern.md`](../../../docs/pattern/004_config_builder_pattern.md) | Source pattern doc — fluent builder API, config struct names, construction |

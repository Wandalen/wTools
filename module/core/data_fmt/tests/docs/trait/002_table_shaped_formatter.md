# Trait: TableShapedFormatter

### Scope

- **Purpose**: Drive test coverage for the removed TableShapedFormatter trait and its migration path.
- **Responsibility**: Documents test cases for the deprecated trait in `docs/trait/002_table_shaped_formatter.md`.
- **In Scope**: Trait removal verification, Format trait replacement, migration from build() to build_view(), former implementor coverage.
- **Out of Scope**: Active Format trait contracts (see `001_format.md`), formatter output correctness (see `../formatter/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TR-1 | trait is not exported from public API | ⏳ |
| TR-2 | former implementors implement Format trait | ⏳ |
| TR-3 | build_view replaces build for TableView construction | ⏳ |
| TR-4 | migration path produces equivalent output | ⏳ |

---

### TR-1: trait is not exported from public API

- **Given:** The `data_fmt` crate v0.3.0+ public API.
- **When:** Attempting to import `TableShapedFormatter` from the crate.
- **Then:** The trait is not available; `use data_fmt::formatters::TableShapedFormatter` fails to compile; the trait was removed in v0.3.0.

---

### TR-2: former implementors implement Format trait

- **Given:** `TableFormatter` and `ExpandedFormatter`, which formerly implemented `TableShapedFormatter`.
- **When:** Checking their trait implementations.
- **Then:** Both implement the `Format` trait; `Format::fmt` accepts `&TableView` and returns `Result<String, FormatError>`; the `Format` trait is the canonical replacement.

---

### TR-3: build_view replaces build for TableView construction

- **Given:** A `RowBuilder` with headers `["A", "B"]` and one row `["1", "2"]`.
- **When:** Calling `build_view()` instead of the removed `build()`.
- **Then:** `build_view()` returns a `TableView` with headers `["A", "B"]` and one row; `build()` is not available on `RowBuilder`; the `TableView` is directly consumable by `Format::fmt`.

---

### TR-4: migration path produces equivalent output

- **Given:** A `TableFormatter` and a `TableView` built from `RowBuilder::build_view()` with headers `["Name"]` and row `["Alice"]`.
- **When:** Formatting via `Format::fmt(&formatter, &view)`.
- **Then:** The output is a well-formed table string containing `"Name"` and `"Alice"`; the result is `Ok(String)` (not infallible — unlike the removed trait which returned `String` directly); error handling is now explicit via `FormatError`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/trait/002_table_shaped_formatter.md`](../../../docs/trait/002_table_shaped_formatter.md) | Source trait doc — removed trait signature, former implementors, migration path |

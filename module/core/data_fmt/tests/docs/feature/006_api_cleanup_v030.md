# Feature: API Cleanup v0.3.0

### Scope

- **Purpose**: Drive test coverage for the API cleanup changes introduced in v0.3.0.
- **Responsibility**: Documents test cases verifying removal of deprecated APIs and correct migration to the new interface in `docs/feature/006_api_cleanup_v030.md`.
- **In Scope**: `TableShapedFormatter` absence from source and tests, deprecated-allow attribute absence, `ExpandedFormatter` `Format` impl, `RowBuilder::build()` absence, `build_view()` correctness, zero-warning compilation.
- **Out of Scope**: Runtime behavior of replaced APIs; migration guide for users upgrading from older versions.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | TableShapedFormatter trait is absent from source and tests | ✅ |
| FT-2 | deprecated-allow attributes are absent from source and tests | ✅ |
| FT-3 | ExpandedFormatter implements the Format trait | ✅ |
| FT-4 | RowBuilder::build() method is absent | ✅ |
| FT-5 | RowBuilder::build_view() produces a valid TableView | ✅ |
| FT-6 | all tests pass with zero warnings | ✅ |

---

### FT-1: TableShapedFormatter trait is absent from source and tests

- **Given:** The current state of `src/` and `tests/` after the v0.3.0 cleanup.
- **When:** `TableShapedFormatter` symbol is searched in `src/` and `tests/`.
- **Then:** Zero matches are returned; the deprecated trait is fully removed from the codebase.
- **Note:** Verified via runtime search — absence is a behavioral guarantee, not just a code-style preference.

---

### FT-2: deprecated-allow attributes are absent from source and tests

- **Given:** The current state of `src/` and `tests/` after the v0.3.0 cleanup.
- **When:** `allow(deprecated)` attribute is searched in `src/` and `tests/`.
- **Then:** Zero matches are returned; no code suppresses deprecation warnings for the removed APIs.

---

### FT-3: ExpandedFormatter implements the Format trait

- **Given:** The current `src/formatters/expanded.rs`.
- **When:** `ExpandedFormatter` is used via `Format::format()` on a valid `TableView`.
- **Then:** Exactly one `impl Format for ExpandedFormatter` exists in source; calling `Format::format()` returns `Ok(non_empty_string)`.

---

### FT-4: RowBuilder::build() method is absent

- **Given:** The current `src/table_tree.rs`.
- **When:** `fn build` is searched within the `RowBuilder` impl block.
- **Then:** Zero matches are returned; callers must use `build_view()` to obtain a `TableView`.

---

### FT-5: RowBuilder::build_view() produces a valid TableView

- **Given:** A `RowBuilder` with 2 headers and 3 rows.
- **When:** `build_view()` is called.
- **Then:** The returned `TableView` has a `TableMetadata` with 2 column names; `rows()` returns 3 entries each with 2 cells; passing the view to `TableFormatter` produces a well-formed table.

---

### FT-6: all tests pass with zero warnings

- **Given:** The codebase at the v0.3.0 boundary.
- **When:** `RUSTFLAGS="-D warnings" cargo nextest run --all-features` is executed.
- **Then:** All tests pass; zero compiler warnings are emitted; the exit code is 0.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/006_api_cleanup_v030.md`](../../../docs/feature/006_api_cleanup_v030.md) | Source feature spec — deprecated removals, new API, clean compilation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/formatters.rs`](../../formatters.rs) | API presence/absence test cases |
| [`tests/fluent_api.rs`](../../fluent_api.rs) | Fluent builder API test cases |
| [`tests/expanded_format_trait.rs`](../../expanded_format_trait.rs) | ExpandedFormatter Format trait test cases |

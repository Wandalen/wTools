# API: Formatters

### Scope

- **Purpose**: Drive test coverage for the formatters API contracts in `docs/api/004_formatters.md`.
- **Responsibility**: Documents API contract test cases for all formatter constructors, `Format` trait, `FormatError`, feature-flag gating, and ANSI/Unicode helpers.
- **In Scope**: `Format::format` return values, `TableFormatter`/`ExpandedFormatter`/`TreeFormatter` constructors, `visual_len`, `pad_to_width`, feature-flag availability, `FormatError` variants.
- **Out of Scope**: Output correctness (see `tests/docs/feature/`); configuration builder patterns (see `003_config_types.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | TableFormatter::new constructs default formatter; format returns Ok(String) | ✅ |
| AP-2 | ExpandedFormatter implements Format trait; format returns Ok(String) | ✅ |
| AP-3 | TreeFormatter::format dispatches via direct method, not Format trait | ✅ |
| AP-4 | visual_len excludes ANSI codes; returns visible character count | ✅ |
| AP-5 | pad_to_width pads to exact width using EAW for CJK characters | ✅ |
| AP-6 | Format::format returns Err(FormatError) for structurally invalid input | ✅ |
| AP-7 | feature-gated formatters absent when feature flag is inactive | ✅ |

---

### AP-1: TableFormatter::new constructs default formatter; format returns Ok(String)

- **Given:** A `TableFormatter::new()` and a valid `TableView` with 2 headers and 1 row.
- **When:** `Format::format(&view)` is called.
- **Then:** Returns `Ok(s)` where `s` is non-empty; `s` contains both header names
  and the row values; no panic occurs.

---

### AP-2: ExpandedFormatter implements Format trait; format returns Ok(String)

- **Given:** An `ExpandedFormatter::new()` and a valid `TableView` with 2 headers and 1 row.
- **When:** `Format::format(&view)` is called.
- **Then:** Returns `Ok(s)` where `s` is non-empty; the output contains key-value
  labeled lines (one per column); no `TableShapedFormatter` trait is involved.

---

### AP-3: TreeFormatter::format dispatches via direct method, not Format trait

- **Given:** A `TreeFormatter::new()` and a `TreeNode<u64>` with root and one child.
- **When:** `TreeFormatter::format(&root, |v| v.to_string())` is called directly.
- **Then:** Returns a non-empty string with tree-drawing characters; `TreeFormatter`
  does NOT implement the `Format` trait — calling `Format::format` on it is a compile error.

---

### AP-4: visual_len excludes ANSI codes; returns visible character count

- **Given:** The string `"\x1b[32mhello\x1b[0m"` (5 visible chars, 16 bytes).
- **When:** `visual_len(s)` is called.
- **Then:** Returns `5`; ANSI escape bytes contribute zero to the result;
  `visual_len("hello") == visual_len("\x1b[32mhello\x1b[0m")`.

---

### AP-5: pad_to_width pads to exact width using EAW for CJK characters

- **Given:** The string `"ab"` (width 2) with target `pad_to_width("ab", 6, false)`.
- **When:** The function is called.
- **Then:** Returns a 6-display-column string (4 trailing spaces added); for a CJK
  string `"中文"` (EAW width 4) padded to 6, returns `"中文  "` (2 trailing spaces);
  the display width is always exactly `target_width` when input is shorter.

---

### AP-6: Format::format returns Err(FormatError) for structurally invalid input

- **Given:** A formatter that validates input structure (e.g. a SqlFormatter where
  the input has zero rows — an edge case the formatter may reject).
- **When:** `Format::format(&empty_view)` is called.
- **Then:** Either `Ok("")` or `Err(FormatError::InvalidData(_))` is returned without
  panic; no formatter panics on empty input; the error (if any) is a named
  `FormatError` variant, not an unwrap panic.
- **Note:** Most formatters return `Ok("")` for empty input rather than an error.
  This case verifies no panic occurs on the degenerate path.

---

### AP-7: feature-gated formatters absent when feature flag is inactive

- **Given:** A build with `--no-default-features` and only the `format_table` feature enabled.
- **When:** The crate is compiled.
- **Then:** `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, `HtmlFormatter`, `SqlFormatter`,
  and `TextFormatter` are not in scope; `TableFormatter` is available;
  compilation succeeds with zero errors (no missing-type errors for ungated formatters).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/004_formatters.md`](../../../docs/api/004_formatters.md) | Source API spec — formatter constructors, Format trait, feature flags |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/formatters.rs`](../../formatters.rs) | Formatter API and feature-flag test cases |
| [`tests/expanded_format_trait.rs`](../../expanded_format_trait.rs) | ExpandedFormatter Format trait test cases |

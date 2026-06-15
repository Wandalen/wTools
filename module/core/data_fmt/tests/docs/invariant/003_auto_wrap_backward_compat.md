# Invariant: Auto-Wrap Backward Compatibility

### Scope

- **Purpose**: Drive test coverage for the auto-wrap backward compatibility invariant.
- **Responsibility**: Documents test cases verifying byte-identical output when `auto_wrap(false)` is set across all 9 table presets in `docs/invariant/003_auto_wrap_backward_compat.md`.
- **In Scope**: All 9 table presets (`plain`, `minimal`, `bordered`, `markdown`, `grid`, `unicode_box`, `csv`, `tsv`, `compact`) with `auto_wrap=false`, byte-identity verification against baseline output.
- **Out of Scope**: Auto-fold backward compatibility; auto-wrap-enabled rendering behavior (see `feature/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | auto_wrap=false produces byte-identical output to pre-wrap rendering | ✅ |
| IN-2 | toggling auto_wrap on non-wrapping content is a no-op | ✅ |
| IN-3 | unicode_box preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-4 | markdown preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-5 | minimal preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-6 | bordered preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-7 | grid preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-8 | csv preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-9 | tsv preset with auto_wrap=false produces byte-identical output | ✅ |
| IN-10 | compact preset with auto_wrap=false produces byte-identical output | ✅ |

---

### IN-1: auto_wrap=false produces byte-identical output to pre-wrap rendering

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::plain().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::plain()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  output of the baseline formatter; disabling `auto_wrap` is a true opt-out with
  no side effects.
- **Note:** Covered by `auto_wrap_false_is_byte_identical` (T06) in
  `tests/auto_wrap_test.rs`. The baseline is a fresh formatter with no `auto_wrap`
  or `terminal_width` call (not a snapshot of pre-wrap code); byte-identity holds
  because `auto_wrap=false` disables the budget-allocation pass entirely.

---

### IN-2: toggling auto_wrap on non-wrapping content is a no-op

- **Given:** A table whose row width is narrower than the terminal width even with
  `auto_wrap=true` active; the same table rendered twice — once with `auto_wrap=true`
  and once with `auto_wrap=false`.
- **When:** Both renders complete.
- **Then:** Both outputs are byte-identical; content that would not wrap regardless
  is unaffected by the `auto_wrap` flag value.
- **Note:** Covered by `auto_wrap_natural_fit_no_wrapping` (T01) in
  `tests/auto_wrap_test.rs` (`terminal_width(Some(120))` with `auto_wrap=true` vs
  `auto_wrap=false`; asserts `output_wrap == output_no_wrap`).

---

### IN-3: unicode_box preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::unicode_box().with_auto_wrap(false).with_terminal_width(Some(40))`; a second
  uses `TableConfig::unicode_box()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the unicode box-drawing characters are unaffected by the
  `auto_wrap=false` setting.
- **Coverage note:** `auto_wrap_unicode_box_style` (T12) in `tests/auto_wrap_test.rs`
  tests unicode_box with wrapping enabled but does NOT provide a dedicated
  `auto_wrap=false` byte-identity assertion. Mechanism coverage is provided by T06
  (IN-1), which confirms `auto_wrap=false` disables the budget pass for all presets.

---

### IN-4: markdown preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::markdown().with_auto_wrap(false).with_terminal_width(Some(40))`; a second
  uses `TableConfig::markdown()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the markdown pipe-and-dash separators are unaffected by the
  `auto_wrap=false` setting.
- **Coverage note:** No dedicated `auto_wrap=false` byte-identity test for markdown.
  Mechanism coverage via T06 (IN-1).

---

### IN-5: minimal preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::minimal().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::minimal()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the minimal border style is unaffected by the `auto_wrap=false`
  setting.
- **Coverage note:** No dedicated `auto_wrap=false` byte-identity test for minimal.
  Mechanism coverage via T06 (IN-1).

---

### IN-6: bordered preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::bordered().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::bordered()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the bordered-style separators are unaffected by the
  `auto_wrap=false` setting.
- **Coverage note:** `auto_wrap_bordered_style` (T11) in `tests/auto_wrap_test.rs`
  tests bordered with wrapping enabled but does NOT provide a dedicated
  `auto_wrap=false` byte-identity assertion. Mechanism coverage via T06 (IN-1).

---

### IN-7: grid preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::grid().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::grid()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the grid-style borders are unaffected by the `auto_wrap=false`
  setting.
- **Coverage note:** No dedicated `auto_wrap=false` byte-identity test for grid.
  Mechanism coverage via T06 (IN-1).

---

### IN-8: csv preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::csv().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::csv()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the CSV preset auto-bypasses `auto_wrap` regardless of setting
  (data format integrity guard), so both configurations produce identical well-formed
  CSV output.
- **Coverage note:** `csv_preset_auto_disables_wrapping` (T07) in
  `tests/auto_wrap_test.rs` verifies that CSV bypasses auto-wrap regardless of
  the `auto_wrap` flag — this is the closest existing test.

---

### IN-9: tsv preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::tsv().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::tsv()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the TSV preset auto-bypasses `auto_wrap` regardless of setting
  (data format integrity guard), so both configurations produce identical well-formed
  TSV output.
- **Coverage note:** `tsv_preset_auto_disables_wrapping` (T08) in
  `tests/auto_wrap_test.rs` verifies that TSV bypasses auto-wrap regardless of
  the `auto_wrap` flag — this is the closest existing test.

---

### IN-10: compact preset with auto_wrap=false produces byte-identical output

- **Given:** A table with cell content; one `TableFormatter` uses
  `TableConfig::compact().with_auto_wrap(false).with_terminal_width(Some(40))`; a second uses
  `TableConfig::compact()` with no `auto_wrap` or `terminal_width` override.
- **When:** Both formatters render the same table data.
- **Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
  baseline formatter; the compact border style is unaffected by the `auto_wrap=false`
  setting.
- **Coverage note:** No dedicated `auto_wrap=false` byte-identity test for compact.
  Mechanism coverage via T06 (IN-1).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/003_auto_wrap_backward_compat.md`](../../../docs/invariant/003_auto_wrap_backward_compat.md) | Source invariant spec — 9 preset list, enforcement mechanism, violation consequences |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_wrap_test.rs`](../../auto_wrap_test.rs) | Invariant test implementation (T06 `auto_wrap_false_is_byte_identical`) |

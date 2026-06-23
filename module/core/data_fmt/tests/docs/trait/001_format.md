# Trait: Format

### Scope

- **Purpose**: Drive test coverage for the Format trait contract.
- **Responsibility**: Documents test cases for the Format trait in `docs/trait/001_format.md`.
- **In Scope**: Trait method signature compliance, error variant construction, cfg-gated Serialization variant, successful format dispatch through the trait interface, trait object usage.
- **Out of Scope**: Formatter-specific output correctness (see `../feature/`); TreeFormatter exclusion rationale (see `docs/trait/001_format.md § Coverage Gaps`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| TR-1 | format method returns Ok on valid input | ⏳ |
| TR-2 | FormatError::InvalidData carries message | ⏳ |
| TR-3 | FormatError::Serialization cfg-gated construction | ⏳ |
| TR-4 | format dispatches through trait object | ⏳ |
| TR-5 | FormatError::UnsupportedOperation carries message | ⏳ |
| TR-6 | format on empty table returns Ok | ⏳ |

---

### TR-1: format method returns Ok on valid input

- **Given:** A `TableView` with two columns and one row built via `RowBuilder`.
- **When:** `Format::format` is called on a `TableFormatter` with `TableConfig::plain()`.
- **Then:** The result is `Ok`; the returned string contains all header names and all cell values from the input view.

---

### TR-2: FormatError::InvalidData carries message

- **Given:** A `FormatError::InvalidData` variant constructed with the message `"missing columns"`.
- **When:** The error is displayed via its `Display` implementation.
- **Then:** The formatted string contains `"Invalid data: missing columns"`; the error matches `FormatError::InvalidData(_)` in a pattern match.

---

### TR-3: FormatError::Serialization cfg-gated construction

- **Given:** The crate is compiled with `feature = "serde_support"` enabled.
- **When:** A `FormatError::Serialization` variant is constructed with the message `"unexpected token"`.
- **Then:** The variant exists and is constructible; its `Display` output contains `"Serialization error: unexpected token"`; the variant is distinct from `InvalidData` and `UnsupportedOperation` in a match arm.
- **Note:** This variant is absent without `serde_support`; the test file must be gated on `#[cfg(feature = "serde_support")]`.

---

### TR-4: format dispatches through trait object

- **Given:** A `TableFormatter` with `TableConfig::plain()` stored behind a `&dyn Format` trait object reference.
- **When:** `format` is called through the trait object with a valid `TableView`.
- **Then:** The result is `Ok`; the output is identical to calling `Format::format` directly on the concrete formatter; the trait is object-safe for this usage pattern.

---

### TR-5: FormatError::UnsupportedOperation carries message

- **Given:** A `FormatError::UnsupportedOperation` variant constructed with the message `"pivot not supported"`.
- **When:** The error is displayed via its `Display` implementation.
- **Then:** The formatted string contains `"Unsupported operation: pivot not supported"`; the error matches `FormatError::UnsupportedOperation(_)` in a pattern match.

---

### TR-6: format on empty table returns Ok

- **Given:** A `TableView` with two column headers but zero rows (empty `rows` vec).
- **When:** `Format::format` is called on a `TableFormatter` with default config.
- **Then:** The result is `Ok`; the returned string contains the header names; no panic or error occurs from the empty row set.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/trait/001_format.md`](../../../docs/trait/001_format.md) | Source trait spec — signature, error type, implementor matrix |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/unified_format_trait.rs`](../../unified_format_trait.rs) | Format trait integration tests |

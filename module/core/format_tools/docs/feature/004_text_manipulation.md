# Feature: Text Manipulation

### Scope

- **Purpose**: Transform plain-text strings through wrapping and markdown math rendering without I/O or terminal interaction.
- **Responsibility**: Documents the text manipulation utilities — their purpose, behavior, and all associated artifacts.
- **In Scope**: Text wrapping at configurable widths, markdown math expression formatting, string utility functions.
- **Out of Scope**: Table formatting (→ feature/002), fallback conversion (→ feature/001), color or terminal output (→ invariant/002).

### Design

Text wrapping breaks long strings into lines at word boundaries up to a maximum width. The wrapping is purely textual — it does not interact with terminal width detection or cursor state. The output is a plain string.

Markdown math formatting renders mathematical expressions into a textual form suitable for inclusion in markdown documents. This is a lightweight transformation that does not require a full markdown parser.

Both utilities operate as pure functions: they accept strings and produce strings with no side effects, no I/O, and no state.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/text_wrap.rs` | Text wrapping implementation |
| source | `src/format/md_math.rs` | Markdown math formatting |
| source | `src/format/string.rs` | String utility functions |
| test | `tests/inc/md_math_test.rs` | Markdown math tests |
| test | `tests/inc/string_test.rs` | String utility tests |
| doc | `docs/invariant/001_pure_data_transformation.md` | Pure transformation constraint |
| doc | `docs/invariant/003_synchronous_only.md` | Synchronous execution constraint |

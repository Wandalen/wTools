# Variant: Text CLI Help

### Scope

- **Purpose**: Drive test coverage for the CLI help text output variant.
- **Responsibility**: Documents test cases for the CliHelp text variant in `docs/variant/033_text_cli_help.md`.
- **In Scope**: Automatic alignment spacing, section grouping, blank line separators, CLI-optimized formatting.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses automatic alignment spacing | ⏳ |
| VT-2 | sections separated by blank lines | ⏳ |
| VT-3 | multi-section option groups rendered | ⏳ |
| VT-4 | empty table produces no CLI help output | ⏳ |

---

### VT-1: output uses automatic alignment spacing

- **Given:** A `TableView` with headers representing CLI option name and description columns.
- **When:** Formatted with `TextFormatter` using `TextVariant::CliHelp`.
- **Then:** Option names and descriptions are aligned with automatic spacing; shorter option names get more padding to align with longer ones.

---

### VT-2: sections separated by blank lines

- **Given:** A `TableView` with multiple rows representing different option groups.
- **When:** Formatted with `TextVariant::CliHelp`.
- **Then:** Logical sections are separated by blank lines; the output reads like a CLI `--help` page.

---

### VT-3: multi-section option groups rendered

- **Given:** A `TableView` with rows spanning multiple option categories (e.g., general options, output options, debug options).
- **When:** Formatted with `TextVariant::CliHelp`.
- **Then:** Each category appears as a distinct section; section headers are visible; options within each section are aligned independently.

---

### VT-4: empty table produces no CLI help output

- **Given:** A `TableView` with headers and zero data rows.
- **When:** Formatted with `TextVariant::CliHelp`.
- **Then:** Output is empty; no section headers, options, or alignment spacing appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/033_text_cli_help.md`](../../../docs/variant/033_text_cli_help.md) | Source variant doc — Text CLI Help attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text_cli_help.rs`](../../text_cli_help.rs) | CLI Help text formatter test implementation |

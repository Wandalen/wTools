# Make `TableConfig` API Misuse-Resistant

## Goal

`TableConfig` fields are currently `pub`, allowing callers to construct partial
configurations via struct literal syntax (e.g. set `header_separator_variant:
HeaderSeparatorVariant::Unicode` but leave `column_separator` at its default
`Spaces(2)`). This produces a broken table where the separator line contains `┼`
but data rows have no `│` between columns. Make the fields private so the only
construction paths are preset constructors (`plain()`, `bordered()`,
`unicode_box()`, etc.) and the existing builder-setter methods
(`.border_variant()`, `.column_separator()`, etc.). After this change, a
misconfigured `TableConfig` struct literal is a compile error. Success is
measured by `w3 .test l::3` passing green with zero warnings.

## In Scope

- `src/config.rs` — remove `pub` from all `TableConfig` fields; keep all preset
  constructors and all builder setter methods as `pub`
- `src/config.rs` — remove the deprecated `show_borders: bool` field and its
  associated deprecated builder method (callers must use `border_variant()`)
- `tree_fmt/task/readme.md` — update index
- Any call sites within the `wtools`/`willbe` workspace that use struct literal
  initialization of `TableConfig` (currently: `gi_infra/src/formatters/style.rs`
  — fix `cli_table()` to use `TableConfig::unicode_box()`)

## Out of Scope

- Adding new preset constructors beyond the existing nine
- Changing `TreeConfig`, `ExpandedConfig`, or any other config struct
- Modifying formatter logic in `src/formatters/table.rs`
- Validating that `ColumnSeparator` and `HeaderSeparatorVariant` are semantically
  consistent at runtime (caller contract enforcement is sufficient with private
  fields)
- Publishing `tree_fmt` to crates.io (tracked separately)

## Description

Diagnosed during investigation of broken `gi .repos.list` output (2026-03-31):
`gi_infra::formatters::style::cli_table()` constructed `TableConfig` via struct
literal, setting `header_separator_variant: HeaderSeparatorVariant::Unicode` but
relying on `..TableConfig::default()` for `column_separator`, which defaults to
`Spaces(2)`. The Unicode header separator emits `┼` between columns, but data
rows use spaces — producing misaligned output.

The correct preset `TableConfig::unicode_box()` already exists and pairs all
three required fields correctly:
```rust
TableConfig {
    border_variant           : BorderVariant::Unicode,
    header_separator_variant : HeaderSeparatorVariant::Unicode,
    column_separator         : ColumnSeparator::Character( '│' ),  // ← the missing field
    outer_padding            : true,
    inner_padding            : 1,
    ..Self::default()
}
```

Making fields private eliminates the risk of a future caller repeating this
mistake. The nine presets cover all standard table styles; builder setters
handle custom requirements.

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `kbase .rulebooks`)
-   Breaking change is acceptable — `tree_fmt` uses a local path dependency
    (`path = "../../wtools/dev/module/core/tree_fmt"`) and is not yet published
-   Do not add `cargo fmt`-formatted code; use 2-space indentation per codestyle
    rulebook

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on codestyle, field
   visibility, and breaking changes.
2. **Write Test Matrix** — populate every row before opening any test file.
3. **Write failing tests** — one test per row confirming that `TableConfig`
   struct literals with private fields fail to compile, and that all presets +
   builder chains produce correct output. Confirm red state.
4. **Implement** — make `TableConfig` fields private; remove deprecated
   `show_borders` field; update `gi_infra::formatters::style::cli_table()` to
   use `TableConfig::unicode_box()`; scan workspace for any remaining struct
   literal callers and fix them.
5. **Green state** — `w3 .test l::3` passes with zero failures and zero
   warnings.
6. **Refactor if needed** — verify no function exceeds 50 lines; all public
   items have `///` doc comments; no duplication introduced.
7. **Walk Validation Checklist** — every answer YES before proceeding.
8. **Update task status** — set ✅ in `task/readme.md`, Priority=0,
   advisability=0, move file to `task/completed/`.

## Test Matrix

*(Write before any test code.)*

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | Struct literal with all fields | Private fields | Compile error (struct literal forbidden) |
| T02 | `TableConfig::unicode_box()` | Full preset | Header and data rows both have `│` between columns |
| T03 | `TableConfig::bordered()` | Full preset | Header and data rows have `\|` between columns |
| T04 | `TableConfig::plain()` | Full preset | Columns separated by spaces, dash separator |
| T05 | Builder chain: `TableConfig::new().border_variant(…).column_separator(…)` | Builder API | Compiles; produces expected output |
| T06 | `cli_table()` after migration | `TableConfig::unicode_box()` | `gi .repos.list`-style output has `│` between all columns |
| T07 | `TableConfig::default()` (via `new()`) | Default preset | Columns separated by `\|` char (default `ColumnSeparator`) |

## Acceptance Criteria

-   All `TableConfig` fields are private (no `pub` qualifier); struct literal
    initialization outside `src/config.rs` is a compile error
-   The deprecated `show_borders: bool` field and its builder method are removed
-   All nine preset constructors compile and pass their T02-T04 style tests
-   Builder-setter API (`border_variant()`, `column_separator()`, etc.) remains
    fully `pub` and functional
-   `gi_infra::formatters::style::cli_table()` uses `TableConfig::unicode_box()`
    and produces a table with `│` between all columns
-   `w3 .test l::3` passes with zero failures and zero warnings across
    `tree_fmt` and all dependent workspace crates

## Validation Checklist

Desired answer for every question is YES.

**`src/config.rs` — field visibility**
-   [ ] Are all `TableConfig` fields declared without `pub`?
-   [ ] Is `show_borders: bool` absent from the struct definition?
-   [ ] Does the `deprecated` builder method `show_borders()` not exist?
-   [ ] Do all nine preset constructors (`plain`, `minimal`, `bordered`,
    `markdown`, `grid`, `unicode_box`, `csv`, `tsv`, `compact`) remain `pub`?
-   [ ] Do all builder setter methods remain `pub`?

**`gi_infra/src/formatters/style.rs` — call site**
-   [ ] Does `cli_table()` call `TableConfig::unicode_box()` (not a struct
    literal)?
-   [ ] Does the `#[allow(deprecated)]` annotation in `cli_table()` no longer
    appear?

**Workspace call sites**
-   [ ] Are there zero struct literal initializations of `TableConfig` outside
    `src/config.rs` in the entire workspace?

**Test coverage**
-   [ ] Does T01 (struct literal → compile error) have a corresponding
    `compile_fail` doc-test or UI test confirming the compile error?
-   [ ] Do T02-T07 have passing tests?

**Out of Scope confirmation**
-   [ ] Are `TreeConfig`, `ExpandedConfig`, and other config structs unchanged?
-   [ ] Is `src/formatters/table.rs` logic unchanged?

**Final gate**
-   [ ] Does `w3 .test l::3` exit 0 with zero warnings?

## Validation Procedure

### Measurements

**M1 — Struct literal callsites**
Baseline: `grep -r "TableConfig {" --include="*.rs" | grep -v "src/config.rs"`.
Currently returns at least 1 hit (`gi_infra/src/formatters/style.rs`).
Expected after: 0 hits. Any non-zero count means private-field enforcement is
incomplete.

**M2 — Test suite pass rate**
Baseline: all tests pass (0 failures). Expected after: 0 failures, 0 warnings
from `w3 .test l::3`.

### Anti-faking checks

**AF1 — Field visibility not bypassed via `pub(crate)` or `pub(super)`**
Run: `grep -n "pub(crate)\|pub(super)" src/config.rs`. The result must be empty
for `TableConfig` fields. Using `pub(crate)` would allow struct literal init
within the crate and does not satisfy the goal of making misuse a compile error
for external callers.

**AF2 — `cli_table()` actually changed**
Run: `grep -A 10 "fn cli_table" gi_infra/src/formatters/style.rs`. Must NOT
contain a struct literal `TableConfig {`. Must contain `TableConfig::unicode_box`.

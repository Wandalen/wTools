# Add sub-row detail lines to `TableFormatter`

## Goal

Extend `TableFormatter` (and the `TableView` / `RowBuilder` data pipeline) to support
optional per-row **detail lines** — short plain-text annotations printed immediately
after each data row, indented by a configurable prefix. No new formatter type is
introduced; the existing `TableFormatter` styles all gain this capability.

The driving use case is the `kbase .rulebooks` command, which must show the
frontmatter `purpose:` value for each discovered rulebook below its row without
adding a wide `Purpose` column that would overflow a standard terminal.

Concrete desired output (plain style):

```
ID  File                       Lines
--  -------------------------  -----
b1  governance.rulebook.md     120
    Governance principles and anti-duplication rules for all projects.
b2  code_design.rulebook.md    340
    Code architecture and module organisation constraints.
b3  test_org.rulebook.md       85
```

Success is measured by all existing `tree_fmt` tests passing unchanged plus a
comprehensive new test file covering all scenarios from the Test Matrix.

## In Scope

- `src/data.rs` — add `row_details: Vec<Option<String>>` field to `TableView`; update
  `TableView::new()`, `TableMetadata`, `RowBuilder::build_view()`, `TableView::to_tree_node()`
- `src/table_tree.rs` — add `row_details: Vec<Option<String>>` field to `RowBuilder`;
  add builder methods `add_row_with_detail` / `add_row_with_detail_mut`; carry details
  through `build_view()`
- `src/config.rs` — add `sub_row_indent: String` field to `TableConfig` (default `"  "`,
  2 spaces); add `TableConfig::sub_row_indent()` getter; add `sub_row_indent()` builder
  setter
- `src/formatters/table.rs` — extend `format_internal()` to emit detail lines after each
  data row; respect `sub_row_indent` config; skip detail rendering when `row_details` is
  empty or shorter than `rows`
- `tests/` — new test file `tests/sub_row_test.rs` covering the full Test Matrix (≥15 cases)
- `task/readme.md` — add this task to the index

## Out of Scope

- Any other formatter (`ExpandedFormatter`, `TreeFormatter`, `TextFormatter`,
  `JsonFormatter`, `YamlFormatter`, `CsvFormatter`, `TsvFormatter`) — sub-rows are a
  display-layer concern for `TableFormatter` only
- ANSI colorisation of sub-row detail text (separate future task if needed)
- Multi-line detail strings (single-line detail only in this task)
- Wrapping long detail strings (caller responsibility to pre-truncate)
- Changes to `kbase` (the consumer) — this task is pure `tree_fmt`

## Description

### Problem

`TableFormatter.format_internal(headers, rows)` renders every row as a uniform grid
line. There is no mechanism to attach a free-form annotation line beneath a row.
Adding a wide `Purpose` column solves visibility but causes horizontal overflow on
standard 80/120-column terminals; removing the column loses the information entirely.

Sub-rows solve this by printing a single indented plain-text line after the grid row.
The line is not bounded by column widths, does not affect column-width calculation, and
is simply omitted when `None`.

### Data Model Extension

`TableView` gains a parallel `row_details` vector:

```rust
pub struct TableView
{
  pub metadata    : TableMetadata,
  pub rows        : Vec< Vec< String > >,           // existing
  pub row_details : Vec< Option< String > >,        // NEW — parallel to rows
}
```

`row_details` is allowed to be empty (backward compat) or shorter than `rows`
(trailing `None` is implied). The renderer uses `row_details.get(idx)` so out-of-bounds
is treated as `None` (no sub-row).

### RowBuilder Extension

Two new methods mirror the existing `add_row` / `add_row_mut` pair:

```rust
/// Add row with optional detail line (consuming).
#[ must_use ]
pub fn add_row_with_detail( mut self, row : Vec< String >, detail : Option< String > ) -> Self

/// Add row with optional detail line (non-consuming / loop-friendly).
pub fn add_row_with_detail_mut( &mut self, row : Vec< String >, detail : Option< String > )
```

`build_view()` populates `TableView::row_details` from the accumulated details.

### Config Extension

```rust
/// Prefix prepended to each sub-row detail line (default: "  ", 2 spaces)
sub_row_indent : String,
```

- Default: `"  "` (2-space indent)
- Applied to all existing `TableConfig` presets via their `Default` impl (the field is
  set in `Default::default()` so all presets inherit it automatically)
- Builder: `pub fn sub_row_indent(mut self, prefix: String) -> Self`

### Rendering Algorithm

In `format_internal`, after emitting each data row (and its optional inter-row separator):

```
for each (idx, row) in rows.iter().enumerate():
  render row grid line(s) as today
  if let Some(detail) = row_details.get(idx).and_then(Option::as_deref):
    if !detail.is_empty():
      output.push_str(&self.config.sub_row_indent())
      output.push_str(detail)
      output.push('\n')
  render inter-row separator if needed (unchanged)
```

Detail lines appear AFTER the row content but BEFORE any inter-row separator (so the
separator visually belongs to the space between the detail and the next row).

For bordered styles (`AsciiGrid`, `Unicode`), the detail line is printed outside the
cell grid (no border pipes on left/right). This is intentional — a sub-row is metadata,
not a cell value.

### Backward Compatibility

- `TableView::new(metadata, rows)` — existing call sites pass two args; `row_details`
  defaults to `vec![]` via a separate convenience constructor or by making the field
  directly accessible with a sensible default
- `RowBuilder::add_row` / `add_row_mut` — unchanged; `row_details` entry for these rows
  is implicitly `None`
- Existing tests — column widths, borders, ANSI coloring, multiline cells all unaffected
  because the detail line is emitted independently and does not participate in width calc

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Sub-row detail lines must NOT affect column width calculation
- Sub-row detail lines must NOT appear for rows where detail is `None` or `""`
- `TableView::new(metadata, rows)` existing two-argument call sites must compile unchanged
- All existing `tree_fmt` tests must pass without modification
- New tests must cover every row in the Test Matrix

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; confirm code style (2-space indent, no
   `cargo fmt`), builder method conventions (`#[must_use]`, return `Self`).

2. **Write Test Matrix** — populate every row before opening any source file.

3. **Write failing tests** — create `tests/sub_row_test.rs`; import `RowBuilder`,
   `TableFormatter`, `TableConfig`, `Format`, `TableView`; write all Test Matrix
   cases; confirm each fails: `cargo nextest run sub_row`.

4. **Implement `src/data.rs`** — add `row_details` field to `TableView`; update
   `TableView::new()` to keep two-arg signature but default `row_details` to `vec![]`;
   add `TableView::with_details(metadata, rows, row_details)` for full construction;
   guard `to_tree_node()` (details are not part of the tree representation — no change
   needed there).

5. **Implement `src/table_tree.rs`** — add `row_details: Vec<Option<String>>` field to
   `RowBuilder`; initialise to `Vec::new()` in `new()`; implement
   `add_row_with_detail` and `add_row_with_detail_mut`; ensure `add_row` /
   `add_row_mut` push `None` into `row_details` so the vectors stay parallel; update
   `build_view()` to set `TableView::row_details`.

6. **Implement `src/config.rs`** — add `sub_row_indent: String` field to `TableConfig`;
   set to `"  "` in `Default::default()`; add `sub_row_indent()` getter returning
   `&str`; add `sub_row_indent()` builder setter.

7. **Implement `src/formatters/table.rs`** — in `format_internal()`, after emitting each
   data row (before or after inter-row separator per algorithm above), check
   `data.row_details.get(idx)` … but note: `format_internal` currently takes
   `headers: &[String]` and `rows: &[Vec<String>]` — it does NOT receive `row_details`.
   Two options:
   - **Option A (preferred):** Change the internal `Format for TableFormatter` impl to
     call a new `format_with_details(headers, rows, details)` overload; the public
     `format(tree)` path passes `&[]` for details (no sub-rows).
   - **Option B:** Store details in a temporary `RefCell` on the formatter — avoid, it
     adds unnecessary complexity.
   Use Option A. The `Format` trait impl calls `format_with_details(headers, rows, &view.row_details)`.

8. **Green state** — all existing + new tests pass:
   `RUSTFLAGS="-D warnings" cargo nextest run --all-features`

9. **Doc test pass** — `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features`

10. **Clippy clean** — `cargo clippy --all-targets --all-features -- -D warnings`

11. **Walk Validation Checklist** — every item YES.

12. **Update task status** — ✅ in `task/readme.md`, Priority=0, Advisability=0, move
    file to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config / API Under Test | Expected Behavior |
|---|---|---|---|
| T01 | Row added via `add_row` (no detail) | `RowBuilder::add_row` | No detail line in output |
| T02 | Row added via `add_row_with_detail(Some("text"))` | `add_row_with_detail`, plain config | `"  text\n"` appears after row grid line |
| T03 | Row added via `add_row_with_detail(None)` | `add_row_with_detail(None, ...)` | No extra line; identical to `add_row` output |
| T04 | Row added via `add_row_with_detail(Some(""))` | empty string detail | No extra line (empty detail suppressed) |
| T05 | Two rows: first has detail, second does not | mixed `add_row_with_detail` | Detail appears only after first row |
| T06 | Two rows: both have detail | two `add_row_with_detail` calls | Detail appears after each row |
| T07 | Custom indent `">>> "` in `TableConfig` | `.sub_row_indent(">>> ".into())` | Detail line starts with `">>> "` not `"  "` |
| T08 | Empty indent `""` in `TableConfig` | `.sub_row_indent(String::new())` | Detail appears flush-left (no prefix) |
| T09 | Detail text does not affect column widths | wide detail, narrow column data | Column width determined by header/cell data only |
| T10 | Three rows all with detail | three `add_row_with_detail` calls, `AsciiGrid` style | Each row followed by indented detail; borders/separators intact |
| T11 | `Unicode` box style with sub-row | `TableConfig::unicode_box()` + details | Unicode borders correct; detail lines appear between rows without pipes |
| T12 | `bordered()` style with sub-row | `TableConfig::bordered()` | Detail printed after row line; no broken pipes |
| T13 | Multiline cell (`\n` in cell) + detail | multiline cell + `add_row_with_detail` | Multiline cell renders correctly; detail appears after all cell lines |
| T14 | `Format::format(&formatter, &view)` path | `TableView::with_details(...)` + `Format::format` | Sub-rows appear in output via `Format` trait path |
| T15 | `add_row_mut` + `add_row_with_detail_mut` intermixed | non-consuming API | Correct detail/no-detail output; vectors stay parallel |
| T16 | `build_view()` on builder with mixed detail | `RowBuilder::build_view()` | `TableView::row_details` has correct `Some`/`None` entries |
| T17 | `TableView::new(metadata, rows)` (old two-arg form) | backward compat | Compiles and renders; no sub-rows emitted |
| T18 | Single row table, detail present | one row + one detail | Output has exactly one detail line after the single data row |

## Validation Checklist

Desired answer for every item is YES.

**Data model**
- [ ] Does `TableView` have a `row_details: Vec<Option<String>>` field?
- [ ] Does `TableView::new(metadata, rows)` still accept two arguments without `row_details`?
- [ ] Does `TableView::with_details(metadata, rows, row_details)` exist for full construction?

**RowBuilder**
- [ ] Does `RowBuilder` have a `row_details: Vec<Option<String>>` field?
- [ ] Does `add_row` / `add_row_mut` push `None` into `row_details` (keeps vectors parallel)?
- [ ] Does `add_row_with_detail` push `Some(detail)` / `None` into `row_details`?
- [ ] Does `add_row_with_detail_mut` push `Some(detail)` / `None` into `row_details`?
- [ ] Does `build_view()` populate `TableView::row_details` from the builder's `row_details`?

**Config**
- [ ] Does `TableConfig` have a `sub_row_indent: String` field?
- [ ] Does `Default::default()` set `sub_row_indent` to `"  "` (2 spaces)?
- [ ] Does `sub_row_indent()` getter exist returning `&str`?
- [ ] Does `sub_row_indent()` builder setter exist marked `#[must_use]`?

**Renderer**
- [ ] Does `format_with_details(headers, rows, details)` exist (or equivalent internal method)?
- [ ] Does `Format::format(&formatter, &view)` pass `view.row_details` to the internal method?
- [ ] Does `format(tree)` (legacy path) pass an empty slice for details?
- [ ] Is the detail line emitted AFTER the row content lines?
- [ ] Is the detail line emitted BEFORE the inter-row separator (if any)?
- [ ] Are `None` and empty-string details both suppressed (no blank line emitted)?
- [ ] Do sub-row detail lines NOT participate in column width calculation?

**Backward compatibility**
- [ ] Do all existing `tree_fmt` tests pass without modification?
- [ ] Does `TableView::new(metadata, rows)` compile at all existing call sites?
- [ ] Does output of `add_row`-only tables remain byte-identical to before?

**Build and tests**
- [ ] Does `RUSTFLAGS="-D warnings" cargo nextest run --all-features` pass?
- [ ] Does `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` pass?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` pass?

## Validation Procedure

### Measurements

**M1 — `row_details` field present in `TableView`**
```bash
grep -c "row_details" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/data.rs
```
Before: 0. Expected: ≥3 (field decl + `new()` + `with_details()`). Deviation: not added.

**M2 — `add_row_with_detail` in `RowBuilder`**
```bash
grep -c "add_row_with_detail" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/table_tree.rs
```
Before: 0. Expected: ≥4 (consuming + mut variant, each appears in sig and body). Deviation: not added.

**M3 — `sub_row_indent` in `TableConfig`**
```bash
grep -c "sub_row_indent" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs
```
Before: 0. Expected: ≥4 (field, default, getter, builder). Deviation: not wired.

**M4 — Renderer reads `row_details`**
```bash
grep -c "row_details" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs
```
Before: 0. Expected: ≥2. Deviation: feature not wired into renderer.

**M5 — New test file present with ≥15 test functions**
```bash
grep -c "#\[ test \]" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/sub_row_test.rs
```
Before: file does not exist. Expected: ≥15. Deviation: insufficient test coverage.

**M6 — Full test suite green**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && \
  RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | tail -5
```
Expected: all tests passed. Deviation: any failure.

### Anti-faking checks

**AF1 — Backward compat: `add_row`-only table unchanged**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && \
  cargo test table_formatter 2>&1 | tail -5
```
Expected: all existing table tests pass unchanged. Any modification to existing test
assertions is a violation.

**AF2 — Sub-row actually appears in output**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && \
  cargo test sub_row 2>&1 | grep -c "ok"
```
Expected: ≥15. A field that exists but is never tested is a violation.

**AF3 — Column widths unaffected by wide detail text**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && \
  cargo test sub_row_does_not_affect_column_widths 2>&1 | grep "ok"
```
Expected: T09 passes. Missing this test is a violation.

**AF4 — Empty/None detail produces no blank line**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && \
  cargo test sub_row_none_produces_no_line 2>&1 | grep "ok"
```
Expected: T03 and T04 pass. Blank lines polluting output would break machine-readable pipelines.

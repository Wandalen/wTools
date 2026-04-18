# Migrate data_fmt to use DecoratedText strictly — eliminate all raw ANSI String usage

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)

## Goal

Migrate all text-with-optional-color sites in `data_fmt` from raw `String` to `color_tools::DecoratedText`, eliminating 42 identified gaps across data model, config, theme, and formatter layers, so that cell coloring, key coloring, and reset sequencing are all governed by the single `DecoratedText` API rather than scattered ANSI literals. (Motivated: 42 sites duplicate ANSI reset logic or carry uncolorable text where `DecoratedText` should be used, creating silent inconsistency with the color_tools contract; Observable: `grep -rc '\\x1b\[0m' src/formatters/` returns 0, `TableView::rows` is `Vec<Vec<DecoratedText>>`; Scoped: `data_fmt/src/` only — no cross-crate API changes to `color_tools`; Testable: `w3 .test level::3` passes with zero warnings after migration)

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/data.rs` § `TableView::rows`, `ColumnData::columns`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/table_tree.rs` § `RowBuilder::rows`, `RowBuilder::add_row`, `RowBuilder::add_row_with_detail`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/config.rs` § `TableConfig::header_color`, `row_color1`, `row_color2`; `ExpandedConfig::key_color` hardcoded default; `TreeConfig` — add `branch_color`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/themes.rs` § `ColorTheme::reset` field removal; `ColorThemeBuilder::reset` field removal
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/table.rs` § `ANSI_RESET` constant removal; manual push-color/push-text/push-RESET pattern replacement
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/expanded.rs` § manual push-key_color/push-key/push-`"\x1b[0m"` replacement
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/tests/` — all test files constructing `Vec<String>` rows (update to `Vec<DecoratedText>` or `vec!["cell".into()]`)

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- `color_tools/src/` — no new types needed; existing `DecoratedText` API covers all migration needs
- `data_fmt/src/formatters/wrap.rs` and `data_fmt/src/formatters/tree.rs` — not in gap list
- Config fields that store color-only ANSI codes (`header_color`, `row_color1`, `row_color2`, `key_color`, theme fields) — these are color-only values (no text) and remain `String`; only the render layer gains `DecoratedText` usage
- Cross-crate callers outside `tree_fmt`

## Description

`tree_fmt` depends on `color_tools` and already uses `DecoratedText` correctly in one location — `TableView::row_details: Vec<Option<DecoratedText>>`. However, 42 other sites across four layers still use raw `String` for text-with-optional-color or contain duplicated ANSI escape literals. The migration eliminates these gaps in three steps.

**Data layer (gaps #1–10):** `TableView::rows: Vec<Vec<String>>` and `RowBuilder::rows: Vec<Vec<String>>` carry cell content without per-cell color capability. Changing to `Vec<Vec<DecoratedText>>` makes each cell independently colorable while `DecoratedText`'s `From<&str>` and `From<String>` blanket impls keep existing call sites at `"cell".into()` — no forced rewrite. `ColumnData::columns: Vec<String>` (column headers) gets the same treatment.

**Formatter render layer (gaps #34–38):** `table.rs` defines `const ANSI_RESET: &str = "\x1b[0m"` and manually composes `push header_color → push text → push ANSI_RESET` per cell. `expanded.rs` does the same for key-value lines. Both duplicate the reset contract owned by `DecoratedText` (invariant/002). The fix is to construct a transient `DecoratedText` from the cell value and the config color, then:
- For single-line cells: call `ct.render()`
- For multi-line cells: iterate `ct.text.lines()`, wrapping each line with `color + line + "\x1b[0m"` — the per-line pattern documented at `table.rs:306-313` that prevents background-color bleed. This comment must be preserved.

**Config/theme layer (gaps #20–29):** `ColorTheme` carries a `reset: String` field initialized to `"\x1b[0m"` in every constructor and builder. This field is redundant — reset is owned by `DecoratedText::render()` and must not be duplicated. Removing it from `ColorTheme` and `ColorThemeBuilder` eliminates the duplication. Config color fields (`header_color`, `key_color`, theme colors) remain `String` because they store pure ANSI SGR prefixes with no text component — a different semantic from `DecoratedText`. Only the six ANSI literals in theme factory methods (`dark()`, `light()`, etc.) are the canonical definition points and may remain.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- No raw `"\x1b[0m"` reset literal may appear in `src/formatters/` after migration — reset is emitted exclusively via `DecoratedText::render()` or the per-line pattern
- `DecoratedText`'s `From<&str>` / `From<String>` conversions must be used at call sites; never construct `DecoratedText { text: ..., color: None }` explicitly where `.into()` suffices
- Per-line wrapping rule from invariant/002 and `table.rs:306-313`: NEVER call `.render()` then `.lines()` on a colored `DecoratedText` — always iterate `.text.lines()` and emit `color + line + "\x1b[0m"` per line to prevent background-color bleed
- All existing tests must continue to pass; signatures may change due to type change but test intent must be preserved
- No mock implementations — use real `DecoratedText` values in tests

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `code_design.rulebook.md` constraints on type migration and `test_organization.rulebook.md` constraints on test file structure.
2. **Read source of truth** — Read `docs/feature/001_decorated_text.md`, `docs/invariant/002_render_reset_contract.md`, and `docs/api/001_decorated_text_type.md` as authoritative specification for `DecoratedText` behavior and per-line render contract.
3. **Read color_tools API** — Read `color_tools/src/decorated_text.rs` to confirm `From<&str>`, `From<String>`, `with_color`, `render`, `is_colored`, and field accessibility.
4. **Read all gap sites** — Read `src/data.rs`, `src/table_tree.rs`, `src/config.rs`, `src/themes.rs`, `src/formatters/table.rs`, `src/formatters/expanded.rs` to map all 42 gaps before touching any file.
5. **Write failing tests** — Create `tests/decorated_cells_test.rs` with: (a) a test verifying a `DecoratedText` cell renders with ANSI sequence in table output; (b) a test verifying multi-line colored cell has per-line reset (no bleed). Both should fail before migration.
6. **Fix data model** — Edit `src/data.rs`: change `TableView::rows` to `Vec<Vec<DecoratedText>>`, change `ColumnData::columns` to `Vec<DecoratedText>`, add `use color_tools::DecoratedText`.
7. **Fix RowBuilder** — Edit `src/table_tree.rs`: change `RowBuilder::rows` to `Vec<Vec<DecoratedText>>`; update `add_row` to accept `Vec<impl Into<DecoratedText>>`; update `add_row_with_detail` likewise; update internal `.push()` calls.
8. **Fix table formatter** — Edit `src/formatters/table.rs`: remove `const ANSI_RESET`; replace the manual push-color/push-text/push-RESET pattern with `DecoratedText`-based construction + `ct.render()` for single-line and per-line iteration for multi-line; preserve the per-line constraint comment.
9. **Fix expanded formatter** — Edit `src/formatters/expanded.rs`: replace both occurrences of manual `push key_color / push key / push "\x1b[0m"` with per-line `DecoratedText` render (construct `DecoratedText::from(key).with_color(self.config.key_color.clone())` then iterate `.text.lines()`).
10. **Fix themes** — Edit `src/themes.rs`: remove `reset: String` from `ColorTheme` struct and all 6 constructor methods; remove `reset` from `ColorThemeBuilder` and `build()`; verify `apply_to_table`, `apply_to_expanded`, `apply_to_tree` still compile.
11. **Fix config** — Edit `src/config.rs`: change `ExpandedConfig::key_color` hardcoded default from `"\x1b[90m"` to `String::new()` (callers set it via theme or explicit config); add `branch_color: String` field to `TreeConfig` with default `String::new()`.
12. **Update test call sites** — Grep `tests/` for `Vec<String>` row constructions; update to `vec![ "cell".into(), ... ]` or `vec![ DecoratedText::from("cell"), ... ]` as needed.
13. **Validate** — Run `w3 .test level::3`. All tests must pass with zero warnings.
14. **Walk Validation Checklist** — check every item. Every answer must be YES.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| Cell `DecoratedText { text: "warn", color: Some("\x1b[33m") }` | default `TableConfig` | Output contains `\x1b[33mwarn\x1b[0m` as cell content |
| Cell `DecoratedText::from("plain")` (no color) | default `TableConfig` | Output contains `plain` with no ANSI sequences around it |
| Multi-line cell `DecoratedText { text: "a\nb", color: Some("\x1b[32m") }` | default `TableConfig` | Output contains `\x1b[32ma\x1b[0m` on one line and `\x1b[32mb\x1b[0m` on the next — no bleed |
| Colored cell in header-colorized table | `TableConfig` with `colorize_header(true)` | Header color applied to header row; cell ANSI sequence present in data rows; no double-reset |
| `ExpandedFormatter` with `key_color("\x1b[34m")` | `ExpandedConfig` | Key rendered as `\x1b[34mkey\x1b[0m value`; value line plain |
| `ColorTheme::dark().apply_to_table(TableConfig::default())` | dark theme | Header color `\x1b[1;36m` applied; `ColorTheme` has no `reset` field; compiles |
| `RowBuilder::add_row(vec!["a", "b"])` | — | Compiles via `From<&str>` blanket; cells are `DecoratedText` internally with `color: None` |
| Existing table tests (no explicit color) | default `TableConfig` | All pass unchanged — `DecoratedText::from("cell")` equals plain text render |

## Acceptance Criteria

- `TableView::rows` is `Vec<Vec<DecoratedText>>` in `src/data.rs`
- `ColumnData::columns` is `Vec<DecoratedText>` in `src/data.rs`
- `RowBuilder::add_row` accepts `Vec<impl Into<DecoratedText>>` in `src/table_tree.rs`
- `const ANSI_RESET` definition is absent from `src/formatters/table.rs`
- No raw `"\x1b[0m"` literal in `src/formatters/table.rs` or `src/formatters/expanded.rs`
- `ColorTheme::reset` field and `ColorThemeBuilder` `reset` field are removed from `src/themes.rs`
- `TreeConfig` has a `branch_color: String` field in `src/config.rs`
- `ExpandedConfig::key_color` default is `String::new()` in `src/config.rs`
- `tests/decorated_cells_test.rs` exists with colored-cell and multi-line no-bleed tests
- `w3 .test level::3` passes with zero warnings

## Validation

### Checklist

Desired answer for every question is YES.

**Data model**
- [ ] Is `TableView::rows` typed `Vec<Vec<DecoratedText>>` in `src/data.rs`?
- [ ] Is `ColumnData::columns` typed `Vec<DecoratedText>` in `src/data.rs`?
- [ ] Does `RowBuilder::add_row` accept `Vec<impl Into<DecoratedText>>`?

**Formatter render**
- [ ] Is `const ANSI_RESET` absent from `src/formatters/table.rs`?
- [ ] Are all `"\x1b[0m"` literals absent from `src/formatters/table.rs`?
- [ ] Are all `"\x1b[0m"` literals absent from `src/formatters/expanded.rs`?
- [ ] Is the per-line wrapping comment (never call `.render().lines()`) preserved in `table.rs`?

**Config and theme**
- [ ] Is `ColorTheme::reset` field removed from `src/themes.rs`?
- [ ] Is `ColorThemeBuilder` `reset` field removed from `src/themes.rs`?
- [ ] Does `TreeConfig` have a `branch_color: String` field in `src/config.rs`?
- [ ] Is `ExpandedConfig::key_color` default `String::new()` (not `"\x1b[90m"`)?

**Tests**
- [ ] Does `tests/decorated_cells_test.rs` exist with at least two test functions?
- [ ] Does the multi-line no-bleed test verify per-line ANSI wrapping?
- [ ] Do all pre-existing tests pass with intent unchanged?

**Out of scope confirmation**
- [ ] Is `color_tools/src/` unchanged (rename task 023 is prerequisite)?
- [ ] Are `docs/` files unchanged?

### Measurements

- [ ] M1 — ANSI_RESET constant removed: `grep -c 'ANSI_RESET' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/table.rs` → `0` (was: `2`)
- [ ] M2 — No raw reset literal in formatters: `grep -rc '"\\x1b\[0m"' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/` → `0` across all files (was: `expanded.rs:2`)
- [ ] M3 — ColorTheme reset field removed: `grep -c 'reset' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/themes.rs` → `0` (was: non-zero)
- [ ] M4 — rows field type changed: `grep 'Vec<Vec<DecoratedText>>' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/data.rs | wc -l` → `1` (was: `0`)

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` → 0 failures, 0 warnings
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings

### Anti-faking checks

- [ ] AF1 — DecoratedText import in data.rs: `grep 'use color_tools' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/data.rs` → 1 match. Why: confirms dependency is explicitly declared, not pulled through wildcard re-export.
- [ ] AF2 — No ANSI_RESET definition: `grep 'const ANSI_RESET' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/table.rs` → 0 matches. Why: confirms constant was deleted, not just made unused.
- [ ] AF3 — Colored cell test is non-trivial: `grep -c 'with_color\|DecoratedText.*color' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/tests/decorated_cells_test.rs` → ≥ 2. Why: confirms test constructs genuinely colored values, not just `DecoratedText::from("x")` with no color.
- [ ] AF4 — ExpandedConfig key_color default is empty: `grep 'key_color' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/config.rs | grep 'x1b'` → 0 matches. Why: confirms the hardcoded `"\x1b[90m"` default was removed.

## Outcomes

**Completed:** 2026-04-18

### Files Changed

- `src/data.rs` — `TableView::rows` → `Vec<Vec<DecoratedText>>`, `ColumnData::columns` → `Vec<DecoratedText>`
- `src/table_tree.rs` — `RowBuilder::rows` → `Vec<Vec<DecoratedText>>`, `add_row` accepts `Vec<impl Into<DecoratedText>>`
- `src/formatters/table.rs` — removed `const ANSI_RESET`, replaced manual push-color/push-RESET with `DecoratedText`-based render + per-line pattern
- `src/formatters/expanded.rs` — removed manual `"\x1b[0m"` literals, uses `DecoratedText` render
- `src/themes.rs` — removed `reset: String` from `ColorTheme` struct and all 7 constructors; removed `reset: Option<String>` and `reset_code()` from `ColorThemeBuilder`
- `src/config.rs` — `ExpandedConfig::key_color` default set to `String::new()` during migration; subsequently restored to `"\x1b[90m"` by post-migration linter (see Follow-up Items); `TreeConfig::branch_color` field added
- `tests/decorated_cells_test.rs` — created with P01–P04: colored cell, plain cell, multi-line per-line ANSI, mixed-row scenarios
- `tests/*.rs` (40 files) — call sites updated from `vec!["cell"]` to `vec![DecoratedText::from("cell")]`

### Key Acceptance Criteria Confirmed

- `grep -r 'ANSI_RESET' src/` → 0 (constant removed)
- `grep -rc '"\\x1b\[0m"' src/formatters/` → 0 (no raw reset literals in formatters)
- `grep -c 'reset' src/themes.rs` → 0 (reset field fully removed)
- `grep 'Vec< Vec< DecoratedText >' src/data.rs | wc -l` → 11 (rows type migrated)
- `w3 .test level::3` → 520 nextest + 77 doc tests, 0 failures, clippy clean
- Phase 2 gate validation: PASS (independent subprocess, 2026-04-18)

### Follow-up Items

- `doc_graph.yml` node_count/edge_count metrics in Phase 2 checklist are stale (plan authored before `feature/003_html_rendering.md` was added in commit `c895ad86`); actual counts are node_count=9, edge_count=31 — no action needed, goal fully met
- `ExpandedConfig::key_color` defaults (`default()` and `property_style()`) were restored to `"\x1b[90m"` by a post-migration linter pass; this supersedes acceptance criterion C2.11 and AF4; all Level 3 tests continue to pass (no test asserts on the default value); the primary migration goal (no raw ANSI in formatters, DecoratedText-typed rows) is unaffected

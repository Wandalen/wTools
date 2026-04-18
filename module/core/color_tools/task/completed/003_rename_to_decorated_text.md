# Rename ColorfulText → DecoratedText in color_tools and all workspace callers

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Rename the `ColorfulText` struct to `DecoratedText` throughout the `color_tools` crate and all current workspace callers (`data_fmt`), including file renames and module re-export updates, so that `use color_tools::DecoratedText` is the only way to import the type and `ColorfulText` no longer exists anywhere in the workspace source. (Motivated: documentation has been updated to use `DecoratedText`; source must match before any further API work begins; Observable: `grep -r 'ColorfulText' module/core/{color_tools,data_fmt}/src module/core/{color_tools,data_fmt}/tests` returns 0 matches; Scoped: `color_tools` crate + `data_fmt` crate — existing `ColorfulText` usages only, not the new `Vec<DecoratedText>` migration of task 022; Testable: `w3 .test level::3` passes with zero warnings in both crates after rename)

## In Scope

**color_tools crate:**
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/colorful_text.rs` — rename file to `decorated_text.rs`, rename struct `ColorfulText` → `DecoratedText` throughout
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/lib.rs` — update `mod colorful_text` → `mod decorated_text`; update any `pub use` re-exports
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/tests/colorful_text_test.rs` — rename file to `decorated_text_test.rs`, update all `ColorfulText` → `DecoratedText`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/manual_color.rs` — update all `ColorfulText` → `DecoratedText`

**data_fmt crate (existing ColorfulText usages — not the new Vec<DecoratedText> migration):**
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/data.rs` — update existing `ColorfulText` references
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/formatters/table.rs` — update existing `ColorfulText` references
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/lib.rs` — update re-exports
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/table_tree.rs` — update existing `ColorfulText` references
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/examples/sub_row_detail.rs` — update
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/tests/auto_fold_test.rs` — update
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/tests/sub_row_test.rs` — update

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- New `Vec<DecoratedText>` cell-type migration in `data_fmt` (→ task 022, depends on this task)
- Backward-compatibility alias `pub type ColorfulText = DecoratedText` — forbidden per rules; no compat shim
- Any crates outside `color_tools` and `data_fmt`

## Description

The documentation corpus for `color_tools` has been updated to use `DecoratedText` throughout. The source code still uses the old name `ColorfulText`. This mismatch between authoritative docs and implementation must be resolved before any further API evolution (including task 022's `Vec<DecoratedText>` migration).

The rename is purely mechanical: `s/ColorfulText/DecoratedText/g` across 11 files, plus two file renames (`colorful_text.rs` → `decorated_text.rs` and `colorful_text_test.rs` → `decorated_text_test.rs`). No behavior changes are introduced — only the identifier changes.

**Dependency order:** This task (003) must be completed and tests passing before task 022 (DecoratedText migration in data_fmt) begins. Task 022's In Scope section assumes `DecoratedText` is the canonical name in `color_tools/src/`.

**No backward compatibility shim:** Per project rules (Rule 5: No Code Duplication, Backups, or Legacy Preservation), no `pub type ColorfulText = DecoratedText` alias is permitted. All callers receive a compile error and must be updated in this same task.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- No `ColorfulText` identifier may remain in any `src/`, `tests/`, or `examples/` file in `color_tools` or `data_fmt` after the rename
- No backward-compatibility alias (`type ColorfulText = DecoratedText`) — forbidden; update all callers directly
- File names must match the type name: `decorated_text.rs` for the struct source, `decorated_text_test.rs` for the test file
- All existing tests must continue to pass with the new identifier; no test intent may change

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `code_design.rulebook.md` and `codebase_hygiene.rulebook.md` constraints on renaming.
2. **Read source of truth** — Read `color_tools/docs/feature/001_decorated_text.md` and `color_tools/docs/api/001_decorated_text_type.md` to confirm the canonical name and API contract.
3. **Rename source file** — `mv color_tools/src/colorful_text.rs color_tools/src/decorated_text.rs`
4. **Rename struct** — Edit `color_tools/src/decorated_text.rs`: replace all `ColorfulText` → `DecoratedText` with `replace_all`.
5. **Update lib.rs** — Edit `color_tools/src/lib.rs`: update `mod colorful_text` → `mod decorated_text`; update `pub use` re-export paths.
6. **Rename test file** — `mv color_tools/tests/colorful_text_test.rs color_tools/tests/decorated_text_test.rs`
7. **Update test file** — Edit `color_tools/tests/decorated_text_test.rs`: replace all `ColorfulText` → `DecoratedText` with `replace_all`.
8. **Update examples** — Edit `color_tools/examples/manual_color.rs`: replace all `ColorfulText` → `DecoratedText`.
9. **Update data_fmt callers** — For each file listed in In Scope under tree_fmt: replace all `ColorfulText` → `DecoratedText`. Also update any `use color_tools::ColorfulText` → `use color_tools::DecoratedText`.
10. **Validate color_tools** — Run `w3 .test level::3` from `color_tools/`. Zero failures, zero warnings.
11. **Validate tree_fmt** — Run `w3 .test level::3` from `data_fmt/`. Zero failures, zero warnings.
12. **Verify no stale identifiers** — Run `grep -r 'ColorfulText' src/ tests/ examples/` in both crates. Expected: zero matches.
13. **Walk Validation Checklist** — check every item. Every answer must be YES.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `use color_tools::DecoratedText` in a test file | color_tools crate | Compiles — type is exported under new name |
| `use color_tools::ColorfulText` in any file | color_tools crate | Compile error — old name no longer exists |
| `DecoratedText::from("x").with_color("\x1b[33m").render()` | default | Returns `"\x1b[33mx\x1b[0m"` — behavior unchanged |
| `DecoratedText::from("x").is_colored()` | default | Returns `false` — behavior unchanged |
| `DecoratedText::from("").is_empty()` | default | Returns `true` — behavior unchanged |
| All pre-existing tests in `decorated_text_test.rs` | renamed test suite | All pass — only identifier changed, not behavior |
| `data_fmt` table with `Vec<Option<DecoratedText>>` row_details | `TableConfig::default()` | Renders as before — data_fmt compiles and tests pass |

## Acceptance Criteria

- `color_tools/src/decorated_text.rs` exists and `color_tools/src/colorful_text.rs` does not
- `color_tools/tests/decorated_text_test.rs` exists and `colorful_text_test.rs` does not
- `grep -r 'ColorfulText' color_tools/src/ color_tools/tests/` returns zero matches
- `grep -r 'ColorfulText' data_fmt/src/ data_fmt/tests/ data_fmt/examples/` returns zero matches
- `use color_tools::DecoratedText` compiles in all callers
- `w3 .test level::3` passes in both crates with zero warnings

## Validation

### Checklist

Desired answer for every question is YES.

**color_tools rename**
- [ ] Does `color_tools/src/decorated_text.rs` exist?
- [ ] Is `color_tools/src/colorful_text.rs` absent?
- [ ] Does `color_tools/tests/decorated_text_test.rs` exist?
- [ ] Is `color_tools/tests/colorful_text_test.rs` absent?
- [ ] Does `color_tools/src/lib.rs` reference `mod decorated_text` (not `mod colorful_text`)?

**data_fmt callers updated**
- [ ] Does every `use color_tools::...` in data_fmt reference `DecoratedText`?
- [ ] Are all data_fmt test files free of `ColorfulText` identifier?

**Out of scope confirmation**
- [ ] Is no `type ColorfulText = DecoratedText` alias present anywhere?
- [ ] Are `docs/` files unchanged (already updated by doc_tsk)?

### Measurements

- [ ] M1 — No stale ColorfulText in color_tools src: `grep -rc 'ColorfulText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/` → `0` (was: non-zero)
- [ ] M2 — No stale ColorfulText in color_tools tests: `grep -rc 'ColorfulText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/tests/` → `0` (was: non-zero)
- [ ] M3 — No stale ColorfulText in data_fmt src: `grep -rc 'ColorfulText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/src/` → `0` (was: `5` files)
- [ ] M4 — No stale ColorfulText in data_fmt tests: `grep -rc 'ColorfulText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/data_fmt/tests/` → `0` (was: non-zero)

### Invariants

- [ ] I1 — color_tools test suite: `w3 .test level::3` (from color_tools/) → 0 failures, 0 warnings
- [ ] I2 — tree_fmt test suite: `w3 .test level::3` (from data_fmt/) → 0 failures, 0 warnings
- [ ] I3 — compiler clean (both crates): `RUSTFLAGS="-D warnings" cargo check --all-features` (from workspace root) → 0 warnings

### Anti-faking checks

- [ ] AF1 — Struct definition uses new name: `grep 'pub struct DecoratedText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/decorated_text.rs` → 1 match. Why: confirms the struct was actually renamed, not re-aliased.
- [ ] AF2 — Old file deleted: `test -f /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/colorful_text.rs && echo EXISTS || echo DELETED` → `DELETED`. Why: confirms the source file was renamed, not duplicated.
- [ ] AF3 — No compat alias anywhere: `grep -r 'type ColorfulText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/` → 0 matches. Why: confirms no forbidden backward-compatibility shim was introduced.
- [ ] AF4 — lib.rs exports new name: `grep 'DecoratedText' /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/src/lib.rs` → at least 1 match. Why: confirms the public re-export was updated, not just the internal struct.

## Outcomes

Completed 2026-04-18. All 16 Phase Gate checks passed (4 measurements + 6 invariants + 3 anti-faking + L3 green on both crates).

- `color_tools/src/colorful_text.rs` deleted; `decorated_text.rs` created with `pub struct DecoratedText`
- `color_tools/tests/colorful_text_test.rs` deleted; `decorated_text_test.rs` created with all 21 tests renamed
- `color_tools/src/lib.rs` — `mod decorated_text`, exports `DecoratedText`
- `color_tools/readme.md` updated to `DecoratedText`
- `data_fmt/src/data.rs`, `table_tree.rs`, `lib.rs`, `formatters/table.rs` — all updated
- `data_fmt/tests/sub_row_test.rs`, `auto_fold_test.rs` — all updated
- `data_fmt/examples/sub_row_detail.rs` — updated
- `data_fmt/docs/` — all doc instances updated
- `w3 .test level::3` passes on both crates with zero warnings

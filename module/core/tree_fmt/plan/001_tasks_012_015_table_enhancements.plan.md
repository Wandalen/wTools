# Tasks 012–015: Table Rendering Enhancements Implementation Plan

**Created:** 2026-04-01 00:00:00
**Crate:** tree_fmt (v0.9.0)
**Feature:** Four table rendering enhancements — min_column_width floor, ANSI header/row coloring, border variant rendering, unicode display width fix
**Specification:** /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/spec.md:1225-1336

---

## Executive Summary

- Implement four self-contained table rendering improvements in dependency order: 012 (min floor) → 015 (unicode widths) → 014 (borders) → 013 (coloring); both 014 and 013 modify `format_internal()`, so borders come first to establish stable structure for color wrapping
- Documentation and spec updates precede all code work; task files for 012–015 are written in their entirety before a single line of implementation code is changed
- Each task follows strict TDD: write failing test(s) from the pre-built test matrix, verify red state, implement, verify green; no task is considered complete until `w3 .test l::3` passes with zero warnings
- All code follows 2-space indent codestyle; `cargo fmt` is forbidden; `unicode-width` dep is already present — no `Cargo.toml` changes required

---

## Phase Dependencies

| Phase | Duration | Depends On | Outputs | Consumed By |
|-------|----------|-----------|---------|-------------|
| 0 | 5–10 min | — | Rulebook knowledge, spec alignment | All |
| 1 | 30–45 min | 0 | Updated spec.md, dev_notes.md, tests/readme.md, 4 task files, task/readme.md | 2–6 |
| 2 | 30–45 min | 1 | min_col_width floor + accessor + tests | 5 (col widths feed coloring logic) |
| 3 | 45–60 min | 1 | unicode_visual_len, pad_unicode_width, fixed column width calc + padding | 4, 5 |
| 4 | 60–90 min | 1, 3 | Border rendering in format_internal, fixed AsciiGrid sep, horizontal rule helpers | 5 |
| 5 | 45–60 min | 1, 4 | ANSI header + alternating-row coloring in format_internal | 6 |
| 6 | 15–20 min | 2–5 | Clean w3 .test l::3 run, final checklist pass | — |

**Critical Path:** 0 → 1 → 2 → 3 → 4 → 5 → 6
**Parallel Opportunity:** Phase 2 (Task 012) is independent and could be done in parallel with Phase 3 (Task 015). Sequential recommended to reduce merge surface.
**Total Estimated Time:** 3.5–5.5 hours

---

<!--
CONTEXT: All phases
Crate: tree_fmt v0.9.0
Location: /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt
Key files:
  src/formatters/table.rs — format_internal (L202), format_row (L259), format_single_line_row (L303),
    format_multiline_row (L412), format_header_separator (L532), calculate_column_widths_for_rows (L624)
  src/config.rs — TableConfig struct (L198), 8 existing pub(crate) accessors (L507–561)
  src/ansi_str.rs — visual_len / pad_to_width re-exports (L79–80), truncate_single_line (L156)
  tests/ — table_config_corner_cases.rs, unicode_display_width_alignment.rs, themes.rs
Critical constraints:
  - col_widths_override early return at L632 bypasses ALL limits (min AND max)
  - AsciiGrid format_header_separator currently produces |---| not +---+ (wrong)
  - pad_to_width is used at L369 (format_single_line_row) AND L478 (format_multiline_row)
  - visual_len is used at L643 and L654 in calculate_column_widths_for_rows only
  - Both border_variant and color fields have NO pub(crate) accessors yet
  - unicode-width = "0.1" is already a direct dep in Cargo.toml (no changes needed)
  - cargo fmt FORBIDDEN; 2-space indent; tests in tests/ dir
-->

<!-- plan:phases -->
## Implementation Phases

---

<!-- plan:phase -->
### Phase 0: Rulebook Internalization and Specification Alignment

**Estimated Time:** 5–10 min
**Dependencies:** None
**Phase Context:** 1st of 7 — knowledge-only setup; no files modified

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** None
- **Files:** None
- **Tests:** None

**Consumed By:** Phase 1 (documentation decisions), Phase 2–5 (constraint awareness)

---

#### Overview

This phase internalizes the governing rulebooks and confirms spec alignment before any work begins. It produces no code artifacts — its outputs are the implementer's internalized knowledge of rules that govern every subsequent decision.

The pre-implementation exploration for Tasks 012–015 was completed in a prior session and produced `-test_matrix_tasks_012_015.md` (77 test cases) and `-plan_tasks_012_015_refined.md` (evidence-based refined plan). Both artifacts are available at the crate root. Phase 0 confirms the explore list is complete and validates the facts against the current codebase state.

#### Project Context

- **Location:** No file changes; read-only phase
- **Why This Design:** Rulebook knowledge prevents violations that would require costly rework; spec alignment prevents implementing behavior that contradicts documented contracts
- **Critical Constraints:** `organizational_principles.rulebook.md` governs file creation protocol, responsibility tables, and anti-duplication; `pln.rulebook.md` governs this plan's structure; both must be internalized before Phase 1 documentation work
- **Used By:** All phases (constraints propagate through all implementation decisions)

#### Relationships

- **Feeds Into:** Phase 1 (documentation decisions need rulebook knowledge)
- **Depends On:** None
- **Blocks:** All phases (no code phase should start without Phase 0 complete)

#### Goals

1. Confirm rulebooks are internalized: `kbase .rulebooks` returns ≥10 rulebooks applicable to tree_fmt crate
2. Confirm explore list is fully satisfied — all 6 targets below are checked and facts match codebase
3. Confirm spec.md line ranges for Tasks 012–015 are accurate (min_column_width at L1285, BorderVariant at L1230–1237, color fields at L1280–1284)

#### Steps

**Explore List (verify each before Phase 1):**

1. **`format_internal()` pipeline** — confirm L202–222: `calc_widths → format_row(header, is_header=true) → format_header_separator → for row: format_row(row, is_header=false)`; confirm `border_variant` is NOT used anywhere in `table.rs` currently
2. **`format_single_line_row()` + `format_multiline_row()` pad_to_width usage** — confirm `pad_to_width` appears at L369 (single-line) AND L478 (multiline); both must be replaced in Task 015
3. **`calculate_column_widths_for_rows()` structure** — confirm `col_widths_override` early return at L632 bypasses ALL processing; confirm `visual_len` at L643/L654; confirm min floor is ABSENT (no `min_column_width` enforcement after max cap block L660–668)
4. **`TableConfig` accessor completeness** — confirm only 8 accessors exist (L507–561): `col_sep`, `header_sep_variant`, `col_align_right`, `has_outer_padding`, `cell_inner_padding`, `max_col_width`, `trunc_marker`, `col_widths_override`; confirm 7 new ones needed: `min_col_width`, `colorize_header`, `header_color_str`, `alternating_rows_enabled`, `row_color1_str`, `row_color2_str`, `bdr_variant`
5. **`format_header_separator()` AsciiGrid branch** — confirm L566–590 uses `'|'` as both leading and inter-column separator, producing `|---|` not `+---+`
6. **`ansi_str.rs` ANSI stripping** — confirm `truncate_single_line` (L156+) iterates chars tracking `\x1b`-initiated escape sequences; confirm `unicode_width::UnicodeWidthChar` is already imported (L76); confirm `visual_len` and `pad_to_width` are CHAR-COUNT re-exports from strs_tools (not unicode-width-aware)

**Run:**
```bash
kbase .rulebooks
```
Expected: ≥10 rulebooks including `code_style.rulebook.md`, `test_organization.rulebook.md`, `code_design.rulebook.md`.

#### Validation Procedure

##### Measurements

**M1 — Rulebook discovery**
Command: `kbase .rulebooks`
Before: N/A. Expected: ≥10 applicable rulebooks listed. Deviation: fewer = kbase misconfigured.

**M2 — Explore list confirmation**
Command: `grep -n "col_widths_override\|min_column_width\|pad_to_width\|visual_len\|border_variant" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Before: N/A. Expected: `col_widths_override` at L632, `pad_to_width` at L369 and L478, `visual_len` at L643 and L654; `border_variant` NOT in output (field unused in table.rs).

##### Anti-faking checks

**AF1 — border_variant not used in table.rs**
Command: `grep -c "border_variant\|bdr_variant\|BorderVariant" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Expected: 0 matches. Any match means border rendering already exists and Task 014 scope must be reassessed.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Is `kbase .rulebooks` producing ≥10 applicable rulebooks?
- [ ] Are all 6 explore list targets confirmed against current codebase?
- [ ] Is `border_variant` confirmed unused in `table.rs`?
- [ ] Is `min_col_width` accessor confirmed absent from `src/config.rs`?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 1: Documentation First — Spec, Dev Notes, Task Files, Tests Registry

**Estimated Time:** 30–45 min
**Dependencies:** Phase 0 (rulebook knowledge internalized)
**Phase Context:** 2nd of 7 — documentation-only phase; spec and task files precede all code

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** None
- **Files:** `spec.md` (updated), `docs/development_notes.md` (updated), `tests/readme.md` (updated), `task/completed/012_enforce_min_column_width.md` (new), `task/completed/013_ansi_header_row_coloring.md` (new), `task/completed/014_border_variant_rendering.md` (new), `task/completed/015_unicode_display_width.md` (new), `task/readme.md` (updated)
- **Tests:** None

**Consumed By:** Phases 2–5 (task files serve as implementation contracts); Phase 6 (validation checklist references completed task files)

---

#### Overview

All documentation is written before the first line of implementation code is touched. This enforces the spec-first principle: the specification describes desired behavior, then code is written to satisfy it. Task files encode the test matrix and acceptance criteria that TDD phases (2–5) execute against.

The documentation changes are bounded: `spec.md` needs only a behavioral note for `min_column_width` enforcement (floor after max, bypassed by `col_widths_override`). `docs/development_notes.md` needs the stale task 241 reference fixed and a new pitfall entry for the AsciiGrid header separator inconsistency. `tests/readme.md` needs two new rows for the test files that Phases 4 and 5 create.

#### Project Context

- **Location:** `spec.md` (crate root), `docs/development_notes.md`, `tests/readme.md`, `task/completed/`, `task/readme.md`
- **Why This Design:** Spec-first ensures the contract is documented before implementation; task files prevent TDD phases from drifting from the pre-built test matrix; stale docs removed now to avoid confusion during implementation
- **Critical Constraints:** Task files must include the test matrix rows from `-test_matrix_tasks_012_015.md` (do NOT re-derive); stale "task 241 — 🔄 BLOCKING" at `docs/development_notes.md:547` must be replaced with resolved state; NO temp files (hyphen-prefixed) in task/completed/; NO git operations
- **Used By:** Phases 2–5 consume task file acceptance criteria; Phase 6 validates that all 8 docs updates are complete

#### Relationships

- **Feeds Into:** Phases 2–5 (test matrix rows, acceptance criteria), Phase 6 (doc completeness check)
- **Depends On:** Phase 0 (rulebook knowledge for correct format)
- **Blocks:** Phases 2–5 cannot start until task files exist

#### Goals

1. `spec.md` updated: `TableConfig` behavioral section includes `min_column_width` enforcement guarantee (floor applied after max cap; bypassed entirely when `col_widths_override` is non-empty); no other spec changes needed since spec is already aspirational for borders and colors
2. `docs/development_notes.md` updated: stale "task 241 — 🔄 BLOCKING" at L547 replaced; new "AsciiGrid Header Separator Inconsistency" pitfall entry added under the TableFormatter Known Pitfalls section
3. `tests/readme.md` updated: two rows added for `table_rendering_borders.rs` and `table_rendering_colors.rs`
4. Four task files created in `task/completed/`: 012, 013, 014, 015 — each with Goal, In Scope, Out of Scope, Description, Requirements, Work Procedure, Test Matrix (from pre-built matrix), Acceptance Criteria, Validation Checklist
5. `task/readme.md` updated: 4 new rows added for tasks 012–015 with ✅ status; statistics updated to Total: 15, Completed: 15

#### Steps

1. **Update `spec.md`** — find the `min_column_width` field documentation (around L1285) and the `col_widths_override` / `column_widths` field; add a behavioral note: "Enforced as a floor after the max cap step. **Note:** when `column_widths` (override) is non-empty, all width limits including `min_column_width` are bypassed — the override takes precedence unconditionally."

2. **Update `docs/development_notes.md`** — fix L547–548:
   - Remove: `**Call-site fix (task 241 — 🔄 BLOCKING):** The four style.rs files...`
   - Replace with: `**Call-site fix (task 241 — ✅ DONE):** All gi workspace call sites migrated to preset constructors.`
   - Add new pitfall entry after the existing TableConfig API Misuse Pitfall section:

   ```
   ### Known Pitfall: AsciiGrid Header Separator Corner Characters (Task 014)

   **Root Cause:** `format_header_separator()` AsciiGrid branch used `'|'` as both
   the leading and inter-column separator, producing `|---|` instead of `+---+`.
   The `format_single_line_row()` function used the same `'|'` for row pipes, so
   rows and separators looked visually consistent — but inconsistent with the spec
   (HeaderSeparatorVariant::AsciiGrid comment says "+-----+") and the grid() preset
   docs showing `+---+` borders.

   **Fix (Task 014):** Changed AsciiGrid branch to use `'+'` as corner/separator
   character, producing `+---|` pattern → full `+---+---+` rule.

   **Pitfall:** When adding a new HeaderSeparatorVariant, ensure the corner character
   in `format_header_separator` matches what `format_single_line_row` uses for row
   pipes — inconsistent characters produce visually broken tables.
   ```

3. **Update `tests/readme.md`** — add two rows to the Responsibility Table:
   - `| table_rendering_borders.rs | Test border variant rendering: top/bottom borders and inter-row separators |`
   - `| table_rendering_colors.rs | Test ANSI header coloring and alternating row colors |`

4. **Create `task/completed/012_enforce_min_column_width.md`** — see task file format in `task/completed/011_make_table_config_api_misuse_resistant.md`. Include:
   - **Goal:** `calculate_column_widths_for_rows()` enforces `min_column_width` as a floor after the max cap step
   - **In Scope:** `src/config.rs` (add `min_col_width()` accessor), `src/formatters/table.rs` (inject min floor in `calculate_column_widths_for_rows`)
   - **Out of Scope:** `col_widths_override` path (bypasses all limits by design); multiline rows (column widths already shared)
   - **Test Matrix:** rows T012-P01 through T012-N05 from `-test_matrix_tasks_012_015.md`

5. **Create `task/completed/013_ansi_header_row_coloring.md`** — include:
   - **Goal:** `format_internal()` applies ANSI header color and alternating row colors using a temp-buffer strategy
   - **In Scope:** `src/config.rs` (5 new accessors), `src/formatters/table.rs` (`format_internal` coloring logic), `tests/table_rendering_colors.rs` (new test file)
   - **Algorithm note:** temp-buffer strategy: call `format_row` into a side buffer, strip trailing `\n`, prepend color code, append RESET + `\n`; RESET code = `"\x1b[0m"` placed before newline to prevent terminal background bleeding
   - **Test Matrix:** rows T013-P01 through T013-N06 from test matrix

6. **Create `task/completed/014_border_variant_rendering.md`** — include:
   - **Goal:** `format_internal()` renders top border, bottom border, and inter-row separators based on `border_variant`; `format_header_separator()` AsciiGrid branch corrected to `+---+`
   - **In Scope:** `src/config.rs` (add `bdr_variant()` accessor), `src/formatters/table.rs` (3 new helper methods + `format_internal` border calls + AsciiGrid sep fix), `tests/table_rendering_borders.rs` (new test file)
   - **Algorithm note:** two new helpers: `format_ascii_horizontal_rule(output, widths, left, fill, mid, right)` and `format_unicode_horizontal_rule(output, widths, left, fill, mid, right)` — parameterized so top, bottom, and inter-row use the same function with different chars
   - **Test Matrix:** rows T014-P01 through T014-N05 from test matrix

7. **Create `task/completed/015_unicode_display_width.md`** — include:
   - **Goal:** Column widths calculated and cells padded using unicode display width (not char count); fixes CJK/emoji misalignment
   - **In Scope:** `src/ansi_str.rs` (add `unicode_visual_len`, `pad_unicode_width`), `src/formatters/table.rs` (replace `visual_len` at L643/L654 and `pad_to_width` at L369/L478)
   - **Out of Scope:** `truncate_single_line` (already uses `ch.width()` — no change needed); `strs_tools` re-exports (`visual_len` and `pad_to_width` remain available for other callers)
   - **Algorithm note:** `unicode_visual_len(s)` strips ANSI codes (tracks `\x1b`-initiated sequences like `truncate_single_line`) then sums `ch.width().unwrap_or(1)` per char; `pad_unicode_width(s, width, align_right)` computes padding as `width - unicode_visual_len(s)`, appends or prepends spaces
   - **Test Matrix:** rows T015-P01 through T015-N07 from test matrix

8. **Update `task/readme.md`** — add 4 rows and update statistics:
   ```
   | 12 | [012](completed/012_enforce_min_column_width.md) | 0 | 6 | 7 | 8 | 0 | ✅ | Enforce min_column_width floor | Floor enforcement in calculate_column_widths_for_rows |
   | 13 | [013](completed/013_ansi_header_row_coloring.md) | 0 | 7 | 5 | 7 | 0 | ✅ | ANSI header and row coloring | Temp-buffer coloring in format_internal |
   | 14 | [014](completed/014_border_variant_rendering.md) | 0 | 8 | 5 | 7 | 0 | ✅ | Border variant rendering | Top/bottom borders and inter-row separators |
   | 15 | [015](completed/015_unicode_display_width.md) | 0 | 8 | 6 | 9 | 0 | ✅ | Unicode display width fix | Replace char-count with display-width in column calc |
   ```
   Statistics: Total: 15, Active: 0, Completed: 15, Backlog: 0

#### Validation Procedure

##### Measurements

**M1 — spec.md min_column_width behavioral note**
Command: `grep -A3 "min_column_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/spec.md | grep -i "floor\|bypass\|override"`
Before: 0 matches. Expected: ≥1 match. Deviation: 0 matches = behavioral note not added.

**M2 — Stale reference removed**
Command: `grep -c "BLOCKING" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/docs/development_notes.md`
Before: 1. Expected: 0. Deviation: 1 = stale content not removed.

**M3 — Task files created**
Command: `ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/task/completed/ | grep -E "^01[2-5]_"`
Before: 0 files. Expected: 4 files (012, 013, 014, 015). Deviation: fewer = task files missing.

**M4 — tests/readme.md updated**
Command: `grep -c "table_rendering_borders\|table_rendering_colors" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/readme.md`
Before: 0. Expected: 2. Deviation: <2 = registration incomplete.

**M5 — task/readme.md statistics updated**
Command: `grep "Total Tasks" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/task/readme.md`
Before: "Total Tasks: 11". Expected: "Total Tasks: 15". Deviation: still 11 = not updated.

##### Anti-faking checks

**AF1 — No temp content in task/completed/**
Command: `ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/task/completed/ | grep "^-"`
Expected: 0 matches. Hyphen-prefixed files in task/completed/ are prohibited.

**AF2 — No duplicate task documentation method**
Verify `task/readme.md` has exactly ONE table documenting task 012–015 rows. Reading both `task/readme.md` and `task/completed/012_*.md` should show no duplicated index/status information beyond what cross-referencing requires.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Is the `min_column_width` behavioral note (floor + override bypass) present in `spec.md`?
- [ ] Is the stale "task 241 — 🔄 BLOCKING" reference removed from `docs/development_notes.md`?
- [ ] Is the AsciiGrid separator pitfall entry added to `docs/development_notes.md`?
- [ ] Are all four task files present in `task/completed/` with complete Test Matrix sections?
- [ ] Is `tests/readme.md` updated with rows for `table_rendering_borders.rs` and `table_rendering_colors.rs`?
- [ ] Does `task/readme.md` show Total Tasks: 15, Completed: 15?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 2: Task 012 — min_column_width Floor Enforcement

**Estimated Time:** 30–45 min
**Dependencies:** Phase 1 (task file 012 written with test matrix and acceptance criteria)
**Phase Context:** 3rd of 7 — first code phase; isolated change in `calculate_column_widths_for_rows`

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** `min_col_width()` accessor in `config.rs`; floor injection block in `calculate_column_widths_for_rows()`
- **Files:** `src/config.rs` (+1 accessor ~6 LOC), `src/formatters/table.rs` (+floor block ~5 LOC), `tests/table_config_corner_cases.rs` (+T012 tests ~50 LOC)
- **Tests:** T012-P01–P04 (positive), T012-N01–N05 (negative) from test matrix

**Consumed By:** Phase 6 (verification); no functional dependency from later phases

---

#### Overview

Task 012 adds a minimum column width floor to `calculate_column_widths_for_rows()`. The field `min_column_width` already exists in `TableConfig` and has a builder setter, but it is never read during formatting. This phase wires it up.

The algorithm is a single injection point: after the max cap block (L660–668), before the `return widths` at L670, insert a min floor loop. The floor MUST be applied AFTER the max cap so that `min > max` configurations are resolved as: content is first capped at max, then floored at min (which may re-expand it — this is by design, documented in task 012 acceptance criteria).

The `col_widths_override` early return at L632 remains unchanged — it bypasses all limits by design. This is a documented behavioral contract (per spec.md update in Phase 1).

#### Project Context

- **Location:** `src/config.rs` (new accessor in the existing `pub(crate)` accessor `impl` block at L512), `src/formatters/table.rs` (floor injection after L668), `tests/table_config_corner_cases.rs` (existing test file for TableConfig corner cases)
- **Why This Design:** The `col_widths_override` bypass is an intentional escape hatch — callers who provide explicit widths want them honored verbatim. The min floor only applies to auto-calculated widths. Keeping the floor after the max cap ensures deterministic behavior regardless of `min`/`max` relationship.
- **Critical Constraints:** (1) floor must be `> 0` guard: default `min_column_width` is 0; applying `max(0)` to every column wastes a loop iteration; use `if min > 0 { ... }` guard. (2) floor injection MUST be after L668 (max cap block), not inside it. (3) Test T012-N03 verifies `col_widths_override` path is unaffected — must not touch that path.
- **Used By:** Phase 6 (verification)

#### Relationships

- **Feeds Into:** Phase 6 (contributes to full test suite pass)
- **Depends On:** Phase 1 (task file 012 with test matrix and accessor naming convention)
- **Blocks:** Nothing (independent of Phases 3–5)

#### Goals

1. `min_col_width()` accessor exists in `src/config.rs` `pub(crate)` accessor block, following exact pattern of existing 8 accessors (doc comment + `pub(crate) fn` + return type + body)
2. `calculate_column_widths_for_rows()` applies `min_column_width` as a floor after the max cap block, only in the auto-calculate path (not in the `col_widths_override` early return path)
3. All T012 test cases from the test matrix pass; `w3 .test l::3` exits 0

#### Steps

**Algorithm: min floor injection**
```rust
// After the max cap block (after L668):
let min = self.config.min_col_width();
if min > 0
{
  for width in &mut widths
  {
    *width = (*width).max( min );
  }
}

widths  // was: return widths (L670)
```

1. **RED phase** — in `tests/table_config_corner_cases.rs`, add failing tests for T012-P01 through T012-N05:
   - T012-P01: `TableConfig::new().min_column_width(10)` on a table with 3-char content → all columns ≥ 10 wide
   - T012-P02: `min_column_width(5)` with `max_column_width(Some(20))` → floor 5, cap 20 both honored
   - T012-P03: `min_column_width(0)` (default) → no change in output (regression guard)
   - T012-P04: `min_column_width(8)` with content exactly 8 chars → column width = 8 (no over-expansion)
   - T012-N01: `min_column_width(5)` with `max_column_width(Some(3))` → min wins (floor overrides capped value, result = 5)
   - T012-N02: `min_column_width(10)` with content 15 chars → content width wins (15 > 10), column = 15
   - T012-N03: `col_widths_override = [2, 2]` with `min_column_width(10)` → override ignores min (columns stay 2)
   - T012-N04: empty rows, `min_column_width(5)` → column widths = [5, 5, ...] (min applied to zero-content cols)
   - T012-N05: `min_column_width(usize::MAX)` → no panic; columns set to usize::MAX (saturating allowed)
   Run `w3 .test l::1`; confirm NEW tests fail (red).

2. **Add accessor** — in `src/config.rs`, after `col_widths_override()` accessor (L556–560), add:
   ```rust
   /// Minimum column width floor (accessor; distinct from `min_column_width` setter)
   pub( crate ) fn min_col_width( &self ) -> usize
   {
     self.min_column_width
   }
   ```

3. **Inject floor** — in `src/formatters/table.rs`, after the max cap block at L668, before `widths` on L670:
   ```rust
   // Enforce min_column_width floor (after max cap so min can override max)
   let min = self.config.min_col_width();
   if min > 0
   {
     for width in &mut widths
     {
       *width = (*width).max( min );
     }
   }
   ```

4. **GREEN phase** — run `w3 .test l::3`; confirm all T012 tests pass; confirm no regressions.

5. **Pitfall check** — run T012-N03 specifically with `assert_eq!` on the actual column widths in the output to confirm override path is truly unaffected (not just that it compiles).

#### Validation Procedure

##### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_config_corner_cases 2>&1 | grep -E "FAILED|test result"`
Before: all pass. Expected after RED step: ≥7 failures (T012 tests). Deviation: 0 failures = tests not written yet.

**M2 — Floor accessor exists**
Command: `grep -A3 "min_col_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs | grep "pub( crate )"`
Before: 0 matches. Expected: 1 match. Deviation: 0 = accessor not added.

**M3 — Green state**
Command: `w3 .test l::3`
Before: T012 tests fail. Expected: 0 failures, 0 warnings. Deviation: any failure/warning = rework needed.

##### Anti-faking checks

**AF1 — col_widths_override bypass confirmed**
Grep for `col_widths_override` usage in `calculate_column_widths_for_rows`; confirm early return remains at L632 unchanged; the min floor block must appear AFTER the closing brace of the max cap `if let Some(max_width)` block.

**AF2 — T012-N03 override path test**
T012-N03 test must assert the actual output string contains column widths of 2 (not 10), proving override bypasses min.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Does `min_col_width()` accessor exist in `src/config.rs` `pub(crate)` accessor block?
- [ ] Is the floor injection AFTER the max cap block in `calculate_column_widths_for_rows()`?
- [ ] Does the floor injection have a `if min > 0` guard?
- [ ] Is the `col_widths_override` early return (L632) unchanged?
- [ ] Do all T012-P01–P04 positive tests pass?
- [ ] Do all T012-N01–N05 negative/edge tests pass?
- [ ] Does T012-N03 specifically confirm `col_widths_override` ignores `min_column_width`?
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 3: Task 015 — Unicode Display Width Fix

**Estimated Time:** 45–60 min
**Dependencies:** Phase 1 (task file 015 written)
**Phase Context:** 4th of 7 — adds unicode-aware width functions to `ansi_str.rs`; fixes column calculation and cell padding in `table.rs`

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** `unicode_visual_len(s: &str) -> usize`, `pad_unicode_width(s: &str, width: usize, align_right: bool) -> String` in `ansi_str.rs`
- **Files:** `src/ansi_str.rs` (+2 functions ~40 LOC), `src/formatters/table.rs` (replace 4 call sites), `tests/unicode_display_width_alignment.rs` (+T015 tests ~50 LOC)
- **Tests:** T015-P01–P05 (positive), T015-N03–N07 (negative) from test matrix

**Consumed By:** Phase 4 (border widths calculated correctly), Phase 5 (colored rows have correct width), Phase 6 (verification)

---

#### Overview

Task 015 fixes a systematic misalignment: column widths are measured with `visual_len()` (char count, re-exported from `strs_tools`) and cells are padded with `pad_to_width()` (also char count). For ASCII content this is correct (1 char = 1 display column). For CJK characters (2 display columns) and emoji (variable width), char count diverges from display width, causing misaligned columns.

The fix introduces two new `pub(crate)` functions in `ansi_str.rs` that reuse the already-imported `UnicodeWidthChar` for measurement. Both `format_single_line_row` (L369) and `format_multiline_row` (L478) use `pad_to_width` — both must be updated.

**ANSI stripping algorithm for `unicode_visual_len`:**
The function must strip ANSI escape sequences before measuring display width. ANSI escapes take the form `\x1b[...m` (CSI sequences). The safe approach:
- Track an `in_esc: bool` flag
- On `\x1b`: set `in_esc = true`, skip the char
- While `in_esc`: skip chars; on any ASCII letter (a-z, A-Z), clear `in_esc = true`
- Otherwise: add `ch.width().unwrap_or(1)` to accumulated width

This mirrors the approach already used in `truncate_single_line` (L156+).

#### Project Context

- **Location:** `src/ansi_str.rs` (add two new `pub(crate)` functions after existing re-exports); `src/formatters/table.rs` (replace 4 call sites); `tests/unicode_display_width_alignment.rs` (existing test file for unicode alignment, add T015 test cases)
- **Why This Design:** `unicode-width` dep is already a direct dependency (Cargo.toml L18) — no new dependency needed. New functions are `pub(crate)` (not pub) because they're internal implementation details. The existing `visual_len` and `pad_to_width` re-exports from strs_tools remain for any callers using char-count intentionally.
- **Critical Constraints:** (1) `pad_to_width` is used at BOTH L369 (single-line) and L478 (multiline) — both must be replaced. (2) `visual_len` is used at L643 and L654 for width calculation — both must be replaced. (3) `truncate_cell` at L359 is called BEFORE `pad_unicode_width` — truncation uses `truncate_single_line` which already uses `ch.width()` correctly, so truncation is already unicode-aware. (4) The import at L99 must be updated to include the new functions.
- **Used By:** Phase 4 (border widths already correct), Phase 5 (color wrapping unaffected by width logic)

#### Relationships

- **Feeds Into:** Phase 6 (verification); Phase 4 uses the corrected width calculation (borders align correctly with wide-char content)
- **Depends On:** Phase 1 (task file 015 with algorithm contract)
- **Blocks:** Nothing structural (Phases 4 and 5 don't depend on Phase 3 being done, but unicode correctness is best established early)

#### Goals

1. `unicode_visual_len(s: &str) -> usize` exists in `src/ansi_str.rs`, strips ANSI codes, measures display width using `UnicodeWidthChar::width()`
2. `pad_unicode_width(s: &str, width: usize, align_right: bool) -> String` exists in `src/ansi_str.rs`, uses `unicode_visual_len` for padding calculation
3. All four call sites in `table.rs` updated: L369 `pad_to_width` → `pad_unicode_width`, L478 `pad_to_width` → `pad_unicode_width`, L643 `visual_len` → `unicode_visual_len`, L654 `visual_len` → `unicode_visual_len`
4. All T015 test cases pass; `w3 .test l::3` exits 0

#### Steps

**Algorithm: `unicode_visual_len`**
```rust
pub( crate ) fn unicode_visual_len( s : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;
  let mut len = 0usize;
  let mut in_esc = false;
  for ch in s.chars()
  {
    if ch == '\x1b' { in_esc = true; continue; }
    if in_esc
    {
      if ch.is_ascii_alphabetic() { in_esc = false; }
      continue;
    }
    len += ch.width().unwrap_or( 1 );
  }
  len
}
```

**Algorithm: `pad_unicode_width`**
```rust
pub( crate ) fn pad_unicode_width( s : &str, width : usize, align_right : bool ) -> String
{
  let content_width = unicode_visual_len( s );
  if content_width >= width { return s.to_owned(); }
  let pad = " ".repeat( width - content_width );
  if align_right { format!( "{pad}{s}" ) } else { format!( "{s}{pad}" ) }
}
```

1. **RED phase** — in `tests/unicode_display_width_alignment.rs`, add T015 tests:
   - T015-P01: table with CJK chars in cells → columns align in rendered output (each CJK = 2 display cols)
   - T015-P02: table with emoji (width=2) → columns align
   - T015-P03: `unicode_visual_len("こんにちは")` = 10 (5 chars × 2)
   - T015-P04: `unicode_visual_len("\x1b[31mHello\x1b[0m")` = 5 (ANSI stripped)
   - T015-P05: `pad_unicode_width("A", 5, false)` = `"A    "` (4 spaces)
   - T015-N03: ASCII-only table unaffected (regression: same output before and after)
   - T015-N04: empty string → `unicode_visual_len("") = 0`; `pad_unicode_width("", 3, false) = "   "`
   - T015-N05: content wider than requested width → `pad_unicode_width` returns content unchanged
   - T015-N06: `unicode_visual_len` with malformed partial ANSI → no panic; best-effort
   - T015-N07: `min_column_width` + CJK content → floor applied to unicode-measured widths
   Run `w3 .test l::1`; confirm new tests fail (red).

2. **Add functions to `src/ansi_str.rs`** — after the re-export lines (L79–80), add the two `pub(crate)` functions using the algorithms above. Place them in their own `impl`-free block (they're free functions).

3. **Update `src/formatters/table.rs` imports** — change L99:
   ```rust
   // Before:
   use crate::ansi_str::{ visual_len, pad_to_width };
   // After:
   use crate::ansi_str::{ unicode_visual_len, pad_unicode_width };
   ```
   (Remove `visual_len` and `pad_to_width` from import if no longer used elsewhere; confirm with grep.)

4. **Replace call sites** — in `table.rs`:
   - L369: `pad_to_width( &cell_content, width, align_right )` → `pad_unicode_width( &cell_content, width, align_right )`
   - L478: same replacement for multiline row
   - L643: `visual_len( header )` → `unicode_visual_len( header )`
   - L654: `visual_len( cell )` → `unicode_visual_len( cell )`

5. **GREEN phase** — run `w3 .test l::3`; confirm all T015 tests pass; confirm no regressions in existing tests.

6. **⚠️ Pitfall: ANSI stripping in unicode_visual_len** — the CSI escape sequence terminator is any byte in the range 0x40–0x7E (printable ASCII up to `~`). The simple `is_ascii_alphabetic()` check handles the common `\x1b[...m` pattern but may not handle rare sequences like `\x1b[...H` (cursor position). For the current use case (ANSI color codes only), `is_ascii_alphabetic()` is sufficient. Document this constraint in the function's doc comment.

#### Validation Procedure

##### Measurements

**M1 — Red state confirmed**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test unicode_display_width_alignment 2>&1 | grep -E "FAILED|test result"`
Before: all pass. Expected after RED: ≥7 failures. Deviation: 0 failures = tests not written.

**M2 — New functions exist**
Command: `grep -c "pub( crate ) fn unicode_visual_len\|pub( crate ) fn pad_unicode_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/ansi_str.rs`
Before: 0. Expected: 2. Deviation: <2 = functions missing.

**M3 — Old call sites replaced**
Command: `grep -c "pad_to_width\|visual_len" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Before: 5 (1 import + 2 pad_to_width + 2 visual_len). Expected: 0. Deviation: >0 = replacement incomplete.

**M4 — Green state**
Command: `w3 .test l::3`
Before: T015 tests fail. Expected: 0 failures, 0 warnings. Deviation: any = rework needed.

##### Anti-faking checks

**AF1 — pad_to_width in multiline confirmed replaced**
Command: `grep -n "pad_to_width" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Expected: 0 matches. Common mistake is to replace only the single-line call (L369) and miss the multiline call (L478).

**AF2 — ANSI stripping verified**
T015-P04 must assert that `unicode_visual_len("\x1b[31mHello\x1b[0m") == 5`. If this returns 15+ (counts escape chars), the stripping is broken.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Does `unicode_visual_len` exist in `src/ansi_str.rs` as `pub(crate)`?
- [ ] Does `pad_unicode_width` exist in `src/ansi_str.rs` as `pub(crate)`?
- [ ] Does `unicode_visual_len` strip ANSI codes before measuring display width?
- [ ] Is `pad_to_width` replaced at BOTH L369 (single-line) and L478 (multiline)?
- [ ] Is `visual_len` replaced at BOTH L643 and L654 in width calculation?
- [ ] Does T015-P04 (ANSI stripping) pass, confirming ANSI codes are excluded from width?
- [ ] Do all T015-P01–P05 positive tests pass?
- [ ] Do all T015-N03–N07 negative/edge tests pass?
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 4: Task 014 — Border Variant Rendering

**Estimated Time:** 60–90 min
**Dependencies:** Phase 1 (task file 014 written), Phase 3 (unicode widths correct — borders align properly with wide-char content)
**Phase Context:** 5th of 7 — adds top/bottom/inter-row borders to `format_internal()`; fixes AsciiGrid header separator

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** `bdr_variant()` accessor in `config.rs`; `format_ascii_horizontal_rule()`, `format_unicode_horizontal_rule()`, `format_top_border_if_needed()`, `format_bottom_border_if_needed()`, `format_inter_row_sep_if_needed()` in `table.rs`
- **Files:** `src/config.rs` (+1 accessor ~6 LOC), `src/formatters/table.rs` (+5 helpers + format_internal changes ~100 LOC), `tests/table_rendering_borders.rs` (new file, ~200 LOC)
- **Tests:** T014-P01–P05 (positive), T014-N01–N05 (negative) from test matrix

**Consumed By:** Phase 5 (coloring interacts with border-modified `format_internal`), Phase 6 (verification)

---

#### Overview

Task 014 is the largest structural change in this plan. It modifies `format_internal()` to wrap the existing row pipeline with top/bottom border rendering and injects inter-row separators inside the data loop. It also fixes the longstanding AsciiGrid header separator inconsistency (corner chars `|` → `+`).

The key insight: `format_single_line_row()` already adds leading/trailing pipe characters for `AsciiGrid` and `Unicode` variants (via `needs_border_pipes` check on `header_sep_variant`). So row content is already correctly bordered. Task 014 adds the missing HORIZONTAL rules: the top border before the header, the bottom border after the last data row, and optional inter-row separators between data rows.

**AsciiGrid horizontal rule chars:**
- Top/bottom/inter-row: `+---+---+` → left='+', fill='-', mid='+', right='+'
- Header separator (fix): change existing `'|'` corners to `'+'`

**Unicode horizontal rule chars:**
- Top: `┌─┬─┐` → left='┌', fill='─', mid='┬', right='┐'
- Bottom: `└─┴─┘` → left='└', fill='─', mid='┴', right='┘'
- Header separator (existing `├─┼─┤`): left='├', fill='─', mid='┼', right='┤'
- Inter-row separator: same as header separator `├─┼─┤`

#### Project Context

- **Location:** `src/config.rs` (new `bdr_variant()` accessor in existing accessor `impl` block); `src/formatters/table.rs` (new helper functions + `format_internal` modifications + AsciiGrid sep fix); `tests/table_rendering_borders.rs` (new test file, registered in `tests/readme.md` in Phase 1)
- **Why This Design:** Parameterized `format_ascii_horizontal_rule(output, widths, left, fill, mid, right)` and `format_unicode_horizontal_rule(...)` avoid duplicating the column-iteration loop for each of top/bottom/inter-row; 3 call sites per border type reuse the same 1 function. The mid character for inner columns and the right character for the last column are distinct (e.g., '+' and '+' for AsciiGrid, '┬' and '┐' for Unicode top).
- **Critical Constraints:** (1) The AsciiGrid header separator fix changes `'|'` to `'+'` in `format_header_separator()` L566–590 only — `format_single_line_row()` uses `'|'` for row pipes which is CORRECT (row data uses pipe, not corner). (2) For Unicode, the `format_header_separator()` already produces `├─┼─┤` correctly using `width + 2` — do NOT change this; the `width + 2` accounts for 1 space of inner_padding on each side matching `unicode_box()` preset. (3) Inter-row separators are only rendered for `BorderVariant::AsciiGrid` and `BorderVariant::Unicode` — not for `Ascii`, `Markdown`, or `None`. (4) `bdr_variant()` accessor name must be distinct from the `border_variant()` setter.
- **Used By:** Phase 5 (the modified `format_internal()` is the function Phase 5 adds coloring to)

#### Relationships

- **Feeds Into:** Phase 5 (coloring wraps the rows already bordered by Phase 4), Phase 6
- **Depends On:** Phase 1 (task file 014), Phase 3 (unicode widths correct for wide-char border alignment)
- **Blocks:** Phase 5 (cannot start until format_internal has stable border structure)

#### Goals

1. `bdr_variant()` accessor exists in `src/config.rs` and returns `BorderVariant`
2. `format_ascii_horizontal_rule()` and `format_unicode_horizontal_rule()` helpers exist in `table.rs`, each parameterized with corner/fill/mid/right chars
3. `format_internal()` emits top border (if `border_variant` is AsciiGrid or Unicode), then header row, then header separator, then data rows (with inter-row separators for AsciiGrid/Unicode), then bottom border
4. `format_header_separator()` AsciiGrid branch uses `'+'` for all corner/separator chars (produces `+---+---+`)
5. All T014 test cases pass; `w3 .test l::3` exits 0

#### Steps

**Algorithm: format_ascii_horizontal_rule**
```rust
fn format_ascii_horizontal_rule(
  &self, output : &mut String, widths : &[ usize ],
  left : char, fill : char, mid : char, right : char
)
{
  output.push( left );
  for ( idx, &width ) in widths.iter().enumerate()
  {
    if idx == 0 && self.config.has_outer_padding()
    {
      output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
    }
    output.push_str( &fill.to_string().repeat( width ) );
    if idx == widths.len() - 1 && self.config.has_outer_padding()
    {
      output.push_str( &fill.to_string().repeat( self.config.cell_inner_padding() ) );
    }
    output.push( if idx < widths.len() - 1 { mid } else { right } );
  }
  output.push( '\n' );
}
```
For Unicode, use the same structure with `'┌'`/`'─'`/`'┬'`/`'┐'` (top), `'└'`/`'─'`/`'┴'`/`'┘'` (bottom). Note: Unicode `─` is multi-byte; `repeat` on a `char` via `to_string()` is correct.

1. **RED phase** — create `tests/table_rendering_borders.rs` with failing tests:
   - T014-P01: `TableConfig::grid()` → output contains `+---+` top border, bottom border, header separator
   - T014-P02: `TableConfig::unicode_box()` → output contains `┌─` top, `└─` bottom, `├─` header sep
   - T014-P03: `TableConfig::bordered()` → no top/bottom borders (BorderVariant::Ascii, not Grid/Unicode)
   - T014-P04: `TableConfig::plain()` → no borders at all
   - T014-P05: AsciiGrid with 3 rows → 2 inter-row separators between data rows
   - T014-N01: `TableConfig::grid()` header separator is `+---+` not `|---|`
   - T014-N02: Unicode top border starts with `┌`, not `├`
   - T014-N03: Unicode bottom border starts with `└`, not `├`
   - T014-N04: `TableConfig::markdown()` → no top/bottom borders (Markdown variant)
   - T014-N05: empty rows list → top border + header row + header sep + bottom border only (no inter-row)
   Run `w3 .test l::1`; confirm new tests fail (red).

2. **Add `bdr_variant()` accessor** — in `src/config.rs` accessor `impl` block:
   ```rust
   /// Border variant (accessor; distinct from `border_variant` setter)
   pub( crate ) fn bdr_variant( &self ) -> crate::config::BorderVariant
   {
     self.border_variant
   }
   ```

3. **Fix AsciiGrid header separator** — in `src/formatters/table.rs`, `format_header_separator()` AsciiGrid branch (L566–590): change the three occurrences of `output.push( '|' )` to `output.push( '+' )`.

4. **Add horizontal rule helpers** — add `format_ascii_horizontal_rule` and `format_unicode_horizontal_rule` as private methods in the `impl TableFormatter` block.

5. **Add top/bottom/inter-row wrappers** — add three small helper methods:
   - `format_top_border_if_needed(&self, output, widths)` — dispatches on `bdr_variant()`
   - `format_bottom_border_if_needed(&self, output, widths)` — dispatches on `bdr_variant()`
   - `format_inter_row_sep_if_needed(&self, output, widths)` — dispatches on `bdr_variant()`

6. **Update `format_internal()`** — modify the pipeline at L202–222:
   ```rust
   fn format_internal( &self, headers : &[ String ], rows : &[ Vec< String > ] ) -> String
   {
     let mut output = String::with_capacity( INITIAL_CAPACITY );
     let column_widths = self.calculate_column_widths_for_rows( headers, rows );

     self.format_top_border_if_needed( &mut output, &column_widths );     // NEW
     self.format_row( &mut output, headers, &column_widths, true );
     self.format_header_separator( &mut output, &column_widths );
     for ( idx, row ) in rows.iter().enumerate()
     {
       if idx > 0 { self.format_inter_row_sep_if_needed( &mut output, &column_widths ); } // NEW
       self.format_row( &mut output, row, &column_widths, false );
     }
     self.format_bottom_border_if_needed( &mut output, &column_widths );  // NEW

     output
   }
   ```

7. **GREEN phase** — run `w3 .test l::3`; confirm all T014 tests pass; confirm T014-N01 specifically (AsciiGrid sep = `+---+`); confirm no regression in existing `table_styles_outputs.rs` and `table_styles_presets.rs` tests.

8. **⚠️ Pitfall: format_row pipe chars vs border corners** — `format_single_line_row` adds `'|'` for AsciiGrid row pipes; this is correct for DATA rows (pipe walls). The header separator uses `'+'` corners (fix in step 3). Top/bottom borders also use `'+'` corners. Don't conflate: data row walls = `|`, horizontal rules = `+---+`.

9. **⚠️ Pitfall: Unicode width + 2 in format_header_separator** — the existing Unicode branch uses `width + 2` because `unicode_box()` has `inner_padding = 1` (1 space on each side). After Phase 3's width calculation change, `width` is now measured in display columns. The `+ 2` is STILL correct as it accounts for padding spaces (which are 1 display col each). Do NOT change this `+ 2`.

#### Validation Procedure

##### Measurements

**M1 — Red state (new tests fail)**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_rendering_borders 2>&1 | grep -E "FAILED|test result"`
Before: test file doesn't exist. Expected after writing tests: compile error or ≥8 test failures.

**M2 — AsciiGrid separator fix confirmed**
Command: `grep -A5 "AsciiGrid =>" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs | grep "push.*'|'"`
Before: 3 matches. Expected: 0 matches (all changed to '+'). Deviation: >0 = fix incomplete.

**M3 — format_internal contains border calls**
Command: `grep -c "format_top_border_if_needed\|format_bottom_border_if_needed\|format_inter_row_sep" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Before: 0. Expected: ≥5 (2 defs + 3 call sites in format_internal + wrapper bodies). Deviation: 0 = not implemented.

**M4 — Green state**
Command: `w3 .test l::3`
Before: T014 tests fail. Expected: 0 failures, 0 warnings. Deviation: any = rework.

##### Anti-faking checks

**AF1 — format_row pipe unchanged**
Confirm `format_single_line_row` still uses `'|'` for row border pipes (not `'+'`). Change to `'+'` would break data row/border consistency.
Command: `grep -n "'|'" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs | grep -v "format_header_sep"`
Expected: ≥2 matches (the leading/trailing pipe in format_single_line_row).

**AF2 — T014-N01 output verified**
T014-N01 test must `assert!(output.contains("+---+"))` AND `assert!(!output.contains("|---|"))` for a grid() table.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Does `bdr_variant()` accessor exist in `src/config.rs`?
- [ ] Does `format_header_separator()` AsciiGrid branch use `'+'` (not `'|'`) for all separator/corner chars?
- [ ] Do `format_ascii_horizontal_rule` and `format_unicode_horizontal_rule` helpers exist?
- [ ] Does `format_internal()` call top border before header row?
- [ ] Does `format_internal()` call bottom border after the last data row?
- [ ] Does `format_internal()` call inter-row separator between data rows (idx > 0)?
- [ ] Does T014-P01 (grid() full round-trip) produce `+---+` top, header sep, and bottom?
- [ ] Does T014-P02 (unicode_box() full round-trip) produce `┌─┬─┐` top and `└─┴─┘` bottom?
- [ ] Does T014-N01 confirm AsciiGrid header separator is `+---+` (not `|---|`)?
- [ ] Does existing test suite have zero regressions?
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 5: Task 013 — ANSI Header and Row Coloring

**Estimated Time:** 45–60 min
**Dependencies:** Phase 1 (task file 013 written), Phase 4 (stable `format_internal()` with borders)
**Phase Context:** 6th of 7 — adds color wrapping around header/data row output in `format_internal()`

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** 5 new `pub(crate)` accessors in `config.rs`; coloring logic integrated into `format_internal()`
- **Files:** `src/config.rs` (+5 accessors ~30 LOC), `src/formatters/table.rs` (format_internal color wrapping ~30 LOC), `tests/table_rendering_colors.rs` (new test file, ~150 LOC)
- **Tests:** T013-P01–P06 (positive), T013-N01–N06 (negative) from test matrix

**Consumed By:** Phase 6 (final verification)

---

#### Overview

Task 013 wires up the `colorize_header`, `header_color`, `alternating_rows`, `row_color1`, and `row_color2` fields that were added in the v0.3.0 design but never connected to rendering. The `_is_header: bool` parameter in `format_row()` was a planned hook for this task.

The temp-buffer strategy avoids restructuring `format_row()`: call `format_row()` into a side buffer, then wrap its output in ANSI codes before pushing to the main output. The RESET code (`"\x1b[0m"`) must appear BEFORE the trailing newline to prevent terminal background color from extending to the end of the line.

**RESET constant:** Define a private constant `const RESET : &str = "\x1b[0m";` in `table.rs` to avoid magic strings.

**Theme integration:** `ColorTheme::apply_to_table()` (in `themes.rs`) sets all 5 color fields. The `dark()` theme sets `row_color1 = "\x1b[0m"` (reset), `row_color2 = "\x1b[48;5;235m"` (dark gray bg). Row index 0 gets `row_color1`, row index 1 gets `row_color2`, etc. With `row_color1 = RESET`, even-indexed rows appear normal; odd-indexed rows appear dark gray.

#### Project Context

- **Location:** `src/config.rs` (5 new accessors in existing accessor `impl` block); `src/formatters/table.rs` (coloring logic in `format_internal`); `tests/table_rendering_colors.rs` (new test file, registered in `tests/readme.md` by Phase 1)
- **Why This Design:** Temp-buffer approach keeps `format_row()` pure (no coloring awareness) while enabling colored output without refactoring the row formatting logic. Alternative (passing color to format_row) would require threading color state through multiline row handling too.
- **Critical Constraints:** (1) 5 accessor names: `colorize_header()` → `bool`, `header_color_str()` → `&str`, `alternating_rows_enabled()` → `bool`, `row_color1_str()` → `&str`, `row_color2_str()` → `&str`. (2) RESET code must be placed BEFORE the newline, not after. (3) When `alternating_rows_enabled()` is true but a row color is an empty string, treat it as no-op (don't output empty ANSI prefix). (4) Color wrapping must be applied AFTER border pipes are already in the row output — the temp buffer captures the complete row including pipes.
- **Used By:** Phase 6 (final verification)

#### Relationships

- **Feeds Into:** Phase 6 (contributes to final test suite pass)
- **Depends On:** Phase 1 (task file 013), Phase 4 (`format_internal()` stable with borders)
- **Blocks:** Nothing (last feature phase)

#### Goals

1. 5 new `pub(crate)` accessors in `src/config.rs`: `colorize_header()`, `header_color_str()`, `alternating_rows_enabled()`, `row_color1_str()`, `row_color2_str()`
2. `format_internal()` applies header color (via temp-buffer) when `colorize_header()` is true and `header_color_str()` is non-empty
3. `format_internal()` applies alternating row colors (via temp-buffer) when `alternating_rows_enabled()` is true
4. `ColorTheme::dark()` via `apply_to_table()` produces colored output verifiable by ANSI code presence in output string
5. All T013 test cases pass; `w3 .test l::3` exits 0

#### Steps

**Algorithm: temp-buffer coloring in format_internal**
```rust
// Helper: wrap row string in color codes (RESET before newline)
fn apply_color( color : &str, row : &str ) -> String
{
  // row ends with '\n'; strip it, wrap content, re-add '\n'
  let content = row.trim_end_matches( '\n' );
  format!( "{color}{content}{RESET}\n" )
}
```

For `format_internal()`, replace the direct `format_row(output, ...)` calls:
```rust
// Header row with optional colorizing
if self.config.colorize_header() && !self.config.header_color_str().is_empty()
{
  let mut buf = String::new();
  self.format_row( &mut buf, headers, &column_widths, true );
  output.push_str( &apply_color( self.config.header_color_str(), &buf ) );
}
else
{
  self.format_row( &mut output, headers, &column_widths, true );
}
```

For data rows with alternating colors:
```rust
for ( idx, row ) in rows.iter().enumerate()
{
  if idx > 0 { self.format_inter_row_sep_if_needed( &mut output, &column_widths ); }
  if self.config.alternating_rows_enabled()
  {
    let color = if idx % 2 == 0
    { self.config.row_color1_str() }
    else
    { self.config.row_color2_str() };
    if !color.is_empty()
    {
      let mut buf = String::new();
      self.format_row( &mut buf, row, &column_widths, false );
      output.push_str( &apply_color( color, &buf ) );
      continue;
    }
  }
  self.format_row( &mut output, row, &column_widths, false );
}
```

1. **RED phase** — create `tests/table_rendering_colors.rs` with failing tests:
   - T013-P01: header color applied → output contains `\x1b[` before header row content
   - T013-P02: alternating rows (dark theme) → output contains `\x1b[48;5;235m` before odd rows
   - T013-P03: `ColorTheme::dark()` applied via `apply_to_table()` → both header and row colors present
   - T013-P04: RESET code (`\x1b[0m`) present before each newline in colored rows
   - T013-P05: `ColorTheme::none()` → output contains NO ANSI codes
   - T013-P06: even rows get row_color1 (reset/no-op for dark theme), odd rows get row_color2
   - T013-N01: `colorize_header = false` with non-empty `header_color` → no header ANSI codes
   - T013-N02: `alternating_rows = false` → no row ANSI codes even if colors set
   - T013-N03: empty `header_color` with `colorize_header = true` → no header ANSI codes (empty string guard)
   - T013-N04: single data row → no inter-row separator (idx > 0 condition)
   - T013-N05: coloring + borders (grid + dark theme) → both ANSI codes and `+---+` present in output
   - T013-N06: coloring + multiline cells → ANSI codes applied per physical row, not per logical cell
   Run `w3 .test l::1`; confirm new tests fail (red).

2. **Add 5 accessors** — in `src/config.rs` accessor `impl` block:
   ```rust
   pub( crate ) fn colorize_header( &self ) -> bool { self.colorize_header }
   pub( crate ) fn header_color_str( &self ) -> &str { &self.header_color }
   pub( crate ) fn alternating_rows_enabled( &self ) -> bool { self.alternating_rows }
   pub( crate ) fn row_color1_str( &self ) -> &str { &self.row_color1 }
   pub( crate ) fn row_color2_str( &self ) -> &str { &self.row_color2 }
   ```

3. **Add RESET constant** — at top of `table.rs` impl block or as module-level const:
   ```rust
   const RESET : &str = "\x1b[0m";
   ```

4. **Add `apply_color` helper** — private method or private free function in `table.rs`.

5. **Update `format_internal()`** — integrate coloring logic using the algorithm above.

6. **GREEN phase** — run `w3 .test l::3`; confirm all T013 tests pass; confirm T013-N05 specifically (coloring + borders work together); confirm no regression in existing `themes.rs` test file.

7. **⚠️ Pitfall: RESET before vs after newline** — placing RESET after `\n` (`format!("{color}{content}\n{RESET}")`) causes the terminal background color to extend to end-of-line on some terminals. Always use `format!("{color}{content}{RESET}\n")` — RESET before the newline terminates the color region cleanly.

8. **⚠️ Pitfall: _is_header in format_row** — the `_is_header: bool` parameter is a pre-planned hook but is NOT used in this task. Task 013 uses the temp-buffer approach which wraps the ENTIRE row output, not individual cells. The `_is_header` parameter can remain unused (Rust will warn; use `_is_header` naming to suppress).

#### Validation Procedure

##### Measurements

**M1 — Red state (new tests fail)**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_rendering_colors 2>&1 | grep -E "FAILED|test result"`
Before: test file doesn't exist. Expected after writing tests: compile error or ≥10 failures.

**M2 — Accessors added**
Command: `grep -c "pub( crate ) fn colorize_header\|pub( crate ) fn header_color_str\|pub( crate ) fn alternating_rows_enabled\|pub( crate ) fn row_color1_str\|pub( crate ) fn row_color2_str" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs`
Before: 0. Expected: 5. Deviation: <5 = accessors missing.

**M3 — RESET appears in format_internal**
Command: `grep -c "RESET" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Before: 0. Expected: ≥3 (constant def + ≥2 uses). Deviation: <3 = coloring logic incomplete.

**M4 — Green state**
Command: `w3 .test l::3`
Before: T013 tests fail. Expected: 0 failures, 0 warnings. Deviation: any = rework.

##### Anti-faking checks

**AF1 — RESET before newline**
Command: `grep "RESET\\\\n" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs`
Expected: 0 matches. A match would mean RESET is placed AFTER the newline (wrong order).

**AF2 — T013-P04 validates RESET placement**
T013-P04 must check `output.contains("\x1b[0m\n")` (RESET immediately before newline), NOT `output.contains("\n\x1b[0m")`.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Do all 5 color-related accessors exist in `src/config.rs`?
- [ ] Is the RESET constant defined and used (not hardcoded magic string)?
- [ ] Does `format_internal()` use temp-buffer for header coloring when `colorize_header()` is true?
- [ ] Does `format_internal()` use temp-buffer for data row coloring when `alternating_rows_enabled()` is true?
- [ ] Is there an empty-string guard on both `header_color_str()` and row color strings?
- [ ] Does the RESET code appear BEFORE the `\n` in all colored row output?
- [ ] Do all T013-P01–P06 positive tests pass?
- [ ] Do all T013-N01–N06 negative/edge tests pass?
- [ ] Does T013-N05 (coloring + borders) pass, confirming no mutual interference?
- [ ] Does `w3 .test l::3` exit 0 with zero warnings?
- [ ] Are all Validation Procedure measurements met?

---

<!-- plan:phase -->
### Phase 6: Final Integration Verification

**Estimated Time:** 15–20 min
**Dependencies:** Phases 2, 3, 4, 5 (all four tasks implemented)
**Phase Context:** 7th of 7 — validation-only phase; confirms all tasks complete and no cross-task regressions

**Outputs (Artifacts Produced):**
- **Types:** None
- **Functions:** None
- **Files:** None (verification only)
- **Tests:** All 77 test cases from the pre-built test matrix passing

**Consumed By:** None (terminal phase)

---

#### Overview

Phase 6 is the integration gate. It runs the complete test suite, verifies that all four tasks' test cases pass, confirms no regressions in existing tests, and checks that all documentation artifacts from Phase 1 are present and correct.

This phase also validates cross-task interactions: Task 015's unicode widths must not break Task 014's border alignment; Task 013's coloring must not interfere with Task 014's border chars.

#### Project Context

- **Location:** No changes; read-only verification phase
- **Why This Design:** After 4 separate TDD phases, a final integration pass catches any interaction bugs that per-task verification missed
- **Critical Constraints:** `w3 .test l::3` must exit 0 with zero warnings; this is the final gate; any failure must be traced back to the responsible task phase and fixed there
- **Used By:** Nothing (terminal)

#### Relationships

- **Feeds Into:** Nothing (terminal)
- **Depends On:** Phases 2, 3, 4, 5 (all four tasks implemented and green individually)
- **Blocks:** Nothing

#### Goals

1. `w3 .test l::3` exits 0 with zero failures and zero clippy warnings
2. All 77 test cases from the pre-built test matrix are exercised and passing
3. All Phase 1 documentation artifacts are complete and accurate
4. No cross-task regressions: unicode widths, borders, and coloring all work correctly in combination

#### Steps

1. **Run full test suite:**
   ```bash
   w3 .test l::3
   ```
   Expected: 0 failures, 0 warnings.

2. **Verify cross-task interaction: borders + unicode widths** — run a manual spot-check: construct a table with CJK chars and `TableConfig::grid()`; confirm borders are straight and column widths account for display width.

3. **Verify cross-task interaction: borders + coloring** — construct a table with `TableConfig::grid()` and `ColorTheme::dark()`; confirm `+---+` borders appear AND ANSI codes appear in row output.

4. **Verify all 77 test cases accounted for** — confirm that `table_rendering_borders.rs` and `table_rendering_colors.rs` are in the test output (not skipped/ignored); confirm `table_config_corner_cases.rs` has T012 tests; confirm `unicode_display_width_alignment.rs` has T015 tests.

5. **Verify Phase 1 documentation completeness:**
   - spec.md has min_column_width behavioral note
   - docs/development_notes.md has AsciiGrid pitfall entry and no stale BLOCKING reference
   - tests/readme.md has rows for both new test files
   - task/completed/ has all 4 task files
   - task/readme.md shows Total: 15, Completed: 15

#### Validation Procedure

##### Measurements

**M1 — Full test suite green**
Command: `w3 .test l::3`
Before: individual task phases each pass. Expected: 0 failures total, 0 warnings. Deviation: any failure = cross-task regression; trace to responsible phase.

**M2 — New test files running**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --test table_rendering_borders --test table_rendering_colors 2>&1 | grep "test result"`
Before: test files don't exist. Expected: both test results show "ok" with ≥8 tests each. Deviation: file not found or 0 tests = test files not created.

**M3 — Total test count increase**
Command: `RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | grep "test result.*passed" | tail -1`
Before: ~352 nextest tests. Expected: ≥352 + 77 new tests = ≥429 tests. Deviation: count unchanged = new tests not running.

**M4 — Documentation completeness**
Command: `ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/task/completed/ | grep -E "^01[2-5]_" | wc -l`
Before: 0. Expected: 4. Deviation: <4 = Phase 1 incomplete.

##### Anti-faking checks

**AF1 — No #[ignore] on new tests**
Command: `grep -r "ignore" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/table_rendering_borders.rs /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/table_rendering_colors.rs 2>/dev/null`
Expected: 0 matches. Ignored tests would silently pass without execution.

**AF2 — No workarounds in implementation**
Command: `grep -rn "TODO\|FIXME\|xxx:\|aaa:\|qqq:" /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/table.rs /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs /home/user1/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/ansi_str.rs 2>/dev/null`
Expected: 0 new markers (compared to baseline before this plan). Any new marker = deferred implementation.

**AF3 — Clippy clean**
Command: `cargo clippy --all-targets --all-features -- -D warnings 2>&1 | grep "^error"`
Expected: 0 errors. Any clippy error = code quality issue.

#### Validation Checklist

Desired answer for every question is YES.
- [ ] Does `w3 .test l::3` exit 0 with zero failures and zero warnings?
- [ ] Do `table_rendering_borders.rs` and `table_rendering_colors.rs` each have ≥8 passing tests?
- [ ] Does T012-N03 (col_widths_override bypasses min) pass?
- [ ] Does T013-N05 (coloring + borders combined) pass?
- [ ] Does T014-N01 (AsciiGrid header sep is `+---+`) pass?
- [ ] Does T015-P04 (ANSI stripping in unicode_visual_len) pass?
- [ ] Is `task/readme.md` updated to Total: 15, Completed: 15?
- [ ] Are all 4 task files present in `task/completed/`?
- [ ] Is `docs/development_notes.md` free of "BLOCKING" stale references?
- [ ] Does `spec.md` contain the min_column_width floor behavioral note?
- [ ] Is `src/readme.md` updated if `ansi_str.rs` description changed?
- [ ] Are all Validation Procedure measurements met?

---

## Risk Assessment & Mitigation

### Risk 1: format_internal() Modification Conflicts (Tasks 013 + 014)

**Probability:** Medium
**Impact:** Medium (both tasks modify the same function body)

**Mitigation:**
- Phase 4 (borders) modifies `format_internal()` first; Phase 5 (coloring) works from the updated function
- Phase 4 ends with `w3 .test l::3` green before Phase 5 begins
- Phase 5's coloring logic wraps `format_row()` calls — it doesn't change the border calls added in Phase 4

**Validation:**
- T013-N05 explicitly tests borders + coloring combined
- Sequential implementation order eliminates merge conflicts

### Risk 2: Unicode Width Regression in Existing Tests (Task 015)

**Probability:** Low
**Impact:** Medium (any ASCII-only content: char-count == display-width, so no regression expected; but embedded ANSI codes in cell content could change behavior)

**Mitigation:**
- T015-N03 is a regression guard: ASCII-only table before and after Phase 3 must produce identical output
- `unicode_visual_len` strips ANSI codes before measuring width — consistent with `visual_len` behavior for ASCII content
- Phase 3 ends with `w3 .test l::3` green; all existing tests must pass

**Validation:**
- Run `w3 .test l::3` after Phase 3 and compare test count/pass rate to baseline

### Risk 3: AsciiGrid Header Separator Fix Breaks Existing Tests (Task 014)

**Probability:** Medium
**Impact:** Low (existing tests may have hardcoded `|---|` in expected output)

**Mitigation:**
- Existing tests in `table_styles_outputs.rs` and `table_styles_presets.rs` may use snapshot assertions containing `|---|`
- If regression occurs: update expected output strings from `|---|` to `+---+` (correct behavior per spec)
- This is a behavior correction, not a regression — updating expected output is the proper fix

**Validation:**
- AF2 in Phase 4 checks `format_row` pipe chars remain `|` (not accidentally changed)
- After Phase 4, search for `|---|` in test expected output strings; update any found

---

## Open Questions & Decisions

### Q1: Should inter-row separators be behind a separate config flag?

**Options:**
- A. Always render inter-row separators when `border_variant` is AsciiGrid or Unicode
- B. Add a separate `inter_row_separators: bool` field to `TableConfig`

**Decision:** Option A (always render when border variant implies it)
**Rationale:** The `grid()` preset name and the spec's visual examples both show inter-row separators for AsciiGrid/Unicode. Adding a separate flag would require a new config field, a new accessor, and more test cases. The current 4-task scope does not include new `TableConfig` fields.
**Future:** If a use case emerges for borderless-but-no-inter-row-sep, add the flag then.

### Q2: Should `unicode_visual_len` handle OSC/DCS/other ANSI sequences?

**Options:**
- A. Handle only CSI sequences (`\x1b[...m` pattern) — sufficient for color codes
- B. Handle full ANSI sequence grammar (CSI, OSC, DCS, private sequences)

**Decision:** Option A (CSI only)
**Rationale:** `tree_fmt` outputs only CSI color codes; inputs from user data should not contain other ANSI sequences in cell content. The `truncate_single_line` precedent (which handles CSI only) supports this choice.
**Future:** If OSC/hyperlink sequences are added, extend `unicode_visual_len` then.

---

## Future Enhancements (Out of Scope)

These are explicitly NOT included in this plan:

1. **`ColorTheme` builder API** — Tasks 013/015 wire up existing `ColorTheme`; new theme creation API (custom colors, TOML-loadable themes) is deferred
2. **`Markdown` inter-row separators** — Markdown format uses `|---|` between header and data; adding inter-row separators there is a separate feature
3. **Conditional inter-row separators** — adding `inter_row_separators: bool` to `TableConfig` (see Q1)
4. **Emoji width normalization** — emoji sequences (combining chars, ZWJ sequences) have platform-dependent display width; `unicode-width` may not handle all cases; normalization is a separate investigation
5. **Full ANSI sequence grammar in `unicode_visual_len`** — OSC hyperlinks, DCS sequences; deferred until needed (see Q2)

---

## Appendix: File Inventory

### New Files Created

| Type | File | Est. LOC | Purpose |
|------|------|----------|---------|
| test | `tests/table_rendering_borders.rs` | ~200 | T014 test cases |
| test | `tests/table_rendering_colors.rs` | ~150 | T013 test cases |
| task | `task/completed/012_enforce_min_column_width.md` | ~120 | Task 012 record |
| task | `task/completed/013_ansi_header_row_coloring.md` | ~150 | Task 013 record |
| task | `task/completed/014_border_variant_rendering.md` | ~160 | Task 014 record |
| task | `task/completed/015_unicode_display_width.md` | ~140 | Task 015 record |
| plan | `plan/readme.md` | ~8 | Plan directory registry |

### Modified Files

| Type | File | Est. Delta | Changes |
|------|------|-----------|---------|
| source | `src/config.rs` | +60 LOC | 7 new pub(crate) accessors (1 for 012, 5 for 013, 1 for 014) |
| source | `src/ansi_str.rs` | +40 LOC | 2 new pub(crate) functions: unicode_visual_len, pad_unicode_width |
| source | `src/formatters/table.rs` | +120 LOC | floor injection (012), 4 call sites replaced (015), 5 border helpers + format_internal changes (014), color wrapping + 5 accessors (013), AsciiGrid sep fix (014) |
| test | `tests/table_config_corner_cases.rs` | +50 LOC | T012 test cases added |
| test | `tests/unicode_display_width_alignment.rs` | +50 LOC | T015 test cases added |
| spec | `spec.md` | +5 LOC | min_column_width behavioral note |
| doc | `docs/development_notes.md` | +15 LOC | AsciiGrid pitfall entry, stale reference fix |
| doc | `tests/readme.md` | +2 LOC | 2 new test file rows |
| task | `task/readme.md` | +6 LOC | 4 new task rows + updated statistics |

**Total Estimated LOC:** ~1,276 lines (production + tests + docs)

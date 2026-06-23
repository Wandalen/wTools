# 012 — Create test surface specs for 6 uncovered entity types

## MOST Goal

Create 43 test spec files covering all doc instances in 6 entity types that currently lack full spec coverage (data_structure, input_model, input_type, pattern, trait, variant), bringing `tests/docs/` coverage from 37/80 (46.25%) to 80/80 (100%).

- **Motivated:** Test surface spec files define what must be tested; without them, behavioral contracts for 43 doc instances are invisible to the test system. Existing 37 specs (algorithm/api/builder/feature/formatter/invariant) proved their value by driving Task 002's 56 test implementations. The 43 gaps leave variant output shapes, design patterns, trait contracts, and input model structures undocumented as test surface.
- **Observable:** Every `tests/docs/<entity>/readme.md` Overview Table has one row per doc instance in `docs/<entity>/`; `find tests/docs -name '*.md' ! -name readme.md ! -name procedure.md | wc -l` returns 80.
- **Scoped:** Only creates `.md` spec files under `tests/docs/` and updates `tests/docs/*/readme.md` Overview Tables. No Rust code, no `src/` changes, no `docs/` changes.
- **Testable:** Readme parity check: for each of 12 entity dirs, `ls tests/docs/<entity>/*.md | grep -v readme | wc -l` equals the Overview Table data row count AND the `docs/<entity>/` non-readme non-procedure `.md` count.

## In Scope

Paths relative to crate root (`module/core/data_fmt/`).

**Spec files to create (43 total):**

| Entity Type | Prefix | Min Cases | Count | Source |
|-------------|--------|----------:|------:|--------|
| data_structure | DS- | 4 | 1 | `docs/data_structure/001_variant_attributes.md` |
| input_model | IM- | 4 | 2 | `docs/input_model/001_tabular.md`, `002_hierarchical.md` |
| input_type | IV- | 4 | 2 | `docs/input_type/001_table_view.md`, `002_tree_node.md` |
| pattern | PT- | 3 | 4 | `docs/pattern/001`..`004_*.md` |
| trait | TR- | 4 | 1 | `docs/trait/002_table_shaped_formatter.md` |
| variant | VT- | 4 | 33 | `docs/variant/001`..`033_*.md` |

**Readme updates (6 entity readmes):**
- Add one Overview Table row per new spec file in the corresponding `tests/docs/<entity>/readme.md`

**Root readme update:**
- `tests/docs/readme.md` — no changes needed (entity type count stays 12; instance counts are in `docs/entities.md`)

## Out of Scope

- Rust test code implementation (separate task after specs exist; cf. Task 002 pattern)
- Existing 37 spec files in algorithm/, api/, builder/, feature/, formatter/, invariant/ (already at 100%)
- Source code (`src/`) or behavioral docs (`docs/`) modifications
- Spec quality issues in existing files (P-07, P-11..P-23 from audit — separate hygiene pass)
- Doc filename rename for caption-to-heading drift (TSK-009 residual — separate doc task)

## Requirements

- All spec files follow `l1_imp_surface.rulebook.md` format strictly
- File naming: `NNN_element_name.md` matching the `docs/` source filename (3-digit prefix consistent with existing project convention)
- Test case format: Given/When/Then with `- **Label:** value` list form per `l1_imp_surface.rulebook.md § Spec : Test Case Format`
- Each spec meets minimum case count for its element type (see table above)
- Each spec's content is derived from reading the authoritative `docs/<entity>/NNN_*.md` source — not from memory
- Readme parity maintained per `l1_imp_surface.rulebook.md § Index : Readme Parity`

## Work Procedure

Execute entity types in this priority order (highest-impact first):

1. **Read rulebooks** — `l1_imp_surface.rulebook.md` for spec format; `doc_des.rulebook.md` for understanding doc instance structure.
2. **Process variant/ (33 specs)** — Read each `docs/variant/NNN_*.md`; create `tests/docs/variant/NNN_*.md` with VT-N cases covering: output matches documented format, separator/border chars correct, header rendering, empty-table behavior. Update `tests/docs/variant/readme.md`.
3. **Process pattern/ (4 specs)** — Read each `docs/pattern/NNN_*.md`; create `tests/docs/pattern/NNN_*.md` with PT-N cases covering: pattern is observable in codebase, design constraint holds. Update `tests/docs/pattern/readme.md`.
4. **Process input_model/ (2 specs)** — Read each `docs/input_model/NNN_*.md`; create `tests/docs/input_model/NNN_*.md` with IM-N cases. Update readme.
5. **Process input_type/ (2 specs)** — Read each `docs/input_type/NNN_*.md`; create `tests/docs/input_type/NNN_*.md` with IV-N cases. Update readme.
6. **Process trait/ (1 spec)** — Read `docs/trait/002_table_shaped_formatter.md`; create `tests/docs/trait/002_table_shaped_formatter.md` with TR-N cases. Update readme.
7. **Process data_structure/ (1 spec)** — Read `docs/data_structure/001_variant_attributes.md`; create spec with DS-N cases. Update readme.
8. **Validate** — Run Coverage Gate checks 101-112 on all 12 entity types. All must PASS.

## Test Matrix

| Entity Type | Spec Example | Input Scenario | Expected Behavior |
|-------------|-------------|----------------|-------------------|
| variant | VT-1 in `001_table_plain.md` | 2x2 table, plain preset | No borders; space-separated columns; header row present |
| variant | VT-4 in `006_table_unicode_box.md` | Empty table (0 rows) | Header-only output with box-drawing chars |
| pattern | PT-1 in `001_three_layer_architecture.md` | Trace data flow path | Data types -> Builders -> Formatters layering observable |
| trait | TR-1 in `002_table_shaped_formatter.md` | Implementor calls trait method | Correct dispatch to underlying formatter |
| input_type | IV-1 in `001_table_view.md` | Construct `TableView` with headers + rows | `.headers()` returns original headers; `.rows()` length matches |
| input_model | IM-1 in `001_tabular.md` | Feed tabular data to formatter | Column alignment preserved across rows |
| data_structure | DS-1 in `001_variant_attributes.md` | Inspect `VariantAttributes` fields | All documented fields present and typed correctly |

## Acceptance Criteria

- 43 new spec files exist under `tests/docs/` (one per uncovered doc instance)
- Each spec meets its element type's minimum case count
- All 6 affected entity readme Overview Tables updated with correct row counts (parity check passes)
- All test cases use Given/When/Then format with `- **Label:** value` list form
- Coverage Gate checks 101-112 all PASS across all 12 entity types
- `find tests/docs -name '*.md' ! -name readme.md ! -name procedure.md | wc -l` returns 80

## Validation

### Checklist

Desired answer for every question is YES.

**Completeness**
- [ ] Do all 6 entity types (data_structure, input_model, input_type, pattern, trait, variant) have spec files matching their docs/ instance count?
- [ ] Is the total spec file count 80 (37 existing + 43 new)?

**Format compliance**
- [ ] Does every new spec use the correct case prefix for its entity type (DS-, IM-, IV-, PT-, TR-, VT-)?
- [ ] Does every new spec meet the minimum case count for its element type?
- [ ] Does every test case use `- **Given:**`/`- **When:**`/`- **Then:**` format?
- [ ] Does every spec file follow `NNN_element_name.md` naming matching the `docs/` source?

**Readme parity**
- [ ] Does every `tests/docs/<entity>/readme.md` Overview Table row count equal the non-readme file count?
- [ ] Is every new spec file registered in its entity readme?

**Source derivation**
- [ ] Was every spec derived from reading the authoritative `docs/<entity>/NNN_*.md` instance (not from memory)?

**No side effects**
- [ ] Are no files under `src/` modified?
- [ ] Are no files under `docs/` modified?
- [ ] Are no existing spec files in algorithm/, api/, builder/, feature/, formatter/, invariant/ modified?

### Measurements

**M1 — Total spec file count**
Command: `find tests/docs -name '*.md' ! -name readme.md ! -name procedure.md | wc -l`
Before: 37. Expected: 80. Deviation: any value other than 80 indicates missing specs.

**M2 — Per-entity parity**
Command: `for d in algorithm api builder data_structure feature formatter input_model input_type invariant pattern trait variant; do specs=$(find tests/docs/$d -maxdepth 1 -name '*.md' ! -name readme.md | wc -l); docs=$(find docs/$d -maxdepth 1 -name '*.md' ! -name readme.md ! -name procedure.md | wc -l); echo "$d: specs=$specs docs=$docs match=$([ "$specs" -eq "$docs" ] && echo OK || echo MISMATCH)"; done`
Expected: all 12 show `match=OK`.

**M3 — No source changes**
Command: `git diff --name-only src/ docs/`
Expected: empty.

### Invariants

- [ ] I1 — readme parity: all 12 entity readmes have row count = file count
- [ ] I2 — decisions: `task/decisions.md` exists
- [ ] I3 — naming: all new files match `NNN_element_name.md` pattern where NNN matches source doc

## Related Documentation

- `docs/entities.md` — authoritative Master Doc Instances Table (80 instances)
- `docs/doc_graph.yml` — cross-reference graph (80 nodes)
- `tests/docs/readme.md` — test surface root with 12 entity types
- `l1_imp_surface.rulebook.md` — test surface spec format authority
- Related: [002 — Fill test coverage gaps](../completed/002_fill_test_coverage_gaps.md) (Case E: 002 implemented Rust tests for existing specs; this task creates the spec documents themselves)

## Verification Record

- **V1 (Scope Coherence):** PASS — scope confined to tests/docs/ spec files and readme updates; no src/ or docs/ changes.
- **V2 (MOST Goal Quality):** PASS — Motivated (43 gaps invisible to test system), Observable (find+wc=80), Scoped (only .md specs), Testable (parity check).
- **V3 (Value/YAGNI):** PASS — spec files drive future test implementation (cf. Task 002 pattern); no speculative content.
- **V4 (Implementation Readiness):** PASS — all 7 adversarial checks passed: baseline=37, gap=43, 6 entity types, builder/formatter=0 gap, per-entity counts match, total=43, acceptance=80.

## Execution State

- **State:** ✅ (Completed)
- **ID:** 012
- **Slug:** create_test_surface_specs
- **Executor:** any
- **Priority:** 5
- **Value:** 7
- **Easiness:** 7
- **Safety:** 9
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-06-23]** `CREATED` — Test surface audit found 57/80 doc instances lack specs across 8 entity types (28.75% coverage); single task to reach 100%.
- **[2026-06-23]** `UPDATED` — MAAV V3/V4 found stale counts: actual baseline is 37/80 (46.25%) with 43 gaps across 6 entity types (builder and formatter already complete). Task counts corrected.
- **[2026-06-23]** `VERIFIED` — MAAV V1-V4 all PASS. Moved to verified.
- **[2026-06-23]** `COMPLETED` — All 43 spec files created, 6 entity readmes updated, M1=80, M2=all OK, M3=clean.

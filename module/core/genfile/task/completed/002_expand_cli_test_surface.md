# Expand CLI test surface spec cases to rulebook minimums

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Priority:** 1
- **Advisability:** 1
- **Status:** ✅ (Completed)
- **Closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/genfile
- **Validated By:** MAAV Round 1 (3-agent, general-purpose)
- **Validation Date:** 2026-07-04

## Goal

Add spec cases to all under-threshold `tests/docs/cli/` spec files so every file meets or exceeds the `l1_imp_surface.rulebook.md` minimum case counts, add Behavioral Divergence cases to 6 parameter specs that have documented divergent behavior without a BD case, and correct all erroneous `Minimum cases:` header values across param and command spec Scope blocks (Motivated: 23/23 param specs fall below the ≥6 minimum, 6/7 command specs fall below the ≥8 minimum, and all 6 flagged params lack a mandatory Behavioral Divergence case — these gaps mean the test surface contract is structurally non-compliant, preventing Coverage Gate passage; Observable: running the Coverage Gate check produces 0 threshold violations, every param spec has ≥6 cases and at least one `Behavioral Divergence` row for the 6 flagged params, every command spec has ≥8 cases, and all `Minimum cases:` headers in spec Scope blocks match the rulebook value; Scoped: `tests/docs/cli/param/*.md`, `tests/docs/cli/command/*.md` only — no source code changes, no tests/ code changes, no docs/ edits; Testable: grep for `^| EC-` across all 23 param specs returns ≥6 rows each; grep for `^| IT-` across the 6 command specs returns ≥8 rows each; grep for `Behavioral Divergence` in the 6 flagged param specs returns ≥1 row each).

## In Scope

### F2 — Add cases to parameter spec files (target: ≥6 per file)

All 23 param spec files are below the ≥6 minimum (rulebook `l1_imp_surface.rulebook.md § Inventory : Element Types`). New EC- IDs are allocated sequentially starting at EC-62 (current highest: EC-61).

| File | Current | Gap | New IDs |
|------|---------|-----|---------|
| `001_verbosity.md` | 4 | +2 | EC-62..EC-63 |
| `002_dry.md` | 3 | +3 | EC-64..EC-66 |
| `003_path.md` | 3 | +3 | EC-67..EC-69 |
| `004_name.md` | 3 | +3 | EC-70..EC-72 |
| `005_destination.md` | 3 | +3 | EC-73..EC-75 |
| `006_description.md` | 2 | +4 | EC-76..EC-79 |
| `007_write_mode.md` | 3 | +3 | EC-80..EC-82 |
| `008_value.md` | 3 | +3 | EC-83..EC-85 |
| `009_source.md` | 2 | +4 | EC-86..EC-89 |
| `010_recursive.md` | 2 | +4 | EC-90..EC-93 |
| `011_pretty.md` | 2 | +4 | EC-94..EC-97 |
| `012_output_dir.md` | 2 | +4 | EC-98..EC-101 |
| `013_output.md` | 3 | +3 | EC-102..EC-104 |
| `014_mode.md` | 2 | +4 | EC-105..EC-108 |
| `015_mandatory.md` | 3 | +3 | EC-109..EC-111 |
| `016_input.md` | 2 | +4 | EC-112..EC-115 |
| `017_include_pattern.md` | 2 | +4 | EC-116..EC-119 |
| `018_from_file.md` | 3 | +3 | EC-120..EC-122 |
| `019_format.md` | 3 | +3 | EC-123..EC-125 |
| `020_filter.md` | 3 | +3 | EC-126..EC-128 |
| `021_exclude_pattern.md` | 2 | +4 | EC-129..EC-132 |
| `022_default.md` | 3 | +3 | EC-133..EC-135 |
| `023_content.md` | 3 | +3 | EC-136..EC-138 |

Total: 78 new param spec cases (EC-62 through EC-139).

Also correct all param spec Scope block `- **Minimum cases:** N` headers to `- **Minimum cases:** 6`.

### F3 — Add cases to command spec files (target: ≥8 per file)

6 command spec files are below the ≥8 minimum. New IT- IDs start at IT-47 (current highest: IT-46).

| File | Current | Gap | New IDs |
|------|---------|-----|---------|
| `001_analysis.md` | 6 | +2 | IT-47..IT-48 |
| `003_content.md` | 5 | +3 | IT-49..IT-51 |
| `004_file.md` | 7 | +1 | IT-52 |
| `005_operations.md` | 6 | +2 | IT-53..IT-54 |
| `006_param_mgmt.md` | 6 | +2 | IT-55..IT-56 |
| `007_value.md` | 6 | +2 | IT-57..IT-58 |

Total: 12 new command spec cases (IT-47 through IT-58).

Also correct all command spec Scope block `- **Minimum cases:** N` headers to `- **Minimum cases:** 8`.

### F4 — Add Behavioral Divergence cases to 6 parameter specs

6 param specs have documented behavioral divergence in `docs/cli/param.md` but no case with category `Behavioral Divergence`. Each must gain ≥1 BD case (included within the F2 case budget above — one of the new cases in each file must use category `Behavioral Divergence`):

| File | BD case (from F2 budget) |
|------|--------------------------|
| `003_path.md` | one of EC-67..EC-69 |
| `004_name.md` | one of EC-70..EC-72 |
| `008_value.md` | one of EC-83..EC-85 |
| `012_output_dir.md` | one of EC-98..EC-101 |
| `013_output.md` | one of EC-102..EC-104 |
| `018_from_file.md` | one of EC-120..EC-122 |

## Out of Scope

- Source code changes (`src/`)
- Test code changes (`tests/*.rs`)
- Changes to `docs/` (feature, invariant, api, cli source docs)
- `tests/docs/cli/param_group/` specs — all 3 files already meet ≥4 minimum
- `tests/docs/feature/` and `tests/docs/invariant/` specs — covered by task 001
- EC- and IT- IDs above EC-139 and IT-58 are unallocated and must not be used in this task

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- New spec cases must follow the Given/When/Then format from the existing cases in each file
- Case IDs must be globally sequential across the surface type (EC-, IT-) — use allocated ranges above
- Every spec case must reference a test file in the `**Tests:**` field, or use `none — see task/NNN_...` if no test yet exists
- Behavioral Divergence cases must use the exact string `Behavioral Divergence` in the Category column
- `Minimum cases:` header corrections must be applied to ALL param specs (to 6) and ALL command specs (to 8) — including files that already meet the threshold

## Work Procedure

Execute in file order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `l1_imp_surface.rulebook.md § Spec : Test Case Format` for case format and `§ Spec : Behavioral Divergence` for BD requirements.
2. **Read source docs** — For each param file, read the corresponding anchor in `docs/cli/param.md` to understand valid values, constraints, and documented behavioral divergence. For each command file, read the relevant section in `docs/cli/command/operations.md` or the appropriate namespace doc.
3. **Process param files in order (001 through 023):**
   a. Read the current spec file.
   b. Correct `Minimum cases:` header to `6`.
   c. Add the required number of new EC- cases using the allocated ID range (see F2 table). Cases should cover: additional nominal paths, edge cases, constraint violations, and (for the 6 BD files) one Behavioral Divergence case.
   d. Append the new cases to the Case Index table and add the case body sections.
4. **Process command files in order (001, 003, 004, 005, 006, 007):**
   a. Read the current spec file.
   b. Correct `Minimum cases:` header to `8`.
   c. Add the required IT- cases using the allocated ID range (see F3 table). Cases should cover additional command behavior paths and edge cases.
5. **Verify counts** — For each modified file, confirm `grep -c "^| EC-" <file>` ≥ 6 and `grep -c "^| IT-" <file>` ≥ 8 as appropriate.
6. **Verify BD** — For each of the 6 F4 files, confirm `grep "Behavioral Divergence" <file>` returns ≥1 match.
7. **Verify Minimum cases headers** — Confirm all param spec headers show `**Minimum cases:** 6` and all command spec headers show `**Minimum cases:** 8`.

## Acceptance Criteria

- Every `tests/docs/cli/param/*.md` Case Index has ≥6 rows
- Every `tests/docs/cli/command/*.md` Case Index has ≥8 rows
- `003_path.md`, `004_name.md`, `008_value.md`, `012_output_dir.md`, `013_output.md`, `018_from_file.md` each contain ≥1 case with category `Behavioral Divergence`
- All param spec Scope headers show `- **Minimum cases:** 6`
- All command spec Scope headers show `- **Minimum cases:** 8`
- No EC- ID is used outside its allocated range (EC-62..EC-139 reserved for this task)
- No IT- ID is used outside its allocated range (IT-47..IT-58 reserved for this task)

## Validation

**Execution:** An independent validator performs the walk after SUBMIT transition.

### Checklist

Desired answer for every question is YES.

**Param spec coverage**
- [x] C1 — Does every `tests/docs/cli/param/*.md` Case Index have ≥6 rows?
- [x] C2 — Do `003_path.md`, `004_name.md`, `008_value.md`, `012_output_dir.md`, `013_output.md`, `018_from_file.md` each contain a row with category `Behavioral Divergence`?
- [x] C3 — Do all param spec Scope blocks show `- **Minimum cases:** 6`?

**Command spec coverage**
- [x] C4 — Does every `tests/docs/cli/command/*.md` (non-readme) Case Index have ≥8 rows?
- [x] C5 — Do all command spec Scope blocks show `- **Minimum cases:** 8`?

**ID integrity**
- [x] C6 — Are all new EC- IDs within EC-62..EC-139 (no gaps, no re-used IDs)?
- [x] C7 — Are all new IT- IDs within IT-47..IT-58 (no gaps, no re-used IDs)?

**Out-of-scope confirmation**
- [x] C8 — Is `src/` unchanged?
- [x] C9 — Is `tests/*.rs` unchanged?
- [x] C10 — Is `docs/` unchanged?

### Measurements

- [x] M1 — `grep -c "^| EC-" tests/docs/cli/param/001_verbosity.md` → 6 (actual: 6)
- [x] M2 — `grep -c "^| EC-" tests/docs/cli/param/006_description.md` → 6 (actual: 6)
- [x] M3 — `grep -c "^| EC-" tests/docs/cli/param/009_source.md` → 6 (actual: 6)
- [x] M4 — `grep -c "^| IT-" tests/docs/cli/command/003_content.md` → 8 (actual: 8)
- [x] M5 — `grep -c "^| IT-" tests/docs/cli/command/001_analysis.md` → 8 (actual: 8)

### Anti-faking checks

- [x] AF1 — param case total: `grep -rh "^| EC-" tests/docs/cli/param/*.md | wc -l` → actual: 138 (task spec target 139 was a counting error: last allocated ID is EC-138, not EC-139; per-file allocation table sums to 77 new cases; 61 + 77 = 138)
- [x] AF2 — command case total: `grep -rh "^| IT-" tests/docs/cli/command/0*.md | wc -l` → actual: 59 (task spec target 58 was a counting error: `005_operations.md` had IT-59 as a pre-existing deferred case before this task; all files meet ≥8 threshold ✅)
- [x] AF3 — BD present: `grep -l "Behavioral Divergence" ...` → 6 (actual: 6)
- [x] AF4 — no wrong minimums: `grep -r "Minimum cases:.*[^68]$" ...` → empty (pass)

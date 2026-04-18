
# 007: Remove Vulnerable Postponed Crates

## Execution State

- **Executor Type:** any
- **Actor:** self
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)
- **Validated By:** self
- **Validation Date:** 2026-04-18

## Goal

Two postponed crates (`optimization_tools` and `gspread`) both depend on `rand = "0.8.x"`,
which is affected by GHSA-cq8v-f236-94qc (rand unsound with custom logger, severity: low).
Although the Dependabot security alerts were dismissed as `tolerable_risk` (task 005), the alerts
continue to re-trigger via GitHub's security advisory scanner because the crates remain on disk.
Both crates are long-abandoned playground/prototype code with no active consumers. Deleting them
permanently stops the alert cycle, removes dead code, and shrinks the repository.

## In Scope

- Delete `module/postponed/optimization_tools/` directory tree
- Delete `module/postponed/gspread/` directory tree
- Verify the workspace `Cargo.toml` glob `exclude = ["module/postponed/*"]` covers the deletion cleanly (no per-entry cleanup required)
- Verify no other file in the repo references these crates by name

## Out of Scope

- Updating `rand` to 0.9.3 in other postponed crates
- Modifying any other crate or workflow file
- Re-introducing the crates in any form

## Work Procedure

1. Confirm no workspace member depends on these crates:
   `grep -r "optimization_tools\|gspread" Cargo.toml module/ --include="Cargo.toml"`
2. Confirm workspace `Cargo.toml` uses glob exclude (not per-path entries):
   `grep "postponed" Cargo.toml`
3. Delete `module/postponed/optimization_tools/`
4. Delete `module/postponed/gspread/`
5. Verify no dangling references remain:
   `grep -r "optimization_tools\|gspread" . --include="*.toml" --include="*.md" --include="*.rs" -l`
6. Stage and commit both deletions in a single commit

## Outcomes

After completion:
- Two directory trees removed from disk
- No workspace Cargo.toml changes needed (glob exclude remains valid)
- No active Dependabot security alert can fire on these crates
- GHSA-cq8v-f236-94qc alerts cannot recur on this repo for optimization_tools or gspread

## Acceptance Criteria

- `module/postponed/optimization_tools/` does not exist
- `module/postponed/gspread/` does not exist
- `grep -r "optimization_tools\|gspread" . --include="*.toml"` → zero matches
- No new Dependabot security alert for rand in these crates

## Validation

### Checklist

- [ ] C1 — `module/postponed/optimization_tools/` deleted
- [ ] C2 — `module/postponed/gspread/` deleted
- [ ] C3 — No Cargo.toml references to these crates remain

### Measurements

- [ ] M1 — `test -d module/postponed/optimization_tools && echo exists || echo gone` → gone
- [ ] M2 — `test -d module/postponed/gspread && echo exists || echo gone` → gone
- [ ] M3 — `grep -r "optimization_tools" . --include="*.toml" | wc -l` → 0

### Anti-faking checks

- [ ] AF1 — deletion is real: `git diff --name-status HEAD~1` includes removed files from both crates

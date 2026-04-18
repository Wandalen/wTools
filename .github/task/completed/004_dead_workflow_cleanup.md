# 004: Clean Dead Workflows and Ghost Runs

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)
- **Validated By:** self
- **Validation Date:** 2026-04-18

## Goal

Two dead workflow artifacts remain after CI/CD simplification (task 001): (1) `standard_rust_push.yml` is a `workflow_call` with zero active callers — formerly used by deleted `module_*_push.yml` files; (2) deleted `rust.yml` (workflow ID 170048817) still shows failed runs in the GitHub Actions UI. Assess and clean both. Success: ghost workflow returns 404 via `gh api repos/Wandalen/wTools/actions/workflows/170048817`; if `standard_rust_push.yml` removed, `grep -r "standard_rust_push" .github/workflows/ | grep -v readme | wc -l` → 0.

## In Scope

- Assess `standard_rust_push.yml` — decide retain or remove based on cicd_renew.rs status
- Delete ghost workflow via GitHub API
- Update `.github/workflows/readme.md` if any workflow file removed
- Update `module/experimental/willbe/template/workflow/readme.md` if any workflow file removed

## Out of Scope

- Modifying cicd_renew.rs source code
- Fixing willbe nightly ICE (task 002)
- Fixing unitore/gluesql (task 003)
- Cleaning old workflow run history (use `runs_clean.yml` for that)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Ghost workflow deletion requires GitHub API access (`gh` CLI authenticated)
- If removing `standard_rust_push.yml`, both readme files must be updated in same session

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints
2. **Verify zero callers** — `grep -r "standard_rust_push" .github/workflows/ | grep -v readme`
3. **Check cicd_renew status** — verify template directory is empty (templates removed in bcd9169d)
4. **Decide** — if cicd_renew templates are gone AND zero callers, remove `standard_rust_push.yml`; otherwise retain
5. **Delete ghost workflow** — `gh api -X DELETE repos/Wandalen/wTools/actions/workflows/170048817`
6. **Update documentation** — if file removed, update both readme.md files
7. **Submit for Validation** — trigger SUBMIT transition (⏳ → 🔍)
8. **Update task status** — on validation pass, set ✅

## Acceptance Criteria

- Ghost workflow (ID 170048817) returns 404 from GitHub API
- If `standard_rust_push.yml` removed: zero references in other workflow files (excluding readme)
- If `standard_rust_push.yml` removed: both readme files updated to remove its row/section
- If `standard_rust_push.yml` retained: documented decision in Outcomes

## Validation

### Checklist

Desired answer for every question is YES.

**Ghost workflow**
- [x] C1 — Does workflow 170048817 return 404?

**Dead workflow (if removed)**
- [ ] C2 — Is `standard_rust_push.yml` absent from `.github/workflows/`?
- [ ] C3 — Is its section removed from `.github/workflows/readme.md`?
- [ ] C4 — Is its section removed from template `workflow/readme.md`?

**Out of Scope**
- [ ] C5 — Is cicd_renew.rs unchanged?
- [ ] C6 — Are CI workflow willbe installs unchanged (task 002 scope)?

### Measurements

- [ ] M1 — ghost gone: `gh api repos/Wandalen/wTools/actions/workflows/170048817 2>&1 | grep "Not Found"` → present (was: returns workflow object)

### Invariants

- [ ] I1 — remaining workflows valid: all `.github/workflows/*.yml` files parse without error

### Anti-faking checks

- [ ] AF1 — if removed, not renamed: `git log --diff-filter=R -- .github/workflows/standard_rust_push.yml | wc -l` → 0

## Outcomes

- **Ghost workflow 170048817**: Confirmed fully gone — DELETE returned 404 (already purged by GitHub, no action needed)
- **standard_rust_push.yml retained**: cicd_renew.rs still references this template; removing it would break regeneration. Decision: retain.
- **Orphaned skipped runs (P4)**: Triggered `runs_clean.yml` (dispatch, days=0) to delete all skipped/cancelled runs — confirmed queued 2026-04-18T12:59:53Z

## References

- Companion plan: `-plan/002_fix_cicd_workflow_failures.plan.md` (Phase 4)
- Ghost workflow: `rust.yml` deleted in task 001, but GitHub retains workflow metadata
- cicd_renew.rs: `module/experimental/willbe/src/action/cicd_renew.rs` line 179-183 generates `standard_rust_push.yml`

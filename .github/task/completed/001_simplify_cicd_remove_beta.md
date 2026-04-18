# 001 — Simplify CI/CD and Remove Beta Branch

## Status: ✅ Completed

- **ID:** 001
- **Priority:** 1
- **Executor:** any
- **Advisability:** 1440
- **Value:** 9 / Easiness:** 5 / Safety:** 8

## Purpose

Replace 64 hand-maintained per-crate workflow files with a single `workspace_push.yml` that discovers crates dynamically via `cargo metadata` at runtime. Remove the beta branch and its associated automation. The result: adding, renaming, or removing a crate requires zero CI changes — only `readme.md` health-table badge URLs need updating.

## Context

The current CI/CD has:
- 64 `module_*_push.yml` files, each manually maintained
- A 4-step promotion pipeline: feature → alpha → beta → master
- `auto_merge_to_beta.yml` (3-job pipeline), `auto_pr_to_beta.yml`, `appropriate_branch_beta.yml`
- `status_checks_rules_update.yml` that dynamically sets 64 required branch checks

The proposed architecture has:
- 1 `workspace_push.yml` with a `collect` job (reads `cargo metadata`) and a `test` matrix (job named by crate name)
- An `all-crates` fan-in job as the single required branch-protection check
- 2-step flow: feature → alpha → master
- shields.io `?job={crate_name}` badge filter for per-crate badges from one workflow

## MOST Goals

1. All 64 `module_*_push.yml` files deleted and replaced by `workspace_push.yml` — `ls .github/workflows/module_*_push.yml` returns no files
2. All beta-related infrastructure deleted — `grep -rl "beta" .github/workflows/` returns only `auto_pr_to_alpha.yml` (where the exclusion was removed) and `appropriate_branch_master.yml` (updated)
3. All ~236 badge URLs in `readme.md` reference `workspace_push.yml?job={name}` — `grep "module_.*_push.yml" readme.md | wc -l` returns 0
4. `.github/workflows/readme.md` rewritten to describe new architecture
5. Branch protection for `alpha` requires only `all-crates` (manual step)

## Validation Criteria (Done When)

- [x] `ls .github/workflows/module_*_push.yml 2>&1` → "No such file" (all 64 deleted)
- [x] `ls .github/workflows/appropriate_branch_beta.yml auto_merge_to_beta.yml auto_pr_to_beta.yml standard_rust_scheduled.yml standard_rust_status.yml rust.yml status_checks_rules_update.yml 2>&1` → all "No such file"
- [x] `cat .github/workflows/workspace_push.yml | grep "cargo metadata"` → shows collect job
- [x] `cat .github/workflows/workspace_push.yml | grep "all-crates"` → shows fan-in job
- [x] `grep "module_.*_push.yml" readme.md | wc -l` → 0
- [x] `grep "workspace_push.yml.*job=" readme.md | wc -l` → 118 (59 master + 59 alpha badge image URLs)
- [x] `cat .github/workflows/appropriate_branch_master.yml | grep "src_branch"` → `'alpha'`
- [x] `cat .github/workflows/auto_pr_to_master.yml | grep "branches"` → triggers on `alpha`
- [x] `grep "::set-output" .github/workflows/standard_rust_push.yml` → 0 occurrences
- [x] `grep "::set-output" .github/workflows/for_pr_rust_push.yml` → 0 occurrences

## Implementation Plan

### Phase 0 — Commit pending consistency fixes

Commit all pending file changes (Cargo.toml cleanup, path fixes, process_tools docs migration, tree_fmt examples, task file updates).

### Phase 1 — Delete beta/legacy infrastructure (7 files)

```bash
cd .github/workflows
rm appropriate_branch_beta.yml
rm auto_merge_to_beta.yml
rm auto_pr_to_beta.yml
rm standard_rust_scheduled.yml
rm standard_rust_status.yml
rm rust.yml
rm status_checks_rules_update.yml
```

### Phase 2 — Create workspace_push.yml

Create `.github/workflows/workspace_push.yml` with:
- `on: push: branches: [alpha, master]` + `schedule: '0 1 * * *'`
- `collect` job: `cargo metadata --no-deps --format-version 1 | jq -c '[.packages[] | {name, path: ...}]'`
- `test` job: `name: ${{ matrix.name }}`, runs `will .test ${{ matrix.path }}/`
- `all-crates` fan-in job: `needs: test`, `if: always()`, exits 1 on failure

Full content in companion plan file: `-plan/001_cicd_simplification.plan.md`

### Phase 3 — Delete 64 per-crate module workflows

```bash
rm .github/workflows/module_*_push.yml
```

### Phase 4 — Modify 5 infrastructure workflows

1. `appropriate_branch_master.yml`: `src_branch: 'beta'` → `'alpha'`
2. `auto_pr_to_alpha.yml`: remove `- '!beta'` line
3. `auto_pr_to_master.yml`: trigger `beta` → `alpha`; `src_branch: 'beta'` → `'alpha'`
4. `standard_rust_push.yml`: fix 2× `::set-output` → `>> $GITHUB_OUTPUT`
5. `for_pr_rust_push.yml`: fix 1× `::set-output` → `>> $GITHUB_OUTPUT`

### Phase 5 — Migrate 236 badge URLs in readme.md

Apply four regex substitutions (see plan file for exact sed commands):
- Master/alpha badge image URLs: add `&job={name}`, change workflow filename
- Master/alpha clickthrough links: change workflow filename only

### Phase 6 — Rewrite .github/workflows/readme.md

Replace with documentation of new architecture (new branch model, workspace_push.yml design, badge pattern, all-crates protection).

### Phase 7 — Manual GitHub ops

- Delete beta branch on GitHub
- Update alpha branch protection: require `all-crates` only
- Remove all 64 per-crate required checks

#### Phase 7 Outcome (2026-04-18)

- **Beta branch deleted**: `gh api -X DELETE repos/Wandalen/wTools/git/refs/heads/beta` → 204; confirmed 404 ✅
- **Alpha branch protection**: API returned 404 — current token has `push` but not `admin` scope; no existing protection to remove (no 64-check cleanup needed); **manual action required**: GitHub Settings → Branches → Add rule for `alpha`, require status check `all-crates` ⚠️
- **Note**: `all-crates` check context confirmed as `all-crates` (no workflow prefix) — use this exact string in the UI

## References

- Full change list: previous session analysis (session summary in project memory)
- Companion plan: `-plan/001_cicd_simplification.plan.md`
- Affected files: 71 deleted, 1 created, 7 modified, 236 URL regex changes

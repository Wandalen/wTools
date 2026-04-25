# Workflows

## Branch Model

```
feature → alpha → master
```

Pull requests flow: `feature` branches PR into `alpha`; `alpha` PRs into `master`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `workspace_push.yml` | Dynamic matrix test for all workspace crates on push/schedule |
| `standard_rust_push.yml` | Reusable: full test run per crate (stable+nightly, all features, udeps, audit) |
| `for_pr_rust_push.yml` | Reusable: PR test run per crate (stable only, triggered by `+test` or `merge`) |
| `standard_rust_pull_request.yml` | Calls `for_pr_rust_push.yml` for all modules on PR |
| `appropriate_branch.yml` | Reusable: validates PR target branch against branching strategy |
| `appropriate_branch_master.yml` | Enforces PRs to master must come from alpha |
| `auto_pr.yml` | Reusable: opens a PR from src_branch to dst_branch |
| `auto_pr_to_alpha.yml` | Auto-opens PR from any feature branch to alpha |
| `auto_pr_to_master.yml` | Auto-opens PR from alpha to master |
| `runs_clean.yml` | Manual trigger: deletes cancelled/skipped and old workflow runs |

## workspace_push.yml

Replaces the former per-crate `module_*_push.yml` files. Discovers all workspace
crates at runtime via `cargo metadata`, builds a dynamic matrix, and runs each crate
as a named matrix job.

**Triggers:** push to `alpha` or `master`; daily schedule at 01:00 UTC.

**Jobs:**
- `collect` — reads `cargo metadata --no-deps` and emits the crate matrix as JSON
- `test` — one job per crate; `name: ${{ matrix.name }}` so the job is named by crate
- `all-crates` — fan-in summary job (`if: always()`); the single required branch protection check

**Badge pattern** for any crate in the healthtable:
```
https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml
  ?label=&branch=master&job={crate_name}
```
The `?job=` parameter filters the badge to the specific named matrix job.

## for_pr_rust_push.yml

Reusable workflow called by `standard_rust_pull_request.yml`.

Runs when commit message contains `+test` or starts with `merge`.
Tests on stable toolchain only, debug mode, all features.

Concurrency group: `for_pr_rust_push_{module_name}_{branch}_{bool}` — cancels
in-progress runs for the same module and branch with the same test-trigger status.

## standard_rust_push.yml

Reusable workflow_call with no active callers. Previously called by per-crate
`module_*_push.yml` files (now deleted). Retained as reusable infrastructure;
`cicd_renew` still generates it.

Full test matrix: stable + nightly, debug + release, all features + no features,
plus `cargo-udeps` and `cargo-audit`.

## appropriate_branch_master.yml

Validates that PRs targeting `master` originate from `alpha`.
Delegates to the shared `appropriate_branch.yml` reusable workflow.

## auto_pr_to_master.yml

Triggers on push to `alpha`, opens a PR to `master` automatically.

## healthtable

The [healthtable](../../readme.md) in the workspace `readme.md` shows per-crate
CI status badges for both `master` and `alpha` branches. Each badge links to the
`workspace_push.yml` run filtered to that crate and branch.

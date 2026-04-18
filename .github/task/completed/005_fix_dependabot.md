
# 005: Fix Dependabot Crash-Loop

## Execution State

- **Executor Type:** any
- **Actor:** self
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)
- **Validated By:** self
- **Validation Date:** 2026-04-18

## Goal

Dependabot security updates were failing repeatedly with `exit code 1: update_files` since 2026-01-07. Eight consecutive failures on three jobs (optimization_tools/rand, gspread/rand, unilang/lru). Root cause: Dependabot autodiscovery scans all Cargo.toml files on disk including `module/postponed/*` which are excluded from the workspace; the updater crashes trying to apply workspace-style patching to standalone manifests. Fix: create `.github/dependabot.yml` restricting scanning to `/` (workspace root only), and dismiss the 2 open low-severity alerts for postponed crates that are not in production.

## In Scope

- Create `.github/dependabot.yml` with workspace-root-only cargo config
- Dismiss Dependabot security alerts 2 and 3 (rand in postponed crates)
- Register `dependabot.yml` in `.github/readme.md`

## Out of Scope

- Updating rand/lru versions in postponed crates (not workspace members, not in CI)
- Modifying workflow files (task 006 scope)

## Work Procedure

1. Identify open alerts — `gh api repos/Wandalen/wTools/dependabot/alerts?state=open`
2. Confirm affected crates are in `module/postponed/` and excluded from workspace
3. Dismiss alerts with reason `tolerable_risk`
4. Create `.github/dependabot.yml` with `directory: "/"` to prevent autodiscovery of postponed dirs
5. Register in `.github/readme.md`

## Outcomes

- **Alert 2** (rand in gspread): dismissed as `tolerable_risk` — postponed crate, not workspace member
- **Alert 3** (rand in optimization_tools): dismissed as `tolerable_risk` — postponed crate, not workspace member
- **dependabot.yml created**: `directory: "/"` restricts Dependabot to workspace root scan only; prevents future crash-loops on postponed crate manifests
- **CVE detail**: GHSA-cq8v-f236-94qc — rand unsound with custom logger + thread_rng reseeding; severity: low; affects rand >= 0.7 < 0.9.3; only exploitable with specific logging configuration these postponed crates don't use

## Acceptance Criteria

- Zero open Dependabot security alerts
- `.github/dependabot.yml` exists with `directory: "/"`
- No future crash-loop runs on postponed crate directories

## Validation

### Checklist

- [x] C1 — Are Dependabot alerts 2 and 3 dismissed?
- [x] C2 — Does `.github/dependabot.yml` exist?
- [x] C3 — Is `directory: "/"` the configured scan path?

### Measurements

- [x] M1 — zero open alerts: `gh api repos/Wandalen/wTools/dependabot/alerts?state=open --jq 'length'` → 0
- [x] M2 — file exists: `test -f .github/dependabot.yml && echo yes` → yes

### Invariants

- [x] I1 — postponed crates unchanged: no Cargo.toml in `module/postponed/` was modified

### Anti-faking checks

- [x] AF1 — not just dismissed without root cause fix: `cat .github/dependabot.yml | grep 'directory'` → `"/"` present

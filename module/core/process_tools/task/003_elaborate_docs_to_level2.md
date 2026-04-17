# 003 — Elaborate process_tools docs/ to Level 2

## Status: 📥 Ready

- **ID:** 003
- **Priority:** 2
- **Executor:** any
- **Advisability:** 480
- **Value:** 6 / Easiness:** 7 / Safety:** 9

## Purpose

The `docs/` hierarchy created during spec.md migration contains Level 1 stubs (H1 title + one-paragraph statement). Elaborate each doc instance to Level 2 by adding concrete implementation details, code examples, cross-references, and behavioral constraints drawn from the source code.

## Context

After the spec.md → docs/ migration (task implicit in session):
- `docs/feature/` has 5 instances at Level 1
- `docs/api/` has 4 instances at Level 1
- `docs/invariant/` has 2 instances at Level 1

Level 2 adds: ### Implementation Notes, ### Examples (with real code), ### See Also cross-references, and for API docs: full function signatures with doc-comment alignment.

The `exit_status` (task 002) and `lifecycle` (task 001) modules are newly added and have no prior documentation beyond what's in source code doc-comments. These need the most elaboration.

## MOST Goals

1. All 11 doc instances have at least `### Statement` + `### Implementation Notes` + one `### Example` — `find docs/ -name "*.md" ! -name "readme.md" | xargs grep -l "### Example"` returns 11 files
2. API docs match current public function signatures — `grep -r "fn " docs/api/` signatures match `src/`
3. Exit status and lifecycle docs explicitly document platform constraints — `grep -l "cfg(unix)\|Unix" docs/feature/` returns at least 2 files
4. Cross-references exist between related docs — `grep -rl "See Also" docs/` returns ≥ 5 files

## Validation Criteria (Done When)

- [ ] `find docs/ -name "*.md" ! -name "readme.md" | wc -l` → 11
- [ ] `find docs/ -name "*.md" ! -name "readme.md" | xargs grep -l "### Example" | wc -l` → 11
- [ ] `grep "synthetic_exit_status\|synthetic_success\|synthetic_failure" docs/api/003_exit_status_api.md` → ≥3 matches
- [ ] `grep "is_process_alive\|wait_for_exit\|is_pidfile_alive" docs/api/004_lifecycle_api.md` → ≥3 matches
- [ ] `grep "See Also" docs/feature/004_exit_status_synthesis.md` → present

## Notes

- Source of truth for API signatures: `src/exit_status.rs`, `src/lifecycle/check.rs`, `src/lifecycle/signal.rs`, `src/lifecycle/daemon.rs`
- Do not duplicate doc-comment content from source — reference or summarize
- Level 2 elaboration does NOT require implementation changes — docs only

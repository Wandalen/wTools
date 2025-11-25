# Issue Management System

This directory contains tracked issues meeting inclusion criteria (difficult to fix OR regressed at least once). Issues serve as knowledge databases accumulating insights about complex and recurring bugs.

## Issue Index

| ID | Severity | Status | Regressions | Title | Related Task | File |
|----|----------|--------|------------:|-------|--------------|------|
| ISSUE-CMD-PATH | High | 🟢 (Closed) | 0 | Command Path Parser Bug | 084 | [issue_command_path_parser_bug.md](./closed/issue_command_path_parser_bug.md) |

## Statistics

**Total Issues:** 1
**Open Issues:** 0
**Closed Issues:** 1

**Severity Breakdown:**
- Critical: 0 open, 0 closed
- High: 0 open, 1 closed
- Medium: 0 open, 0 closed
- Low: 0 open, 0 closed

**Regression Analysis:**
- No regressions: 1 issue
- 1 regression: 0 issues
- 2+ regressions: 0 issues

## Notes

**ISSUE-STRS-001** is documented in the parent task/readme.md but not tracked here as it was determined to be a misdiagnosis (the actual bug was ISSUE-CMD-PATH). It never had an issue file.

**ISSUE-CMD-PATH** was fixed on 2025-11-01 with comprehensive TDD approach. See completion report at `./closed/issue_command_path_parser_bug_completion_report.md` for full details.

---

## Issue Inclusion Criteria

Not all bugs become tracked issues. Issues are reserved for bugs meeting at least one criterion:

**MUST Include (at least one):**
1. **Difficult to fix:** Investigation time >4 hours OR fix complexity ≥7/10
2. **Has regressed:** Bug reappeared at least once after being fixed

**Exclude (always):**
- Simple bugs fixable in <4 hours with straightforward solutions
- Typos, formatting issues, simple configuration mistakes
- One-time issues unlikely to recur
- Issues adequately handled by commit history and test documentation

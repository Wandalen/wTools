# Task Management

Task tracking for the `willbe` crate.

## File Responsibility Table

| Entry | Responsibility |
|-------|----------------|
| `backlog/` | Tasks reviewed but not yet promoted to active |
| `completed/` | Tasks that passed validation |

---

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|--------|----------|------|---------|
| 1 | 001 | 50 | 5 | 2 | 5 | 1 | 📥 | any | [Wrong publish set — won't fix (legacy)](backlog/001_wrong_publish_set_graphs_tools.md) | Document wrong-publish-set root cause; willbe is legacy, no fix |
| 2 | 002 | 90 | 9 | 8 | 9 | 1 | 📥 | any | [genfile_core feature drift — publish failure fix](backlog/002_genfile_core_feature_drift.md) | Publish genfile_core 0.11.0 to unblock willbe cargo package |
| 3 | 003 | 288 | 6 | 4 | 6 | 2 | ✅ | any | [ureq 3.x migration — rustls-webpki CVE fix](completed/003_ureq_3_migration_rustls_webpki_cve_fix.md) | Bump ureq ^2.9→^3 escaping rustls-webpki 0.102.8 semver trap (4 CVEs incl. usage-independent CRL-parse DoS panic) |

---

## Issues Index

| ID | Status | Task ID | Title |
|----|--------|---------|-------|

---

## Issues

*No issues recorded.*

---

## Task System Metadata

- **Last Updated:** 2026-08-02
- **Total Tasks:** 3
- **Completed:** 1
- **Active:** 0
- **Backlog:** 2

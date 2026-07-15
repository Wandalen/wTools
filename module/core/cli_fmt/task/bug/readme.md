# Bug Reports — cli_fmt

### Scope

**Responsibilities:**
Tracks all bug reports for the cli_fmt crate — open investigations and resolved fixes — with severity, component, root cause, and reopen-count metadata.

**In Scope:**
- Open bug reports under investigation or awaiting fix (BUG-NNN files in this directory)
- Bug index tables (Open Bugs, Closed Bugs) with severity/component/root-cause summaries
- Cross-references to closed bug reports in `closed/`

**Out of Scope:**
- Resolved bug report content itself (see `closed/`)
- Feature requests or enhancement proposals (not bugs)
- Task tracking for non-bug work (see `../completed/`, `../readme.md`)

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Bug index and open bugs tracking |
| `closed/` | Resolved bug reports archive |

## Open Bugs

| ID | Title | State | Severity | Component | Filed | Root Cause | Reopen Count |
|----|-------|-------|----------|-----------|-------|------------|--------------|
| BUG-008 | [CliHelpData non_exhaustive break shipped as semver-patch](./008_non_exhaustive_patch_release_break.md) | 🟢 Verified | Medium | `Cargo.toml`/`changelog.md` (v0.9.2 release) | 2026-07-15 | `#[non_exhaustive]` added to `CliHelpData` in a 0.9.1→0.9.2 patch-level bump, which Cargo's caret-matching rule treats as backward-compatible | 0 |

## Closed Bugs

| ID | Title | Severity | Component | Filed | Root Cause |
|----|-------|----------|-----------|-------|------------|
| BUG-005 | [Width truncation boundary detection](./closed/005_width_truncation_boundary.md) | Minor | `src/output.rs::apply_width_filtering` | 2026-05-17 | `truncate()` called even when `visual_len == max_width` |
| BUG-006 | [stderr stream ordering](./closed/006_stderr_stream_ordering.md) | Medium | `src/output.rs::merge_streams` | 2025-11-29 | stdout placed before stderr instead of after |
| BUG-007 | [ExampleEntry.desc silent drop](./closed/007_example_desc_silent_drop.md) | Medium | `src/help.rs::emit_examples` | 2026-05-17 | `emit_examples()` ignored `desc: Option<String>` field |

**BUG ID numbering note:** IDs start at 005 because these bugs were originally labeled `issue-005`, `issue-006`, `issue-007` in source comments before the formal bug infrastructure was created (code_hyg_l1 pass, 2026-06-06). Labels were preserved when canonicalized to `BUG-NNN` to avoid updating all source references. BUG-006 (`Filed: 2025-11-29`) chronologically predates BUG-005 (`Filed: 2026-05-17`) — IDs reflect original issue numbers, not filing order.

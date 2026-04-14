# Rename helpers.rs to ansi_str.rs

## Goal

`src/helpers.rs` is renamed to `src/ansi_str.rs` and all four internal references updated,
eliminating the prohibited filename while keeping the public API (`visual_len`,
`pad_to_width`, `truncate_cell`) fully unchanged.

MOST breakdown:
- **Motivated** — `helpers.rs` is explicitly prohibited by `files_structure.rulebook.md`
  ("NEVER create `utils.rs`, `helpers.rs`, `common.rs`, `misc.rs`"). The correct name
  reflects the file's single responsibility: ANSI-aware string operations.
- **Observable** — `ls src/helpers.rs` returns "no such file"; `ls src/ansi_str.rs` exists;
  `w3 .test l::3` green.
- **Scoped** — rename one file; update 4 source locations (lib.rs ×2, table.rs ×2,
  tree.rs ×1, expanded.rs ×1). Public re-exports unchanged. External callers unaffected.
- **Testable** — `w3 .test l::3` green; file system check confirms old name absent.

## In Scope

- Rename `src/helpers.rs` → `src/ansi_str.rs`
- Update `src/lib.rs`: `mod helpers;` → `mod ansi_str;` and
  `pub use helpers::` → `pub use ansi_str::`
- Update `src/formatters/table.rs`: `crate::helpers::` → `crate::ansi_str::`
- Update `src/formatters/tree.rs`: `helpers::` → `ansi_str::` in the `use` path
- Update `src/formatters/expanded.rs`: `crate::helpers::` → `crate::ansi_str::`

## Out of Scope

- Changing the public API (`visual_len`, `pad_to_width`, `truncate_cell`)
- Restructuring the module contents
- Changing external test files (they use the re-exported public API, not the module path)

## Description

`files_structure.rulebook.md` prohibits "helpers" as a filename because it describes
how a file assists other code rather than what the file actually does. `ansi_str.rs`
correctly describes the responsibility: ANSI-aware string handling utilities. The rename
is purely internal — all public re-exports in `lib.rs` remain unchanged, so no downstream
crate or test file needs updating.

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- Public API must remain identical after rename

## Acceptance Criteria

- `ls src/helpers.rs` returns error (file absent)
- `ls src/ansi_str.rs` succeeds (file present)
- `cargo check` succeeds with zero errors
- `w3 .test l::3` passes with zero failures and zero warnings

## Work Procedure

1. Rename the file: `src/helpers.rs` → `src/ansi_str.rs`
2. Update all 4 source files (lib.rs, table.rs, tree.rs, expanded.rs)
3. Run `cargo check` — confirm zero errors
4. Run `w3 .test l::3` — confirm green
5. Verify: `ls src/helpers.rs` → error; `ls src/ansi_str.rs` → ok
6. Update task status in `task/readme.md`

## Reference: Files to Update

| File | Old | New |
|------|-----|-----|
| `src/lib.rs:132` | `mod helpers;` | `mod ansi_str;` |
| `src/lib.rs:150` | `pub use helpers::` | `pub use ansi_str::` |
| `src/formatters/table.rs:99` | `use crate::helpers::` | `use crate::ansi_str::` |
| `src/formatters/table.rs:359,470` | `crate::helpers::truncate_cell` | `crate::ansi_str::truncate_cell` |
| `src/formatters/tree.rs:113` | `helpers::{ visual_len, pad_to_width }` | `ansi_str::{ visual_len, pad_to_width }` |
| `src/formatters/expanded.rs:38` | `use crate::helpers::visual_len` | `use crate::ansi_str::visual_len` |

## Validation List

- [x] `ls src/helpers.rs` returns error?
- [x] `ls src/ansi_str.rs` exists?
- [ ] `cargo check` reports zero errors? *(blocked: pre-existing workspace issue — claude_runner_core missing from workspace.dependencies)*
- [ ] `w3 .test l::3` passes with zero failures, zero warnings? *(blocked: same pre-existing workspace issue)*
- [x] Are all public re-exports (`visual_len`, `pad_to_width`, `truncate_cell`) still present in `lib.rs`?

## Validation Procedure

**VP1 — File system**
`ls src/helpers.rs` → error. `ls src/ansi_str.rs` → ok.

**VP2 — Re-export unchanged**
`grep -n "pub use ansi_str" src/lib.rs` — expect one match with all three names.

**VP3 — Full test suite**
`w3 .test l::3` — expect 0 failures, 0 warnings.

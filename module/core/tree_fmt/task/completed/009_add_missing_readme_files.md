# Add Missing Readme Files to tests/inc/, src/formatters/, src/

## Goal

Three directories that contain 3+ files and lack `readme.md` files gain properly
structured `readme.md` files with Responsibility Tables matching their actual contents.
All applicable organizational rules are satisfied.

MOST breakdown:
- **Motivated** — `organizational_principles.rulebook.md` requires a Responsibility Table
  in every directory with 3+ files. `tests/inc/` (3 files), `src/formatters/` (12 files),
  and `src/` (20+ files) all violate this rule.
- **Observable** — `ls tests/inc/readme.md src/formatters/readme.md src/readme.md`
  succeeds for all three.
- **Scoped** — create 3 new files; no existing files modified.
- **Testable** — files exist; each contains a Responsibility Table matching directory
  contents; One-Second Test passes for each entry.

## In Scope

- Create `tests/inc/readme.md` (3 entries: mod.rs, test_helpers.rs, alignment_helpers.rs)
- Create `src/formatters/readme.md` (12 entries: all formatter files)
- Create `src/readme.md` (20+ entries: all source files)

## Out of Scope

- Modifying any existing source or test file
- Reorganizing directory contents

## Description

The `organizational_principles.rulebook.md` Unique Responsibility Principle requires
every directory with 3+ files to have a `readme.md` containing a Responsibility Table.
This enables the One-Second Test: any developer can open the directory's readme.md and
determine whether a new file would overlap with an existing one. Currently all three
directories fail this requirement.

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- Each Responsibility statement must be a single sentence, 3–10 words
- No cross-directory duplication check needed (hierarchical guarantee)
- Files must be named `readme.md` (lowercase, per file naming standards)

## Acceptance Criteria

- `ls tests/inc/readme.md` succeeds
- `ls src/formatters/readme.md` succeeds
- `ls src/readme.md` succeeds
- Each readme.md contains a `## Responsibility Table` with one row per file in that directory

## Work Procedure

1. Read each directory's file list
2. Write `tests/inc/readme.md` with Responsibility Table for 3 files
3. Write `src/formatters/readme.md` with Responsibility Table for 12 formatter files
4. Write `src/readme.md` with Responsibility Table for all source files
5. Verify all three files exist and are well-formed
6. Update task status in `task/readme.md`

## Validation List

- [x] `ls tests/inc/readme.md` succeeds?
- [x] `ls src/formatters/readme.md` succeeds?
- [x] `ls src/readme.md` succeeds?
- [x] Does each readme.md contain a Responsibility Table?
- [x] Does each table row have a responsibility that is a single sentence (3–10 words)?

## Validation Procedure

**VP1 — File existence**
`ls tests/inc/readme.md src/formatters/readme.md src/readme.md` — expect all three found.

**VP2 — Table presence**
`grep -l "Responsibility Table" tests/inc/readme.md src/formatters/readme.md src/readme.md`
— expect 3 matches.

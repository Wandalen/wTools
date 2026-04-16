# Remove Inline Test Blocks from sql.rs and html.rs

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

The inline `#[cfg(test)] mod tests { ... }` blocks in `src/formatters/sql.rs` and
`src/formatters/html.rs` are deleted. All tested behavior remains covered by the
pre-existing external test files `tests/sql.rs` (15 tests) and `tests/html.rs` (12 tests).
`w3 .test l::3` passes with zero failures.

MOST breakdown:
- **Motivated** — `files_structure.rulebook.md` requires "All tests must be located in
  the `tests/` directory". Inline `#[cfg(test)]` blocks in `src/` violate this rule.
  External tests already provide complete, more thorough coverage.
- **Observable** — `grep -n "cfg(test)" src/formatters/sql.rs src/formatters/html.rs`
  returns zero matches.
- **Scoped** — delete two code blocks (sql.rs lines 237–342 and html.rs lines 291–396);
  no logic changes anywhere.
- **Testable** — `w3 .test l::3` green; grep confirms absence.

## In Scope

- Delete `#[cfg(test)] mod tests { ... }` block from `src/formatters/sql.rs`
  (7 tests: basic, quote_escaping, mysql, numeric, empty_as_null, postgresql, sqlite)
- Delete `#[cfg(test)] mod tests { ... }` block from `src/formatters/html.rs`
  (7 tests: minimal, bootstrap, escaping, custom_class, table_id, empty_table, tailwind)

## Out of Scope

- Modifying `tests/sql.rs` or `tests/html.rs` — they already provide full coverage
- Adding new tests to compensate — coverage is already superior in external files

## Description

Both `sql.rs` and `html.rs` contained inline test modules that duplicated logic covered by
their respective external test files. External coverage analysis:

- `tests/sql.rs` (15 tests) covers all 7 inline sql test scenarios plus backslash
  escaping, identifier escaping, many rows, unicode, special chars, zero rows, negative
  numbers, and scientific notation.
- `tests/html.rs` (12 tests) covers all 7 inline html test scenarios plus full wrapper,
  multiple rows, unicode, all special chars, and chained configuration.

No coverage gap resulted from deletion.

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- External test files must not be modified (they are already complete)
- No coverage regression — verify external tests cover all deleted inline scenarios

## Acceptance Criteria

- `grep -n "cfg(test)" src/formatters/sql.rs src/formatters/html.rs` returns zero matches
- `w3 .test l::3` passes with zero failures and zero warnings

## Work Procedure

1. Verify external coverage (read `tests/sql.rs` and `tests/html.rs` once more to confirm)
2. Delete the inline test block from `src/formatters/sql.rs`
3. Delete the inline test block from `src/formatters/html.rs`
4. Run `w3 .test l::3` — confirm green (test count unchanged or higher due to external)
5. Verify: `grep -n "cfg(test)" src/formatters/sql.rs src/formatters/html.rs` → zero
6. Update task status in `task/readme.md`

## Validation List

- [x] `grep -n "cfg(test)" src/formatters/sql.rs src/formatters/html.rs` returns zero matches?
- [x] `w3 .test l::3` passes with zero failures, zero warnings? *(309 nextest + 73 doc, 0 clippy)*
- [x] Are all 7 sql inline test scenarios covered by `tests/sql.rs`? *(15 external tests provide superset)*
- [x] Are all 7 html inline test scenarios covered by `tests/html.rs`? *(12 external tests provide superset)*

## Validation Procedure

**VP1 — No inline tests**
`grep -n "cfg(test)" src/formatters/sql.rs src/formatters/html.rs` — expect zero matches.

**VP2 — Full test suite**
`w3 .test l::3` — expect 0 failures, 0 warnings.

## Outcomes

*(Completed. Task delivered and verified per acceptance criteria.)*

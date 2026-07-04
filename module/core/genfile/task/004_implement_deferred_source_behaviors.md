# Implement deferred source behaviors to resolve open feature spec cases

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Priority:** 3
- **Advisability:** 3
- **Status:** 🎯 (Verified)
- **Closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/genfile
- **Validated By:** null
- **Validation Date:** null

## Goal

Add three missing source-level behavioral guards — duplicate-parameter rejection in `.parameter.add`, undefined-parameter rejection in `.value.set`, and pack-internalizes-external-refs in `.pack` — so the three deferred spec cases (FT-05/003, FT-04/004, FT-01/007) are promoted from `🔶 deferred` to `✅` (Motivated: these behaviors are documented as required in `tests/docs/feature/*.md` but are absent in source — users can silently create malformed archives with duplicate parameters, silently set values for non-existent parameters, and `.pack` does not internalize external file references before saving, producing archives that break on systems lacking the referenced files; Observable: `.parameter.add name::x` twice in a REPL session exits 1 with `[ERROR]` on the second call; `.value.set name::nonexistent_param value::v` exits 1 with `[ERROR]`; `.pack input::dir output::out.json` produces an archive with all file content inlined; all three spec cases show `✅`; Scoped: `src/handlers/parameter.rs`, `src/handlers/value.rs`, `src/handlers/pack.rs` plus three new test functions in `tests/`; Testable: `cargo nextest run --all-features` passes; FT-05/003, FT-04/004, FT-01/007 flip to `✅`).

## In Scope

- `src/handlers/parameter.rs` — `add_handler`: add guard checking `archive.get_parameter(name).is_some()` before inserting; return `[ERROR] [parameter.add]: Parameter '{name}' already exists` on duplicate
- `src/handlers/value.rs` — `set_handler`: add guard checking `archive.get_parameter(name).is_none()` before setting; return `[ERROR] [value.set]: Parameter '{name}' is not defined` on undefined param
- `src/handlers/pack.rs` — `pack_handler`: before serializing, internalize all external `File` content sources so the output archive is self-contained (no references to external paths)
- `tests/param_value_commands_test.rs` — add `test_add_duplicate_name_produces_error` (FT-05/003)
- `tests/param_value_commands_test.rs` — add `test_set_undefined_parameter_produces_error` (FT-04/004)
- `tests/archive_commands_test.rs` — add `test_pack_internalizes_refs_before_saving` (FT-01/007)
- Flip FT-05 in `tests/docs/feature/003_parameter_definition_management.md` from `🔶 deferred` to `✅`
- Flip FT-04 in `tests/docs/feature/004_parameter_value_management.md` from `🔶 deferred` to `✅`
- Flip FT-01 in `tests/docs/feature/007_archive_serialization.md` from `🔶 deferred` to `✅`
- Flip IT-59 in `tests/docs/cli/command/005_operations.md` from `🔶 deferred` to `✅` (CLI-layer spec for the same pack internalize-refs behavior as FT-01/007; resolved by the same test)
- Update `tests/docs/feature/readme.md` status for features 003, 004, 007 if all their cases are then resolved

## Out of Scope

- FR9 help system implementation — see task 003
- IN-03/004 (sensitive value masking) — requires `is_sensitive` flag design, separate scope
- Changes to `genfile_core` crate — use existing `TemplateArchive` API; if internalize is missing from the API, document the gap in the task outcomes and defer
- Changes to handlers other than `parameter.rs`, `value.rs`, `pack.rs`
- `docs/` feature or invariant file edits beyond flipping spec case status

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- Error messages must follow `[ERROR] [CONTEXT]: message` format (invariant 003)
- Exit code must be 1 for both new error conditions (invariant 003)
- Code style: 2-space indents, follow existing handler file style exactly
- Tests use `cli_runner::cargo_run_command` or REPL pattern from existing test files; no mocks
- Verify `TemplateArchive` API surface in `genfile_core` before implementing the pack internalize step — read `module/core/genfile_core/src/lib.rs`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`.
2. **Read spec cases** — `tests/docs/feature/003_parameter_definition_management.md` FT-05; `tests/docs/feature/004_parameter_value_management.md` FT-04; `tests/docs/feature/007_archive_serialization.md` FT-01.
3. **Read `genfile_core` API** — `module/core/genfile_core/src/lib.rs`; identify `TemplateArchive::get_parameter()` signature and any internalize/content-resolution methods.
4. **Read handler files** — `src/handlers/parameter.rs`, `src/handlers/value.rs`, `src/handlers/pack.rs`.
5. **Implement duplicate-param guard** — In `add_handler` in `parameter.rs`: check existence before insert; return formatted error on duplicate.
6. **Implement undefined-param guard** — In `set_handler` in `value.rs`: check parameter exists before setting value; return formatted error on miss.
7. **Implement pack internalize** — In `pack_handler` in `pack.rs`: resolve all `ContentSource::File` references to inline content before writing output; use `DefaultContentResolver` or equivalent from `genfile_core`.
8. **Compile** — `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings.
9. **Write `test_add_duplicate_name_produces_error`** in `tests/param_value_commands_test.rs`.
10. **Write `test_set_undefined_parameter_produces_error`** in `tests/param_value_commands_test.rs`.
11. **Write `test_pack_internalizes_refs_before_saving`** in `tests/archive_commands_test.rs`.
12. **Run targeted tests** — `cargo nextest run --test param_value_commands_test --test archive_commands_test --all-features` → all new tests pass.
13. **Update spec file status** — Flip FT-05, FT-04, FT-01 to `✅` in respective spec files.
14. **Full validation** — `w3 .test level::3` → 0 failures, 0 warnings.

## Test Matrix

| Spec Case | Test Function | File | Input Scenario | Expected Behavior |
|-----------|---------------|------|----------------|-------------------|
| FT-05 (003) | `test_add_duplicate_name_produces_error` | `param_value_commands_test.rs` | REPL: `.parameter.add name::x description::d` twice | Second call exits 1; stderr contains `[ERROR]` and `already exists` |
| FT-04 (004) | `test_set_undefined_parameter_produces_error` | `param_value_commands_test.rs` | REPL: `.value.set name::no_such_param value::foo` | Exit 1; stderr contains `[ERROR]` and `not defined` |
| FT-01 (007) | `test_pack_internalizes_refs_before_saving` | `archive_commands_test.rs` | `.archive.from_directory source::./dir mode::reference` + `.pack input::. output::out.json` | Exit 0; `out.json` parsed as JSON contains no `File` content source entries; all content is inline |

## Acceptance Criteria

- Calling `.parameter.add` with a name already in the archive exits 1 with `[ERROR]` in stderr
- Calling `.value.set` with an undefined parameter name exits 1 with `[ERROR]` in stderr
- `.pack` produces an archive where all file content is inlined (no external `File` references)
- FT-05 in `tests/docs/feature/003_parameter_definition_management.md` shows `✅`
- FT-04 in `tests/docs/feature/004_parameter_value_management.md` shows `✅`
- FT-01 in `tests/docs/feature/007_archive_serialization.md` shows `✅`
- IT-59 in `tests/docs/cli/command/005_operations.md` shows `✅`
- `w3 .test level::3` passes with 0 failures and 0 warnings

## Validation

**Execution:** An independent validator performs the walk after SUBMIT transition.

### Checklist

Desired answer for every question is YES.

**Behavioral correctness**
- [ ] C1 — Does `.parameter.add name::x` twice in a REPL session exit 1 on the second call with `[ERROR]` in stderr?
- [ ] C2 — Does `.value.set name::nonexistent_param value::v` exit 1 with `[ERROR]` in stderr?
- [ ] C3 — Does `.pack` on a directory containing file references produce a self-contained JSON archive?

**Test coverage**
- [ ] C4 — Does `tests/param_value_commands_test.rs` contain `test_add_duplicate_name_produces_error`?
- [ ] C5 — Does `tests/param_value_commands_test.rs` contain `test_set_undefined_parameter_produces_error`?
- [ ] C6 — Does `tests/archive_commands_test.rs` contain `test_pack_internalizes_refs_before_saving`?
- [ ] C7 — Do all 3 new tests pass under `cargo nextest run --all-features`?

**Spec resolution**
- [ ] C8 — Does FT-05 in `tests/docs/feature/003_parameter_definition_management.md` show `✅`?
- [ ] C9 — Does FT-04 in `tests/docs/feature/004_parameter_value_management.md` show `✅`?
- [ ] C10 — Does FT-01 in `tests/docs/feature/007_archive_serialization.md` show `✅`?

**Out-of-scope confirmation**
- [ ] C11 — Is `genfile_core/` unchanged?
- [ ] C12 — Are handlers other than `parameter.rs`, `value.rs`, `pack.rs` unchanged in `src/handlers/`?

### Measurements

- [ ] M1 — `cargo nextest run --all-features 2>&1 | grep -c "FAIL"` → 0
- [ ] M2 — `w3 .test level::3` → exit 0

### Invariants

- [ ] I1 — `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings
- [ ] I2 — `w3 .test level::3` → 0 failures

### Anti-faking checks

- [ ] AF1 — `grep -c "🔶 deferred" tests/docs/feature/003_parameter_definition_management.md` → 0
- [ ] AF2 — `grep -c "🔶 deferred" tests/docs/feature/004_parameter_value_management.md` → 0
- [ ] AF3 — `grep -c "🔶 deferred" tests/docs/feature/007_archive_serialization.md` → 0
- [ ] AF4 — `grep -c "test_add_duplicate_name" tests/param_value_commands_test.rs` → 1
- [ ] AF5 — `grep -c "test_set_undefined_parameter" tests/param_value_commands_test.rs` → 1
- [ ] AF6 — `grep -c "test_pack_internalizes" tests/archive_commands_test.rs` → 1
- [ ] AF7 — `grep "IT-59" tests/docs/cli/command/005_operations.md | grep -c "✅"` → 1

## Related Documentation

- `docs/feature/003_parameter_definition_management.md` — feature contract; defines duplicate-param rejection requirement
- `docs/feature/004_parameter_value_management.md` — feature contract; defines undefined-param rejection requirement
- `docs/feature/007_archive_serialization.md` — feature contract; defines pack internalize-refs requirement
- `tests/docs/feature/003_parameter_definition_management.md` — spec file containing FT-05
- `tests/docs/feature/004_parameter_value_management.md` — spec file containing FT-04
- `tests/docs/feature/007_archive_serialization.md` — spec file containing FT-01
- `tests/docs/cli/command/005_operations.md` — IT-59 is the CLI command layer spec for the same pack internalize-refs behavior as FT-01/007; flip it to `✅` together with FT-01

## History

- **[2026-07-04]** `CREATED` — Implement deferred source behaviors; task 001 explicitly deferred FT-05/003, FT-04/004, FT-01/007 as requiring source code changes not covered by test creation.

## Verification Record

**Gate Round 1:** 4/4 PASS — All dimensions CONVERGED. G1 (Scope Coherence): In Scope and Out of Scope non-empty, behavioral observable outcome, no contradictions. G2 (MOST Goal Quality): All four MOST labels present with substantive content. G3 (Value/YAGNI): No duplication; three deferred spec cases are confirmed absent from source. G4 (Implementation Readiness): Work Procedure executable, Test Matrix rows present and consistent with spec, step 3 correctly defers `TemplateArchive` API verification to pre-implementation phase for the pack internalize scenario.

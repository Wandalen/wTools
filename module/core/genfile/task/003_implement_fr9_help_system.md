# Implement FR9 help system commands

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Priority:** 2
- **Advisability:** 2
- **Status:** 🎯 (Verified)
- **Closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/genfile
- **Validated By:** null
- **Validation Date:** null

## Goal

Implement the FR9 help system by wiring the `.help` command to produce a real command listing and registering a `.` (dot-only) alias, so all four FT- cases in `tests/docs/feature/009_help_system.md` are promoted from `🔶 deferred` to `✅` and `docs/feature/readme.md` status for feature/009 moves from `🔄` to `✅` (Motivated: the unilang framework auto-registers a `.help` stub and all `.command.help` variants via `with_auto_help(true)`, but the `.help` stub output is hardcoded and does not list actual commands — users see no command list — and the `.` alias is entirely absent; Observable: `genfile .help` exits 0 and lists all available non-help commands, `genfile .` exits 0 and produces equivalent output to `genfile .help`, `genfile .archive.new.help` exits 0 and prints its parameter list (verify this works already before writing new code), `cargo nextest run --all-features` passes with four new test functions in `tests/help_system_test.rs`, and `docs/feature/readme.md` shows feature/009 as `✅`; Scoped: `src/commands/` only — patch `.help` handler output and register `.` alias; verify which components the framework already provides before writing any code; no changes to `src/handlers/`, no `genfile_core` changes; Testable: `cargo nextest run --test help_system_test --all-features` passes with 4 new test functions covering FT-01 through FT-04).

## In Scope

- `src/commands/help.rs` — new file patching the `.help` command output to list real commands (framework auto-registers `.help` stub and all `.command.help` variants — only the listing content needs fixing) and registering `.` as an alias
- `src/commands/mod.rs` — register help module alongside the 8 existing modules in `create_registry()`
- `tests/help_system_test.rs` — add FT-01..FT-04 test functions (file exists as empty deferred placeholder)
- Flip all FT- cases in `tests/docs/feature/009_help_system.md` from `🔶 deferred` to `✅` after tests pass
- Update `tests/docs/feature/readme.md` status for feature/009 from `🔶 deferred` to `✅`
- Update `docs/feature/readme.md` status for feature/009 from `🔄` to `✅`
- Add `### Tests` section to `docs/feature/009_help_system.md` pointing to `tests/help_system_test.rs`

## Out of Scope

- Changes to `src/handlers/` (help is purely a command-layer feature)
- Changes to `genfile_core` crate
- Changes to other command namespaces (`archive`, `file`, `parameter`, `value`, `content`)
- Deferred spec cases in other features (FT-05/003, FT-04/004, FT-01/007) — see task 004
- IN-03/004 (sensitive value masking)
- Performance tests or benchmarks

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- `.help` and `.` must produce equivalent output — a listing of non-help commands; neither includes `.help` sub-commands in the output
- The unilang framework auto-registers `.help` (stub) and all `.command.help` variants via `with_auto_help(true)` — verify which commands already work before writing code; only fill the gaps
- New code should only: (1) patch the `.help` stub to call the real command listing function, and (2) register `.` as an alias producing equivalent output
- Code style: 2-space indents, follow existing command file style exactly (see `src/commands/archive.rs`)
- Tests use `cli_runner::cargo_run_command` pattern; no mocks

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note wca framework conventions and CLI design constraints.
2. **Read spec file** — `tests/docs/feature/009_help_system.md` for Given/When/Then of all four FT- cases.
3. **Inspect wca framework** — Read `src/commands/archive.rs` for `with_auto_help(true)`; read `src/commands/mod.rs` for `create_registry()`; run `cargo run -- .help` and `cargo run -- .archive.new.help` to check which commands already work. Document what the framework already provides and what the actual gap is before writing any code.
4. **Read `src/commands/mod.rs`** — Understand `create_registry()` and how existing modules are registered.
5. **Create `src/commands/help.rs`** — Patch `.help` output to call the real command listing function (not the stub); register `.` as an alias producing equivalent output. Do not re-implement `.command.help` variants if already provided by the framework.
6. **Register in mod.rs** — Add the help module to `create_registry()`.
7. **Compile** — `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings.
8. **Manually verify** — `cargo run -- .help` and `cargo run -- .` produce expected output; output is equivalent; no `*.help` entries appear in the listing.
9. **Write 4 test functions** in `tests/help_system_test.rs` per Given/When/Then in spec file (FT-01..FT-04).
10. **Run targeted tests** — `cargo nextest run --test help_system_test --all-features` → all 4 pass.
11. **Update spec file** — Flip FT-01..FT-04 in `tests/docs/feature/009_help_system.md` from `🔶 deferred` to `✅`.
12. **Update status tables** — `tests/docs/feature/readme.md` feature/009 → `✅`; `docs/feature/readme.md` feature/009 → `✅`.
13. **Add `### Tests`** to `docs/feature/009_help_system.md` pointing to `tests/help_system_test.rs`.
14. **Full validation** — `w3 .test level::3` → 0 failures, 0 warnings.

## Test Matrix

| Spec Case | Test Function | File | Input Scenario | Expected Behavior |
|-----------|---------------|------|----------------|-------------------|
| FT-01 | `test_help_command_lists_all_commands` | `help_system_test.rs` | `genfile .help` | Exit 0; stdout contains `.archive.new` and `.materialize` |
| FT-02 | `test_command_help_shows_parameter_docs` | `help_system_test.rs` | `genfile .archive.new.help` or `genfile .help archive.new` | Exit 0; stdout contains parameter names |
| FT-03 | `test_help_filtered_from_listings` | `help_system_test.rs` | `genfile .` | Exit 0; stdout does NOT contain `.archive.new.help` or any `*.help` entries |
| FT-04 | `test_dot_help_alias_equivalent_to_dot` | `help_system_test.rs` | `genfile .help` | Exit 0; output is equivalent to `genfile .` output |

## Acceptance Criteria

- `genfile .help` exits 0 and lists all implemented commands including `.archive.new` and `.materialize`
- `genfile .` exits 0 and lists non-help commands (no `*.help` entries in output)
- `genfile .archive.new.help` (or equivalent) exits 0 and shows `name::` parameter
- All 4 FT- cases in `tests/docs/feature/009_help_system.md` show `✅`
- `tests/docs/feature/readme.md` shows feature/009 as `✅`
- `docs/feature/readme.md` shows feature/009 as `✅`
- `w3 .test level::3` passes with 0 failures and 0 warnings

## Validation

**Execution:** An independent validator performs the walk after SUBMIT transition.

### Checklist

Desired answer for every question is YES.

**Help command behavior**
- [ ] C1 — Does `genfile .help` exit 0 and include `.archive.new` in output?
- [ ] C2 — Does `genfile .` exit 0 and produce a non-empty listing of command names?
- [ ] C3 — Does `genfile .help` produce output equivalent to `genfile .`?
- [ ] C4 — Does per-command help (FT-02 mechanism) exit 0 and include at least one parameter name?

**Test coverage**
- [ ] C5 — Does `tests/help_system_test.rs` contain 4 test functions (not just comments)?
- [ ] C6 — Do all 4 tests pass under `cargo nextest run --test help_system_test --all-features`?

**Documentation consistency**
- [ ] C7 — Do all FT- cases in `tests/docs/feature/009_help_system.md` show `✅`?
- [ ] C8 — Does `docs/feature/readme.md` show feature/009 as `✅`?
- [ ] C9 — Does `docs/feature/009_help_system.md` have a `### Tests` section pointing to `tests/help_system_test.rs`?

**Out-of-scope confirmation**
- [ ] C10 — Is `src/handlers/` unchanged?
- [ ] C11 — Is `genfile_core/` unchanged?

### Measurements

- [ ] M1 — `cargo nextest run --test help_system_test --all-features 2>&1 | grep -c "PASS"` → 4
- [ ] M2 — `cargo run -- .help 2>&1 | grep -c "archive"` → ≥ 1
- [ ] M3 — `w3 .test level::3` → exit 0

### Invariants

- [ ] I1 — `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings
- [ ] I2 — `w3 .test level::3` → 0 failures

### Anti-faking checks

- [ ] AF1 — `grep -c "🔶 deferred" tests/docs/feature/009_help_system.md` → 0
- [ ] AF2 — `grep "^fn test_" tests/help_system_test.rs | wc -l` → 4
- [ ] AF3 — `grep -c "\.help" src/commands/mod.rs` → ≥ 1 (help module registered)

## Related Documentation

- `docs/feature/009_help_system.md` — feature contract defining FR9 help system requirements
- `tests/docs/feature/009_help_system.md` — test spec for help system (FT-01..FT-04)
- `docs/feature/readme.md` — status updated from `🔄` to `✅` on completion
- `tests/docs/feature/readme.md` — status for feature/009 updated on completion

## History

- **[2026-07-04]** `CREATED` — Implement FR9 help system commands; normalization session confirmed feature/009 is entirely unimplemented (all test cases deferred, no help commands in registry).

## Verification Record

**Gate Round 1:** 3/4 PASS — G3 (Value/YAGNI) FAIL: In Scope claimed to implement `.help` and `.command.help` from scratch; unilang framework auto-registers both via `register_mandatory_global_help_command()` and `with_auto_help(true)`. Fixes applied: Goal, In Scope, Requirements, and Work Procedure narrowed to reflect actual gap (patch `.help` stub output + register `.` alias only).

**Gate Round 2:** 5/5 PASS (4 original agents + 1 fresh challenger) — All dimensions CONVERGED after fixes. Framework investigation confirmed: `.help` stub is hardcoded non-listing, `.command.help` variants are auto-generated, `.` works via semantic analyzer. Task now accurately describes only the implementation gap. FT-03 and FT-04 Test Matrix rows verified consistent with spec.

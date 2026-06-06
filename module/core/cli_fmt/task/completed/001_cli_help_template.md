# Implement CliHelpTemplate in cli_fmt — typed, configurable CLI help renderer

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** ✅ (Completed)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** .
- **Validated By:** author-inline
- **Validation Date:** 2026-05-17

## Goal

Add `CliHelpTemplate`, `CliHelpStyle`, and `CliHelpData` to `cli_fmt` under a new `cli_help_template` feature flag, so that consumers can render column-aligned, ANSI-colored CLI help text from typed structured data without coupling to `data_fmt` (Motivated: `claude_profile::print_usage()` is a hardcoded 45-line renderer that cannot be reused or tested in isolation — a typed template enables reuse, testability, and style customization; Observable: three new public types and one feature flag exist in `cli_fmt`, confirmed by `grep -r "CliHelpTemplate" wtools/dev/module/core/cli_fmt/src/`; Scoped: `cli_fmt` crate only — no changes to `data_fmt`, `claude_profile`, or any other crate; Testable: `cargo nextest run --manifest-path /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml --features cli_help_template 2>&1 | tail -3`).

The current `claude_profile::print_usage()` is a 47-line function in `src/lib.rs` (lines 269–316) that hardcodes column widths, indents, ANSI colors, and content. It cannot be extracted, parameterized, or unit-tested without significant coupling. This task implements the `cli_fmt`-side renderer that will replace it.

The design decision (documented in `wtools/dev/module/core/cli_fmt/docs/feature/002_cli_help_template.md`) is that `CliHelpTemplate` belongs in `cli_fmt`, not `data_fmt`. `data_fmt` is domain-agnostic and operates on `TableView`; `cli_fmt` is CLI-specific. The two crates remain parallel with no cross-dependency.

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/src/help.rs` (to create) — `CliHelpStyle`, `CliHelpData` subtypes, `CliHelpData`, `CliHelpTemplate`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/src/lib.rs` — add `pub mod help;` under `cli_help_template` feature flag; re-export types in `own`, `orphan`, `exposed`, `prelude`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml` — add `cli_help_template = ["enabled"]` feature; add to `full` feature
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/tests/help.rs` (to create) — rendering correctness, TTY detection behavior, column alignment, zero-color mode
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/tests/readme.md` — register `help.rs` in Responsibility Table

## Out of Scope

- `claude_profile` changes — covered by Task `claude_tools/task/claude_profile/141`
- `data_fmt` changes — no coupling; this task must NOT touch data_fmt
- Per-command help rendering — unilang concern; only overview help (non-registry) is in scope
- ANSI width-aware line wrapping — out of scope; descriptions render on one line

## Requirements

- All work must strictly adhere to all applicable rulebooks
  (discover via `kbase .rulebooks`)
- 2-space indentation per code_style.rulebook.md
- All public items must carry `///` doc comments
- No function body exceeds 50 lines
- Feature flag must be opt-in: `cli_help_template = ["enabled"]`; the `full` feature enables it
- Default feature set in Cargo.toml: include `cli_help_template` under `default = [...]` since cli_fmt is a utility crate used with full features by default

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks` from `wtools/dev/module/core/cli_fmt/`; note code_style.rulebook.md requirements for 2-space indent, pub mod placement, and doc comment format.
2. **Read feature doc** — Read `/home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/docs/feature/002_cli_help_template.md` as source of truth for types and rendering algorithm.
3. **Read existing source** — Read `cli_fmt/src/lib.rs` (module structure, namespace pattern), `cli_fmt/src/output.rs` (code style reference), `cli_fmt/tests/output.rs` (test structure reference).
4. **Read print_usage()** — Read `claude_profile/src/lib.rs` lines 269–316 to capture the exact column widths (cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2) and color codes (bold=`"\x1b[1m"`, cyan=`"\x1b[1;36m"`, yell=`"\x1b[33m"`, dim=`"\x1b[2m"`, rst=`"\x1b[0m"`) that `CliHelpStyle::default()` must reproduce.
5. **Write Test Matrix rows** — Populate the Test Matrix below before opening any test file.
6. **Write failing tests** — Create `tests/help.rs` with one test per Test Matrix row. Confirm compile errors (types absent). Register `help.rs` in `tests/readme.md`.
7. **Add feature flag** — Edit `Cargo.toml`: add `cli_help_template = ["enabled"]` to `[features]`; add to `full = [...]` and `default = [...]`.
8. **Implement `src/help.rs`** — `CliHelpStyle`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`, `CliHelpData`, `CliHelpTemplate::new()`, `CliHelpTemplate::render()`. TTY check: `std::io::stdout().is_terminal()`.
9. **Wire into `src/lib.rs`** — Under `#[cfg(feature = "cli_help_template")]` add `pub mod help;` and re-exports in `own`/`orphan`/`exposed`/`prelude`.
10. **Green state** — `cargo nextest run --manifest-path /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml --features cli_help_template` → all tests pass, 0 warnings.
11. **Walk Validation Checklist** — every item must answer YES.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | CliHelpData with 2 groups (2 cmds each), 2 options, 2 examples | CliHelpStyle::default(), tty_detect=false | Output string contains group names, cmd names left-padded to 22 chars (cmd_name_width=20 + col_gap=2), options padded to 20 chars (opt_name_width=18 + col_gap=2), no ANSI codes |
| T02 | CliHelpData identical to T01 | tty_detect=false | No ANSI escape sequences in output (verify with regex `\x1b\[` not found) |
| T03 | CliHelpData identical to T01, TTY simulated via tty_detect=false | tty_detect=false explicitly | Same as T01; renders without colors |
| T04 | cmd_name_width=10 in custom style | 1 group, 1 cmd with 11-char name | Name is NOT truncated — width is a minimum padding, not a hard truncation limit |
| T05 | Empty options vec | CliHelpStyle::default() | No "Options:" section emitted |
| T06 | Empty examples vec | CliHelpStyle::default() | No "Examples:" section emitted |
| T07 | Single group, single command | CliHelpStyle::default(), tty_detect=false | Binary name appears in usage line; group header and command appear; no ANSI |
| T08 | CliHelpStyle::default() field verification | N/A (struct construction only) | cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2 |
| T09 | ExampleEntry with desc=Some("text") and desc=None | CliHelpStyle::default(), tty_detect=false | Output contains "  # text" after example line with desc; plain example line for desc=None; bug_reproducer for emit_examples() desc omission |

## Acceptance Criteria

- `CliHelpStyle`, `CliHelpData`, `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`, `CliHelpTemplate` are all publicly exported from `cli_fmt` under the `cli_help_template` feature
- `CliHelpStyle::default()` produces: cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2 (matches hardcoded values in `claude_profile::print_usage()`)
- `CliHelpTemplate::render()` with `tty_detect=false` produces plain text with no ANSI escape sequences
- `grep -r "data_fmt" /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml` returns empty (no data_fmt dependency introduced)
- `cargo nextest run --manifest-path .../cli_fmt/Cargo.toml --features cli_help_template` → `test result: ok. N passed; 0 failed; 0 ignored`
- `RUSTFLAGS="-D warnings" cargo check --manifest-path .../cli_fmt/Cargo.toml --features cli_help_template` → 0 warnings

## Validation

### Checklist

Desired answer for every question is YES.

**CliHelpStyle**
- [x] C1 — Is `CliHelpStyle` publicly exported under `cli_help_template` feature?
- [x] C2 — Does `CliHelpStyle::default()` have cmd_indent=4, cmd_name_width=20, grp_indent=2, opt_indent=2, opt_name_width=18, col_gap=2, example_indent=2?
- [x] C3 — Do all public fields carry `///` doc comments?

**CliHelpData**
- [x] C4 — Is `CliHelpData` publicly exported with `binary`, `tagline`, `groups`, `options`, `examples` fields?
- [x] C5 — Are `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry` all public and exported?

**CliHelpTemplate**
- [x] C6 — Does `CliHelpTemplate::render()` with `tty_detect=false` produce zero ANSI escape sequences?
- [x] C7 — Does command column padding equal `cmd_name_width + col_gap` characters?
- [x] C8 — Does option column padding equal `opt_name_width + col_gap` characters?
- [x] C9 — Is the "Options:" section absent when `data.options` is empty?
- [x] C10 — Is the "Examples:" section absent when `data.examples` is empty?

**Tests**
- [x] C11 — Does `tests/help.rs` exist with all 9 Test Matrix rows covered (T01–T08 original + T09 bug reproducer)?
- [x] C12 — Is `tests/help.rs` registered in `tests/readme.md`?

**Architecture**
- [x] C13 — Does `cli_fmt/Cargo.toml` have zero mention of `data_fmt`?
- [x] C14 — Is the `cli_help_template` feature registered in `[features]` and included in `full`?

**Out of Scope confirmation**
- [ ] C15 — Are `data_fmt`, `claude_profile`, and `unilang` source files unchanged? *(not statically verifiable without reading those crates)*

### Measurements

- [x] M1 — test suite: `cargo nextest run --manifest-path /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml --features cli_help_template 2>&1 | tail -3` → `test result: ok` *(verified 2026-05-17: 40 passed, 0 failed)*
- [x] M2 — compile clean: `RUSTFLAGS="-D warnings" cargo check --manifest-path /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml --features cli_help_template 2>&1 | tail -3` → `0 warnings` *(verified 2026-05-17: nextest + clippy both clean under -D warnings)*

### Invariants

- [x] I1 — cli_fmt full test suite: `cargo nextest run --manifest-path /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml --all-features` → 0 failures *(verified 2026-05-17: 40 passed, 0 failed; 4 doc tests passed)*
- [x] I2 — no data_fmt dependency: `grep -c "data_fmt" /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml` → `0`

### Anti-faking checks

- [x] AF1 — types are real: `grep -c "pub struct CliHelpTemplate" /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/src/help.rs` → `1`
- [x] AF2 — feature flag wired: `grep -c "cli_help_template" /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/Cargo.toml` → `≥ 2` (feature definition + full entry)
- [x] AF3 — tests not trivial: `grep -c "assert_eq\|assert\|contains" /home/user1/pro/lib/wip_core/wtools/dev/module/core/cli_fmt/tests/help.rs` → `≥ 8`
- [x] AF4 — column padding enforced: `cargo test --manifest-path .../cli_fmt/Cargo.toml --features cli_help_template --test help -- test_column_alignment 2>&1 | grep "ok"` → passes (T01 row) *(verified 2026-05-17: PASS [0.007s] cli_fmt::help test_column_alignment)*

## Outcomes

**Completed:** 2026-05-17

`CliHelpStyle`, `CliHelpData` (with `CommandGroup`, `CommandEntry`, `OptionEntry`, `ExampleEntry`), and `CliHelpTemplate` implemented in `cli_fmt/src/help.rs`. Feature flag `cli_help_template = ["enabled"]` added to `cli_fmt/Cargo.toml`. Module wired into `src/lib.rs` under `#[cfg(feature = "cli_help_template")]`. Test coverage in `tests/help.rs` with 9 cases matching the Test Matrix (T01–T08 + T09 bug reproducer). `cli_fmt` has zero dependency on `data_fmt`.

T09 bug reproducer added post-completion: `emit_examples()` was silently ignoring `ExampleEntry.desc` despite it being documented. Fix applied and verified. All 9 tests pass with zero warnings under `w3 .test l::3`.

### Validation Results

Formal validation run 2026-05-17 via `w3 .test level::3` (local nextest + workspace nextest + doc tests + clippy, all with `-D warnings`):

- **40 integration tests**: all PASS (9 help + 31 output)
- **4 doc tests**: all PASS
- **Clippy**: 0 warnings
- **Total**: 44 tests, 0 failures, 0 warnings

All Checklist items (C1–C14, I2, AF1–AF4 statically; M1, M2, I1, AF4 by test run), Measurements, and Invariants fully verified. C15 remains non-verifiable (requires reading external crates). No `data_fmt` dependency introduced.

## History

- **[2026-05-17]** `CREATED` — Task filed. Goal: implement CliHelpTemplate typed CLI help renderer in cli_fmt.
- **[2026-05-17]** `COMPLETED` — All 9 tests pass (T01–T09 including T09 bug reproducer). Validated via w3 .test level::3.

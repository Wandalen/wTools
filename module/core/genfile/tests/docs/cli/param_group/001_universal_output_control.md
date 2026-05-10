# CLI Spec: Universal Output Control

### Scope

- **Element:** `param_group :: 1. Universal Output Control`
- **Source:** `docs/cli/param_group.md#group--1-universal-output-control`
- **Prefix:** `CC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| CC-01 | verbosity_0_suppresses_all_normal_output | nominal | ✅ |
| CC-02 | verbosity_1_shows_summary_line | nominal | ✅ |
| CC-03 | verbosity_2_shows_per_item_progress | nominal | ✅ |
| CC-04 | every_command_accepts_verbosity | invariant | ✅ |

---

### CC-01: verbosity 0 suppresses all normal output

- **Given:** An archive is loaded
- **When:** Any command is run with `verbosity::0`
- **Then:** Exit code 0; stdout is empty; only errors would appear
- **Tests:** `tests/archive_commands_test.rs`

### CC-02: verbosity 1 shows summary line

- **Given:** An archive is loaded
- **When:** `.archive.save path::<file> verbosity::1` is run (default)
- **Then:** Exit code 0; one summary line on stdout (e.g., `Saved archive to <file>`)
- **Tests:** `tests/archive_commands_test.rs`

### CC-03: verbosity 2 shows per-item progress

- **Given:** An archive with multiple files is loaded
- **When:** `.materialize destination::<dir> verbosity::2` is run
- **Then:** Exit code 0; stdout includes per-file lines and totals
- **Tests:** `tests/materialization_test.rs`

### CC-04: every command accepts verbosity parameter

- **Given:** Any command is invoked
- **When:** `verbosity::0` is appended to any valid command invocation
- **Then:** Exit code 0; command succeeds and respects verbosity setting
- **Tests:** `tests/invariant_test.rs`

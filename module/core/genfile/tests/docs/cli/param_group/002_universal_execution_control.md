# CLI Spec: Universal Execution Control

### Scope

- **Element:** `param_group :: 2. Universal Execution Control`
- **Source:** `docs/cli/param_group.md#group--2-universal-execution-control`
- **Prefix:** `CC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| CC-05 | dry_1_prevents_filesystem_changes | nominal | ✅ |
| CC-06 | dry_0_executes_normally | nominal | ✅ |
| CC-07 | dry_run_output_prefixed_with_dry_run | nominal | ✅ |
| CC-08 | dry_run_exit_code_matches_real_execution | invariant | ✅ |

---

### CC-05: dry 1 prevents filesystem changes

- **Given:** An archive is loaded
- **When:** `.archive.save path::<file> dry::1` is run
- **Then:** Exit code 0; no file is created at the specified path
- **Tests:** `tests/archive_commands_test.rs`

### CC-06: dry 0 executes normally

- **Given:** An archive is loaded
- **When:** `.archive.save path::<file> dry::0` is run (default)
- **Then:** Exit code 0; file is created at the specified path
- **Tests:** `tests/archive_commands_test.rs`

### CC-07: dry run output prefixed with DRY RUN

- **Given:** An archive is loaded with files
- **When:** `.materialize destination::<dir> dry::1 verbosity::2` is run
- **Then:** Exit code 0; all output lines begin with `[DRY RUN]`
- **Tests:** `tests/materialization_test.rs`

### CC-08: dry run exit code matches real execution result

- **Given:** A command that would fail (e.g., missing mandatory value)
- **When:** The command is run with `dry::1`
- **Then:** Exit code 1 (same as real execution would produce); validation errors still reported
- **Tests:** `tests/materialization_test.rs`

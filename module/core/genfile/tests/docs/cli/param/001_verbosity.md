# Parameter Spec: verbosity::

### Scope

- **Element:** `parameter/verbosity`
- **Source:** `docs/cli/param.md#parameter--1-verbosity`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-01 | default_is_normal_output | nominal | ✅ |
| EC-02 | zero_silences_non_error_output | nominal | ✅ |
| EC-03 | two_shows_verbose_detail | nominal | ✅ |
| EC-04 | out_of_range_value_rejected | error | ✅ |
| EC-62 | env_variable_sets_verbosity | nominal | 🚧 |
| EC-63 | five_shows_most_detailed_output | nominal | 🚧 |

---

### EC-01: default is normal output

- **Given:** No `verbosity::` is specified
- **When:** Any command runs (e.g., `.archive.new name::test`)
- **Then:** Exit code 0; one-line summary printed to stdout
- **Tests:** `tests/archive_commands_test.rs`

### EC-02: zero silences non-error output

- **Given:** A valid command with `verbosity::0`
- **When:** `.archive.load path::<file>.json verbosity::0` runs successfully
- **Then:** Exit code 0; no output on stdout (errors still appear on stderr)
- **Tests:** `tests/archive_commands_test.rs`

### EC-03: two shows verbose detail

- **Given:** An archive with multiple files is loaded
- **When:** `.file.list verbosity::2` is run
- **Then:** Exit code 0; per-file metadata (size, mode) shown in addition to summary
- **Tests:** `tests/file_commands_test.rs`

### EC-04: out of range value rejected

- **Given:** A command with `verbosity::6` (value out of 0–5 range)
- **When:** `.archive.new name::test verbosity::6` is run
- **Then:** Exit code 1; error message indicates invalid verbosity value
- **Tests:** `tests/archive_commands_test.rs`

### EC-62: env variable sets verbosity

- **Given:** `GENFILE_VERBOSITY=3` is set in the environment; no `verbosity::` parameter given
- **When:** `.archive.new name::test` is run
- **Then:** Exit code 0; output detail matches verbosity level 3
- **Tests:** `tests/archive_commands_test.rs`

### EC-63: five shows most detailed output

- **Given:** An archive command is run with `verbosity::5`
- **When:** `.archive.new name::test verbosity::5` is run
- **Then:** Exit code 0; maximum debug-level detail shown on stdout
- **Tests:** `tests/archive_commands_test.rs`

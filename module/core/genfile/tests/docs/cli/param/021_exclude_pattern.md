# Parameter Spec: exclude_pattern::

### Scope

- **Element:** `parameter/exclude_pattern`
- **Source:** `docs/cli/param.md#parameter--21-exclude_pattern`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-54 | pattern_removes_matching_files | nominal | ✅ |
| EC-55 | no_pattern_excludes_nothing | nominal | ✅ |
| EC-129 | exclude_applied_after_include | nominal | 🚧 |
| EC-130 | env_variable_sets_exclude_pattern | nominal | 🚧 |
| EC-131 | multi_extension_exclusion_works | nominal | 🚧 |
| EC-132 | invalid_glob_pattern_returns_error | error | 🚧 |

---

### EC-54: pattern removes matching files

- **Given:** Directory `./project` contains `main.rs` and `target/build.rs`
- **When:** `.archive.from_directory source::"./project" exclude_pattern::"**/target/**"` is run
- **Then:** Exit code 0; archive contains `main.rs` but not `target/build.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-55: no pattern excludes nothing

- **Given:** Directory `./project` contains `main.rs` and `target/build.rs`
- **When:** `.archive.from_directory source::"./project"` is run (no `exclude_pattern::`)
- **Then:** Exit code 0; archive contains both files
- **Tests:** `tests/archive_commands_test.rs`

### EC-129: exclude applied after include

- **Given:** Directory `./project` contains `main.rs`, `test.rs`, and `build.rs`
- **When:** `.archive.from_directory source::"./project" include_pattern::"**/*.rs" exclude_pattern::"**/build.rs"` is run
- **Then:** Exit code 0; archive contains `main.rs` and `test.rs` but not `build.rs` (exclude wins)
- **Tests:** `tests/archive_commands_test.rs`

### EC-130: env variable sets exclude pattern

- **Given:** `GENFILE_EXCLUDE_PATTERN=**/target/**` set in environment; no `exclude_pattern::` provided
- **When:** `.archive.from_directory source::"./project"` is run on a directory with a `target/` subdir
- **Then:** Exit code 0; files in `target/` excluded (env var applied as exclude filter)
- **Tests:** `tests/archive_commands_test.rs`

### EC-131: multi extension exclusion works

- **Given:** Directory `./project` contains `main.rs`, `debug.log`, and `temp.tmp`
- **When:** `.archive.from_directory source::"./project" exclude_pattern::"**/*.{log,tmp}"` is run
- **Then:** Exit code 0; archive contains only `main.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-132: invalid glob pattern returns error

- **Given:** A syntactically invalid glob pattern is provided as `exclude_pattern::`
- **When:** `.archive.from_directory source::"./project" exclude_pattern::"[invalid"` is run
- **Then:** Exit code 1; error message indicates the glob pattern is invalid
- **Tests:** `tests/archive_commands_test.rs`

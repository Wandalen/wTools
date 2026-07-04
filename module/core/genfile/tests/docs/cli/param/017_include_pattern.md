# Parameter Spec: include_pattern::

### Scope

- **Element:** `parameter/include_pattern`
- **Source:** `docs/cli/param.md#parameter--17-include_pattern`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-43 | pattern_limits_imported_files | nominal | ✅ |
| EC-44 | no_pattern_imports_all_files | nominal | ✅ |
| EC-116 | multi_extension_pattern_limits_correctly | nominal | 🚧 |
| EC-117 | include_applied_before_exclude | nominal | 🚧 |
| EC-118 | env_variable_sets_include_pattern | nominal | 🚧 |
| EC-119 | invalid_glob_pattern_returns_error | error | 🚧 |

---

### EC-43: pattern limits imported files

- **Given:** Directory `./project` contains `main.rs`, `lib.rs`, and `readme.md`
- **When:** `.archive.from_directory source::"./project" include_pattern::"**/*.rs"` is run
- **Then:** Exit code 0; archive contains `main.rs` and `lib.rs` but not `readme.md`
- **Tests:** `tests/archive_commands_test.rs`

### EC-44: no pattern imports all files

- **Given:** Directory `./project` contains `main.rs`, `lib.rs`, and `readme.md`
- **When:** `.archive.from_directory source::"./project"` is run (no `include_pattern::`)
- **Then:** Exit code 0; archive contains all three files
- **Tests:** `tests/archive_commands_test.rs`

### EC-116: multi extension pattern limits correctly

- **Given:** Directory `./project` contains `main.rs`, `config.toml`, and `readme.md`
- **When:** `.archive.from_directory source::"./project" include_pattern::"**/*.{rs,toml}"` is run
- **Then:** Exit code 0; archive contains `main.rs` and `config.toml` but not `readme.md`
- **Tests:** `tests/archive_commands_test.rs`

### EC-117: include applied before exclude

- **Given:** Directory `./project` contains `main.rs`, `test.rs`, and `build.rs`
- **When:** `.archive.from_directory source::"./project" include_pattern::"**/*.rs" exclude_pattern::"**/build.rs"` is run
- **Then:** Exit code 0; archive contains `main.rs` and `test.rs` but not `build.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-118: env variable sets include pattern

- **Given:** `GENFILE_INCLUDE_PATTERN=**/*.rs` set in environment; no `include_pattern::` provided
- **When:** `.archive.from_directory source::"./project"` is run on a directory with `.rs` and `.md` files
- **Then:** Exit code 0; only `.rs` files included (env var applied as include filter)
- **Tests:** `tests/archive_commands_test.rs`

### EC-119: invalid glob pattern returns error

- **Given:** A syntactically invalid glob pattern is provided
- **When:** `.archive.from_directory source::"./project" include_pattern::"[invalid"` is run
- **Then:** Exit code 1; error message indicates the glob pattern is invalid
- **Tests:** `tests/archive_commands_test.rs`

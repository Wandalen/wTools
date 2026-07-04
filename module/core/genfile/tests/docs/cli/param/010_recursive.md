# Parameter Spec: recursive::

### Scope

- **Element:** `parameter/recursive`
- **Source:** `docs/cli/param.md#parameter--10-recursive`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-27 | recursive_one_includes_subdirs | nominal | ✅ |
| EC-28 | recursive_zero_scans_top_level_only | nominal | ✅ |
| EC-90 | default_is_recursive | nominal | 🚧 |
| EC-91 | invalid_recursive_value_rejected | error | 🚧 |
| EC-92 | env_variable_sets_recursive | nominal | 🚧 |
| EC-93 | recursive_with_empty_subdirectory_skips_subdir | nominal | 🚧 |

---

### EC-27: recursive one includes subdirs

- **Given:** Directory `./src` contains `main.rs` at top level and `util/helper.rs` in a subdirectory
- **When:** `.archive.from_directory source::"./src" recursive::1` is run
- **Then:** Exit code 0; archive contains both `main.rs` and `util/helper.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-28: recursive zero scans top level only

- **Given:** Directory `./src` contains `main.rs` at top level and `util/helper.rs` in a subdirectory
- **When:** `.archive.from_directory source::"./src" recursive::0` is run
- **Then:** Exit code 0; archive contains only `main.rs` (subdirectory files excluded)
- **Tests:** `tests/archive_commands_test.rs`

### EC-90: default is recursive

- **Given:** Directory `./src` contains `main.rs` and `util/helper.rs`; no `recursive::` specified
- **When:** `.archive.from_directory source::"./src"` is run
- **Then:** Exit code 0; archive contains both files (default behavior is recursive)
- **Tests:** `tests/archive_commands_test.rs`

### EC-91: invalid recursive value rejected

- **Given:** A command with `recursive::2` (not a boolean 0/1)
- **When:** `.archive.from_directory source::"./src" recursive::2` is run
- **Then:** Exit code 1; error message indicates invalid value for `recursive::`
- **Tests:** `tests/archive_commands_test.rs`

### EC-92: env variable sets recursive

- **Given:** `GENFILE_RECURSIVE=0` is set in environment; no `recursive::` parameter given
- **When:** `.archive.from_directory source::"./src"` is run on a directory with subdirs
- **Then:** Exit code 0; only top-level files imported (env var disables recursion)
- **Tests:** `tests/archive_commands_test.rs`

### EC-93: recursive with empty subdirectory skips subdir

- **Given:** Directory `./src` contains `main.rs` and an empty subdirectory `empty/`
- **When:** `.archive.from_directory source::"./src" recursive::1` is run
- **Then:** Exit code 0; archive contains `main.rs`; empty directory produces no entries
- **Tests:** `tests/archive_commands_test.rs`

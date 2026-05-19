# CLI Spec: Filesystem Filtering

### Scope

- **Element:** `param_group :: 3. Filesystem Filtering`
- **Source:** `docs/cli/param_group.md#group--3-filesystem-filtering`
- **Prefix:** `CC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| CC-09 | recursive_1_includes_subdirectory_files | nominal | ✅ |
| CC-10 | recursive_0_includes_top_level_only | nominal | ✅ |
| CC-11 | include_pattern_acts_as_whitelist | nominal | ✅ |
| CC-12 | exclude_pattern_acts_as_blacklist | nominal | ✅ |
| CC-13 | include_before_exclude_ordering | invariant | ✅ |

---

### CC-09: recursive 1 includes subdirectory files

- **Given:** A directory with files in both root and subdirectory
- **When:** `.archive.from_directory source::<dir> recursive::1` is run
- **Then:** Exit code 0; archive contains files from both root and subdirectory
- **Tests:** `tests/archive_commands_test.rs`

### CC-10: recursive 0 includes top-level only

- **Given:** A directory with files in both root and subdirectory
- **When:** `.archive.from_directory source::<dir> recursive::0` is run
- **Then:** Exit code 0; archive contains only root-level files; subdirectory files are absent
- **Tests:** `tests/archive_commands_test.rs`

### CC-11: include_pattern acts as whitelist

- **Given:** A directory with `.rs` and `.toml` files
- **When:** `.archive.from_directory source::<dir> include_pattern::"**/*.rs"` is run
- **Then:** Exit code 0; archive contains only `.rs` files
- **Tests:** `tests/archive_commands_test.rs`

### CC-12: exclude_pattern acts as blacklist

- **Given:** A directory with files including a `target/` subdirectory
- **When:** `.archive.from_directory source::<dir> exclude_pattern::"**/target/**"` is run
- **Then:** Exit code 0; archive contains no files from `target/`
- **Tests:** `tests/archive_commands_test.rs`

### CC-13: include pattern applied before exclude pattern

- **Given:** A directory with `.rs` and `.toml` files
- **When:** `.archive.from_directory source::<dir> include_pattern::"**/*.{rs,toml}" exclude_pattern::"**/config.toml"` is run
- **Then:** Exit code 0; `.rs` files and non-config `.toml` files included; `config.toml` excluded
- **Tests:** `tests/archive_commands_test.rs`

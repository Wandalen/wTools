# Parameter Spec: exclude_pattern::

### Scope

- **Element:** `parameter/exclude_pattern`
- **Source:** `docs/cli/param.md#parameter--21-exclude_pattern`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-54 | pattern_removes_matching_files | nominal | ✅ |
| EC-55 | no_pattern_excludes_nothing | nominal | ✅ |

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

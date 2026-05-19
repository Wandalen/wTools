# Parameter Spec: include_pattern::

### Scope

- **Element:** `parameter/include_pattern`
- **Source:** `docs/cli/param.md#parameter--17-include_pattern`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-43 | pattern_limits_imported_files | nominal | ✅ |
| EC-44 | no_pattern_imports_all_files | nominal | ✅ |

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

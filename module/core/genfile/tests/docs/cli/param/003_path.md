# Parameter Spec: path::

### Scope

- **Element:** `parameter/path`
- **Source:** `docs/cli/param.md#parameter--3-path`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-08 | existing_json_file_loaded | nominal | ✅ |
| EC-09 | existing_yaml_file_loaded | nominal | ✅ |
| EC-10 | nonexistent_file_returns_exit_one | error | ✅ |
| EC-67 | path_semantics_differ_by_command_context | Behavioral Divergence | 🚧 |
| EC-68 | path_with_nested_subdir_creates_archive_structure | nominal | 🚧 |
| EC-69 | missing_parent_dir_for_save_path_causes_error | error | 🚧 |

---

### EC-08: existing JSON file loaded

- **Given:** A valid `.json` archive file exists at the specified path
- **When:** `.archive.load path::"template.json"` is run
- **Then:** Exit code 0; archive loaded into session state
- **Tests:** `tests/archive_commands_test.rs`

### EC-09: existing YAML file loaded

- **Given:** A valid `.yaml` archive file exists at the specified path
- **When:** `.archive.load path::"template.yaml"` is run
- **Then:** Exit code 0; archive loaded, format auto-detected from extension
- **Tests:** `tests/archive_commands_test.rs`

### EC-10: nonexistent file returns exit one

- **Given:** No file exists at the specified path
- **When:** `.archive.load path::"missing.json"` is run
- **Then:** Exit code 1; error message indicates file not found
- **Tests:** `tests/archive_commands_test.rs`

### EC-67: path semantics differ by command context

- **Given:** Same `path::` parameter value used in two commands
- **When:** `.archive.load path::"archive.json"` checks disk existence; `.file.add path::"src/main.rs"` addresses a path inside the archive
- **Then:** In `.archive.load` context exit 1 if file missing; in `.file.add` context the path is an in-archive key (no disk check)
- **Tests:** `tests/archive_commands_test.rs`

### EC-68: path with nested subdir creates archive structure

- **Given:** An archive is loaded in session state
- **When:** `.file.add path::"src/lib/utils.rs" content::"fn foo() {}"` is run
- **Then:** Exit code 0; archive entry key is `src/lib/utils.rs` (nested path preserved)
- **Tests:** `tests/file_commands_test.rs`

### EC-69: missing parent dir for save path causes error

- **Given:** Parent directory `nonexistent/subdir/` does not exist on disk
- **When:** `.archive.save path::"nonexistent/subdir/out.json"` is run
- **Then:** Exit code 1; error message indicates parent directory does not exist
- **Tests:** `tests/archive_commands_test.rs`

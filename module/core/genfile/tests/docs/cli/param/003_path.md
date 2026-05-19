# Parameter Spec: path::

### Scope

- **Element:** `parameter/path`
- **Source:** `docs/cli/param.md#parameter--3-path`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-08 | existing_json_file_loaded | nominal | ✅ |
| EC-09 | existing_yaml_file_loaded | nominal | ✅ |
| EC-10 | nonexistent_file_returns_exit_one | error | ✅ |

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

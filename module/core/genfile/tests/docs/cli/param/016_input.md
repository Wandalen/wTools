# Parameter Spec: input::

### Scope

- **Element:** `parameter/input`
- **Source:** `docs/cli/param.md#parameter--16-input`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-41 | existing_input_dir_scanned | nominal | ✅ |
| EC-42 | nonexistent_input_returns_exit_one | error | ✅ |

---

### EC-41: existing input dir scanned

- **Given:** Directory `./templates` with files exists
- **When:** `.pack input::"./templates" output::"archive.json"` is run
- **Then:** Exit code 0; `archive.json` contains all files from `./templates`
- **Tests:** `tests/archive_commands_test.rs`

### EC-42: nonexistent input returns exit one

- **Given:** Directory `./missing` does not exist
- **When:** `.pack input::"./missing" output::"archive.json"` is run
- **Then:** Exit code 1; error message indicates input directory not found
- **Tests:** `tests/archive_commands_test.rs`

# Parameter Spec: input::

### Scope

- **Element:** `parameter/input`
- **Source:** `docs/cli/param.md#parameter--16-input`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-41 | existing_input_dir_scanned | nominal | ✅ |
| EC-42 | nonexistent_input_returns_exit_one | error | ✅ |
| EC-112 | input_required | error | 🚧 |
| EC-113 | input_as_file_not_directory_returns_error | error | 🚧 |
| EC-114 | input_subdirectories_included_recursively | nominal | 🚧 |
| EC-115 | input_with_nested_empty_subdirs_succeeds | nominal | 🚧 |

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

### EC-112: input required

- **Given:** No `input::` parameter is provided
- **When:** `.pack output::"archive.json"` is run without `input::`
- **Then:** Exit code 1; error message indicates `input::` is required
- **Tests:** `tests/archive_commands_test.rs`

### EC-113: input as file not directory returns error

- **Given:** Path `./archive.json` points to a regular file (not a directory)
- **When:** `.pack input::"./archive.json" output::"out.json"` is run
- **Then:** Exit code 1; error message indicates input must be a directory
- **Tests:** `tests/archive_commands_test.rs`

### EC-114: input subdirectories included recursively

- **Given:** Directory `./templates` contains `main.rs` and `util/helper.rs`
- **When:** `.pack input::"./templates" output::"archive.json"` is run
- **Then:** Exit code 0; archive contains both `main.rs` and `util/helper.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-115: input with nested empty subdirs succeeds

- **Given:** Directory `./templates` contains `main.rs` and an empty subdirectory `empty/`
- **When:** `.pack input::"./templates" output::"archive.json"` is run
- **Then:** Exit code 0; archive contains `main.rs`; empty directory produces no archive entries
- **Tests:** `tests/archive_commands_test.rs`

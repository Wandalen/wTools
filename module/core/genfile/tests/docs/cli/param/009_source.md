# Parameter Spec: source::

### Scope

- **Element:** `parameter/source`
- **Source:** `docs/cli/param.md#parameter--9-source`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-25 | existing_directory_imported | nominal | ✅ |
| EC-26 | nonexistent_directory_returns_exit_one | error | ✅ |
| EC-86 | source_must_be_directory_not_file | error | 🚧 |
| EC-87 | source_is_required | error | 🚧 |
| EC-88 | source_with_nested_dirs_imports_recursively | nominal | 🚧 |
| EC-89 | source_with_only_subdirs_imports_contents | nominal | 🚧 |

---

### EC-25: existing directory imported

- **Given:** A directory `./templates` contains two text files
- **When:** `.archive.from_directory source::"./templates"` is run
- **Then:** Exit code 0; archive contains both files from the directory
- **Tests:** `tests/archive_commands_test.rs`

### EC-26: nonexistent directory returns exit one

- **Given:** The path `./missing` does not exist
- **When:** `.archive.from_directory source::"./missing"` is run
- **Then:** Exit code 1; error message indicates directory not found
- **Tests:** `tests/archive_commands_test.rs`

### EC-86: source must be directory not file

- **Given:** A regular file `template.json` exists (not a directory)
- **When:** `.archive.from_directory source::"template.json"` is run
- **Then:** Exit code 1; error message indicates source must be a directory
- **Tests:** `tests/archive_commands_test.rs`

### EC-87: source is required

- **Given:** No `source::` parameter is provided
- **When:** `.archive.from_directory` is run without `source::`
- **Then:** Exit code 1; error message indicates `source::` is required
- **Tests:** `tests/archive_commands_test.rs`

### EC-88: source with nested dirs imports recursively

- **Given:** Directory `./project` contains `src/main.rs` and `src/lib.rs` in a subdirectory
- **When:** `.archive.from_directory source::"./project"` is run
- **Then:** Exit code 0; archive contains `src/main.rs` and `src/lib.rs` with nested paths preserved
- **Tests:** `tests/archive_commands_test.rs`

### EC-89: source with only subdirs imports contents

- **Given:** Directory `./templates` contains only subdirectories (no root-level files)
- **When:** `.archive.from_directory source::"./templates"` is run
- **Then:** Exit code 0; archive contains files from subdirectories with relative paths
- **Tests:** `tests/archive_commands_test.rs`

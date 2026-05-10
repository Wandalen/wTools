# Parameter Spec: source::

### Scope

- **Element:** `parameter/source`
- **Source:** `docs/cli/param.md#parameter--9-source`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-25 | existing_directory_imported | nominal | ✅ |
| EC-26 | nonexistent_directory_returns_exit_one | error | ✅ |

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

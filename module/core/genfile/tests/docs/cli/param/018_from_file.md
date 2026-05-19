# Parameter Spec: from_file::

### Scope

- **Element:** `parameter/from_file`
- **Source:** `docs/cli/param.md#parameter--18-from_file`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-45 | source_file_content_embedded | nominal | ✅ |
| EC-46 | nonexistent_source_file_returns_error | error | ✅ |
| EC-47 | content_and_from_file_are_mutually_exclusive | error | ✅ |

---

### EC-45: source file content embedded

- **Given:** A file `./README.md` exists with content `"# Hello"`
- **When:** `.file.add path::"readme.md" from_file::"./README.md"` is run
- **Then:** Exit code 0; archive entry `readme.md` contains `"# Hello"`
- **Tests:** `tests/file_commands_test.rs`

### EC-46: nonexistent source file returns error

- **Given:** File `./missing.md` does not exist
- **When:** `.file.add path::"readme.md" from_file::"./missing.md"` is run
- **Then:** Exit code 2; error message indicates source file not found
- **Tests:** `tests/file_commands_test.rs`

### EC-47: content and from_file are mutually exclusive

- **Given:** A valid source file and inline content string
- **When:** `.file.add path::"readme.md" content::"text" from_file::"./README.md"` is run
- **Then:** Exit code 1; error message indicates `content::` and `from_file::` cannot both be specified
- **Tests:** `tests/file_commands_test.rs`

# Parameter Spec: from_file::

### Scope

- **Element:** `parameter/from_file`
- **Source:** `docs/cli/param.md#parameter--18-from_file`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-45 | source_file_content_embedded | nominal | ✅ |
| EC-46 | nonexistent_source_file_returns_error | error | ✅ |
| EC-47 | content_and_from_file_are_mutually_exclusive | error | ✅ |
| EC-120 | binary_file_embedded_without_error | Behavioral Divergence | 🚧 |
| EC-121 | from_file_default_is_null | nominal | 🚧 |
| EC-122 | from_file_with_write_mode_skip_does_not_read_file | nominal | 🚧 |

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

### EC-120: binary file embedded without error

- **Given:** A binary file `./image.png` exists on disk
- **When:** `.file.add path::"assets/image.png" from_file::"./image.png"` is run
- **Then:** Exit code 0; archive entry contains binary content stored without modification or error
- **Tests:** `tests/file_commands_test.rs`

### EC-121: from file default is null

- **Given:** `.file.add path::"main.rs" content::"fn main() {}"` is run without `from_file::`
- **When:** Archive entry for `main.rs` is inspected
- **Then:** Exit code 0; `content::` value used; `from_file` field is absent/null in the archive
- **Tests:** `tests/file_commands_test.rs`

### EC-122: from file with write mode skip does not read file

- **Given:** Archive already has `main.rs` with content; source file `./main.rs` exists
- **When:** `.file.add path::"main.rs" from_file::"./main.rs" write_mode::skip` is run
- **Then:** Exit code 0; existing archive entry preserved; source file not read (skip applies before file read)
- **Tests:** `tests/file_commands_test.rs`

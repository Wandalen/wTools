# Parameter Spec: content::

### Scope

- **Element:** `parameter/content`
- **Source:** `docs/cli/param.md#parameter--23-content`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-59 | content_string_stored_as_inline | nominal | 🚧 |
| EC-60 | content_and_from_file_mutually_exclusive | error | 🚧 |
| EC-61 | empty_string_content_accepted | nominal | 🚧 |

---

### EC-59: content string stored as inline file

- **Given:** An archive is loaded
- **When:** `.file.add path::"main.rs" content::"fn main() {}"` is run
- **Then:** Exit code 0; archive has `main.rs` with content `fn main() {}`; mode is inline
- **Tests:** `tests/file_commands_test.rs`

### EC-60: content and from_file are mutually exclusive

- **Given:** An archive is loaded; a source file exists
- **When:** `.file.add path::"a.rs" content::"x" from_file::"./a.rs"` is run
- **Then:** Exit code 1; error message indicates `content::` and `from_file::` cannot both be specified
- **Tests:** `tests/file_commands_test.rs`

### EC-61: empty string content accepted

- **Given:** An archive is loaded
- **When:** `.file.add path::"empty.txt" content::""` is run
- **Then:** Exit code 0; `empty.txt` added with zero-byte inline content
- **Tests:** `tests/file_commands_test.rs`

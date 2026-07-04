# Parameter Spec: content::

### Scope

- **Element:** `parameter/content`
- **Source:** `docs/cli/param.md#parameter--23-content`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-59 | content_string_stored_as_inline | nominal | 🚧 |
| EC-60 | content_and_from_file_mutually_exclusive | error | 🚧 |
| EC-61 | empty_string_content_accepted | nominal | 🚧 |
| EC-136 | content_with_template_placeholder_stored_verbatim | nominal | 🚧 |
| EC-137 | multiline_content_accepted | nominal | 🚧 |
| EC-138 | content_is_required_when_from_file_not_provided | error | 🚧 |

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

### EC-136: content with template placeholder stored verbatim

- **Given:** An archive is loaded
- **When:** `.file.add path::"template.rs" content::"let x = {{value}};"` is run
- **Then:** Exit code 0; archive stores `{{value}}` literally — not expanded at store time
- **Tests:** `tests/file_commands_test.rs`

### EC-137: multiline content accepted

- **Given:** An archive is loaded
- **When:** `.file.add path::"script.sh" content::"#!/bin/bash\necho hello\n"` is run
- **Then:** Exit code 0; archive stores content with embedded newlines intact
- **Tests:** `tests/file_commands_test.rs`

### EC-138: content is required when from file not provided

- **Given:** An archive is loaded; no `from_file::` is provided
- **When:** `.file.add path::"main.rs"` is run without `content::` or `from_file::`
- **Then:** Exit code 1; error message indicates either `content::` or `from_file::` is required
- **Tests:** `tests/file_commands_test.rs`

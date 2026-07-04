# Parameter Spec: write_mode::

### Scope

- **Element:** `parameter/write_mode`
- **Source:** `docs/cli/param.md#parameter--7-write_mode`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-19 | rewrite_overwrites_existing_file | nominal | ✅ |
| EC-20 | skip_leaves_existing_file_unchanged | nominal | ✅ |
| EC-21 | append_adds_to_existing_content | nominal | ✅ |
| EC-80 | default_write_mode_is_rewrite | nominal | 🚧 |
| EC-81 | case_insensitive_value_accepted | nominal | 🚧 |
| EC-82 | invalid_write_mode_rejected | error | 🚧 |

---

### EC-19: rewrite overwrites existing file

- **Given:** An archive already contains a file at `"main.rs"` with content A
- **When:** `.file.add path::"main.rs" content::"B" write_mode::rewrite` is run
- **Then:** Exit code 0; file `"main.rs"` in archive now contains content B
- **Tests:** `tests/file_commands_test.rs`

### EC-20: skip leaves existing file unchanged

- **Given:** An archive already contains a file at `"main.rs"` with content A
- **When:** `.file.add path::"main.rs" content::"B" write_mode::skip` is run
- **Then:** Exit code 0; file `"main.rs"` in archive still contains content A; output indicates skipped
- **Tests:** `tests/file_commands_test.rs`

### EC-21: append adds to existing content

- **Given:** An archive contains a file at `"config.txt"` with content `"line1\n"`
- **When:** `.file.add path::"config.txt" content::"line2\n" write_mode::append` is run
- **Then:** Exit code 0; file content is `"line1\nline2\n"`
- **Tests:** `tests/file_commands_test.rs`

### EC-80: default write mode is rewrite

- **Given:** An archive contains a file at `"main.rs"` with content A; no `write_mode::` specified
- **When:** `.file.add path::"main.rs" content::"B"` is run
- **Then:** Exit code 0; file content replaced with B (default is `rewrite`)
- **Tests:** `tests/file_commands_test.rs`

### EC-81: case insensitive value accepted

- **Given:** An archive contains a file at `"main.rs"`
- **When:** `.file.add path::"main.rs" content::"B" write_mode::SKIP` is run
- **Then:** Exit code 0; skip behavior applied (value matched case-insensitively)
- **Tests:** `tests/file_commands_test.rs`

### EC-82: invalid write mode rejected

- **Given:** A command with unrecognized `write_mode::` value
- **When:** `.file.add path::"main.rs" content::"B" write_mode::merge` is run
- **Then:** Exit code 1; error message indicates `merge` is not a valid write mode
- **Tests:** `tests/file_commands_test.rs`

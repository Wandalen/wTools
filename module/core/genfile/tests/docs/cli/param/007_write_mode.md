# Parameter Spec: write_mode::

### Scope

- **Element:** `parameter/write_mode`
- **Source:** `docs/cli/param.md#parameter--7-write_mode`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-19 | rewrite_overwrites_existing_file | nominal | ✅ |
| EC-20 | skip_leaves_existing_file_unchanged | nominal | ✅ |
| EC-21 | append_adds_to_existing_content | nominal | ✅ |

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

# CLI Spec: File Commands

### Scope

- **Element:** Commands `12–15` (`.file.*` namespace)
- **Source:** `docs/cli/command/file.md`
- **Prefix:** `IT-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-22 | file_add_with_inline_content | nominal | ✅ |
| IT-23 | file_add_from_filesystem_source | nominal | ✅ |
| IT-24 | file_add_write_mode_skip_no_overwrite | nominal | ✅ |
| IT-25 | file_remove_deletes_existing_file | nominal | ✅ |
| IT-26 | file_remove_nonexistent_exits_1 | error | ✅ |
| IT-27 | file_list_shows_all_files | nominal | ✅ |
| IT-28 | file_show_displays_content_with_placeholders | nominal | ✅ |

---

### IT-22: file.add with inline content

- **Given:** An archive is loaded or created
- **When:** `.file.add path::"main.rs" content::"fn main() {}"` is run
- **Then:** Exit code 0; archive now contains `main.rs` with the specified content
- **Tests:** `tests/file_commands_test.rs`

### IT-23: file.add from filesystem source

- **Given:** A file `readme.md` exists on disk; archive is loaded
- **When:** `.file.add path::"readme.md" from_file::"./readme.md"` is run
- **Then:** Exit code 0; archive contains `readme.md` with content read from disk
- **Tests:** `tests/file_commands_test.rs`

### IT-24: file.add write_mode skip does not overwrite

- **Given:** Archive has `config.toml`; another `.file.add` with same path
- **When:** `.file.add path::"config.toml" content::"new" write_mode::skip` is run
- **Then:** Exit code 0; original `config.toml` content unchanged; output indicates skip
- **Tests:** `tests/file_commands_test.rs`

### IT-25: file.remove deletes existing file

- **Given:** Archive contains `temp.rs`
- **When:** `.file.remove path::"temp.rs"` is run
- **Then:** Exit code 0; archive no longer contains `temp.rs`
- **Tests:** `tests/file_commands_test.rs`

### IT-26: file.remove nonexistent file exits 1

- **Given:** Archive does not contain `ghost.rs`
- **When:** `.file.remove path::"ghost.rs"` is run
- **Then:** Exit code 1; error message on stderr mentions file not found
- **Tests:** `tests/file_commands_test.rs`

### IT-27: file.list shows all files with metadata

- **Given:** Archive with 3 files (mix of inline and reference)
- **When:** `.file.list` is run
- **Then:** Exit code 0; all 3 file paths listed; content mode shown for each
- **Tests:** `tests/file_commands_test.rs`

### IT-28: file.show displays content with placeholders intact

- **Given:** Archive contains `main.rs` with `{{project_name}}` placeholder
- **When:** `.file.show path::"main.rs"` is run
- **Then:** Exit code 0; output contains `{{project_name}}` verbatim (no substitution)
- **Tests:** `tests/file_commands_test.rs`

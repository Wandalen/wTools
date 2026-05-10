# Feature Spec: File Content Operations

### Scope

- **Element:** `feature/002_file_content_operations`
- **Source:** `docs/feature/002_file_content_operations.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | add_stores_text_file_in_archive | nominal | ✅ |
| FT-02 | remove_deletes_file_from_archive | nominal | ✅ |
| FT-03 | list_shows_all_files | nominal | ✅ |
| FT-04 | show_displays_text_content | nominal | ✅ |
| FT-05 | add_binary_file_safely_embedded | nominal | ✅ |
| FT-06 | remove_nonexistent_file_produces_error | error | ✅ |

---

### FT-01: add stores text file in archive

- **Given:** An archive is loaded in session state
- **When:** `.file.add path::hello.txt content::hello` is run
- **Then:** Exit code 0; `.file.list` shows `hello.txt` in the archive
- **Tests:** `tests/file_commands_test.rs`

### FT-02: remove deletes file from archive

- **Given:** An archive with `hello.txt` is loaded
- **When:** `.file.remove path::hello.txt` is run
- **Then:** Exit code 0; `.file.list` no longer shows `hello.txt`
- **Tests:** `tests/file_commands_test.rs`

### FT-03: list shows all files

- **Given:** An archive with two files `a.txt` and `b.txt` is loaded
- **When:** `.file.list` is run
- **Then:** Exit code 0; output contains both `a.txt` and `b.txt`
- **Tests:** `tests/file_commands_test.rs`

### FT-04: show displays text content

- **Given:** An archive with `hello.txt` containing `Hello World` is loaded
- **When:** `.file.show path::hello.txt` is run
- **Then:** Exit code 0; output contains `Hello World`
- **Tests:** `tests/file_commands_test.rs`

### FT-05: add binary file safely embedded

- **Given:** An archive is loaded; a binary file (e.g. a small PNG) exists on disk
- **When:** `.file.add path::logo.png source::<path>` is run
- **Then:** Exit code 0; file is stored and `.file.show path::logo.png` indicates binary
- **Tests:** `tests/file_commands_test.rs`

### FT-06: remove nonexistent file produces error

- **Given:** An archive is loaded; no file `ghost.txt` exists in it
- **When:** `.file.remove path::ghost.txt` is run
- **Then:** Exit code 1; error message indicates file not found in archive
- **Tests:** `tests/file_commands_test.rs`

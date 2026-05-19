# Feature Spec: Archive Lifecycle Management

### Scope

- **Element:** `feature/001_archive_lifecycle_management`
- **Source:** `docs/feature/001_archive_lifecycle_management.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | new_creates_empty_archive_with_name | nominal | ✅ |
| FT-02 | load_reads_json_archive_by_extension | nominal | ✅ |
| FT-03 | load_reads_yaml_archive_by_extension | nominal | ✅ |
| FT-04 | save_writes_archive_to_disk | nominal | ✅ |
| FT-05 | from_directory_inlines_all_files | nominal | ✅ |
| FT-06 | load_nonexistent_file_produces_error | error | ✅ |

---

### FT-01: new creates empty archive with name

- **Given:** No archive is loaded
- **When:** `.archive.new name::test-project` is run
- **Then:** Exit code 0; archive named `test-project` is created in session state
- **Tests:** `tests/archive_commands_test.rs`

### FT-02: load reads JSON archive by extension

- **Given:** A `.json` archive file exists on disk
- **When:** `.archive.load path::<file>.json` is run
- **Then:** Exit code 0; archive is loaded into session state with correct name
- **Tests:** `tests/archive_commands_test.rs`

### FT-03: load reads YAML archive by extension

- **Given:** A `.yaml` archive file exists on disk
- **When:** `.archive.load path::<file>.yaml` is run
- **Then:** Exit code 0; archive is loaded and format auto-detected from extension
- **Tests:** `tests/archive_commands_test.rs`

### FT-04: save writes archive to disk

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::<file>.json` is run
- **Then:** Exit code 0; file appears on disk at the specified path
- **Tests:** `tests/archive_commands_test.rs`

### FT-05: from_directory inlines all files

- **Given:** A directory with two text files exists
- **When:** `.archive.from_directory source::<dir>` is run
- **Then:** Exit code 0; archive in session has two files with inline content
- **Tests:** `tests/archive_commands_test.rs`

### FT-06: load nonexistent file produces error

- **Given:** No file exists at the specified path
- **When:** `.archive.load path::/nonexistent.json` is run
- **Then:** Exit code 1; error message indicates file not found
- **Tests:** `tests/archive_commands_test.rs`

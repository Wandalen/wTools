# CLI Spec: Archive Commands

### Scope

- **Element:** Commands `5–8` (`.archive.*` namespace)
- **Source:** `docs/cli/command/archive.md`
- **Prefix:** `IT-`
- **Minimum cases:** 8

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-01 | archive_new_creates_named_archive | nominal | ✅ |
| IT-02 | archive_new_with_description | nominal | ✅ |
| IT-03 | archive_load_json_by_extension | nominal | ✅ |
| IT-04 | archive_load_yaml_by_extension | nominal | ✅ |
| IT-05 | archive_load_nonexistent_file_exits_1 | error | ✅ |
| IT-06 | archive_save_writes_json_by_extension | nominal | ✅ |
| IT-07 | archive_save_writes_yaml_with_format_param | nominal | ✅ |
| IT-08 | archive_save_dry_run_creates_no_file | nominal | ✅ |
| IT-09 | archive_from_directory_imports_all_files | nominal | ✅ |
| IT-10 | archive_from_directory_respects_include_pattern | nominal | ✅ |

---

### IT-01: archive.new creates named archive

- **Given:** No archive loaded
- **When:** `.archive.new name::my-template` is run
- **Then:** Exit code 0; summary confirms archive named `my-template`; 0 files, 0 parameters
- **Tests:** `tests/archive_commands_test.rs`

### IT-02: archive.new with description

- **Given:** No archive loaded
- **When:** `.archive.new name::my-template description::"A test template"` is run
- **Then:** Exit code 0; archive created; description set
- **Tests:** `tests/archive_commands_test.rs`

### IT-03: archive.load reads JSON by extension

- **Given:** A valid `.json` archive file exists on disk
- **When:** `.archive.load path::<file>.json` is run
- **Then:** Exit code 0; archive loaded with correct name and file count
- **Tests:** `tests/archive_commands_test.rs`

### IT-04: archive.load reads YAML by extension

- **Given:** A valid `.yaml` archive file exists on disk
- **When:** `.archive.load path::<file>.yaml` is run
- **Then:** Exit code 0; archive loaded; format auto-detected from extension
- **Tests:** `tests/archive_commands_test.rs`

### IT-05: archive.load nonexistent file exits 1

- **Given:** No file at the specified path
- **When:** `.archive.load path::/nonexistent.json` is run
- **Then:** Exit code 1; error message on stderr mentions file not found
- **Tests:** `tests/archive_commands_test.rs`

### IT-06: archive.save writes JSON by extension

- **Given:** An archive is loaded in session
- **When:** `.archive.save path::<file>.json` is run
- **Then:** Exit code 0; `.json` file appears on disk with valid JSON structure
- **Tests:** `tests/archive_commands_test.rs`

### IT-07: archive.save writes YAML with format parameter

- **Given:** An archive is loaded in session
- **When:** `.archive.save path::<file>.json format::yaml` is run
- **Then:** Exit code 0; file written as YAML despite `.json` extension (format:: overrides)
- **Tests:** `tests/archive_commands_test.rs`

### IT-08: archive.save dry run creates no file

- **Given:** An archive is loaded in session
- **When:** `.archive.save path::<file>.json dry::1` is run
- **Then:** Exit code 0; no file created at path; output contains `[DRY RUN]`
- **Tests:** `tests/archive_commands_test.rs`

### IT-09: archive.from_directory imports all files

- **Given:** A directory with 3 files
- **When:** `.archive.from_directory source::<dir>` is run
- **Then:** Exit code 0; archive in session has 3 files
- **Tests:** `tests/archive_commands_test.rs`

### IT-10: archive.from_directory respects include_pattern

- **Given:** A directory with `.rs` and `.md` files
- **When:** `.archive.from_directory source::<dir> include_pattern::"**/*.rs"` is run
- **Then:** Exit code 0; archive contains only `.rs` files
- **Tests:** `tests/archive_commands_test.rs`

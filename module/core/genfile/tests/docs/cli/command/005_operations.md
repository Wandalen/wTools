# CLI Spec: Core Operations Commands

### Scope

- **Element:** Commands `16–18` (`.materialize`, `.unpack`, `.pack`)
- **Source:** `docs/cli/command/operations.md`
- **Prefix:** `IT-`
- **Minimum cases:** 8

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-29 | materialize_substitutes_placeholders | nominal | ✅ |
| IT-30 | materialize_fails_when_mandatory_missing | error | ✅ |
| IT-31 | materialize_dry_run_no_files_created | nominal | ✅ |
| IT-32 | unpack_preserves_placeholders | nominal | ✅ |
| IT-33 | pack_creates_self_contained_archive | nominal | ✅ |
| IT-34 | pack_dry_run_writes_no_file | nominal | ✅ |
| IT-53 | materialize_with_default_value_used_when_no_explicit_value | nominal | 🚧 |
| IT-54 | unpack_creates_destination_if_not_exists | nominal | 🚧 |
| IT-59 | pack_internalizes_external_file_refs_before_saving | nominal | 🔶 deferred |

---

### IT-29: materialize substitutes placeholders

- **Given:** Archive with `{{project_name}}` in `main.rs`; value set to `my-app`
- **When:** `.materialize destination::<dir>` is run
- **Then:** Exit code 0; `<dir>/main.rs` exists with `my-app` where `{{project_name}}` was
- **Tests:** `tests/materialization_test.rs`

### IT-30: materialize fails when mandatory value missing

- **Given:** Archive with mandatory parameter `project_name`; no value set
- **When:** `.materialize destination::<dir>` is run
- **Then:** Exit code 1; error message identifies `project_name` as missing
- **Tests:** `tests/materialization_test.rs`

### IT-31: materialize dry run creates no files

- **Given:** Archive with all values set
- **When:** `.materialize destination::<dir> dry::1` is run
- **Then:** Exit code 0; no files in `<dir>`; output contains `[DRY RUN]`
- **Tests:** `tests/materialization_test.rs`

### IT-32: unpack preserves placeholders verbatim

- **Given:** Archive with `{{project_name}}` in `main.rs`
- **When:** `.unpack destination::<dir>` is run
- **Then:** Exit code 0; `<dir>/main.rs` contains `{{project_name}}` unchanged
- **Tests:** `tests/materialization_test.rs`

### IT-33: pack creates self-contained archive from directory

- **Given:** A directory with 3 files
- **When:** `.pack input::<dir> output::<file>.json` is run
- **Then:** Exit code 0; `<file>.json` exists; all 3 files embedded inline in JSON
- **Tests:** `tests/archive_commands_test.rs`

### IT-34: pack dry run writes no archive file

- **Given:** A directory with files
- **When:** `.pack input::<dir> output::<file>.json dry::1` is run
- **Then:** Exit code 0; no file created at `<file>.json`; output contains `[DRY RUN]`
- **Tests:** `tests/archive_commands_test.rs`

### IT-53: materialize with default value used when no explicit value

- **Given:** Archive with parameter `port` (default `"3000"`); no explicit `.value.set` called
- **When:** `.materialize destination::<dir>` is run; archive file contains `{{port}}`
- **Then:** Exit code 0; output file contains `3000` (default substituted)
- **Tests:** `tests/materialization_test.rs`

### IT-54: unpack creates destination if not exists

- **Given:** Archive with files; destination directory `./new_dir` does not exist
- **When:** `.unpack destination::"./new_dir"` is run
- **Then:** Exit code 0; `./new_dir/` is created; files written with placeholders intact
- **Tests:** `tests/materialization_test.rs`

### IT-59: pack internalizes external file references before saving

- **Given:** An archive is loaded that contains `ContentSource::File` references to external paths (created via `.archive.from_directory mode::reference`)
- **When:** `.pack output::<file>.json` is run
- **Then:** Exit code 0; `<file>.json` contains all file content inline; no external path references remain in the output
- **Tests:** none — see task/004_implement_deferred_source_behaviors.md

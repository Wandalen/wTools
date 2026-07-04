# Parameter Spec: destination::

### Scope

- **Element:** `parameter/destination`
- **Source:** `docs/cli/param.md#parameter--5-destination`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-14 | output_written_to_destination | nominal | ✅ |
| EC-15 | nonexistent_destination_created | nominal | ✅ |
| EC-16 | destination_is_required | error | ✅ |
| EC-73 | unpack_also_uses_destination | nominal | 🚧 |
| EC-74 | destination_with_path_traversal_rejected | error | 🚧 |
| EC-75 | nested_destination_creates_parent_dirs | nominal | 🚧 |

---

### EC-14: output written to destination

- **Given:** An archive with files is loaded; destination directory exists
- **When:** `.materialize destination::"./output"` is run
- **Then:** Exit code 0; files appear in `./output/` directory
- **Tests:** `tests/materialization_test.rs`

### EC-15: nonexistent destination created

- **Given:** An archive is loaded; destination directory does not exist
- **When:** `.materialize destination::"./new_dir"` is run
- **Then:** Exit code 0; `./new_dir/` is created and files are written into it
- **Tests:** `tests/materialization_test.rs`

### EC-16: destination is required

- **Given:** An archive is loaded
- **When:** `.materialize` is run without `destination::`
- **Then:** Exit code 1; error message indicates missing required parameter
- **Tests:** `tests/materialization_test.rs`

### EC-73: unpack also uses destination

- **Given:** A packed archive file exists
- **When:** `.unpack input::"archive.json" destination::"./out"` is run
- **Then:** Exit code 0; archive contents written to `./out/`
- **Tests:** `tests/archive_commands_test.rs`

### EC-74: destination with path traversal rejected

- **Given:** An archive is loaded
- **When:** `.materialize destination::"../../etc"` is run
- **Then:** Exit code 1; error message indicates path traversal is not allowed
- **Tests:** `tests/materialization_test.rs`

### EC-75: nested destination creates parent dirs

- **Given:** An archive is loaded; `./a/b/c` does not exist
- **When:** `.materialize destination::"./a/b/c"` is run
- **Then:** Exit code 0; all parent directories `a/`, `a/b/`, `a/b/c/` created; files written
- **Tests:** `tests/materialization_test.rs`

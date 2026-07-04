# Parameter Spec: output_dir::

### Scope

- **Element:** `parameter/output_dir`
- **Source:** `docs/cli/param.md#parameter--12-output_dir`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-31 | inline_files_written_to_output_dir | nominal | ✅ |
| EC-32 | output_dir_created_if_not_exists | nominal | ✅ |
| EC-98 | reference_only_archive_writes_nothing_to_output_dir | Behavioral Divergence | 🚧 |
| EC-99 | output_dir_is_required | error | 🚧 |
| EC-100 | path_traversal_in_output_dir_rejected | error | 🚧 |
| EC-101 | multiple_files_preserve_relative_paths | nominal | 🚧 |

---

### EC-31: inline files written to output dir

- **Given:** An archive with two inline files is loaded
- **When:** `.content.externalize output_dir::"./external"` is run
- **Then:** Exit code 0; both files are written under `./external/` preserving relative paths
- **Tests:** `tests/content_commands_test.rs`

### EC-32: output dir created if not exists

- **Given:** Directory `./new_external` does not exist; archive has inline content
- **When:** `.content.externalize output_dir::"./new_external"` is run
- **Then:** Exit code 0; `./new_external/` is created and files written into it
- **Tests:** `tests/content_commands_test.rs`

### EC-98: reference only archive writes nothing to output dir

- **Given:** An archive where all files are already `reference` mode (no inline content)
- **When:** `.content.externalize output_dir::"./out"` is run
- **Then:** Exit code 0; `./out/` may be created but contains no files (nothing to externalize)
- **Tests:** `tests/content_commands_test.rs`

### EC-99: output dir is required

- **Given:** An archive with inline content is loaded
- **When:** `.content.externalize` is run without `output_dir::`
- **Then:** Exit code 1; error message indicates `output_dir::` is required
- **Tests:** `tests/content_commands_test.rs`

### EC-100: path traversal in output dir rejected

- **Given:** An archive with inline content is loaded
- **When:** `.content.externalize output_dir::"../../etc"` is run
- **Then:** Exit code 1; error message indicates path traversal is not allowed
- **Tests:** `tests/content_commands_test.rs`

### EC-101: multiple files preserve relative paths

- **Given:** An archive has inline files at `src/main.rs` and `src/lib.rs`
- **When:** `.content.externalize output_dir::"./out"` is run
- **Then:** Exit code 0; files written as `./out/src/main.rs` and `./out/src/lib.rs`
- **Tests:** `tests/content_commands_test.rs`

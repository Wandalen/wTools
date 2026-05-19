# Parameter Spec: output_dir::

### Scope

- **Element:** `parameter/output_dir`
- **Source:** `docs/cli/param.md#parameter--12-output_dir`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-31 | inline_files_written_to_output_dir | nominal | ✅ |
| EC-32 | output_dir_created_if_not_exists | nominal | ✅ |

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

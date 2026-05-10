# Feature Spec: Template Materialization

### Scope

- **Element:** `feature/006_template_materialization`
- **Source:** `docs/feature/006_template_materialization.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | materialize_renders_template_placeholders | nominal | ✅ |
| FT-02 | materialize_fails_on_missing_mandatory_param | error | ✅ |
| FT-03 | unpack_copies_raw_without_substitution | nominal | ✅ |
| FT-04 | dry_run_previews_without_writing | nominal | ✅ |
| FT-05 | path_traversal_in_destination_is_rejected | security | ✅ |

---

### FT-01: materialize renders template placeholders

- **Given:** Archive with `hello.txt` containing `Hello, {{name}}!`; `name=World` is set
- **When:** `.materialize destination::<dir>` is run
- **Then:** Exit code 0; `<dir>/hello.txt` contains `Hello, World!`
- **Tests:** `tests/materialization_test.rs`

### FT-02: materialize fails on missing mandatory parameter

- **Given:** Archive with mandatory parameter `project_name` and no value set
- **When:** `.materialize destination::<dir>` is run
- **Then:** Exit code 1; error indicates `project_name` is mandatory but unset; no files written
- **Tests:** `tests/materialization_test.rs`

### FT-03: unpack copies raw content without substitution

- **Given:** Archive with `hello.txt` containing `Hello, {{name}}!`
- **When:** `.unpack destination::<dir>` is run
- **Then:** Exit code 0; `<dir>/hello.txt` contains `Hello, {{name}}!` — placeholder preserved
- **Tests:** `tests/materialization_test.rs`

### FT-04: dry run previews planned operations without writing

- **Given:** Archive with `hello.txt`; destination directory does not exist
- **When:** `.materialize destination::<dir> dry::1` is run
- **Then:** Exit code 0; output shows planned files; no files written to disk
- **Tests:** `tests/materialization_test.rs`

### FT-05: path traversal in destination is rejected

- **Given:** An archive is loaded
- **When:** `.materialize destination::/tmp/safe/../../../etc` is run
- **Then:** Exit code 1; error indicates invalid path; no files written
- **Tests:** `tests/materialization_test.rs`

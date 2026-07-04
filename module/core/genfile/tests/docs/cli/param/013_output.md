# Parameter Spec: output::

### Scope

- **Element:** `parameter/output`
- **Source:** `docs/cli/param.md#parameter--13-output`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-33 | json_extension_produces_json_archive | nominal | ✅ |
| EC-34 | yaml_extension_produces_yaml_archive | nominal | ✅ |
| EC-35 | output_is_required | error | ✅ |
| EC-102 | output_overwrites_existing_file_silently | Behavioral Divergence | 🚧 |
| EC-103 | output_parent_dir_created_if_not_exists | nominal | 🚧 |
| EC-104 | output_with_non_standard_extension_uses_json | nominal | 🚧 |

---

### EC-33: json extension produces json archive

- **Given:** A directory `./src` with files exists
- **When:** `.pack input::"./src" output::"archive.json"` is run
- **Then:** Exit code 0; `archive.json` is a valid JSON archive file on disk
- **Tests:** `tests/archive_commands_test.rs`

### EC-34: yaml extension produces yaml archive

- **Given:** A directory `./src` with files exists
- **When:** `.pack input::"./src" output::"archive.yaml"` is run
- **Then:** Exit code 0; `archive.yaml` is a valid YAML archive file on disk
- **Tests:** `tests/archive_commands_test.rs`

### EC-35: output is required

- **Given:** A directory `./src` with files exists
- **When:** `.pack input::"./src"` is run without `output::`
- **Then:** Exit code 1; error message indicates `output::` is required
- **Tests:** `tests/archive_commands_test.rs`

### EC-102: output overwrites existing file silently

- **Given:** A file `archive.json` already exists on disk
- **When:** `.pack input::"./src" output::"archive.json"` is run
- **Then:** Exit code 0; `archive.json` is overwritten without a warning or prompt
- **Tests:** `tests/archive_commands_test.rs`

### EC-103: output parent dir created if not exists

- **Given:** Directory `./dist/` does not exist
- **When:** `.pack input::"./src" output::"./dist/archive.json"` is run
- **Then:** Exit code 0; `./dist/` directory created; `archive.json` written inside it
- **Tests:** `tests/archive_commands_test.rs`

### EC-104: output with non standard extension uses json

- **Given:** An output path with an unrecognized extension
- **When:** `.pack input::"./src" output::"archive.tpl"` is run
- **Then:** Exit code 0; `archive.tpl` contains valid JSON (defaults to JSON when extension is unknown)
- **Tests:** `tests/archive_commands_test.rs`

# Parameter Spec: output::

### Scope

- **Element:** `parameter/output`
- **Source:** `docs/cli/param.md#parameter--13-output`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-33 | json_extension_produces_json_archive | nominal | ✅ |
| EC-34 | yaml_extension_produces_yaml_archive | nominal | ✅ |
| EC-35 | output_is_required | error | ✅ |

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

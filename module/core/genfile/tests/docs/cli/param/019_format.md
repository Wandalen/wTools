# Parameter Spec: format::

### Scope

- **Element:** `parameter/format`
- **Source:** `docs/cli/param.md#parameter--19-format`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-48 | format_json_produces_json_file | nominal | ✅ |
| EC-49 | format_yaml_produces_yaml_file | nominal | ✅ |
| EC-50 | format_auto_detected_from_extension | nominal | ✅ |
| EC-123 | invalid_format_value_rejected | error | 🚧 |
| EC-124 | yml_accepted_as_alias_for_yaml | nominal | 🚧 |
| EC-125 | format_json_with_pretty_one_produces_indented_json | nominal | 🚧 |

---

### EC-48: format json produces json file

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.yaml" format::json` is run
- **Then:** Exit code 0; `out.yaml` contains valid JSON (format overrides extension)
- **Tests:** `tests/archive_commands_test.rs`

### EC-49: format yaml produces yaml file

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" format::yaml` is run
- **Then:** Exit code 0; `out.json` contains valid YAML (format overrides extension)
- **Tests:** `tests/archive_commands_test.rs`

### EC-50: format auto detected from extension

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"template.yaml"` is run (no `format::` specified)
- **Then:** Exit code 0; `template.yaml` contains YAML (auto-detected from `.yaml` extension)
- **Tests:** `tests/archive_commands_test.rs`

### EC-123: invalid format value rejected

- **Given:** An unrecognized format value is provided
- **When:** `.archive.save path::"out.xml" format::xml` is run
- **Then:** Exit code 1; error message indicates `xml` is not a supported format
- **Tests:** `tests/archive_commands_test.rs`

### EC-124: yml accepted as alias for yaml

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" format::yml` is run
- **Then:** Exit code 0; `out.json` contains valid YAML (`yml` is accepted as an alias for `yaml`)
- **Tests:** `tests/archive_commands_test.rs`

### EC-125: format json with pretty one produces indented json

- **Given:** An archive is loaded; `pretty::1` is the default
- **When:** `.archive.save path::"out.txt" format::json` is run
- **Then:** Exit code 0; `out.txt` contains indented JSON (format and pretty interact correctly)
- **Tests:** `tests/archive_commands_test.rs`

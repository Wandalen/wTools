# Parameter Spec: format::

### Scope

- **Element:** `parameter/format`
- **Source:** `docs/cli/param.md#parameter--19-format`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-48 | format_json_produces_json_file | nominal | ✅ |
| EC-49 | format_yaml_produces_yaml_file | nominal | ✅ |
| EC-50 | format_auto_detected_from_extension | nominal | ✅ |

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

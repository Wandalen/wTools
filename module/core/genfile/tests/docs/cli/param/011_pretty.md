# Parameter Spec: pretty::

### Scope

- **Element:** `parameter/pretty`
- **Source:** `docs/cli/param.md#parameter--11-pretty`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-29 | pretty_one_produces_indented_json | nominal | ✅ |
| EC-30 | pretty_zero_produces_compact_json | nominal | ✅ |
| EC-94 | default_is_pretty_one | nominal | 🚧 |
| EC-95 | pretty_has_no_effect_on_yaml | nominal | 🚧 |
| EC-96 | invalid_pretty_value_rejected | error | 🚧 |
| EC-97 | pretty_json_is_valid_and_parseable | nominal | 🚧 |

---

### EC-29: pretty one produces indented json

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" pretty::1` is run
- **Then:** Exit code 0; output file contains JSON with newlines and indentation
- **Tests:** `tests/archive_commands_test.rs`

### EC-30: pretty zero produces compact json

- **Given:** An archive is loaded in session state
- **When:** `.archive.save path::"out.json" pretty::0` is run
- **Then:** Exit code 0; output file is a single-line compact JSON with no extra whitespace
- **Tests:** `tests/archive_commands_test.rs`

### EC-94: default is pretty one

- **Given:** An archive is loaded; no `pretty::` parameter specified
- **When:** `.archive.save path::"out.json"` is run
- **Then:** Exit code 0; output file contains indented JSON (default is `pretty::1`)
- **Tests:** `tests/archive_commands_test.rs`

### EC-95: pretty has no effect on yaml

- **Given:** An archive is loaded
- **When:** `.archive.save path::"out.yaml" pretty::0` is run
- **Then:** Exit code 0; output file is valid YAML (the `pretty::` flag is ignored for YAML format)
- **Tests:** `tests/archive_commands_test.rs`

### EC-96: invalid pretty value rejected

- **Given:** A command with `pretty::2` (not a boolean 0/1)
- **When:** `.archive.save path::"out.json" pretty::2` is run
- **Then:** Exit code 1; error message indicates invalid value for `pretty::`
- **Tests:** `tests/archive_commands_test.rs`

### EC-97: pretty json is valid and parseable

- **Given:** An archive is loaded
- **When:** `.archive.save path::"out.json" pretty::1` is run
- **Then:** Exit code 0; `out.json` is parseable as valid JSON (indentation does not corrupt structure)
- **Tests:** `tests/archive_commands_test.rs`

# Parameter Spec: value::

### Scope

- **Element:** `parameter/value`
- **Source:** `docs/cli/param.md#parameter--8-value`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-22 | value_stored_for_parameter | nominal | ✅ |
| EC-23 | empty_string_is_valid_value | nominal | ✅ |
| EC-24 | value_required_alongside_name | error | ✅ |
| EC-83 | template_placeholder_in_value_stored_verbatim | Behavioral Divergence | 🚧 |
| EC-84 | unicode_content_stored_correctly | nominal | 🚧 |
| EC-85 | value_persists_in_saved_archive | nominal | 🚧 |

---

### EC-22: value stored for parameter

- **Given:** An archive has a defined parameter `project_name`
- **When:** `.value.set name::project_name value::"my-app"` is run
- **Then:** Exit code 0; `project_name` now resolves to `"my-app"` in templates
- **Tests:** `tests/param_value_commands_test.rs`

### EC-23: empty string is valid value

- **Given:** An archive has a defined parameter `author`
- **When:** `.value.set name::author value::""` is run
- **Then:** Exit code 0; `author` value is stored as empty string
- **Tests:** `tests/param_value_commands_test.rs`

### EC-24: value required alongside name

- **Given:** An archive has a defined parameter
- **When:** `.value.set name::project_name` is run without `value::`
- **Then:** Exit code 1; error message indicates `value::` is required
- **Tests:** `tests/param_value_commands_test.rs`

### EC-83: template placeholder in value stored verbatim

- **Given:** An archive has a defined parameter `message`
- **When:** `.value.set name::message value::"Hello {{world}}"` is run
- **Then:** Exit code 0; value stored literally as `"Hello {{world}}"` — double-braces are NOT expanded at store time
- **Tests:** `tests/param_value_commands_test.rs`

### EC-84: unicode content stored correctly

- **Given:** An archive has a defined parameter `greeting`
- **When:** `.value.set name::greeting value::"こんにちは"` is run
- **Then:** Exit code 0; value stored and retrieved as `"こんにちは"` without corruption
- **Tests:** `tests/param_value_commands_test.rs`

### EC-85: value persists in saved archive

- **Given:** An archive has parameter `port` with value set to `"8080"`; archive saved and reloaded
- **When:** After `.archive.load`, parameter `port` value is inspected
- **Then:** Exit code 0; value `"8080"` preserved through serialization round-trip
- **Tests:** `tests/param_value_commands_test.rs`

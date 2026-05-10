# Parameter Spec: value::

### Scope

- **Element:** `parameter/value`
- **Source:** `docs/cli/param.md#parameter--8-value`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-22 | value_stored_for_parameter | nominal | ✅ |
| EC-23 | empty_string_is_valid_value | nominal | ✅ |
| EC-24 | value_required_alongside_name | error | ✅ |

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

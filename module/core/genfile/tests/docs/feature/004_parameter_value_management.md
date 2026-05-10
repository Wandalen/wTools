# Feature Spec: Parameter Value Management

### Scope

- **Element:** `feature/004_parameter_value_management`
- **Source:** `docs/feature/004_parameter_value_management.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | set_stores_value_for_defined_parameter | nominal | ✅ |
| FT-02 | list_shows_all_set_values | nominal | ✅ |
| FT-03 | clear_removes_all_values | nominal | ✅ |
| FT-04 | set_undefined_parameter_produces_error | error | 🔶 deferred |

---

### FT-01: set stores value for defined parameter

- **Given:** An archive with parameter `author` defined is loaded
- **When:** `.value.set name::author value::Alice` is run
- **Then:** Exit code 0; `.value.list` shows `author = Alice`
- **Tests:** `tests/param_value_commands_test.rs`

### FT-02: list shows all set values

- **Given:** An archive with values `author=Alice` and `version=1.0` set is loaded
- **When:** `.value.list` is run
- **Then:** Exit code 0; output contains both `author` and `version` with their values
- **Tests:** `tests/param_value_commands_test.rs`

### FT-03: clear removes all values

- **Given:** An archive with value `author=Alice` is loaded
- **When:** `.value.clear` is run
- **Then:** Exit code 0; `.value.list` shows no set values
- **Tests:** `tests/param_value_commands_test.rs`

### FT-04: set value for undefined parameter produces error

- **Given:** An archive with no `ghost` parameter defined is loaded
- **When:** `.value.set name::ghost value::foo` is run
- **Then:** Exit code 1; error indicates parameter `ghost` is not defined
- **Tests:** `tests/param_value_commands_test.rs`

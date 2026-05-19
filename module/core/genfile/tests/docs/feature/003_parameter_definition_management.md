# Feature Spec: Parameter Definition Management

### Scope

- **Element:** `feature/003_parameter_definition_management`
- **Source:** `docs/feature/003_parameter_definition_management.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | add_creates_parameter_with_name | nominal | ✅ |
| FT-02 | list_shows_all_parameters | nominal | ✅ |
| FT-03 | remove_deletes_parameter | nominal | ✅ |
| FT-04 | add_mandatory_sets_mandatory_flag | nominal | ✅ |
| FT-05 | add_duplicate_name_produces_error | error | 🔶 deferred |

---

### FT-01: add creates parameter with name

- **Given:** An archive is loaded in session state
- **When:** `.parameter.add name::author` is run
- **Then:** Exit code 0; `.parameter.list` shows `author` in the parameter list
- **Tests:** `tests/param_value_commands_test.rs`

### FT-02: list shows all parameters

- **Given:** An archive with parameters `author` and `version` is loaded
- **When:** `.parameter.list` is run
- **Then:** Exit code 0; output lists both `author` and `version`
- **Tests:** `tests/param_value_commands_test.rs`

### FT-03: remove deletes parameter

- **Given:** An archive with parameter `author` is loaded
- **When:** `.parameter.remove name::author` is run
- **Then:** Exit code 0; `.parameter.list` no longer shows `author`
- **Tests:** `tests/param_value_commands_test.rs`

### FT-04: add mandatory parameter sets mandatory flag

- **Given:** An archive is loaded
- **When:** `.parameter.add name::project_name mandatory::1` is run
- **Then:** Exit code 0; parameter listed as mandatory in `.parameter.list` output
- **Tests:** `tests/param_value_commands_test.rs`

### FT-05: add parameter with duplicate name produces error

- **Given:** An archive with parameter `author` already defined is loaded
- **When:** `.parameter.add name::author` is run again
- **Then:** Exit code 1; error indicates duplicate parameter name
- **Tests:** `tests/param_value_commands_test.rs`

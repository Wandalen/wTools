# CLI Spec: Value Operations Commands

### Scope

- **Element:** Commands `22–24` (`.value.*` namespace)
- **Source:** `docs/cli/command/value.md`
- **Prefix:** `IT-`
- **Minimum cases:** 5

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-41 | value_set_stores_value_for_defined_parameter | nominal | ✅ |
| IT-42 | value_set_undefined_parameter_exits_1 | error | ✅ |
| IT-43 | value_set_overwrites_previous_value | nominal | ✅ |
| IT-44 | value_list_shows_set_and_unset_params | nominal | ✅ |
| IT-45 | value_clear_resets_all_values | nominal | ✅ |
| IT-46 | value_clear_dry_run_makes_no_changes | nominal | ✅ |

---

### IT-41: value.set stores value for defined parameter

- **Given:** An archive with parameter `project_name` defined
- **When:** `.value.set name::project_name value::"my-app"` is run
- **Then:** Exit code 0; parameter `project_name` has value `"my-app"`
- **Tests:** `tests/param_value_commands_test.rs`

### IT-42: value.set on undefined parameter exits 1

- **Given:** An archive with no parameter named `undefined_param`
- **When:** `.value.set name::undefined_param value::"foo"` is run
- **Then:** Exit code 1; error message indicates parameter not defined
- **Tests:** `tests/param_value_commands_test.rs`

### IT-43: value.set overwrites previous value without error

- **Given:** An archive where `port` is defined and set to `"3000"`
- **When:** `.value.set name::port value::"8080"` is run
- **Then:** Exit code 0; `port` value is now `"8080"`
- **Tests:** `tests/param_value_commands_test.rs`

### IT-44: value.list shows set and unset parameters

- **Given:** An archive with two parameters: `project_name` (set), `author` (unset, optional with default `""`)
- **When:** `.value.list` is run
- **Then:** Exit code 0; output lists `project_name` with its value and `author` as using default
- **Tests:** `tests/param_value_commands_test.rs`

### IT-45: value.clear resets all parameter values

- **Given:** An archive with two parameters, both with values set
- **When:** `.value.clear` is run
- **Then:** Exit code 0; all parameter values cleared; subsequent `.value.list` shows all unset
- **Tests:** `tests/param_value_commands_test.rs`

### IT-46: value.clear dry run makes no changes

- **Given:** An archive with parameter values set
- **When:** `.value.clear dry::1` is run
- **Then:** Exit code 0; values remain set; output contains `[DRY RUN]`
- **Tests:** `tests/param_value_commands_test.rs`

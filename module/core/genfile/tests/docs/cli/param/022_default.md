# Parameter Spec: default::

### Scope

- **Element:** `parameter/default`
- **Source:** `docs/cli/param.md#parameter--22-default`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-56 | default_used_when_no_value_set | nominal | 🚧 |
| EC-57 | explicit_value_overrides_default | nominal | 🚧 |
| EC-58 | null_default_with_mandatory_requires_explicit_value | nominal | 🚧 |
| EC-133 | empty_string_as_default_is_accepted | nominal | 🚧 |
| EC-134 | no_default_means_null | nominal | 🚧 |
| EC-135 | default_visible_in_parameter_list | nominal | 🚧 |

---

### EC-56: default value used when no value is set

- **Given:** Parameter `port` defined with `default::"3000"`; no `.value.set` called
- **When:** `.materialize destination::<dir>` is run; archive file has `{{port}}`
- **Then:** Exit code 0; output file contains `3000`
- **Tests:** `tests/materialization_test.rs`

### EC-57: explicit value overrides default

- **Given:** Parameter `port` defined with `default::"3000"`; `.value.set name::port value::"8080"` called
- **When:** `.materialize destination::<dir>` is run; archive file has `{{port}}`
- **Then:** Exit code 0; output file contains `8080` (explicit value wins over default)
- **Tests:** `tests/materialization_test.rs`

### EC-58: null default with mandatory flag requires explicit value

- **Given:** Parameter `project_name` defined with `mandatory::1` and no default
- **When:** `.materialize destination::<dir>` is run without setting a value
- **Then:** Exit code 1; error identifies `project_name` as missing
- **Tests:** `tests/materialization_test.rs`

### EC-133: empty string as default is accepted

- **Given:** `.parameter.add name::author default::""` is run
- **When:** Archive is saved and reloaded; parameter `author` is inspected
- **Then:** Exit code 0; default value stored as empty string `""` (not null)
- **Tests:** `tests/parameter_commands_test.rs`

### EC-134: no default means null

- **Given:** `.parameter.add name::author` is run without specifying `default::`
- **When:** Archive is saved and reloaded; parameter `author` is inspected
- **Then:** Exit code 0; `author` has no default (null); unset parameter produces empty substitution during materialization
- **Tests:** `tests/parameter_commands_test.rs`

### EC-135: default visible in parameter list

- **Given:** Parameter `port` defined with `default::"3000"`
- **When:** `.parameter.list` is run
- **Then:** Exit code 0; output shows `port` with default value `3000` listed
- **Tests:** `tests/parameter_commands_test.rs`

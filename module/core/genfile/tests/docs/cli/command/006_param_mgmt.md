# CLI Spec: Parameter Management Commands

### Scope

- **Element:** Commands `19–21` (`.parameter.*` namespace)
- **Source:** `docs/cli/command/param_mgmt.md`
- **Prefix:** `IT-`
- **Minimum cases:** 8

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-35 | parameter_add_registers_mandatory_param | nominal | ✅ |
| IT-36 | parameter_add_registers_optional_param_with_default | nominal | ✅ |
| IT-37 | parameter_add_duplicate_name_exits_1 | error | 🔶 deferred |
| IT-38 | parameter_list_shows_all_definitions | nominal | ✅ |
| IT-39 | parameter_remove_deletes_existing_definition | nominal | ✅ |
| IT-40 | parameter_remove_nonexistent_exits_1 | error | ✅ |
| IT-55 | parameter_list_empty_archive_shows_empty_output | nominal | 🚧 |
| IT-56 | parameter_add_persists_after_save_and_load | nominal | 🚧 |

---

### IT-35: parameter.add registers mandatory parameter

- **Given:** An archive is loaded with no parameters
- **When:** `.parameter.add name::project_name mandatory::true` is run
- **Then:** Exit code 0; archive has 1 mandatory parameter named `project_name`
- **Tests:** `tests/param_value_commands_test.rs`

### IT-36: parameter.add registers optional parameter with default

- **Given:** An archive is loaded
- **When:** `.parameter.add name::port mandatory::false default::"3000"` is run
- **Then:** Exit code 0; archive has parameter `port` with default value `"3000"`
- **Tests:** `tests/param_value_commands_test.rs`

### IT-37: parameter.add duplicate name exits 1

- **Given:** An archive with parameter `project_name` already defined
- **When:** `.parameter.add name::project_name mandatory::false` is run again
- **Then:** Exit code 1; error message indicates duplicate parameter name
- **Tests:** none — see task/004_implement_deferred_source_behaviors.md

### IT-38: parameter.list shows all definitions with metadata

- **Given:** An archive with two parameters (one mandatory, one optional with default)
- **When:** `.parameter.list` is run
- **Then:** Exit code 0; both parameters listed with mandatory flag and default shown
- **Tests:** `tests/param_value_commands_test.rs`

### IT-39: parameter.remove deletes existing definition

- **Given:** An archive with parameter `port` defined
- **When:** `.parameter.remove name::port` is run
- **Then:** Exit code 0; archive has no parameter named `port`
- **Tests:** `tests/param_value_commands_test.rs`

### IT-40: parameter.remove nonexistent parameter exits 1

- **Given:** An archive with no parameter named `ghost`
- **When:** `.parameter.remove name::ghost` is run
- **Then:** Exit code 1; error message indicates parameter not found
- **Tests:** `tests/param_value_commands_test.rs`

### IT-55: parameter.list empty archive shows empty output

- **Given:** An archive with no parameters is loaded
- **When:** `.parameter.list` is run
- **Then:** Exit code 0; output is empty or shows a zero-count message (no error)
- **Tests:** `tests/param_value_commands_test.rs`

### IT-56: parameter.add persists after save and load

- **Given:** An archive is loaded; `.parameter.add name::env mandatory::true` is run; archive saved and reloaded
- **When:** `.parameter.list` is run after reload
- **Then:** Exit code 0; `env` appears in the list as mandatory (persists through round-trip)
- **Tests:** `tests/param_value_commands_test.rs`

# Parameter Spec: mandatory::

### Scope

- **Element:** `parameter/mandatory`
- **Source:** `docs/cli/param.md#parameter--15-mandatory`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-38 | mandatory_true_blocks_materialize_without_value | nominal | ✅ |
| EC-39 | mandatory_false_allows_materialize_without_value | nominal | ✅ |
| EC-40 | mandatory_default_is_false | nominal | ✅ |
| EC-109 | invalid_mandatory_value_rejected | error | 🚧 |
| EC-110 | mandatory_true_with_value_set_allows_materialize | nominal | 🚧 |
| EC-111 | mandatory_true_with_default_allows_materialize | nominal | 🚧 |

---

### EC-38: mandatory true blocks materialize without value

- **Given:** An archive has parameter `project_name` with `mandatory::true`; no value is set
- **When:** `.materialize destination::"./out"` is run
- **Then:** Exit code 1; error message indicates missing mandatory parameter `project_name`
- **Tests:** `tests/materialization_test.rs`

### EC-39: mandatory false allows materialize without value

- **Given:** An archive has parameter `author` with `mandatory::false`; no value is set; `author` has a default
- **When:** `.materialize destination::"./out"` is run
- **Then:** Exit code 0; materialization succeeds using the default value
- **Tests:** `tests/materialization_test.rs`

### EC-40: mandatory default is false

- **Given:** `.parameter.add name::port` is run without specifying `mandatory::`
- **When:** `.parameter.list` is checked
- **Then:** `port` shows as optional (mandatory flag is false by default)
- **Tests:** `tests/param_value_commands_test.rs`

### EC-109: invalid mandatory value rejected

- **Given:** `.parameter.add name::port mandatory::yes` is run
- **When:** The command is executed
- **Then:** Exit code 1; error message indicates `yes` is not a valid value (must be `0`, `1`, `true`, or `false`)
- **Tests:** `tests/parameter_commands_test.rs`

### EC-110: mandatory true with value set allows materialize

- **Given:** An archive has parameter `project_name` with `mandatory::true`; value set to `"my-app"`
- **When:** `.materialize destination::"./out"` is run
- **Then:** Exit code 0; materialization succeeds (mandatory requirement satisfied by provided value)
- **Tests:** `tests/materialization_test.rs`

### EC-111: mandatory true with default allows materialize

- **Given:** An archive has parameter `port` with `mandatory::true` and `default::"3000"`; no explicit value set
- **When:** `.materialize destination::"./out"` is run
- **Then:** Exit code 0; materialization succeeds using the default value `"3000"`
- **Tests:** `tests/materialization_test.rs`

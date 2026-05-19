# Parameter Spec: mandatory::

### Scope

- **Element:** `parameter/mandatory`
- **Source:** `docs/cli/param.md#parameter--15-mandatory`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-38 | mandatory_true_blocks_materialize_without_value | nominal | ✅ |
| EC-39 | mandatory_false_allows_materialize_without_value | nominal | ✅ |
| EC-40 | mandatory_default_is_false | nominal | ✅ |

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

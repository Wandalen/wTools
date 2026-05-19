# Parameter Spec: default::

### Scope

- **Element:** `parameter/default`
- **Source:** `docs/cli/param.md#parameter--22-default`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-56 | default_used_when_no_value_set | nominal | 🚧 |
| EC-57 | explicit_value_overrides_default | nominal | 🚧 |
| EC-58 | null_default_with_mandatory_requires_explicit_value | nominal | 🚧 |

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

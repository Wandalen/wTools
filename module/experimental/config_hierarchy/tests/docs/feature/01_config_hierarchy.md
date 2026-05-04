# Feature Spec: Config Hierarchy

### Scope

- **Element:** `feature/001_config_hierarchy`
- **Source:** `docs/feature/001_config_hierarchy.md`
- **Feature flag:** `enabled` (core); `file_ops` (persistence)
- **Prefix:** `FT-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | default_source_is_lowest_priority | nominal | ✅ |
| FT-02 | runtime_overrides_all_sources | nominal | ✅ |
| FT-03 | env_var_overrides_default | nominal | ✅ |
| FT-04 | source_provenance_tracked_per_value | nominal | ✅ |
| FT-05 | no_validator_accepted_as_type_param | nominal | ✅ |
| FT-06 | missing_parameter_returns_null_default | boundary | ✅ |
| FT-07 | resolve_all_enumerates_all_declared_params | nominal | ✅ |

---

### FT-01: default source is lowest priority

- **Given:** No runtime params, no env vars, no config files; `get_defaults()` returns `"param1" → "default_value"`
- **When:** `resolve_config_value("param1", &{})` is called
- **Then:** Returns `("default_value", ConfigSource::Default)`
- **Tests:** `tests/hierarchy_tests.rs::test_default_source`

### FT-02: runtime parameters override all other sources

- **Given:** Runtime params map contains `"param1" → "runtime_value"` and env var `TESTAPP_PARAM1` is also set
- **When:** `resolve_config_value("param1", &runtime_params)` is called
- **Then:** Returns `("runtime_value", ConfigSource::Runtime)` — runtime wins over env
- **Tests:** `tests/hierarchy_tests.rs::test_runtime_overrides_default`, `test_runtime_overrides_env`

### FT-03: environment variable overrides default

- **Given:** Env var `TESTAPP_ENVPARAM=env_value` is set; no runtime params
- **When:** `resolve_config_value("envparam", &{})` is called
- **Then:** Returns `("env_value", ConfigSource::Environment)` — env wins over default
- **Tests:** `tests/hierarchy_tests.rs::test_env_overrides_default`

### FT-04: source provenance tracked per resolved value

- **Given:** A global config file exists with `"param1" → "global_value"`; no runtime or env override
- **When:** `resolve_config_value("param1", &{})` is called
- **Then:** Returns `ConfigSource::Global(_)` — source path is embedded in the enum variant
- **Tests:** `tests/hierarchy_tests.rs::test_global_config_overrides_default`

### FT-05: NoValidator accepted as type parameter

- **Given:** `ConfigManager< D, P, NoValidator >` is used as the manager type
- **When:** Any resolution or path function is called
- **Then:** Compiles and runs without error; NoValidator never rejects any value
- **Tests:** `tests/validator_tests.rs::test_no_validator_accepts_all`

### FT-06: unknown parameter returns (Null, Default)

- **Given:** Parameter name not in defaults, not in env, not in any config file
- **When:** `resolve_config_value("unknown_param", &{})` is called
- **Then:** Returns `(JsonValue::Null, ConfigSource::Default)` — no panic
- **Tests:** `tests/hierarchy_tests.rs::test_unknown_parameter_returns_null`

### FT-07: resolve_all enumerates all declared parameter names

- **Given:** `get_parameter_names()` returns `["param1", "param2"]`; all sources clear
- **When:** `resolve_all_config(&{})` is called
- **Then:** Result map contains both `"param1"` and `"param2"` keys
- **Tests:** `tests/hierarchy_tests.rs::test_resolve_all_config`

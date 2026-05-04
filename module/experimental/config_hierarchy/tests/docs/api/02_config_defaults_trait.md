# API Spec: ConfigDefaults Trait

### Scope

- **Element:** `api/002_config_defaults_trait`
- **Source:** `docs/api/002_config_defaults_trait.md`
- **Feature flag:** `enabled`
- **Prefix:** `AP-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AP-01 | defaults_are_lowest_priority | nominal | ✅ |
| AP-02 | param_absent_from_defaults_returns_null | boundary | ✅ |
| AP-03 | parameter_names_drives_resolve_all | nominal | ✅ |
| AP-04 | undeclared_file_param_included_in_resolve_all | secondary_scan | ⏳ |
| AP-05 | param_in_defaults_not_in_names_directly_resolvable | boundary | ✅ |

---

### AP-01: get_defaults() values are lowest priority

- **Given:** `get_defaults()` returns `"param1" → "default_value"`; env var `TESTAPP_PARAM1=env_value` is set
- **When:** `resolve_config_value("param1", &{})` is called
- **Then:** Returns `"env_value"` from `ConfigSource::Environment` — env overrides default
- **Tests:** `tests/hierarchy_tests.rs::test_env_overrides_default`

### AP-02: parameter not in get_defaults() returns (Null, Default)

- **Given:** `get_defaults()` does not contain `"unknown_param"`; no env var, no config file
- **When:** `resolve_config_value("unknown_param", &{})` is called
- **Then:** Returns `(JsonValue::Null, ConfigSource::Default)` — no error, graceful fallback
- **Tests:** `tests/hierarchy_tests.rs::test_unknown_parameter_returns_null`

### AP-03: get_parameter_names() drives resolve_all enumeration

- **Given:** `get_parameter_names()` returns `["param1", "param2"]`; no files or env vars
- **When:** `resolve_all_config(&{})` is called
- **Then:** Returned map contains exactly `"param1"` and `"param2"` keys (plus any secondary scan additions)
- **Tests:** `tests/hierarchy_tests.rs::test_resolve_all_config`

### AP-04: param in config file but not in get_parameter_names() included via secondary scan

- **Given:** Global config file contains `"extra_param" → "from_file"`; `get_parameter_names()` does not list `"extra_param"`
- **When:** `resolve_all_config(&{})` is called
- **Then:** Result map contains `"extra_param"` — secondary scan of config files picks it up
- **Tests:** `tests/hierarchy_tests.rs::test_resolve_all_includes_undeclared_config_file_params` ⏳ (not yet written)

### AP-05: param in defaults but not in parameter_names still resolvable directly

- **Given:** `get_defaults()` contains `"hidden_param" → 42`; `get_parameter_names()` does not list `"hidden_param"`
- **When:** `resolve_config_value("hidden_param", &{})` is called directly
- **Then:** Returns `(42, ConfigSource::Default)` — direct resolution bypasses the enumeration gate
- **Tests:** `tests/hierarchy_tests.rs::test_default_source` (implicit — defaults map access is unconstrained)

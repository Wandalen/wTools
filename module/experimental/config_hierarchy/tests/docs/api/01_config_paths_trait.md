# API Spec: ConfigPaths Trait

### Scope

- **Element:** `api/001_config_paths_trait`
- **Source:** `docs/api/001_config_paths_trait.md`
- **Feature flag:** `enabled`
- **Prefix:** `AP-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AP-01 | only_app_name_required | nominal | ✅ |
| AP-02 | empty_app_name_rejected | security | ✅ |
| AP-03 | slash_in_app_name_rejected | security | ⏳ |
| AP-04 | dotdot_in_app_name_rejected | security | ✅ |
| AP-05 | custom_env_var_prefix_used | behavioral_divergence | ✅ |
| AP-06 | preserve_app_name_casing | behavioral_divergence | ⏳ |
| AP-07 | custom_local_permanent_prefix | behavioral_divergence | ✅ |
| AP-08 | xdg_config_home_fallback | nominal | ⏳ |

---

### AP-01: only app_name() is required; all others have defaults

- **Given:** A `ConfigPaths` implementation that overrides only `app_name()` returning `"myapp"`
- **When:** `get_local_config_path()`, `get_global_config_path()`, and `resolve_config_value()` are called
- **Then:** All functions work without error; paths are derived from `"myapp"` using default formulas
- **Tests:** `tests/path_standards_tests.rs::test_no_flexibility_all_paths_derived`

### AP-02: empty app_name rejected at path construction

- **Given:** A `ConfigPaths` implementation returning `""` from `app_name()`
- **When:** `get_local_config_path()` is called
- **Then:** Returns `Err` containing `"app_name must not be empty"` — no panic, no invalid path constructed
- **Tests:** `tests/edge_cases_tests.rs::test_empty_app_name_rejected`

### AP-03: app_name containing "/" alone rejected

- **Given:** A `ConfigPaths` implementation returning `"my/app"` from `app_name()` (forward slash without `..`)
- **When:** `get_local_config_path()` is called
- **Then:** Returns `Err` containing `"app_name contains invalid characters"` — `/` alone is a path separator
- **Tests:** `tests/edge_cases_tests.rs::test_slash_only_in_app_name_rejected` ⏳ (not yet written)

### AP-04: app_name containing ".." rejected

- **Given:** A `ConfigPaths` implementation returning `"../../etc/passwd"` from `app_name()`
- **When:** `get_local_config_path()` is called
- **Then:** Returns `Err` containing `"app_name contains invalid characters"` — path traversal blocked
- **Tests:** `tests/edge_cases_tests.rs::test_path_traversal_rejected`

### AP-05: custom env_var_prefix() is actually used

- **Given:** `ConfigPaths` overrides `env_var_prefix()` → `"MYPREFIX"` and `env_var_separator()` → `"__"` ; env var `MYPREFIX__timeout=999` set
- **When:** `resolve_config_value("timeout", &{})` is called
- **Then:** Returns `999` from `ConfigSource::Environment` — proves prefix method is called not hardcoded `app_name().to_uppercase()`
- **Tests:** `tests/configurability_tests.rs::custom_env_var_prefix_actually_used`

### AP-06: PreserveAppName casing preserves prefix, uppercases param

- **Given:** `ConfigPaths` overrides `env_var_casing()` → `EnvVarCasing::PreserveAppName`; `app_name()` → `"myApp"`; env var `MYPREFIX__TIMEOUT=42` set
- **When:** `resolve_config_value("timeout", &{})` is called
- **Then:** Env var name uses the app prefix as-is and uppercases only the param part — `MYPREFIX__TIMEOUT` is looked up
- **Tests:** `tests/configurability_tests.rs::custom_env_var_casing_preserve_app_name` ⏳ (not yet written)

### AP-07: custom local_permanent_prefix() is actually used

- **Given:** `ConfigPaths` overrides `local_permanent_prefix()` → `"_PERM_"`; `app_name()` → `"custom"`
- **When:** `get_local_config_path()` is called
- **Then:** Returned path contains `_PERM_custom` — proves prefix method is called not hardcoded `"."`
- **Tests:** `tests/configurability_tests.rs::custom_local_permanent_prefix_actually_used`

### AP-08: XDG_CONFIG_HOME used when PRO is unset

- **Given:** `PRO` env var is unset; `XDG_CONFIG_HOME=/tmp/xdg_test` is set
- **When:** `get_global_config_path()` is called
- **Then:** Returns a path under `/tmp/xdg_test/` — OS fallback activated
- **Tests:** `tests/configurability_tests.rs::xdg_config_home_used_as_fallback` ⏳ (not yet written)

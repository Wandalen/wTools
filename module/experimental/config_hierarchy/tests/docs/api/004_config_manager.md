# API Spec: ConfigManager Type

### Scope

- **Element:** `api/004_config_manager`
- **Source:** `docs/api/004_config_manager.md`
- **Feature flag:** `enabled` (core operations); `file_ops` (I/O); `display_table`/`display_json`/`display_yaml` (display)
- **Prefix:** `CM-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| CM-01 | manager_type_has_zero_size | structural | ✅ |
| CM-02 | resolve_all_returns_all_declared_params | nominal | ✅ |
| CM-03 | file_io_methods_require_file_ops_feature | nominal | ✅ |
| CM-04 | validate_all_calls_cross_param_hook | nominal | ✅ |
| CM-05 | different_type_params_are_independent_types | structural | ✅ |
| CM-06 | missing_value_resolves_to_null_not_error | edge | ✅ |

---

### CM-01: manager type occupies zero bytes

- **Given:** A `ConfigManager<D, P, V>` with any valid combination of trait implementations
- **When:** `std::mem::size_of::<ConfigManager<D, P, V>>()` is evaluated
- **Then:** Returns `0` — no fields, no heap allocation from construction
- **Tests:** `tests/configurability_tests.rs`

### CM-02: resolve_all returns all declared parameters

- **Given:** `get_parameter_names()` returns `["a", "b", "c"]`; defaults provide values for all three
- **When:** `resolve_all_config()` is called with no runtime, env, or file overrides
- **Then:** Result map contains entries for all three parameter names with their default values and `Default` source label
- **Tests:** `tests/basic_operations_tests.rs`

### CM-03: file I/O methods require file_ops feature

- **Given:** The `file_ops` feature is not enabled
- **When:** A file I/O operation (load/save global or local config) is attempted
- **Then:** The method does not compile — it is not part of the public API without `file_ops`
- **Tests:** Compile-time property; verified by feature-gated method availability in `tests/feature_tests.rs`

### CM-04: validate_all calls the cross-parameter validation hook

- **Given:** A validator that rejects the combination of two parameters when both are present
- **When:** `validate_all_config()` is called with a config map containing both parameters
- **Then:** Returns the expected `ValidationError` — the cross-parameter hook is invoked with the full map
- **Tests:** `tests/validator_tests.rs`

### CM-05: different type parameter combinations produce independent types

- **Given:** `ConfigManager<D1, P, V>` and `ConfigManager<D2, P, V>` where `D1` and `D2` are different structs
- **When:** Type checking occurs at compile time
- **Then:** The two manager types are distinct; one cannot be passed where the other is expected
- **Tests:** Compile-time type safety; implicit in any test using concrete type aliases

### CM-06: missing parameter resolves to null without error

- **Given:** A parameter not present in defaults, runtime map, env, or any config file
- **When:** `resolve_config_value()` is called for that parameter
- **Then:** Returns `(JsonValue::Null, ConfigSource::Default)` — no panic, no error, just a null with default source label
- **Tests:** `tests/basic_operations_tests.rs`

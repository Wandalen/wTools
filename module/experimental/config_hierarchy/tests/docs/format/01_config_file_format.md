# Format Spec: Config File Format

### Scope

- **Element:** `format/001_config_file_format`
- **Source:** `docs/format/001_config_file_format.md`
- **Feature flag:** `file_ops`
- **Prefix:** `FM-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FM-01 | canonical_format_loads_correctly | nominal | ✅ |
| FM-02 | missing_parameters_section_returns_empty | boundary | ⏳ |
| FM-03 | unknown_top_level_keys_ignored | boundary | ⏳ |
| FM-04 | created_at_preserved_on_resave | invariant | ⏳ |
| FM-05 | null_values_valid_and_round_trip | nominal | ⏳ |
| FM-06 | corrupted_file_returns_error | error | ✅ |

---

### FM-01: canonical format (metadata + parameters) loads correctly

- **Given:** A YAML file with both `metadata` and `parameters` sections; `parameters.timeout = 60`
- **When:** `load_config_file()` is called
- **Then:** Returns `Ok(map)` where `map["timeout"] == JsonValue::Number(60)`; `metadata` fields not in map
- **Tests:** `tests/basic_operations_tests.rs::*` (multiple tests exercise this path)

### FM-02: missing `parameters` section returns empty map

- **Given:** A valid YAML file with only `metadata` and no `parameters` key
- **When:** `load_config_file()` is called
- **Then:** Returns `Ok({})` — empty map, no error
- **Tests:** `tests/edge_cases_tests.rs::test_missing_parameters_section_returns_empty` ⏳ (not yet written)

### FM-03: unknown top-level keys are ignored on load

- **Given:** A YAML file with `metadata:`, `parameters:`, and an unknown key `custom_section: {foo: bar}`
- **When:** `load_config_file()` is called
- **Then:** Returns `Ok(map)` with only the parameters content; `custom_section` not in map; no error
- **Tests:** `tests/edge_cases_tests.rs::test_unknown_top_level_keys_ignored` ⏳ (not yet written)

### FM-04: `created_at` is preserved across saves

- **Given:** A config file saved with `save_config_file()`; original `created_at` timestamp recorded
- **When:** `save_config_file()` is called again on the same path with modified parameters
- **Then:** `created_at` in the re-saved file equals the original timestamp; `last_modified` is updated
- **Tests:** `tests/edge_cases_tests.rs::test_created_at_preserved_on_resave` ⏳ (not yet written)

### FM-05: null parameter values round-trip correctly

- **Given:** Config map contains `"key" → JsonValue::Null`
- **When:** `save_config_file()` then `load_config_file()` is called
- **Then:** Loaded map contains `"key" → JsonValue::Null` — null survives the YAML round-trip
- **Tests:** `tests/edge_cases_tests.rs::test_null_value_round_trips` ⏳ (not yet written)

### FM-06: corrupted YAML file returns Err

- **Given:** A file at the config path contains invalid YAML (`"invalid: yaml: [unclosed"`)
- **When:** `load_config_file()` is called
- **Then:** Returns `Err(String)` — parse failure propagated cleanly, no panic
- **Tests:** `tests/edge_cases_tests.rs::test_corrupted_yaml_returns_error`

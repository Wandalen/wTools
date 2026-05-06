# Algorithm Spec: Resolution Waterfall

### Scope

- **Element:** `algorithm/002_resolution_waterfall`
- **Source:** `docs/algorithm/002_resolution_waterfall.md`
- **Feature flag:** `enabled` (levels 1–2, 6); `file_ops` (levels 3–5)
- **Prefix:** `RW-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| RW-01 | runtime_override_wins_over_all | invariant | ✅ |
| RW-02 | env_var_wins_over_file_levels | invariant | ✅ |
| RW-03 | local_current_wins_over_local_parent | invariant | ✅ |
| RW-04 | local_wins_over_global | invariant | ✅ |
| RW-05 | global_wins_over_default | invariant | ✅ |
| RW-06 | short_circuit_stops_at_first_hit | invariant | ✅ |
| RW-07 | secondary_scan_discovers_undeclared_keys | nominal | ✅ |

---

### RW-01: runtime override wins over all other levels

- **Given:** Runtime override map, env var, local config file, and default all provide a value for the same parameter
- **When:** `resolve_config_value()` is called with a non-empty runtime override map
- **Then:** Runtime value returned; all lower-priority sources ignored
- **Tests:** `tests/hierarchy_tests.rs::test_runtime_overrides_env`

### RW-02: environment variable wins over file levels

- **Given:** Env var is set for a parameter; local and global config files also contain a value for it; no runtime override
- **When:** `resolve_config_value()` is called
- **Then:** Env var value returned — level 2 beats levels 3–5
- **Tests:** `tests/hierarchy_tests.rs::test_env_overrides_default`

### RW-03: LocalCurrent wins over LocalParent

- **Given:** Config file exists in both the current directory and a parent directory with different values; no runtime/env override
- **When:** `resolve_config_value()` is called with `file_ops` feature
- **Then:** Current-directory value returned — depth 0 beats depth 1+
- **Tests:** `tests/hierarchy_tests.rs::test_local_current_overrides_local_parent`

### RW-04: local config wins over global config

- **Given:** Both a local config file and a global config file contain a value for the same parameter; no runtime/env override
- **When:** `resolve_config_value()` is called with `file_ops` feature
- **Then:** Local config value returned — levels 3–4 beat level 5
- **Tests:** `tests/hierarchy_tests.rs::test_local_config_overrides_global`

### RW-05: global config wins over application default

- **Given:** Global config file contains a value for a parameter; no runtime, env, or local override
- **When:** `resolve_config_value()` is called with `file_ops` feature
- **Then:** Global config value returned — level 5 beats level 6
- **Tests:** `tests/hierarchy_tests.rs::test_global_config_overrides_default`

### RW-06: short-circuit stops at first hit

- **Given:** Runtime override contains a value for a parameter; env var and config files also contain values
- **When:** `resolve_config_value()` is called
- **Then:** Only the runtime value is used; the algorithm does not read env vars or open any config files for this parameter (observable via test isolation)
- **Tests:** `tests/hierarchy_tests.rs::test_runtime_overrides_env`

### RW-07: secondary scan discovers undeclared keys from config files

- **Given:** A config file contains a key that is not in `get_parameter_names()`
- **When:** `resolve_all_config()` is called
- **Then:** The undeclared key appears in the resolved map, resolved through the standard 6-level waterfall
- **Tests:** `tests/feature_tests.rs` (secondary scan coverage)

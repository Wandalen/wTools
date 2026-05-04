# Invariant Spec: Resolution Hierarchy

### Scope

- **Element:** `invariant/001_resolution_hierarchy`
- **Source:** `docs/invariant/001_resolution_hierarchy.md`
- **Feature flag:** `enabled` (env level); `file_ops` (file levels 3–5)
- **Prefix:** `IN-`
- **Minimum cases:** 2

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | priority_ordering_enforced | invariant | ✅ |
| IN-02 | depth_beats_pattern_within_local | invariant | ✅ |
| IN-03 | local_config_overrides_global | invariant | ✅ |
| IN-04 | no_merging_first_wins | invariant | ✅ |

---

### IN-01: priority ordering enforced (runtime > env > file > default)

- **Given:** Runtime params, env var, and default all provide a value for the same parameter
- **When:** `resolve_config_value()` is called
- **Then:** Runtime value returned when present; env value returned when runtime absent; default returned when both absent — no mixing, first source wins completely
- **Tests:** `tests/hierarchy_tests.rs::test_runtime_overrides_env`, `test_env_overrides_default`

### IN-02: directory depth beats pattern type

- **Given:** A permanent (`.app`) config in the current directory and a temporary (`-app`) config in a parent directory
- **When:** `resolve_config_value()` is called with `file_ops` feature
- **Then:** Current directory value returned — L3 (LocalCurrent) beats L4 (LocalParent) regardless of temp vs. perm pattern
- **Tests:** `tests/hierarchy_tests.rs::test_local_current_overrides_local_parent`

### IN-03: local config overrides global config

- **Given:** Both a local config file and a global config file exist with different values for the same parameter; no runtime or env override
- **When:** `resolve_config_value()` is called with `file_ops` feature
- **Then:** Local config value returned — L3/L4 (Local) beats L5 (Global)
- **Tests:** `tests/hierarchy_tests.rs::test_local_config_overrides_global`

### IN-04: no merging — first source wins completely

- **Given:** Both global and local config files exist with different sets of keys; local has `"a"`, global has `"b"`
- **When:** `resolve_config_value("b", &{})` is called
- **Then:** Returns global value for `"b"` — the `"a"` from local does not suppress global `"b"`; each parameter resolved independently
- **Tests:** `tests/hierarchy_tests.rs::test_global_config_overrides_default` (implicit — global value used when local absent)

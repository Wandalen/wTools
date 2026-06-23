# Test Surface: Invariant — Feature Gating Contract

### Source

- **Doc Instance:** [invariant/002_feature_gating_contract.md](../../../docs/invariant/002_feature_gating_contract.md)

### Cases

| # | Status | Case |
|---|--------|------|
| IN-1 | ✅ | Default features include enabled and core capabilities |
| IN-2 | ✅ | Enabled-only compiles without warnings |
| IN-3 | ⏳ | Each capability is individually gated |
| IN-4 | ⏳ | Full feature activates all capabilities |

### IN-1 — Default features include enabled and core capabilities

- **Given:** A fresh `cargo add strs_tools` with no feature overrides
- **When:** The crate is compiled with default features
- **Then:** The `enabled`, `string_indentation`, and `string_parse_number` features are active
- **Test:** `tests/issue_002_example_feature_guards.rs`

### IN-2 — Enabled-only compiles without warnings

- **Given:** The crate compiled with only the `enabled` feature (`default-features = false, features = ["enabled"]`)
- **When:** The compiler runs with `-D warnings`
- **Then:** Compilation succeeds with zero warnings and no public symbols beyond the crate root
- **Test:** `tests/issue_002_example_feature_guards.rs`

### IN-3 — Each capability is individually gated

- **Given:** A specific capability feature (e.g., `string_split`) is disabled
- **When:** Code attempts to use the split API
- **Then:** The symbols are absent — compilation fails with an unresolved import
- **Test:** ⏳

### IN-4 — Full feature activates all capabilities

- **Given:** The crate compiled with the `full` feature
- **When:** All capability APIs are accessed
- **Then:** Every API is available and functional
- **Test:** ⏳

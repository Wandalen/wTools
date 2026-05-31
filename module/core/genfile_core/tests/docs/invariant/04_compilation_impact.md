# Test Spec: Compilation Impact

- **Source**: `docs/invariant/004_compilation_impact.md`
- **Prefix**: `IN-04`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-04-1 | clean_build_time_delta_within_bound | ⏳ |
| IN-04-2 | dependency_count_remains_minimal | ⏳ |

[PENDING — build time measurement infrastructure not yet in place]

---

### IN-04-1: clean_build_time_delta_within_bound

- **Given:** A reference crate measured before and after adding `genfile_core` as a dependency
- **When:** A clean build is timed for both versions
- **Then:** The delta is ≤ 5 seconds on standard development hardware

[PENDING — requires timed build tooling — see task for CI build measurement]

---

### IN-04-2: dependency_count_remains_minimal

- **Given:** The `Cargo.toml` dependency declarations for `genfile_core`
- **When:** `cargo tree --package genfile_core` is inspected
- **Then:** The transitive dependency count reflects only the declared minimal footprint (Handlebars, serde, base64, regex, error_tools)

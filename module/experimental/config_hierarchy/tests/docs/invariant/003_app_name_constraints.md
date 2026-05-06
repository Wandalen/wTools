# Invariant Spec: App Name Constraints

### Scope

- **Element:** `invariant/003_app_name_constraints`
- **Source:** `docs/invariant/003_app_name_constraints.md`
- **Feature flag:** `enabled`
- **Prefix:** `AN-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AN-01 | empty_name_rejected | invariant | ✅ |
| AN-02 | forward_slash_rejected | invariant | ✅ |
| AN-03 | backslash_rejected | invariant | ✅ |
| AN-04 | double_dot_rejected | invariant | ✅ |
| AN-05 | valid_alphanumeric_accepted | nominal | ✅ |
| AN-06 | valid_hyphen_underscore_accepted | nominal | ✅ |

---

### AN-01: empty app name is rejected

- **Given:** `app_name()` returns `""`
- **When:** Any path construction function is called (global config path, local config path, or discovery)
- **Then:** Returns `Err(String)` — does not construct a path with an empty segment
- **Tests:** `tests/path_standards_tests.rs`

### AN-02: forward slash in app name is rejected

- **Given:** `app_name()` returns a name containing `/` (e.g., `"my/app"`)
- **When:** Any path construction function is called
- **Then:** Returns `Err(String)` — directory traversal via path separator is blocked
- **Tests:** `tests/path_standards_tests.rs`

### AN-03: backslash in app name is rejected

- **Given:** `app_name()` returns a name containing `\` (e.g., `"my\\app"`)
- **When:** Any path construction function is called
- **Then:** Returns `Err(String)` — directory traversal via Windows path separator is blocked
- **Tests:** `tests/path_standards_tests.rs`

### AN-04: double-dot in app name is rejected

- **Given:** `app_name()` returns a name containing `..` (e.g., `"../etc"`, `"my..app"`)
- **When:** Any path construction function is called
- **Then:** Returns `Err(String)` — path traversal attack via parent directory reference is blocked
- **Tests:** `tests/path_standards_tests.rs`, `tests/edge_cases_tests.rs`

### AN-05: valid alphanumeric name proceeds to path construction

- **Given:** `app_name()` returns `"myapp123"` — alphanumeric, no special characters
- **When:** Path construction is called
- **Then:** Returns `Ok(path)` — constraint validation passes, path is constructed
- **Tests:** `tests/path_standards_tests.rs`

### AN-06: valid name with hyphen and underscore is accepted

- **Given:** `app_name()` returns `"my-app_v2"` — hyphens and underscores are allowed
- **When:** Path construction is called
- **Then:** Returns `Ok(path)` — constraint validation passes, path is constructed
- **Tests:** `tests/path_standards_tests.rs`

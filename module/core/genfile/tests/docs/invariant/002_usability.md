# Invariant Spec: Usability

### Scope

- **Element:** `invariant/002_usability`
- **Source:** `docs/invariant/002_usability.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | all_commands_follow_dot_prefix_naming | nominal | 🔶 deferred |
| IN-02 | all_params_use_double_colon_format | nominal | 🔶 deferred |
| IN-03 | verbosity_zero_suppresses_non_error_output | nominal | 🔶 deferred |

---

### IN-01: all commands follow dot-prefix naming convention

- **Given:** The list of all registered commands is obtained
- **When:** Each command name is checked against the dot-prefix snake_case pattern
- **Then:** Every command name begins with `.` and uses only lowercase letters, digits, underscores, and dots
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### IN-02: all parameters use double-colon format

- **Given:** Any command that accepts parameters is invoked with `param::value` format
- **When:** The parameter is passed to the command
- **Then:** Command executes successfully; the parameter is parsed correctly
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### IN-03: verbosity::0 suppresses non-error output

- **Given:** Any command is run
- **When:** `verbosity::0` is passed
- **Then:** On success, stdout is empty; errors still appear on stderr
- **Behavioral Divergence:** `verbosity::0` → empty stdout on success; `verbosity::1` → progress/result lines present
- **Tests:** none — see task/001_fill_test_surface_gaps.md

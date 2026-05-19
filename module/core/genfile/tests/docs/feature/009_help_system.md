# Feature Spec: Help System

### Scope

- **Element:** `feature/009_help_system`
- **Source:** `docs/feature/009_help_system.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | universal_help_lists_non_help_commands | nominal | 🔶 deferred |
| FT-02 | per_command_help_shows_parameters | nominal | 🔶 deferred |
| FT-03 | help_commands_filtered_from_listings | nominal | 🔶 deferred |
| FT-04 | dot_help_alias_works_same_as_dot | nominal | 🔶 deferred |

---

### FT-01: universal help lists non-help commands

- **Given:** genfile binary is available
- **When:** `.` or `.help` is invoked
- **Then:** Exit code 0; output lists `.archive.new`, `.file.add`, and other non-help commands; no `.archive.new.help` entries in the list
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### FT-02: per-command help shows parameters and examples

- **Given:** genfile binary is available
- **When:** `.archive.new.help` is invoked
- **Then:** Exit code 0; output describes `.archive.new`, lists its parameters with kinds/defaults, and shows usage examples
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### FT-03: help commands filtered from normal listings

- **Given:** genfile binary is available
- **When:** `.` is invoked (universal help)
- **Then:** Output does NOT include `.archive.new.help` or any `*.help` entries
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### FT-04: .help alias behaves identically to .

- **Given:** genfile binary is available
- **When:** `.help` is invoked
- **Then:** Exit code 0; output is equivalent to `.` invocation
- **Tests:** none — see task/001_fill_test_surface_gaps.md

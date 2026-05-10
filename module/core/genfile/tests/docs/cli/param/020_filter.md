# Parameter Spec: filter::

### Scope

- **Element:** `parameter/filter`
- **Source:** `docs/cli/param.md#parameter--20-filter`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-51 | filter_inline_shows_only_inline_files | nominal | ✅ |
| EC-52 | filter_reference_shows_only_reference_files | nominal | ✅ |
| EC-53 | no_filter_shows_all_files | nominal | ✅ |

---

### EC-51: filter inline shows only inline files

- **Given:** Archive has 2 inline files and 3 reference files
- **When:** `.content.list filter::inline` is run
- **Then:** Exit code 0; output lists only the 2 inline files
- **Tests:** `tests/content_commands_test.rs`

### EC-52: filter reference shows only reference files

- **Given:** Archive has 2 inline files and 3 reference files
- **When:** `.content.list filter::reference` is run
- **Then:** Exit code 0; output lists only the 3 reference files
- **Tests:** `tests/content_commands_test.rs`

### EC-53: no filter shows all files

- **Given:** Archive has 2 inline files and 3 reference files
- **When:** `.content.list` is run (no `filter::`)
- **Then:** Exit code 0; output shows all 5 files grouped by mode
- **Tests:** `tests/content_commands_test.rs`

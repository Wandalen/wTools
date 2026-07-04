# Parameter Spec: filter::

### Scope

- **Element:** `parameter/filter`
- **Source:** `docs/cli/param.md#parameter--20-filter`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-51 | filter_inline_shows_only_inline_files | nominal | ✅ |
| EC-52 | filter_reference_shows_only_reference_files | nominal | ✅ |
| EC-53 | no_filter_shows_all_files | nominal | ✅ |
| EC-126 | filter_with_no_matches_shows_empty_list | nominal | 🚧 |
| EC-127 | filter_with_partial_path_match | nominal | 🚧 |
| EC-128 | filter_null_shows_all_content | nominal | 🚧 |

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

### EC-126: filter with no matches shows empty list

- **Given:** Archive has only inline files (no reference files)
- **When:** `.content.list filter::reference` is run
- **Then:** Exit code 0; output is empty or shows empty section (no error for zero matches)
- **Tests:** `tests/content_commands_test.rs`

### EC-127: filter with partial path match

- **Given:** Archive has files at `src/main.rs`, `src/lib.rs`, and `tests/test.rs`
- **When:** `.content.list filter::"src/**"` is run
- **Then:** Exit code 0; output lists only `src/main.rs` and `src/lib.rs`
- **Tests:** `tests/content_commands_test.rs`

### EC-128: filter null shows all content

- **Given:** Archive has 2 inline files and 3 reference files
- **When:** `.content.list filter::""` is run (empty string as filter)
- **Then:** Exit code 0; output shows all 5 files (empty filter treated as no filter)
- **Tests:** `tests/content_commands_test.rs`

# CLI Spec: Content Commands

### Scope

- **Element:** Commands `9–11` (`.content.*` namespace)
- **Source:** `docs/cli/command/content.md`
- **Prefix:** `IT-`
- **Minimum cases:** 5

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-17 | content_internalize_embeds_reference_files | nominal | ✅ |
| IT-18 | content_internalize_dry_run_no_changes | nominal | ✅ |
| IT-19 | content_externalize_writes_files_to_dir | nominal | ✅ |
| IT-20 | content_list_shows_both_modes | nominal | ✅ |
| IT-21 | content_list_filter_inline_shows_inline_only | nominal | ✅ |

---

### IT-17: content.internalize embeds reference files

- **Given:** An archive with 2 reference-mode files (paths, not content)
- **When:** `.content.internalize` is run
- **Then:** Exit code 0; archive now has 2 inline files; reference count is 0
- **Tests:** `tests/content_commands_test.rs`

### IT-18: content.internalize dry run makes no changes

- **Given:** An archive with reference files
- **When:** `.content.internalize dry::1` is run
- **Then:** Exit code 0; archive reference files unchanged; output contains `[DRY RUN]`
- **Tests:** `tests/content_commands_test.rs`

### IT-19: content.externalize writes files to output directory

- **Given:** An archive with 2 inline files
- **When:** `.content.externalize output_dir::<dir>` is run
- **Then:** Exit code 0; files appear on disk in `<dir>`; archive switches to reference mode
- **Tests:** `tests/content_commands_test.rs`

### IT-20: content.list shows both modes

- **Given:** An archive with 2 inline files and 3 reference files
- **When:** `.content.list` is run
- **Then:** Exit code 0; output shows `Inline (2 files)` section and `Reference (3 files)` section
- **Tests:** `tests/content_commands_test.rs`

### IT-21: content.list filter inline shows inline only

- **Given:** An archive with inline and reference files
- **When:** `.content.list filter::inline` is run
- **Then:** Exit code 0; only inline files listed; reference section absent
- **Tests:** `tests/content_commands_test.rs`

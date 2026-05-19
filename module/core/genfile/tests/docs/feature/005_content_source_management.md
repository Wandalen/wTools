# Feature Spec: Content Source Management

### Scope

- **Element:** `feature/005_content_source_management`
- **Source:** `docs/feature/005_content_source_management.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | internalize_embeds_file_references | nominal | ✅ |
| FT-02 | list_shows_content_by_type | nominal | ✅ |
| FT-03 | internalize_produces_self_contained_archive | nominal | ✅ |
| FT-04 | externalize_replaces_inline_with_references | nominal | ✅ |

---

### FT-01: internalize embeds external file references

- **Given:** An archive with a file referencing an external path is loaded
- **When:** `.content.internalize` is run
- **Then:** Exit code 0; the file's content is now inline (no external reference)
- **Tests:** `tests/content_commands_test.rs`

### FT-02: list shows content sources by type

- **Given:** An archive with a mix of inline and reference content is loaded
- **When:** `.content.list` is run
- **Then:** Exit code 0; output differentiates inline vs file-reference entries
- **Tests:** `tests/content_commands_test.rs`

### FT-03: internalize produces self-contained archive

- **Given:** An archive with external references is loaded
- **When:** `.content.internalize` followed by `.archive.save` is run
- **Then:** The saved archive loads correctly with no external file dependencies
- **Tests:** `tests/content_commands_test.rs`

### FT-04: externalize replaces inline content with references

- **Given:** An archive with inline content is loaded; destination directory exists
- **When:** `.content.externalize` is run
- **Then:** Exit code 0; inline content is replaced by file references
- **Tests:** `tests/content_commands_test.rs`

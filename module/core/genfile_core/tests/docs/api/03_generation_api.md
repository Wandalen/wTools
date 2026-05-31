# Test Spec: Generation API

- **Source**: `docs/api/003_generation_api.md`
- **Prefix**: `AP-03`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| AP-03-1 | holder_generate_renders_template_and_writes_file | ⏳ |
| AP-03-2 | holder_generate_returns_written_paths | ⏳ |
| AP-03-3 | holder_generate_fails_on_missing_mandatory | ⏳ |
| AP-03-4 | archive_generate_uses_stored_values | ⏳ |
| AP-03-5 | static_descriptor_skips_renderer | ⏳ |

---

### AP-03-1: holder_generate_renders_template_and_writes_file

- **Given:** A template holder with one file descriptor (template content `"Hello {{name}}"`), a value map with `name = "World"`, and a memory file system
- **When:** `generate` is called with a base output path
- **Then:** The file system contains the rendered content `"Hello World"` at the expected output path

---

### AP-03-2: holder_generate_returns_written_paths

- **Given:** A template holder with two file descriptors targeting different relative paths
- **When:** `generate` is called
- **Then:** The return value is `Ok(paths)` where `paths` contains both output file paths

---

### AP-03-3: holder_generate_fails_on_missing_mandatory

- **Given:** A template holder with one mandatory parameter `"version"` and a value map that does not supply `"version"`
- **When:** `generate` is called
- **Then:** The return value is `Err(Error::MissingParameters(...))` containing `"version"` in the parameter list

---

### AP-03-4: archive_generate_uses_stored_values

- **Given:** A template archive loaded from a JSON/YAML document that embeds parameter values inline
- **When:** `generate` is called without supplying additional external values
- **Then:** The rendered output uses the values embedded in the archive document

---

### AP-03-5: static_descriptor_skips_renderer

- **Given:** A file descriptor with `is_template = false` and static content `"static text"`
- **When:** `generate` is called
- **Then:** The output file content is `"static text"` verbatim with no template substitution applied

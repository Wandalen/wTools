# Test Spec: File Descriptor

- **Source**: `docs/feature/008_file_descriptor.md`
- **Prefix**: `FT-08`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-08-1 | descriptor_stores_output_path | ⏳ |
| FT-08-2 | template_flag_controls_rendering | ⏳ |
| FT-08-3 | write_mode_is_stored | ⏳ |
| FT-08-4 | content_is_stored | ⏳ |

---

### FT-08-1: descriptor_stores_output_path

- **Given:** A file descriptor built with path `"src/main.rs"`
- **When:** The path field is accessed
- **Then:** It returns `"src/main.rs"`

---

### FT-08-2: template_flag_controls_rendering

- **Given:** One file descriptor with `is_template = true` and another with `is_template = false`
- **When:** Each is inspected
- **Then:** The template flag accurately reflects whether the renderer should be invoked for that descriptor

---

### FT-08-3: write_mode_is_stored

- **Given:** A file descriptor built with `WriteMode::Rewrite`
- **When:** The write mode field is accessed
- **Then:** It returns the `Rewrite` variant; a descriptor built with `TomlExtend` returns that variant

---

### FT-08-4: content_is_stored

- **Given:** A file descriptor built with template content `"{{greeting}}, world!"`
- **When:** The content field is accessed
- **Then:** It returns the original template string unchanged

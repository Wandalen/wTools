# Test Spec: Template Generation

- **Source**: `docs/feature/014_template_generation.md`
- **Prefix**: `FT-14`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-14-1 | generation_renders_template_descriptors | ⏳ |
| FT-14-2 | generation_copies_static_descriptors_verbatim | ⏳ |
| FT-14-3 | generation_fails_before_writing_on_missing_mandatory | ⏳ |
| FT-14-4 | generation_returns_list_of_written_paths | ⏳ |
| FT-14-5 | generation_joins_base_dir_with_relative_path | ⏳ |

---

### FT-14-1: generation_renders_template_descriptors

- **Given:** A template holder with one template descriptor `"{{title}}"` and value `title = "Hello"`
- **When:** `generate("/out")` is called
- **Then:** The output file contains `"Hello"`

---

### FT-14-2: generation_copies_static_descriptors_verbatim

- **Given:** A template holder with one static descriptor (not a template) containing `"<literal>"`
- **When:** `generate` is called
- **Then:** The output file contains `"<literal>"` unchanged (no template substitution)

---

### FT-14-3: generation_fails_before_writing_on_missing_mandatory

- **Given:** A template holder with mandatory parameter `"db_url"` not present in the value map and one file descriptor
- **When:** `generate` is called
- **Then:** The return is `Err(MissingParameters(...))` and no file is written to the file system

---

### FT-14-4: generation_returns_list_of_written_paths

- **Given:** A template holder with three file descriptors targeting `"a.txt"`, `"b.txt"`, `"c.txt"`
- **When:** `generate("/base")` is called successfully
- **Then:** The returned `Ok(paths)` contains all three resolved paths

---

### FT-14-5: generation_joins_base_dir_with_relative_path

- **Given:** A template holder with a descriptor at relative path `"sub/file.rs"` and base directory `"/project"`
- **When:** `generate("/project")` is called
- **Then:** The file is written at `"/project/sub/file.rs"` (base + relative joined)

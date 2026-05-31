# Test Spec: Write Mode Support

- **Source**: `docs/feature/009_write_mode_support.md`
- **Prefix**: `FT-09`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-09-1 | rewrite_mode_overwrites_existing_file | ⏳ |
| FT-09-2 | rewrite_mode_creates_new_file | ⏳ |
| FT-09-3 | toml_extend_preserves_existing_keys | ⏳ |
| FT-09-4 | toml_extend_adds_new_keys | ⏳ |

---

### FT-09-1: rewrite_mode_overwrites_existing_file

- **Given:** A memory file system with an existing file at `"config.toml"` containing `"old"` and a descriptor using `WriteMode::Rewrite` with content `"new"`
- **When:** Generation writes the file
- **Then:** The file at `"config.toml"` contains `"new"`, not `"old"`

---

### FT-09-2: rewrite_mode_creates_new_file

- **Given:** A memory file system with no existing file and a descriptor using `WriteMode::Rewrite`
- **When:** Generation writes the file
- **Then:** The file exists in the file system with the expected content

---

### FT-09-3: toml_extend_preserves_existing_keys

- **Given:** An existing TOML file with key `[package] name = "existing"` and a descriptor using `WriteMode::TomlExtend` that introduces a new key
- **When:** Generation writes the file
- **Then:** The existing `name = "existing"` key is still present in the output

---

### FT-09-4: toml_extend_adds_new_keys

- **Given:** An existing TOML file missing key `"version"` and a descriptor using `WriteMode::TomlExtend` that supplies `version = "1.0.0"`
- **When:** Generation writes the file
- **Then:** The output file contains `version = "1.0.0"`

# Test Spec: File System Trait

- **Source**: `docs/feature/010_file_system_trait.md`
- **Prefix**: `FT-10`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-10-1 | write_operation_stores_bytes | ⏳ |
| FT-10-2 | read_operation_returns_stored_bytes | ⏳ |
| FT-10-3 | read_fails_for_missing_path | ⏳ |
| FT-10-4 | create_directory_all_succeeds | ⏳ |

---

### FT-10-1: write_operation_stores_bytes

- **Given:** An implementation of the file system trait (memory or real)
- **When:** `write("file.txt", b"content")` is called
- **Then:** The file exists and its bytes equal `b"content"`

---

### FT-10-2: read_operation_returns_stored_bytes

- **Given:** A file system with a previously written file at `"data.bin"` containing bytes `[1, 2, 3]`
- **When:** `read("data.bin")` is called
- **Then:** The returned bytes are `[1, 2, 3]`

---

### FT-10-3: read_fails_for_missing_path

- **Given:** A file system with no file at `"nonexistent.txt"`
- **When:** `read("nonexistent.txt")` is called
- **Then:** An error is returned (not a panic)

---

### FT-10-4: create_directory_all_succeeds

- **Given:** A file system implementation
- **When:** `create_directory_all("a/b/c")` is called
- **Then:** The call returns `Ok(())` without error

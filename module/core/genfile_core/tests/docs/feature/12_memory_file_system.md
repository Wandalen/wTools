# Test Spec: Memory File System

- **Source**: `docs/feature/012_memory_file_system.md`
- **Prefix**: `FT-12`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-12-1 | write_stores_bytes_in_memory | ⏳ |
| FT-12-2 | read_returns_stored_bytes | ⏳ |
| FT-12-3 | read_fails_for_absent_path | ⏳ |
| FT-12-4 | create_directory_all_is_noop | ⏳ |

---

### FT-12-1: write_stores_bytes_in_memory

- **Given:** A `MemoryFileSystem` with no entries
- **When:** `write("readme.md", b"# Hello")` is called
- **Then:** `read("readme.md")` returns `Ok(b"# Hello")` without touching the real filesystem

---

### FT-12-2: read_returns_stored_bytes

- **Given:** A `MemoryFileSystem` that has an entry `"data.json"` = `b"{}"`
- **When:** `read("data.json")` is called
- **Then:** The returned bytes are `b"{}"`

---

### FT-12-3: read_fails_for_absent_path

- **Given:** An empty `MemoryFileSystem`
- **When:** `read("missing.txt")` is called
- **Then:** An error is returned indicating the file does not exist

---

### FT-12-4: create_directory_all_is_noop

- **Given:** A `MemoryFileSystem`
- **When:** `create_directory_all("a/b/c")` is called
- **Then:** The call returns `Ok(())` and no phantom directory entries are added to the internal map; a subsequent `read("a/b/c")` still returns `Err`

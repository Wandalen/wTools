# Test Spec: Real File System

- **Source**: `docs/feature/011_real_file_system.md`
- **Prefix**: `FT-11`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-11-1 | write_creates_parent_directories | ⏳ |
| FT-11-2 | write_creates_file_with_correct_content | ⏳ |
| FT-11-3 | read_returns_bytes_written_earlier | ⏳ |
| FT-11-4 | io_error_is_wrapped_in_typed_variant | ⏳ |

---

### FT-11-1: write_creates_parent_directories

- **Given:** A `RealFileSystem` and a nested path `"a/b/c/file.txt"` where `a/b/c/` does not exist in a temp directory
- **When:** `write` is called with that path
- **Then:** The directory `a/b/c/` is created on disk and the file exists

---

### FT-11-2: write_creates_file_with_correct_content

- **Given:** A `RealFileSystem` and a target path in a temp directory
- **When:** `write(path, b"hello")` is called
- **Then:** Reading the file at that path with standard I/O returns `b"hello"`

---

### FT-11-3: read_returns_bytes_written_earlier

- **Given:** A `RealFileSystem` that has written `b"round-trip"` to a path in a temp directory
- **When:** `read` is called on the same path
- **Then:** The returned bytes are identical to `b"round-trip"` (byte-for-byte equality)

---

### FT-11-4: io_error_is_wrapped_in_typed_variant

- **Given:** A `RealFileSystem` and a `read` call on a path that does not exist
- **When:** The call is made
- **Then:** The error is returned as the filesystem error variant (not a raw panic or unhandled `io::Error`)

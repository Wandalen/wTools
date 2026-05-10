# File Operations

File management commands for adding, removing, listing, and inspecting files in archives. Provides CRUD operations on individual file entities.

- **Namespace:** file
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 12 | [.file.add](#command--12-fileadd) | Add file to archive | 5 | 7 |
| 13 | [.file.remove](#command--13-fileremove) | Remove file from archive | 2 | 4 |
| 14 | [.file.list](#command--14-filelist) | List all files with metadata | 1 | 1 |
| 15 | [.file.show](#command--15-fileshow) | Display file content from archive | 2 | 4 |

---

### Command :: 12. `.file.add`

### Description

Adds a file to the current archive with specified content or from a filesystem source. Use this to build an archive incrementally, one file at a time.

-- **Parameters:** path::, content::, from_file::, write_mode::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (invalid path or missing content source) | 2 (file read error)

### Syntax

```bash
genfile .file.add path::"src/main.rs" content::"fn main() {}"
genfile .file.add path::"readme.md" from_file::"./README.md"
genfile .file.add path::"config.toml" content::"[package]" write_mode::rewrite
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `path::` | [FilePath](../type.md#type--3-filepath) | — | ✅ Yes | File path within the archive |
| `content::` | [ContentString](../type.md#type--9-contentstring) | `null` | No | File content as a string |
| `from_file::` | [FilePath](../type.md#type--3-filepath) | `null` | No | Source file to read content from |
| `write_mode::` | [WriteMode](../type.md#type--12-writemode) | `rewrite` | No | Behavior when file already exists (rewrite \| append \| skip) |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .file.add path::"main.rs" content::"fn main() { println!(\"Hello\"); }"
# Output:
# Added file: main.rs (27 bytes)

genfile .file.add path::"readme.md" from_file::"./README.md"
# Output:
# Added file: readme.md (1.2 KB, read from ./README.md)

genfile .file.add path::"config.toml" content::"[package]" write_mode::skip
# Output:
# Skipped: config.toml (already exists)
```

### Notes

- Exactly one of `content::` or `from_file::` must be provided; using both simultaneously is an error
- `from_file::` requires the source file to exist at invocation time
- `write_mode::rewrite` overwrites an existing archive entry; `skip` leaves it unchanged; `append` appends to existing content
- Operation modifies the in-memory archive only — does not write to the filesystem

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 13 | [`.file.remove`](#command--13-fileremove) | Remove a file from the archive |
| 14 | [`.file.list`](#command--14-filelist) | List files to confirm addition |
| 8 | [`.archive.from_directory`](archive.md#command--8-archivefrom_directory) | Bulk file addition from a directory |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 7
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

---

### Command :: 13. `.file.remove`

### Description

Removes a file from the current in-memory archive. Use this to delete unwanted files before saving or materializing.

-- **Parameters:** path::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (file not found in archive) | 2 (runtime error)

### Syntax

```bash
genfile .file.remove path::"old_file.rs"
genfile .file.remove path::"temp.txt" verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `path::` | [FilePath](../type.md#type--3-filepath) | — | ✅ Yes | Archive path of the file to remove |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .file.remove path::"temp.rs"
# Output:
# Removed file: temp.rs

genfile .file.remove path::"src/old_impl.rs" verbosity::2
# Output:
# Removed file: src/old_impl.rs (was 3.2 KB, inline)
```

### Notes

- File must exist in the archive; fails with exit code 1 if the path is not found
- Operation modifies the in-memory archive only — does not delete files from the filesystem
- Use `.file.list` first to confirm the exact archive path before removing

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 12 | [`.file.add`](#command--12-fileadd) | Add a file to the archive |
| 14 | [`.file.list`](#command--14-filelist) | List files to identify removal targets |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 4
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

---

### Command :: 14. `.file.list`

### Description

Lists all files in the current archive with path, size, and content mode metadata. Use this to inspect archive contents before saving, materializing, or editing.

-- **Parameters:** verbosity::
-- **Exit Codes:** 0 (success) | 2 (runtime error)

### Syntax

```bash
genfile .file.list
genfile .file.list verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .file.list
# Output:
# Files (12 total, 45 KB):
#   src/main.rs (3.2 KB, inline)
#   src/lib.rs (8.1 KB, reference)
#   readme.md (2.5 KB, inline)
#   ...

genfile .file.list verbosity::2
# Includes per-file parameter placeholder counts and last-modified info
```

### Notes

- Files are sorted alphabetically by archive path
- Content mode (inline vs reference) is shown for each file
- Reference files display path only (no size; content is not embedded in archive)

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 11 | [`.content.list`](content.md#command--11-contentlist) | List files grouped by content mode |
| 15 | [`.file.show`](#command--15-fileshow) | Display content of a specific file |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Query
**Complexity:** 1
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

---

### Command :: 15. `.file.show`

### Description

Displays the content of a single file from the archive to stdout. Use this to inspect file contents or verify placeholder syntax without materializing.

-- **Parameters:** path::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (file not found in archive) | 2 (read error)

### Syntax

```bash
genfile .file.show path::"main.rs"
genfile .file.show path::"config.toml" verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `path::` | [FilePath](../type.md#type--3-filepath) | — | ✅ Yes | Archive path of the file to display |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .file.show path::"main.rs"
# Output:
# === main.rs ===
# fn main() {
#     println!("Hello, {{project_name}}!");
# }

genfile .file.show path::"config.toml" verbosity::2
# Includes content mode, size, and parameter placeholder list
```

### Notes

- Works with both inline files (content embedded) and reference files (read from filesystem)
- For reference files, the source file must exist at its recorded path
- Placeholder tokens (`{{name}}`) are displayed as-is — no substitution occurs

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 14 | [`.file.list`](#command--14-filelist) | List available files to find the right path |
| 12 | [`.file.add`](#command--12-fileadd) | Add or replace the file shown |
| 16 | [`.materialize`](operations.md#command--16-materialize) | Render files with placeholder substitution |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Query
**Complexity:** 4
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

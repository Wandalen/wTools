# File Operations

File management commands for adding, removing, listing, and inspecting files in archives. Provides CRUD operations on individual file entities.

- **Namespace:** file
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 12 | [.file.add](#command-12-fileadd) | Add file | 5 | Medium |
| 13 | [.file.remove](#command-13-fileremove) | Remove file | 2 | Low |
| 14 | [.file.list](#command-14-filelist) | List files | 1 | Low |
| 15 | [.file.show](#command-15-fileshow) | Show content | 2 | Low |

---

### Command :: 12. `.file.add`

Adds file to current archive with specified content or from filesystem source. Use this to build archive incrementally.

**Syntax:**
```bash
genfile .file.add path::"src/main.rs" content::"fn main() {}"
genfile .file.add path::"readme.md" from_file::"./README.md"
genfile .file.add path::"config.toml" content::"[package]" write_mode::rewrite
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `path::` | [FilePath](../type.md#type-filepath) | File path within archive | - | ✅ Yes |
| `content::` | [ContentString](../type.md#type-contentstring) | File content (text) | - | Conditional |
| `from_file::` | [FilePath](../type.md#type-filepath) | Source file to read content from | `null` | Conditional |
| `write_mode::` | [WriteMode](../type.md#type-writemode) | Write mode (rewrite \| append \| skip) | - | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*Must provide either `content::` OR `from_file::` (not both)*</small>
<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Adds file to archive in-memory structure
- Content can be specified directly or read from file
- Write mode controls behavior when file already exists
- Does not modify filesystem (only archive)

**Examples:**

**Direct content:**
```bash
genfile .file.add path::"main.rs" content::"fn main() { println!(\"Hello\"); }"
# Output:
# Added file: main.rs (27 bytes)
```

**From filesystem:**
```bash
genfile .file.add path::"readme.md" from_file::"./README.md"
# Output:
# Added file: readme.md (1.2 KB, read from ./README.md)
```

**Exit Codes:** 0 (success) | 1 (invalid path/content) | 2 (file read error)

**Interactions:**
- Conflicts with: Using both `content::` and `from_file::` simultaneously
- Dependencies: `from_file::` requires source file to exist

**Related Commands:**
- [.file.remove](#command-13-fileremove) - Remove file from archive
- [.file.list](#command-14-filelist) - List files in archive
- [.archive.from_directory](archive.md#command-8-archivefromdirectory) - Bulk file addition

---

### Command :: 13. `.file.remove`

Removes file from current archive. Use this to remove unwanted files from archive.

**Syntax:**
```bash
genfile .file.remove path::"old_file.rs"
genfile .file.remove path::"temp.txt" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `path::` | [FilePath](../type.md#type-filepath) | File path to remove | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Removes file from archive structure
- File must exist in archive (error if not found)
- Does not modify filesystem

**Examples:**

**Basic usage:**
```bash
genfile .file.remove path::"temp.rs"
# Output:
# Removed file: temp.rs
```

**Exit Codes:** 0 (success) | 1 (file not found in archive) | 2 (runtime error)

**Related Commands:**
- [.file.add](#command-12-fileadd) - Add file to archive
- [.file.list](#command-14-filelist) - List files to identify removal targets

---

### Command :: 14. `.file.list`

Lists all files in current archive with metadata. Use this to inspect archive contents.

**Syntax:**
```bash
genfile .file.list
genfile .file.list verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Lists all files with paths and sizes
- Shows content mode (inline vs reference)
- Sorted alphabetically by path

**Examples:**

**Basic listing:**
```bash
genfile .file.list
# Output:
# Files (12 total, 45 KB):
#   src/main.rs (3.2 KB, inline)
#   src/lib.rs (8.1 KB, reference)
#   readme.md (2.5 KB, inline)
#   ...
```

**Exit Codes:** 0 (success) | 2 (runtime error)

**Related Commands:**
- [.content.list](content.md#command-11-contentlist) - List by content mode
- [.file.show](#command-15-fileshow) - Show file content

---

### Command :: 15. `.file.show`

Displays file content from archive. Use this to inspect file contents without materialization.

**Syntax:**
```bash
genfile .file.show path::"main.rs"
genfile .file.show path::"config.toml" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `path::` | [FilePath](../type.md#type-filepath) | File path to show | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Displays file content to stdout
- Works with both inline and reference files
- For reference files, reads from filesystem

**Examples:**

**Show file:**
```bash
genfile .file.show path::"main.rs"
# Output:
# === main.rs ===
# fn main() {
#     println!("Hello, {{project_name}}!");
# }
```

**Exit Codes:** 0 (success) | 1 (file not found) | 2 (read error)

**Related Commands:**
- [.file.list](#command-14-filelist) - List available files
- [.file.add](#command-12-fileadd) - Add files to show

---

### See Also

- [Archive Operations](archive.md) - Archive lifecycle
- [Content Management](content.md) - Content mode control
- [Parameters Reference](../param.md) - Parameter documentation

# Content Management

Content storage mode management commands for controlling inline vs reference content. Allows converting between embedded content and file references.

- **Namespace:** content
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 9 | [.content.internalize](#command-9-contentinternalize) | Convert refs to inline | 2 | Low |
| 10 | [.content.externalize](#command-10-contentexternalize) | Convert inline to refs | 3 | Medium |
| 11 | [.content.list](#command-11-contentlist) | List by mode | 2 | Low |

---

### Command :: 9. `.content.internalize`

Converts file references to inline content, embedding all file data into archive. Use this to make archive portable (self-contained).

**Syntax:**
```bash
genfile .content.internalize
genfile .content.internalize verbosity::2
genfile .content.internalize dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Reads content from referenced files
- Embeds content directly into archive
- Makes archive portable (no external dependencies)
- Increases archive size
- Requires source files to be accessible

**Examples:**

**Basic usage:**
```bash
genfile .content.internalize
# Output:
# Internalizing 8 reference files...
# Read 8 files (total: 45 KB)
# Archive now fully portable (12 inline, 0 reference)
```

**Dry run preview:**
```bash
genfile .content.internalize dry::1 verbosity::2
# Output:
# [DRY RUN] Would internalize 8 files
# [INFO] Files to read:
#   src/main.rs (3.2 KB)
#   src/lib.rs (8.1 KB)
#   ...
# [INFO] Total size: 45 KB
# [DRY RUN] No changes made
```

**Exit Codes:** 0 (success) | 1 (source files not found) | 2 (read error)

**Related Commands:**
- [.content.externalize](#command-10-contentexternalize) - Opposite operation
- [.archive.save](archive.md#command-7-archivesave) - Save portable archive

---

### Command :: 10. `.content.externalize`

Converts inline content to file references, writing content to filesystem. Use this to reduce archive size or work with external files.

**Syntax:**
```bash
genfile .content.externalize output_dir::"./templates"
genfile .content.externalize output_dir::"./src" verbosity::2
genfile .content.externalize output_dir::"./files" dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `output_dir::` | [OutputPath](../type.md#type-outputpath) | Directory to write externalized content | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Writes inline content to filesystem
- Converts archive to reference mode
- Creates directory structure as needed
- Reduces archive size
- Archive depends on external files after operation

**Examples:**

**Basic usage:**
```bash
genfile .content.externalize output_dir::"./templates"
# Output:
# Externalizing 12 inline files...
# Wrote 12 files to ./templates (total: 48 KB)
# Archive now uses references (0 inline, 12 reference)
```

**Exit Codes:** 0 (success) | 1 (invalid output directory) | 2 (write error)

**Related Commands:**
- [.content.internalize](#command-9-contentinternalize) - Opposite operation
- [.archive.save](archive.md#command-7-archivesave) - Save smaller archive

---

### Command :: 11. `.content.list`

Lists all files in archive grouped by content storage mode (inline vs reference).

**Syntax:**
```bash
genfile .content.list
genfile .content.list filter::inline
genfile .content.list verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `filter::` | [PatternString](../type.md#type-patternstring) | Filter by content type (inline or reference) | `null` | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Lists files grouped by storage mode
- Shows file counts and sizes
- Can filter to show only inline or reference files

**Examples:**

**List all:**
```bash
genfile .content.list
# Output:
# Inline (4 files, 18 KB):
#   config.toml (1.2 KB)
#   readme.md (3.5 KB)
#   ...
# Reference (8 files):
#   src/main.rs
#   src/lib.rs
#   ...
```

**Filter inline only:**
```bash
genfile .content.list filter::inline
# Output:
# Inline (4 files, 18 KB):
#   config.toml (1.2 KB)
#   readme.md (3.5 KB)
#   license.txt (0.8 KB)
#   changelog.md (12.5 KB)
```

**Exit Codes:** 0 (success) | 1 (invalid filter) | 2 (runtime error)

**Related Commands:**
- [.file.list](file.md#command-14-filelist) - List all files
- [.content.internalize](#command-9-contentinternalize) - Convert to inline
- [.content.externalize](#command-10-contentexternalize) - Convert to reference

---

### See Also

- [Archive Operations](archive.md) - Archive lifecycle management
- [Dictionary: Content Mode](../dictionary.md#content-mode) - Content mode explanation
- [Parameters Reference](../param.md) - Parameter documentation

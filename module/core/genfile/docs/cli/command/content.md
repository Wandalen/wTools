# Content Management

Content storage mode management commands for controlling inline vs reference content. Allows converting between embedded content and file references.

- **Namespace:** content
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 9 | [.content.internalize](#command--9-contentinternalize) | Convert refs to inline | 2 | 2 |
| 10 | [.content.externalize](#command--10-contentexternalize) | Convert inline to refs | 3 | 5 |
| 11 | [.content.list](#command--11-contentlist) | List by mode | 2 | 2 |

---

### Command :: 9. `.content.internalize`

### Description

Converts file references to inline content, embedding all file data directly into the archive. Use this to make an archive portable and self-contained.

-- **Parameters:** verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (source files not found) | 2 (read error)

### Syntax

```bash
genfile .content.internalize
genfile .content.internalize verbosity::2
genfile .content.internalize dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .content.internalize
# Output:
# Internalizing 8 reference files...
# Read 8 files (total: 45 KB)
# Archive now fully portable (12 inline, 0 reference)

genfile .content.internalize dry::1 verbosity::2
# Output:
# [DRY RUN] Would internalize 8 files
# [INFO] Files to read:
#   src/main.rs (3.2 KB)
#   src/lib.rs (8.1 KB)
# [INFO] Total size: 45 KB
# [DRY RUN] No changes made
```

### Notes

- Requires all referenced source files to be accessible on disk
- Increases archive size (content embedded rather than referenced)
- After internalization, archive has no external file dependencies
- Use before `.archive.save` to ensure a portable, shareable archive

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 10 | [`.content.externalize`](#command--10-contentexternalize) | Opposite: inline → reference |
| 7 | [`.archive.save`](archive.md#command--7-archivesave) | Save portable archive after internalizing |
| 6 | [`.archive.load`](archive.md#command--6-archiveload) | Load archive before converting |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Write
**Complexity:** 2
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

---

### Command :: 10. `.content.externalize`

### Description

Converts inline content to file references, writing embedded archive content out to the filesystem. Use this to reduce archive size or work with files as external editable sources.

-- **Parameters:** output_dir::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (invalid output directory) | 2 (write error)

### Syntax

```bash
genfile .content.externalize output_dir::"./templates"
genfile .content.externalize output_dir::"./src" verbosity::2
genfile .content.externalize output_dir::"./files" dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `output_dir::` | [OutputPath](../type.md#type--4-outputpath) | — | ✅ Yes | Directory to write externalized content |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .content.externalize output_dir::"./templates"
# Output:
# Externalizing 12 inline files...
# Wrote 12 files to ./templates (total: 48 KB)
# Archive now uses references (0 inline, 12 reference)

genfile .content.externalize output_dir::"./files" dry::1
# Output:
# [DRY RUN] Would externalize 12 files to ./files
# [DRY RUN] No changes made
```

### Notes

- Writes inline content to the specified output directory, preserving relative paths
- After externalization, archive depends on external files being present at those paths
- Reduces archive file size but loses portability — use `.content.internalize` to reverse
- Creates output directory and subdirectories as needed

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 9 | [`.content.internalize`](#command--9-contentinternalize) | Opposite: reference → inline |
| 7 | [`.archive.save`](archive.md#command--7-archivesave) | Save lighter archive after externalizing |
| 11 | [`.content.list`](#command--11-contentlist) | Verify content mode distribution |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Write
**Complexity:** 5
**API Requirement:** None
**Idempotent:** No
**Risk Level:** High

---

### Command :: 11. `.content.list`

### Description

Lists all files in the archive grouped by content storage mode (inline vs reference). Use this to inspect mode distribution before converting or saving.

-- **Parameters:** filter::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (runtime error)

### Syntax

```bash
genfile .content.list
genfile .content.list filter::inline
genfile .content.list verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `filter::` | [ContentMode](../type.md#type--10-contentmode) | `null` | No | Show only `inline` or `reference` files |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .content.list
# Output:
# Inline (4 files, 18 KB):
#   config.toml (1.2 KB)
#   readme.md (3.5 KB)
#   license.txt (0.8 KB)
#   changelog.md (12.5 KB)
# Reference (8 files):
#   src/main.rs
#   src/lib.rs

genfile .content.list filter::inline
# Output:
# Inline (4 files, 18 KB):
#   config.toml (1.2 KB)
#   readme.md (3.5 KB)
#   license.txt (0.8 KB)
#   changelog.md (12.5 KB)
```

### Notes

- Without `filter::`, lists all files in two groups: Inline and Reference
- `filter::inline` shows only inline files; `filter::reference` shows only reference files
- Requires a loaded archive; fails with exit code 1 if none exists

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 14 | [`.file.list`](file.md#command--14-filelist) | List all files regardless of mode |
| 9 | [`.content.internalize`](#command--9-contentinternalize) | Convert reference files to inline |
| 10 | [`.content.externalize`](#command--10-contentexternalize) | Convert inline files to reference |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Query
**Complexity:** 2
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

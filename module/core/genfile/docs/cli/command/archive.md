# Archive Operations

Archive lifecycle management commands for creating, loading, saving, and managing template archives. Provides CRUD operations on archive entities.

- **Namespace:** archive
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 5 | [.archive.new](#command--5-archivenew) | Create empty archive | 3 | 5 |
| 6 | [.archive.load](#command--6-archiveload) | Load from file | 2 | 4 |
| 7 | [.archive.save](#command--7-archivesave) | Save to file | 5 | 7 |
| 8 | [.archive.from_directory](#command--8-archivefrom_directory) | Create from filesystem | 6 | 8 |

---

### Command :: 5. `.archive.new`

### Description

Creates a new empty template archive with basic metadata. Use this when starting a new template project from scratch rather than importing existing files.

-- **Parameters:** name::, description::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (invalid name format) | 2 (runtime error)

### Syntax

```bash
genfile .archive.new name::"template-name"
genfile .archive.new name::"api-scaffold" description::"REST API template"
genfile .archive.new name::"my-template" description::"Project template" verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `name::` | [IdentifierString](../type.md#type--6-identifierstring) | — | ✅ Yes | Archive name (alphanumeric + underscore) |
| `description::` | [DescriptionText](../type.md#type--7-descriptiontext) | `""` | No | Human-readable archive description |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .archive.new name::"rust-cli-template"
# Output:
# Created archive 'rust-cli-template'
# Files: 0
# Parameters: 0

genfile .archive.new name::"web-app" description::"Full-stack web application template"
# Output:
# Created archive 'web-app'
# Description: Full-stack web application template
# Files: 0
# Parameters: 0
```

### Notes

- Archive name must be a valid identifier (alphanumeric + underscore, no spaces)
- Archive exists only in memory until `.archive.save` is called
- Does not persist to disk — use `.archive.save` afterwards
- Typical workflow: `.archive.new` → `.file.add` (multiple) → `.parameter.add` (multiple) → `.archive.save`

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 7 | [`.archive.save`](#command--7-archivesave) | Persist in-memory archive to disk |
| 8 | [`.archive.from_directory`](#command--8-archivefrom_directory) | Alternative creation from existing directory |
| 12 | [`.file.add`](file.md#command--12-fileadd) | Add files to newly created archive |
| 19 | [`.parameter.add`](param_mgmt.md#command--19-parameteradd) | Add parameters to newly created archive |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 5
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

---

### Command :: 6. `.archive.load`

### Description

Loads an existing template archive from JSON or YAML file into memory. Use this to work with previously saved archives.

-- **Parameters:** path::, verbosity::
-- **Exit Codes:** 0 (success) | 1 (file not found) | 2 (invalid archive format)

### Syntax

```bash
genfile .archive.load path::"template.json"
genfile .archive.load path::"./archives/api.yaml"
genfile .archive.load path::"backup.yaml" verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `path::` | [FilePath](../type.md#type--3-filepath) | — | ✅ Yes | Path to archive file (JSON or YAML) |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .archive.load path::"template.json"
# Output:
# Loaded archive 'my-template' from template.json
# Files: 12
# Parameters: 5

genfile .archive.load path::"backup.yaml" verbosity::2
# Output:
# [INFO] Reading archive from backup.yaml
# [INFO] Detected format: YAML
# [INFO] Loaded archive 'backup-template'
# Files: 15 (inline: 3, reference: 12)
# Parameters: 6 (mandatory: 2, optional: 4)
```

### Notes

- Auto-detects format from file extension: `.json` → JSON, `.yaml`/`.yml` → YAML
- Loading replaces the current in-memory archive without confirmation
- Validates archive structure during load — fails with exit code 2 if malformed
- For reference-mode content, source files must still exist on disk

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 7 | [`.archive.save`](#command--7-archivesave) | Save modified archive after loading |
| 1 | [`.info`](operations.md#command--1-info) | Inspect loaded archive metadata |
| 16 | [`.materialize`](operations.md#command--16-materialize) | Render loaded archive to filesystem |
| 9 | [`.content.internalize`](content.md#command--9-contentinternalize) | Convert reference content to inline after load |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |

---

**Category:** Write
**Complexity:** 4
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

---

### Command :: 7. `.archive.save`

### Description

Saves the current in-memory archive to a JSON or YAML file. Use this to persist archive changes or export in a different format.

-- **Parameters:** path::, format::, pretty::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (invalid path) | 2 (write error or permission denied)

### Syntax

```bash
genfile .archive.save path::"output.json"
genfile .archive.save path::"template.yaml" format::yaml
genfile .archive.save path::"backup.json" pretty::1 dry::1
genfile .archive.save path::"compact.json" pretty::0 verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `path::` | [OutputPath](../type.md#type--4-outputpath) | — | ✅ Yes | Output file path |
| `format::` | [SerializationFormat](../type.md#type--11-serializationformat) | `json` | No | Serialization format (`json` \| `yaml`) |
| `pretty::` | [PrettyPrintFlag](../type.md#type--14-prettyprintflag) | `1` | No | Pretty-print JSON (0 or 1) |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .archive.save path::"template.json"
# Output:
# Saved archive to template.json (JSON, 2.4 KB)

genfile .archive.save path::"template.yaml" format::yaml
# Output:
# Saved archive to template.yaml (YAML, 1.8 KB)

genfile .archive.save path::"compact.json" pretty::0
# Output:
# Saved archive to compact.json (JSON compact, 1.2 KB)

genfile .archive.save path::"test.json" dry::1 verbosity::2
# Output:
# [DRY RUN] Would save archive to test.json
# [INFO] Format: JSON (pretty-printed)
# [INFO] Estimated size: 2.4 KB
# [DRY RUN] No changes made
```

### Notes

- Format auto-detected from extension if `format::` not specified: `.json` → JSON, `.yaml`/`.yml` → YAML
- Pretty-print only affects JSON — YAML is always human-readable
- Overwrites existing file without confirmation — use `dry::1` to preview first
- Requires a loaded or created archive in memory; fails if none exists

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 6 | [`.archive.load`](#command--6-archiveload) | Load archive that was previously saved |
| 5 | [`.archive.new`](#command--5-archivenew) | Create new archive to save |
| 9 | [`.content.internalize`](content.md#command--9-contentinternalize) | Ensure portable save by inlining reference content |
| 18 | [`.pack`](operations.md#command--18-pack) | One-step alternative: directory → archive file |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Write
**Complexity:** 7
**API Requirement:** None
**Idempotent:** No
**Risk Level:** High

---

### Command :: 8. `.archive.from_directory`

### Description

Creates a template archive by scanning a filesystem directory and importing its files. Use this when converting an existing project structure into a reusable template.

-- **Parameters:** source::, mode::, recursive::, include_pattern::, exclude_pattern::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (source directory not found) | 2 (permission denied or I/O error)

### Syntax

```bash
genfile .archive.from_directory source::"./templates"
genfile .archive.from_directory source::"./src" mode::inline
genfile .archive.from_directory source::"./project" recursive::1 include_pattern::"**/*.rs"
genfile .archive.from_directory source::"./code" exclude_pattern::"**/target/**" verbosity::2
genfile .archive.from_directory source::"./project" dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `source::` | [DirectoryPath](../type.md#type--5-directorypath) | — | ✅ Yes | Source directory to scan |
| `mode::` | [ContentMode](../type.md#type--10-contentmode) | `reference` | No | Content storage mode (`inline` \| `reference`) |
| `recursive::` | [RecursiveFlag](../type.md#type--13-recursiveflag) | `1` | No | Scan subdirectories (0 or 1) |
| `include_pattern::` | [PatternString](../type.md#type--8-patternstring) | `null` | No | Include only files matching glob pattern |
| `exclude_pattern::` | [PatternString](../type.md#type--8-patternstring) | `null` | No | Exclude files matching glob pattern |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .archive.from_directory source::"./templates"
# Output:
# Scanning ./templates (recursive, reference mode)
# Added 24 files
# Archive created in memory (use .archive.save to persist)

genfile .archive.from_directory source::"./src" mode::inline
# Output:
# Scanning ./src (recursive, inline mode)
# Added 12 files (total content: 48 KB)

genfile .archive.from_directory source::"./project" include_pattern::"**/*.rs"
# Output:
# Scanning ./project (recursive, pattern: **/*.rs)
# Matched 34 files, skipped 156 files
# Added 34 files

genfile .archive.from_directory \
  source::"./src" \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**" \
  mode::inline \
  verbosity::2
# Output:
# [INFO] Scanning ./src
# [INFO] Include: **/*.{rs,toml,md} | Exclude: **/target/**
# [INFO] Mode: inline
# Matched 67 files — Added 67 files (234 KB)

genfile .archive.from_directory source::"./project" dry::1
# Output:
# [DRY RUN] Would scan ./project (recursive, reference mode)
# [DRY RUN] Would add 24 files
# [DRY RUN] No archive created
```

### Notes

- Creates archive in memory only — use `.archive.save` to persist to disk
- Recursive scan is enabled by default (`recursive::1`)
- Include and exclude patterns use `**` for any subdirectory, `*` for any filename segment
- Glob patterns are case-sensitive on Unix, case-insensitive on Windows
- Binary files can be included — use inline mode for portability

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 7 | [`.archive.save`](#command--7-archivesave) | Persist the created archive to disk |
| 18 | [`.pack`](operations.md#command--18-pack) | One-step alternative: directory → saved archive |
| 9 | [`.content.internalize`](content.md#command--9-contentinternalize) | Convert reference content to inline after creation |
| 14 | [`.file.list`](file.md#command--14-filelist) | List files in the created archive |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |
| 3 | [Filesystem Filtering](../param_group.md#group--3-filesystem-filtering) | Full | `recursive::`, `include_pattern::`, `exclude_pattern::` |

---

**Category:** Write
**Complexity:** 9
**API Requirement:** None
**Idempotent:** No
**Risk Level:** Low

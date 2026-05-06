# Archive Operations

Archive lifecycle management commands for creating, loading, saving, and managing template archives. Provides CRUD operations on archive entities.

- **Namespace:** archive
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 5 | [.archive.new](#command-5-archivenew) | Create empty archive | 3 | Low |
| 6 | [.archive.load](#command-6-archiveload) | Load from file | 2 | Low |
| 7 | [.archive.save](#command-7-archivesave) | Save to file | 5 | Medium |
| 8 | [.archive.from_directory](#command-8-archivefromdirectory) | Create from filesystem | 7 | High |

---

### Command :: 5. `.archive.new`

Creates a new empty template archive with basic metadata. Use this when starting a new template project from scratch rather than importing existing files.

**Syntax:**
```bash
genfile .archive.new name::"template-name"
genfile .archive.new name::"api-scaffold" description::"REST API template"
genfile .archive.new name::"my-template" description::"Project template" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `name::` | [IdentifierString](../type.md#type-identifierstring) | Archive name (alphanumeric + underscore) | - | ✅ Yes |
| `description::` | [DescriptionText](../type.md#type-descriptiontext) | Human-readable archive description | `""` | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Creates in-memory archive with provided metadata
- Initializes empty file list and parameter definitions
- Does NOT persist to disk (use `.archive.save` afterwards)
- Idempotent: No (creates new archive each time)

**Examples:**

**Basic usage:**
```bash
genfile .archive.new name::"rust-cli-template"
# Output:
# Created archive 'rust-cli-template'
# Files: 0
# Parameters: 0
```

**With description:**
```bash
genfile .archive.new name::"web-app" description::"Full-stack web application template"
# Output:
# Created archive 'web-app'
# Description: Full-stack web application template
# Files: 0
# Parameters: 0
```

**Verbose output:**
```bash
genfile .archive.new name::"minimal" verbosity::3
# Output (verbosity 3):
# [DEBUG] Initializing archive metadata
# [DEBUG] Archive name: minimal
# [DEBUG] Description: (empty)
# [INFO] Created archive 'minimal'
# [DEBUG] Files: 0
# [DEBUG] Parameters: 0
# [DEBUG] Archive ready for file additions
```

**Exit Codes:** 0 (success) | 1 (invalid name format) | 2 (runtime error)

**Interactions:**
- Typical workflow: `.archive.new` → `.file.add` (multiple) → `.parameter.add` (multiple) → `.archive.save`
- Alternative: Use `.archive.from_directory` instead of `.archive.new` when starting from existing files

**Notes:**
- Archive name must be valid identifier (alphanumeric + underscore, no spaces)
- Empty description is allowed (defaults to empty string)
- Archive exists only in memory until `.archive.save` is called

**Related Commands:**
- [.archive.save](#command-7-archivesave) - Persist archive to disk
- [.archive.from_directory](#command-8-archivefromdirectory) - Alternative: create from existing directory
- [.file.add](file.md#command-12-fileadd) - Add files to archive
- [.parameter.add](param_mgmt.md#command-19-parameteradd) - Add parameters to archive

---

### Command :: 6. `.archive.load`

Loads an existing template archive from JSON or YAML file into memory. Use this to work with previously saved archives.

**Syntax:**
```bash
genfile .archive.load path::"template.json"
genfile .archive.load path::"./archives/api.yaml"
genfile .archive.load path::"backup.yaml" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `path::` | [FilePath](../type.md#type-filepath) | Path to archive file (JSON or YAML) | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Auto-detects format from file extension (.json, .yaml, .yml)
- Replaces current in-memory archive (if any)
- Validates archive structure during load
- Idempotent: Yes (multiple loads produce same state)

**Examples:**

**Load JSON archive:**
```bash
genfile .archive.load path::"template.json"
# Output:
# Loaded archive 'my-template' from template.json
# Files: 12
# Parameters: 5
```

**Load YAML archive:**
```bash
genfile .archive.load path::"./archives/api.yaml"
# Output:
# Loaded archive 'rest-api-scaffold' from ./archives/api.yaml
# Files: 24
# Parameters: 8
```

**With verbose output:**
```bash
genfile .archive.load path::"backup.yaml" verbosity::2
# Output (verbosity 2):
# [INFO] Reading archive from backup.yaml
# [INFO] Detected format: YAML
# [INFO] Parsing archive structure...
# [INFO] Validating file entries: 15 files
# [INFO] Validating parameters: 6 parameters
# [INFO] Loaded archive 'backup-template'
# Files: 15 (inline: 3, reference: 12)
# Parameters: 6 (mandatory: 2, optional: 4)
```

**Exit Codes:** 0 (success) | 1 (file not found) | 2 (invalid archive format)

**Interactions:**
- Typical workflow: `.archive.load` → `.value.set` (multiple) → `.materialize`
- Alternative workflow: `.archive.load` → modify archive → `.archive.save`

**Notes:**
- File must exist and be readable
- Format auto-detection uses extension (.json → JSON, .yaml/.yml → YAML)
- Loading replaces current archive without warning (no confirmation prompt)
- For reference-mode content, source files must still exist

**Related Commands:**
- [.archive.save](#command-7-archivesave) - Save modified archive
- [.materialize](operations.md#command-16-materialize) - Render loaded archive
- [.info](operations.md#command-1-info) - Inspect loaded archive
- [.content.internalize](content.md#command-9-contentinternalize) - Convert references to inline

---

### Command :: 7. `.archive.save`

Saves current in-memory archive to JSON or YAML file. Use this to persist archive changes or export in different formats.

**Syntax:**
```bash
genfile .archive.save path::"output.json"
genfile .archive.save path::"template.yaml" format::yaml
genfile .archive.save path::"backup.json" pretty::1 dry::1
genfile .archive.save path::"compact.json" pretty::0 verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `path::` | [OutputPath](../type.md#type-outputpath) | Output file path | - | ✅ Yes |
| `format::` | [SerializationFormat](../type.md#type-serializationformat) | Serialization format (json \| yaml) | `json` | No |
| `pretty::` | [PrettyPrintFlag](../type.md#type-prettyprintflag) | Pretty-print JSON (0 or 1) | `1` | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Auto-detects format from extension if `format::` not specified
- Creates parent directories if needed
- Overwrites existing file without confirmation
- Idempotent: No (writes to disk each time)

**Examples:**

**Save to JSON (default format):**
```bash
genfile .archive.save path::"template.json"
# Output:
# Saved archive to template.json (JSON, 2.4 KB)
```

**Save to YAML:**
```bash
genfile .archive.save path::"template.yaml" format::yaml
# Output:
# Saved archive to template.yaml (YAML, 1.8 KB)
```

**Compact JSON (no pretty-print):**
```bash
genfile .archive.save path::"compact.json" pretty::0
# Output:
# Saved archive to compact.json (JSON compact, 1.2 KB)
```

**Dry run preview:**
```bash
genfile .archive.save path::"test.json" dry::1 verbosity::2
# Output (dry run):
# [DRY RUN] Would save archive to test.json
# [INFO] Format: JSON (pretty-printed)
# [INFO] Estimated size: 2.4 KB
# [INFO] Files to save: 12 (inline: 4, reference: 8)
# [INFO] Parameters to save: 5
# [DRY RUN] No changes made
```

**Exit Codes:** 0 (success) | 1 (invalid path) | 2 (write error, disk full, permissions)

**Interactions:**
- Conflicts with: None
- Enhances: Combine with `dry::1` to preview before actual save
- Dependencies: Requires loaded or created archive in memory

**Notes:**
- Pretty-print only affects JSON (YAML is always formatted)
- Format auto-detection: `.json` → JSON, `.yaml`/`.yml` → YAML
- Overwrites existing files without warning (use `dry::1` to preview)
- For reference-mode content, only paths are saved (not file contents)

**Related Commands:**
- [.archive.load](#command-6-archiveload) - Load saved archive
- [.archive.new](#command-5-archivenew) - Create archive to save
- [.content.internalize](content.md#command-9-contentinternalize) - Ensure portable save
- [.pack](operations.md#command-18-pack) - Alternative: one-step directory → archive file

---

### Command :: 8. `.archive.from_directory`

Creates template archive by scanning filesystem directory and importing files. Use this when converting existing project structure into reusable template.

**Syntax:**
```bash
genfile .archive.from_directory source::"./templates"
genfile .archive.from_directory source::"./src" mode::inline
genfile .archive.from_directory source::"./project" recursive::1 include_pattern::"**/*.rs"
genfile .archive.from_directory source::"./code" exclude_pattern::"**/target/**" verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `source::` | [DirectoryPath](../type.md#type-directorypath) | Source directory to scan | - | ✅ Yes |
| `mode::` | [ContentMode](../type.md#type-contentmode) | Content mode (inline \| reference) | `reference` | No |
| `recursive::` | [RecursiveFlag](../type.md#type-recursiveflag) | Scan subdirectories (0 or 1) | `1` | No |
| `include_pattern::` | [PatternString](../type.md#type-patternstring) | Include files matching glob pattern | `null` | No |
| `exclude_pattern::` | [PatternString](../type.md#type-patternstring) | Exclude files matching glob pattern | `null` | No |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`recursive::`, `include_pattern::`, `exclude_pattern::` are part of [Filesystem Filtering](../param_group.md#group-3-filesystem-filtering) parameter group*</small>
<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Scans source directory and adds all matching files to archive
- Creates archive in memory (not persisted until `.archive.save`)
- Mode determines content storage: inline (embedded) or reference (file paths)
- Recursive scan is enabled by default
- Glob patterns use `**` for any subdirectory, `*` for any file

**Examples:**

**Basic usage (reference mode, recursive):**
```bash
genfile .archive.from_directory source::"./templates"
# Output:
# Scanning ./templates (recursive, reference mode)
# Added 24 files
# Archive created in memory (use .archive.save to persist)
```

**Inline mode (portable archive):**
```bash
genfile .archive.from_directory source::"./src" mode::inline
# Output:
# Scanning ./src (recursive, inline mode)
# Reading file contents...
# Added 12 files (total content: 48 KB)
# Archive created in memory (use .archive.save to persist)
```

**Include only Rust files:**
```bash
genfile .archive.from_directory source::"./project" include_pattern::"**/*.rs"
# Output:
# Scanning ./project (recursive, pattern: **/*.rs)
# Matched 34 files
# Skipped 156 files (pattern mismatch)
# Added 34 files
```

**Exclude build artifacts:**
```bash
genfile .archive.from_directory source::"./workspace" exclude_pattern::"**/target/**"
# Output:
# Scanning ./workspace (recursive, exclude: **/target/**)
# Matched 89 files
# Excluded 2,341 files (target directories)
# Added 89 files
```

**Complex filtering:**
```bash
genfile .archive.from_directory \
  source::"./src" \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**" \
  mode::inline \
  verbosity::2
# Output:
# [INFO] Scanning ./src
# [INFO] Include pattern: **/*.{rs,toml,md}
# [INFO] Exclude pattern: **/target/**
# [INFO] Content mode: inline
# Matched 67 files
# Excluded 1,234 files (target + pattern mismatch)
# Reading file contents...
# Added 67 files (total content: 234 KB)
```

**Exit Codes:** 0 (success) | 1 (source directory not found) | 2 (permission denied, I/O error)

**Interactions:**
- Conflicts with: None
- Enhances: Combine `include_pattern::` and `exclude_pattern::` for precise filtering
- Dependencies: Source directory must exist and be readable

**Notes:**
- Glob patterns are case-sensitive on Unix, case-insensitive on Windows
- Recursive scan follows symlinks (be careful with circular links)
- Empty directories are not added to archive (only files)
- Binary files can be included (use inline mode for portability)
- Very large files in inline mode can create huge archives

**Related Commands:**
- [.archive.save](#command-7-archivesave) - Persist created archive
- [.pack](operations.md#command-18-pack) - Alternative: one-step directory → archive file
- [.content.internalize](content.md#command-9-contentinternalize) - Convert reference → inline later
- [.file.list](file.md#command-14-filelist) - List files in created archive

---

### Common Workflows

**Create archive from scratch:**
```bash
# 1. Create empty archive
genfile .archive.new name::"my-template"

# 2. Add files
genfile .file.add path::"main.rs" from_file::"src/main.rs"
genfile .file.add path::"readme.md" content::"# {{project_name}}"

# 3. Add parameters
genfile .parameter.add name::project_name mandatory::true

# 4. Save
genfile .archive.save path::"template.yaml"
```

**Convert directory to archive:**
```bash
# One-step: use .pack
genfile .pack input::"./templates" output::"template.json"

# OR two-step: use .archive.from_directory + .archive.save
genfile .archive.from_directory source::"./templates" mode::inline
genfile .archive.save path::"template.json" pretty::1
```

**Load and modify archive:**
```bash
# Load existing
genfile .archive.load path::"template.yaml"

# Modify (add files, parameters, etc.)
genfile .file.add path::"new-file.rs" content::"// new file"

# Save changes
genfile .archive.save path::"template-v2.yaml"
```

### See Also

- [Content Management](content.md) - Control inline vs reference content
- [File Operations](file.md) - Manage individual files
- [Operations](operations.md) - Materialize, pack, analyze archives
- [Parameters Reference](../param.md) - Complete parameter documentation

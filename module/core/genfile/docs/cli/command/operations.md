# Core Operations

Core operational commands for materialization, analysis, and utility functions. These commands perform primary archive operations that don't fit entity-specific namespaces.

- **Namespace:** operations (mixed)
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 1 | [.info](#command--1-info) | Show archive metadata | 1 | 1 |
| 2 | [.discover.parameters](#command--2-discoverparameters) | Auto-detect params | 2 | 2 |
| 3 | [.status](#command--3-status) | Readiness check | 1 | 1 |
| 4 | [.analyze](#command--4-analyze) | Analyze structure | 2 | 2 |
| 16 | [.materialize](#command--16-materialize) | Render templates | 3 | 5 |
| 17 | [.unpack](#command--17-unpack) | Unpack raw files | 3 | 5 |
| 18 | [.pack](#command--18-pack) | Pack to archive | 4 | 8 |

---

### Command :: 1. `.info`

### Description

Displays archive metadata and statistics. Use this to inspect the current in-memory archive state.

-- **Parameters:** verbosity::
-- **Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (runtime error)

### Syntax

```bash
genfile .info
genfile .info verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .info
# Output:
# Archive: my-template
# Description: Rust CLI template
# Files: 12 (inline: 4, reference: 8)
# Parameters: 3 (mandatory: 1, optional: 2)

genfile .info verbosity::2
# Includes content mode breakdown and parameter status
```

### Notes

- Requires an archive loaded or created in memory; fails with exit code 1 if none exists
- Shows content mode distribution (inline vs reference counts)

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 3 | [`.status`](#command--3-status) | Deeper readiness check with missing-value report |
| 4 | [`.analyze`](#command--4-analyze) | Detailed structural analysis with recommendations |

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

### Command :: 2. `.discover.parameters`

### Description

Auto-detects template parameters in archive files by scanning for `{{param_name}}` placeholder syntax. Adds discovered parameters as optional definitions without defaults.

-- **Parameters:** verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (scan error)

### Syntax

```bash
genfile .discover.parameters
genfile .discover.parameters verbosity::2
genfile .discover.parameters dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .discover.parameters
# Output:
# Scanning 12 files for parameters...
# Discovered 5 parameters:
#   project_name (found in 3 files)
#   version (found in 2 files)
#   author (found in 1 file)
# Added 5 parameter definitions

genfile .discover.parameters dry::1 verbosity::2
# Preview: shows discovered params without adding them
```

### Notes

- Scans all file content for `{{parameter}}` placeholders (double-brace syntax)
- Skips parameters already defined in the archive
- Newly added parameters are non-mandatory with no defaults — review and adjust with `.parameter.add`

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 19 | [`.parameter.add`](param_mgmt.md#command--19-parameteradd) | Manual parameter addition with full metadata |
| 20 | [`.parameter.list`](param_mgmt.md#command--20-parameterlist) | List parameters after discovery |

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

### Command :: 3. `.status`

### Description

Shows archive readiness and completeness status. Use this before `.materialize` to verify all mandatory parameter values are set.

-- **Parameters:** verbosity::
-- **Exit Codes:** 0 (archive ready) | 1 (not ready — missing mandatory values) | 2 (runtime error)

### Syntax

```bash
genfile .status
genfile .status verbosity::2
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |

### Examples

```bash
genfile .status
# Output (ready):
# Archive Status: READY
# Files: 12
# Parameters: 3 (all mandatory values set)
# Ready for materialization

genfile .status
# Output (not ready):
# Archive Status: NOT READY
# Missing mandatory parameter values:
#   - project_name
# Set missing values before materialization
```

### Notes

- Exit code 1 means not ready — use this in scripts to gate `.materialize`
- Use `verbosity::2` for the full breakdown of which files reference which parameters

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 22 | [`.value.set`](value.md#command--22-valueset) | Set missing mandatory values |
| 1 | [`.info`](#command--1-info) | General archive info |
| 16 | [`.materialize`](#command--16-materialize) | Run after status reports ready |

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

### Command :: 4. `.analyze`

### Description

Performs comprehensive archive analysis including file structure, parameter usage, and unused-parameter detection. Use this for detailed archive inspection and quality validation.

-- **Parameters:** verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (analysis error)

### Syntax

```bash
genfile .analyze
genfile .analyze verbosity::2
genfile .analyze dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .analyze
# Output:
# Archive Analysis:
# Files: 12 total (inline: 4, reference: 8)
# Parameters: 3 defined
#   - Used in templates: 2
#   - Unused: 1 (port)
# Recommendations:
#   - Remove unused parameter: port

genfile .analyze verbosity::2
# Per-file parameter usage breakdown
```

### Notes

- Detects unused parameters (defined but not referenced in any file)
- Identifies undefined placeholders (found in files but not defined as parameters)

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 1 | [`.info`](#command--1-info) | Basic archive info |
| 2 | [`.discover.parameters`](#command--2-discoverparameters) | Auto-detect undefined parameters |
| 21 | [`.parameter.remove`](param_mgmt.md#command--21-parameterremove) | Remove unused parameters |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Query
**Complexity:** 2
**API Requirement:** None
**Idempotent:** Yes
**Risk Level:** Low

---

### Command :: 16. `.materialize`

### Description

Renders template archive to destination directory with full parameter substitution. All `{{placeholder}}` tokens in template files are replaced with their assigned values.

-- **Parameters:** destination::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (missing mandatory values) | 2 (write error or permission denied)

### Syntax

```bash
genfile .materialize destination::"./output"
genfile .materialize destination::"./my-project" verbosity::2
genfile .materialize destination::"./preview" dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `destination::` | [OutputPath](../type.md#type--4-outputpath) | — | ✅ Yes | Output directory for materialized files |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .materialize destination::"./output"
# Output:
# Materializing 12 files to ./output...
# Created: src/main.rs
# Created: src/lib.rs
# Materialized 12 files successfully

genfile .materialize destination::"./preview" dry::1 verbosity::2
# Output:
# [DRY RUN] Would materialize to ./preview
# [INFO] Files to create: src/main.rs, src/lib.rs, ...
# [INFO] Substitutions: project_name -> "my-app", version -> "1.0.0"
# [DRY RUN] No files created
```

### Notes

- All mandatory parameters must have values before `.materialize` succeeds — use `.status` first
- Destination directory is created if it does not exist; parent must be writable
- Existing files in destination are overwritten without confirmation

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 3 | [`.status`](#command--3-status) | Verify readiness before materializing |
| 17 | [`.unpack`](#command--17-unpack) | Extract raw templates without substitution |
| 22 | [`.value.set`](value.md#command--22-valueset) | Set parameter values before materializing |
| 6 | [`.archive.load`](archive.md#command--6-archiveload) | Load archive before materializing |

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

### Command :: 17. `.unpack`

### Description

Unpacks raw template files to destination directory without parameter substitution. Preserves `{{placeholder}}` tokens as-is. Use this to extract template source for inspection or editing.

-- **Parameters:** destination::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (destination error) | 2 (write error)

### Syntax

```bash
genfile .unpack destination::"./template-files"
genfile .unpack destination::"./output" verbosity::2
genfile .unpack destination::"./preview" dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `destination::` | [OutputPath](../type.md#type--4-outputpath) | — | ✅ Yes | Output directory for unpacked files |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .unpack destination::"./templates"
# Output:
# Unpacking 12 files to ./templates...
# Extracted: src/main.rs (with placeholders)
# Extracted: src/lib.rs (with placeholders)
# Unpacked 12 files successfully

genfile .unpack destination::"./templates" dry::1
# Preview without writing
```

### Notes

- Does not require parameter values — outputs files verbatim with placeholders intact
- Useful for inspecting or editing template files before creating a new archive version

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 16 | [`.materialize`](#command--16-materialize) | Render with parameter substitution |
| 7 | [`.archive.save`](archive.md#command--7-archivesave) | Save archive without extracting |

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

### Command :: 18. `.pack`

### Description

Creates a portable self-contained archive from a directory in one step, embedding all file content inline. Shortcut for `.archive.from_directory mode::inline` + `.archive.save`.

-- **Parameters:** input::, output::, verbosity::, dry::
-- **Exit Codes:** 0 (success) | 1 (input directory not found) | 2 (write error)

### Syntax

```bash
genfile .pack input::"./templates" output::"template.json"
genfile .pack input::"./src" output::"backup.yaml" verbosity::2
genfile .pack input::"./code" output::"archive.json" dry::1
```

### Parameters

| Parameter | Type | Default | Required | Purpose |
|-----------|------|---------|----------|---------|
| `input::` | [DirectoryPath](../type.md#type--5-directorypath) | — | ✅ Yes | Source directory to pack |
| `output::` | [OutputPath](../type.md#type--4-outputpath) | — | ✅ Yes | Output file path (JSON or YAML) |
| `verbosity::` | [VerbosityLevel](../type.md#type--1-verbositylevel) | `1` | No | Output detail level (0-5) |
| `dry::` | [DryRunFlag](../type.md#type--2-dryrunflag) | `0` | No | Preview mode (0 or 1) |

### Examples

```bash
genfile .pack input::"./templates" output::"template.json"
# Output:
# Scanning ./templates...
# Found 24 files
# Reading file contents...
# Saved to template.json (125 KB)

genfile .pack input::"./src" output::"src.yaml" dry::1 verbosity::2
# Preview without writing
```

### Notes

- Always uses inline mode — all file contents are embedded in the output archive
- Output format is auto-detected from extension: `.json` → JSON, `.yaml`/`.yml` → YAML
- Scans recursively with no filtering — use `.archive.from_directory` for include/exclude patterns

### Related Commands

| # | Command | Relationship |
|---|---------|-------------|
| 8 | [`.archive.from_directory`](archive.md#command--8-archivefrom_directory) | Two-step version with filtering support |
| 7 | [`.archive.save`](archive.md#command--7-archivesave) | Two-step version: save step |

### Referenced Parameter Groups

| # | Group | Membership | Parameters Bound |
|---|-------|------------|-----------------|
| 1 | [Universal Output Control](../param_group.md#group--1-universal-output-control) | Full | `verbosity::` |
| 2 | [Universal Execution Control](../param_group.md#group--2-universal-execution-control) | Full | `dry::` |

---

**Category:** Write
**Complexity:** 8
**API Requirement:** None
**Idempotent:** No
**Risk Level:** High
